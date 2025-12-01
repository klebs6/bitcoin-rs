# bitcoinleveldb-logreader

A low-level, LevelDB-compatible log reader for Rust, tailored for Bitcoin Core–style environments and other append-only WAL/redo-log use cases.

This crate provides a faithful, allocation-conscious reimplementation of the LevelDB log format reader, including fragmentation handling, checksumming, corruption reporting, and initial-offset based resynchronization.

---

## Design Overview

`LogReader` is designed to read *logical* records out of an *append-only, block-structured* log file that uses the LevelDB log format. In this format, each logical record is represented as one or more *physical fragments* stored in fixed-size blocks (typically 32 KiB):

- Each fragment has a header and a payload
- Fragment headers include CRC32C checksums, length, and type
- Fragment types: `Full`, `First`, `Middle`, `Last`
- A logical record is either a single `Full` fragment or a `First` + zero or more `Middle` + `Last` fragments

`LogReader` wraps a `SequentialFile` abstraction to read from the underlying file descriptor/handle, reconstructs fragmented records, and validates them (optionally with checksums). It also tracks *physical offsets* and an *initial logical offset* for resuming from arbitrary positions.

### Key Properties

- **Streaming, forward-only**: reads sequentially; no seeking back
- **Fragment reassembly**: handles `First`/`Middle`/`Last` fragments into full logical records
- **Corruption-aware**:
  - CRC32C-based checksum validation (optional)
  - Dropped bytes and malformed records are logged via a `LogReaderReporter`
  - Partial ending records at EOF are tolerated without being reported as corruption
- **Offset-aware**:
  - `initial_offset` to start from an arbitrary physical position
  - Resynchronization logic skips partial fragments until a clean record boundary is found
- **Low-level, pointer-based**: uses a pre-allocated backing buffer, raw pointers, and a `Slice` abstraction for minimizing allocations and copies

The behavior is closely aligned with LevelDB’s original C++ `log::Reader`, which is particularly relevant when interoperating with Bitcoin Core data directories or other systems using the same log encoding.

---

## Crate Layout and Core Types

### `LogReaderReporter`

```rust
/// Interface for reporting errors.
pub trait LogReaderReporter {
    fn corruption(&mut self, bytes: usize, status: &Status);
}
```

Implement this trait to hook into corruption reporting. `bytes` is the number of bytes dropped from the log, and `status` encodes a `corruption(...)` `Status` created by the reader. Typical implementations log, meter, or abort on corruption.

### `LogReader`

```rust
#[derive(Builder, Setters, Getters, MutGetters)]
pub struct LogReader  {
    file:                 Box<dyn SequentialFile>,
    reporter:             Box<dyn LogReaderReporter>,
    checksum:             bool,
    backing_store:        *const u8,
    buffer:               Slice,
    eof:                  bool,
    last_record_offset:   u64,
    end_of_buffer_offset: u64,
    initial_offset:       u64,
    resyncing:            bool,
}
```

The primary entry point is the constructor and `read_record`:

```rust
impl LogReader {
    pub fn new(
        file:           Box<dyn SequentialFile>,
        reporter:       Box<dyn LogReaderReporter>,
        checksum:       bool,
        initial_offset: u64,
    ) -> Self { /* ... */ }

    /// Read the next logical record into `record`.
    /// Returns `true` on success, `false` on EOF.
    pub fn read_record(&mut self, record: &mut Slice, scratch: &mut Vec<u8>) -> bool { /* ... */ }
}
```

#### Internal Concepts

- **`SequentialFile`**: Abstraction of a forward-only file. You must provide a concrete implementation that supports `read(block_size, result_ptr, scratch_ptr)` and `skip(n)` semantics. This is often backed by POSIX or OS-specific APIs.
- **`Slice`**: Thin, pointer-based view over a region of memory (data pointer + length). Both incoming file data and exposed record payloads are represented as `Slice`s.
- **Backing store**: `LogReader` owns a fixed-sized backing buffer (`LOG_BLOCK_SIZE`) allocated once in `new()`. `read_into_buffer_from_file` refills `buffer` by reading into this backing store.
- **Offsets**:
  - `end_of_buffer_offset` tracks the physical offset *just past* the last byte read from the file
  - `last_record_offset` tracks the physical offset of the last successfully delivered logical record
  - `initial_offset` is the target physical offset to start delivering records from

---

## Usage

Below is a conceptual usage sketch. Types like `SequentialFile`, `Slice`, `Status`, and enum types such as `LogRecordType` and `ExtendedRecordTypes` are provided by this crate’s ecosystem (e.g. a Bitcoin LevelDB compatibility layer) and are not reproduced here.

```rust
use bitcoinleveldb_logreader::{LogReader, LogReaderReporter};
use bitcoinleveldb_env::PosixSequentialFile; // example
use bitcoinleveldb_types::{Slice, Status};

struct MyReporter;

impl LogReaderReporter for MyReporter {
    fn corruption(&mut self, bytes: usize, status: &Status) {
        eprintln!("dropped {} bytes due to corruption: {:?}", bytes, status);
        // choose your own policy: log, metrics, panic, etc.
    }
}

fn read_all_records(path: &str) -> Result<(), Status> {
    // Build a SequentialFile (implementation-specific)
    let file: Box<dyn SequentialFile> = Box::new(PosixSequentialFile::open(path)?);

    let reporter: Box<dyn LogReaderReporter> = Box::new(MyReporter);
    let checksum = true;        // enable CRC32C verification
    let initial_offset = 0u64;  // start from beginning of file

    let mut reader = LogReader::new(file, reporter, checksum, initial_offset);

    let mut record = Slice::default();
    let mut scratch = Vec::new();

    while reader.read_record(&mut record, &mut scratch) {
        // `record` is valid only until the next reader mutation or scratch change
        let bytes = unsafe { std::slice::from_raw_parts(*record.data(), *record.size()) };
        // Process record bytes
        handle_record(bytes);
    }

    Ok(())
}

fn handle_record(bytes: &[u8]) {
    // Application-specific record decoding
}
```

