# bitcoinleveldb-versionedit

Low-level encoding/decoding and manipulation of LevelDB `VersionEdit` records, extracted from the `bitcoin-rs` project. This crate provides a faithful, byte-for-byte compatible Rust implementation of LevelDB's manifest version-edit logic as used by Bitcoin-like workloads.

---

## Overview

LevelDB stores the evolution of its on-disk state (files per level, sequence-number metadata, compaction pointers, etc.) in a manifest file. Each record in the manifest is a **VersionEdit**: a compact, varint-encoded description of mutations to the logical database version.

This crate implements:

- A `VersionEdit` struct mirroring LevelDB's internal representation
- Deterministic encoding of `VersionEdit` into the manifest binary format
- Robust decoding from manifest records back into a `VersionEdit`
- Convenience APIs to:
  - Track added files (per level)
  - Track deleted files (per level)
  - Maintain compaction pointers
  - Maintain log / sequence-number bookkeeping
  - Derive human-readable debug summaries

It is designed to be interoperable with existing LevelDB/Bitcoin data, focusing on correctness of serialization and deterministic ordering.

This crate is **not** a full LevelDB implementation; it is targeted infrastructure for higher-level components (like `VersionSet` and the full storage engine) in `bitcoin-rs`.

---

## Features

- **Binary compatibility with LevelDB manifest format**
  - Uses varint32/varint64 and length-prefixed slices to match LevelDB's on-disk representation
  - Tags and field semantics match LevelDB's `VersionEdit`:
    - `kComparator` (tag 1)
    - `kLogNumber` (tag 2)
    - `kNextFileNumber` (tag 3)
    - `kLastSequence` (tag 4)
    - `kCompactPointer` (tag 5)
    - `kDeletedFile` (tag 6)
    - `kNewFile` (tag 7)
    - `kPrevLogNumber` (tag 9)

- **Deterministic encoding**
  - Deleted-file entries are sorted by `(level, file_number)` prior to encoding, guaranteeing that
    `encode_to -> decode_from -> encode_to` produces **bit-identical** manifest bytes.

- **Convenient high-level mutation API**
  - `add_file(level, file, size, smallest, largest)`
  - `delete_file(level, file)`
  - `set_comparator_name`, `set_log_number`, `set_prev_log_number`, `set_next_file`, `set_last_sequence`
  - `set_compact_pointer(level, key)`

- **Introspectable**
  - `debug_string()` yields a multi-line, human-readable summary suitable for logging and debugging, including all scalar fields, compaction pointers, deletions, and new files.

- **Safe defaults & state reset**
  - `Default` constructs an empty, "no-op" `VersionEdit` with all `has_` flags cleared.
  - `clear()`/`reset_core_state()` allow reuse of a `VersionEdit` while preserving compaction pointers if desired.

---

## Crate Status

- **License:** MIT
- **Edition:** Rust 2021
- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **Intended users:** implementers of LevelDB-compatible storage layers, Bitcoin node developers, and systems programmers requiring exact reproduction of LevelDB manifest semantics.

---

## Core Data Structures

### `VersionEdit`

```rust
pub struct VersionEdit  {
    comparator:           String,
    log_number:           u64,
    prev_log_number:      u64,
    next_file_number:     u64,
    last_sequence:        SequenceNumber,
    has_comparator:       bool,
    has_log_number:       bool,
    has_prev_log_number:  bool,
    has_next_file_number: bool,
    has_last_sequence:    bool,
    compact_pointers:     Vec<(i32, InternalKey)>,
    deleted_files:        VersionEditDeletedFileSet,
    new_files:            Vec<(i32, FileMetaData)>,
}

pub type VersionEditDeletedFileSet = HashSet<(i32, u64)>;
```

Conceptually, a `VersionEdit` is a sparse patch to the current logical version:

- **Scalar metadata**
  - `comparator`: name of the key comparator
  - `log_number`: current log file number
  - `prev_log_number`: previous log file number
  - `next_file_number`: global file-number allocator watermark
  - `last_sequence`: maximal sequence number visible after applying this edit
  - `has_*` flags: which of the above are present in this edit

- **Collections**
  - `compact_pointers: Vec<(level, InternalKey)>`
  - `deleted_files: HashSet<(level, file_number)>`
  - `new_files: Vec<(level, FileMetaData)>`

These mutate the file layout per compaction level.

### Helper functions

These implement the manifest's binary protocol for specific logical units:

```rust
pub fn get_level(input: &mut Slice, level: &mut i32) -> bool { ... }

pub fn get_internal_key(input: &mut Slice, key: &mut InternalKey) -> bool { ... }
```

- `get_level` reads a LevelDB level (0..N) from a varint32-encoded field.
- `get_internal_key` reads a length-prefixed slice and decodes it into an `InternalKey`.

---

## Encoding & Decoding Semantics

### Encoding: `VersionEdit::encode_to`

```rust
impl VersionEdit {
    pub fn encode_to(&self, dst: *mut String) { ... }
}
```

- Accepts a raw pointer to an owned `String` that serves as a byte buffer.
- Serializes the `VersionEdit` fields into the LevelDB manifest wire format:
  - Scalars are emitted only if the corresponding `has_*` flag is true.
  - `compact_pointers`, `deleted_files`, and `new_files` are written sequentially.
- `deleted_files` are pre-sorted:

```rust
let mut deleted_files_sorted: Vec<(i32, u64)> =
    self.deleted_files().iter().copied().collect();
deleted_files_sorted.sort_unstable();
```

This guarantees deterministic encoding irrespective of the internal `HashSet` iteration order.

**Safety model:**
- The method uses `unsafe` for the raw pointer; you must ensure:
  - `dst` is non-null and points to a valid `String`
  - The `String` outlives the call

A higher-level wrapper can be constructed to hide the raw pointer, e.g. by allocating and passing `&mut String` and then casting internally.

### Decoding: `VersionEdit::decode_from`

```rust
impl VersionEdit {
    pub fn decode_from(&mut self, src: &Slice) -> Status { ... }
}
```

- Resets the core scalar state and file collections before decoding.
- Consumes a copy of the input `Slice` and incrementally parses tagged fields.
- Each tag is matched against the LevelDB tag set; unknown or malformed tags result in a `Status::corruption` with contextual diagnostics.
- Parsed values are routed through the higher-level mutation functions (`set_*`, `add_file`, `delete_file`, `set_compact_pointer`).

The loop structure is essentially:

```rust
while msg.is_none() && get_varint32(&mut input, &mut tag) {
    match tag {
        1 => { /* comparator */ }
        2 => { /* log number */ }
        3 => { /* next file number */ }
        4 => { /* last sequence */ }
        5 => { /* compact pointer */ }
        6 => { /* deleted file */ }
        7 => { /* new file */ }
        9 => { /* prev log number */ }
        _ => { msg = Some("unknown tag"); }
    }
}
```

Post-conditions:
- On success: returns `Status::ok()` and a fully-populated `VersionEdit`.
- On failure: returns a corruption `Status` indicating the failing component, and leaves the `VersionEdit` in a reset state (partial mutations are not guaranteed useful).

---

## Public API Usage

### Constructing a basic `VersionEdit`

```rust
use bitcoinleveldb_versionedit::VersionEdit;
use bitcoinleveldb_types::{InternalKey, SequenceNumber};

fn build_simple_edit() -> VersionEdit {
    let mut edit = VersionEdit::default();

    // set comparator name
    let cmp_name = Slice::from("leveldb.BytewiseComparator".as_bytes());
    edit.set_comparator_name(&cmp_name);

    // log / sequence metadata
    edit.set_log_number(42);
    edit.set_prev_log_number(41);
    edit.set_next_file(1000);
    edit.set_last_sequence(123_456 as SequenceNumber);

    edit
}
```

### Adding a new file

```rust
fn add_new_sstable(
    edit: &mut VersionEdit,
    level: i32,
    file_number: u64,
    file_size: u64,
    smallest: &InternalKey,
    largest: &InternalKey,
) {
    edit.add_file(level, file_number, file_size, smallest, largest);
}
```