### Starting from a Non-Zero Offset

For fast resume or partial replay, specify `initial_offset`:

```rust
let last_known_offset: u64 = load_checkpoint();
let mut reader = LogReader::new(file, reporter, true, last_known_offset);
```

`LogReader` will:

1. Call `skip_to_initial_block()` to jump to the first candidate block
2. Enter *resync mode* and discard partial fragments until a clean `Full` or `First` fragment is observed at or after `initial_offset`
3. Begin delivering logical records from a correct boundary

This mirrors LevelDB’s robust recovery semantics in the presence of torn writes or partial truncations.

---

## Error and Corruption Semantics

From a correctness and data-integrity perspective, the key behaviors are:

- **Checksum errors** (`checksum == true`):
  - Header-payload region CRC32C mismatch → record is discarded
  - Bytes corresponding to the entire buffer block are dropped and reported as corruption (`"checksum mismatch"`)
- **Bad lengths**:
  - If the header length field exceeds remaining buffer bytes, the entire buffer is dropped
  - If EOF has not been seen, this is reported as corruption (`"bad record length"`)
  - If EOF was already set, it is treated as partial tail and not reported as corruption
- **Partial fragmented records at EOF**:
  - If EOF is reached in the middle of an assembled logical record, the partial is silently dropped (no corruption report) because this typically indicates an unflushed or partially written tail
- **Unknown record types**:
  - The fragment and any accumulated scratch are dropped
  - Reported as corruption (`"unknown record type"`)
- **Reporter gating by `initial_offset`**:
  - `report_drop` only emits corruption when the physical `current_offset` is ≥ `initial_offset`
  - However, failures in the initial `skip` to the target block are treated specially and reported regardless to avoid silent data loss

### Reporter Contract

Your `LogReaderReporter` implementation must:

- Be valid for the entire lifetime of the `LogReader`
- Tolerate multiple invocations and potentially large `bytes` values
- Not panic in normal operation if you want robust replay (panic-based fail-fast is also a legitimate policy in some systems)

---

## Memory and Lifetime Model

Important invariants for a safe integration:

- `LogReader` owns a heap-allocated `[u8; LOG_BLOCK_SIZE]` backing buffer
  - It keeps only a raw pointer `*const u8` to it (`backing_store`), to pass to `SequentialFile::read`
  - On `Drop`, it reconstructs the `Box<[u8; LOG_BLOCK_SIZE]>` and frees it
- `read_record` exposes a `Slice` pointing directly into this backing buffer **or** into the provided `scratch` buffer when assembling fragmented records
- As a consequence:
  - The `Slice` returned through `record` is valid only until:
    - The next `read_record` call, or
    - The next mutation of `scratch`
  - If you need to keep a record long-term, copy its bytes out

From a performance standpoint, this model keeps heap allocations predictable and reduces copying, which is vital when scanning large Bitcoin block or mempool logs.

---

## Concurrency and Threading

`LogReader` is not designed to be used from multiple threads concurrently without external synchronization:

- It holds internal mutable state (buffer, offsets, EOF flag, resync flag)
- It operates on raw pointers and assumes exclusive access

Typical patterns:

- Single-threaded log replay loop
- Sharded readers, each owning its own `SequentialFile` and `LogReader` instance for disjoint files or segments

If you must share access, wrap `LogReader` in a `Mutex` or design a higher-level ingestion pipeline that uses channels to publish decoded records to multiple worker threads.

---

## Implementation Notes and Mathematics

### CRC32C Validation

LevelDB uses CRC32C (Castagnoli polynomial) with a masking scheme to protect against certain adversarial patterns and coincidences.

In this crate, the relevant operations are:

- `decode_fixed32(header_ptr)` – read the stored masked CRC32C from the header
- `crc32c_unmask(encoded)` – recover the actual CRC32C from the masked one
- `crc32c_value(header_ptr.add(6), 1 + length)` – compute CRC32C over the concatenation of the record type byte and the payload

The record is valid if `actual_crc == expected_crc`. This is a practical compromise: strong enough against random bit flips and most disk-level corruption, extremely fast, and matches the on-disk format used by LevelDB and Bitcoin.

### Offsets and Block Geometry

Given:

- `LOG_BLOCK_SIZE = B`
- `LOG_HEADER_SIZE = H`
- `initial_offset = O`

`skip_to_initial_block` computes:

- `offset_in_block = O mod B`
- `block_start_location = O - offset_in_block`
- If `offset_in_block > B - 6`, the block is treated as a trailing fragment trailer; we skip to `block_start_location + B`

This guarantees that the scan begins at a block that *can* contain the first complete logical record at or after `O`, respecting the requirement that headers must fit inside a single block (no cross-block headers).

---

## When to Use This Crate

This crate is appropriate when you:

- Need to read LevelDB-style logs produced by Bitcoin Core or a LevelDB-compatible implementation
- Want precise control over corruption semantics (which bytes are dropped, what gets reported)
- Require predictable memory behavior and minimal allocations for large-scale scanning
- Prefer a low-level API that you can integrate into a custom storage or indexing layer

It is *not* a high-level database API; instead, it is a specialized building block for storage engines, replication log readers, forensic tools, and Bitcoin-related infrastructure.

---

## License and Metadata

- **License**: MIT
- **Edition**: Rust 2024
- **Authors**: `YourName <you@example.com>`

You are encouraged to audit the implementation, especially around unsafe pointers and `Slice` lifetimes, to ensure that it matches your safety and reliability requirements.