**Preconditions (mirroring LevelDB's invariants):**
- `smallest` and `largest` must be the true extremal internal keys in the file.
- The file must not have been persisted to the VersionSet yet (`VersionSet::SaveTo()` expectation).

### Deleting a file

```rust
fn mark_file_deleted(edit: &mut VersionEdit, level: i32, file_number: u64) {
    edit.delete_file(level, file_number);
}
```

Internally, this records `(level, file_number)` in `deleted_files`, which will be serialized as one or more `kDeletedFile` entries.

### Compaction pointers

```rust
fn update_compaction_pointer(
    edit: &mut VersionEdit,
    level: i32,
    key: &InternalKey,
) {
    edit.set_compact_pointer(level, key);
}
```

This denotes the logical *resume key* for future compactions at that level.

### Debugging

```rust
fn log_version_edit(edit: &VersionEdit) {
    println!("{}", edit.debug_string());
}
```

Example output:

```text
VersionEdit {
  Comparator: leveldb.BytewiseComparator
  LogNumber: 42
  PrevLogNumber: 41
  NextFile: 1000
  LastSeq: 123456
  CompactPointer: 1 userkey1@123
  DeleteFile: 2 57
  AddFile: 1 1001 1048576 smallest_key .. largest_key
}
```

### Clearing and reusing a `VersionEdit`

```rust
fn reuse_edit(edit: &mut VersionEdit) {
    // Reset scalar state and file collections; compact_pointers remain.
    edit.clear();

    // Now you can repopulate it with new metadata and file deltas.
}
```

`clear()` simply delegates to `reset_core_state()`, which zeroes scalars, clears `deleted_files` and `new_files`, and resets `has_*` flags.

---

## Binary Format Details

This crate encodes/decodes the same schema as canonical LevelDB:

- **Tags** are varint32-encoded integers.
- **Levels** are varint32-encoded unsigned integers, cast to `i32` in-memory.
- **File numbers and sizes** use varint64.
- **Internal keys** are serialized as a length-prefixed slice (`len` as varint32, followed by bytes) and then decoded via `InternalKey::decode_from`.
- **Comparator name** is also a length-prefixed slice of UTF-8 bytes.

The serialization order is purely determined by the order of fields in the `VersionEdit` and the order of `compact_pointers` and `new_files` vectors, except for `deleted_files`, which are explicitly sorted, providing deterministic binary output.

This determinism is critical when one wants to ensure that two logically identical `VersionEdit`s result in the same manifest bytes, which facilitates:

- Reproducible tests
- Content-addressable storage and hashing
- Stable replication and snapshot mechanics across nodes

---

## Relationship to LevelDB and Bitcoin

In LevelDB (and by inheritance, Bitcoin Core's database layout), `VersionEdit` is the backbone for describing structural mutations in the set of SSTables. Bitcoin stores UTXO and block index information in LevelDB-style databases; exact adherence to manifest semantics is mandatory if you need to:

- Read or write existing Bitcoin Core databases
- Implement alternative nodes that share storage layouts
- Perform analysis or replay of historical LevelDB states from archived manifests

This crate intentionally mirrors the C++ LevelDB logic, with additional Rust idioms (e.g., `Default`, strong typing around `Status`, and improved logging).

---

## Safety & Concurrency Considerations

- The core APIs are `&mut self` and therefore **not** thread-safe by themselves; wrap in synchronization primitives (`Mutex`, etc.) if accessed concurrently.
- `encode_to` uses a raw pointer. Incorrect usage can lead to undefined behavior. If you design higher-level APIs on top of this crate, you are encouraged to encapsulate this unsafety in a small, well-tested layer that exposes only safe abstractions.
- `decode_from` trusts the `Slice` size; it validates structure but not cryptographic authenticity. For untrusted input, pair it with higher-level validation or checksums.

---

## When to Use This Crate

Use this crate if you need:

- Precise, LevelDB-compatible manifest handling in Rust
- To interoperate with Bitcoin or other LevelDB-based systems at the storage format level
- Deterministic, testable `VersionEdit` encoding/decoding

This crate is probably **too low-level** if you only need a high-level key-value database abstraction; in that case, integrate through whatever higher-layer `VersionSet` or storage API `bitcoin-rs` exposes.

---

## License

This crate is distributed under the **MIT** license, consistent with the `bitcoin-rs` repository.

---

## Provenance

This crate is part of the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) repository and focuses exclusively on the `VersionEdit` component of a LevelDB-compatible storage engine.
