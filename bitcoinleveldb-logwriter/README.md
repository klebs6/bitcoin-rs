# bitcoinleveldb-logwriter

A faithful, low-level implementation of the LevelDB log writer as used in Bitcoin-oriented storage backends. This crate focuses on deterministic fragmentation of logical records into physical log records, block-based layout, and CRC32C integrity, exposing a small but precise surface area suitable for consensus-critical environments.

## Overview

`bitcoinleveldb-logwriter` implements the write half of the LevelDB log format. It is designed to be used as the append-only WAL (write-ahead log) layer for higher-level key–value stores or state machines, particularly where reproducibility and binary compatibility with existing LevelDB-style logs is required.

Key properties:

- Fixed-size **blocks** of size `LOG_BLOCK_SIZE` bytes (typical LevelDB value: 32 KiB).
- Each physical record has a **7-byte header** (`LOG_HEADER_SIZE`), composed of:
  - 4 bytes: little-endian masked CRC32C of the record type and payload.
  - 2 bytes: little-endian payload length.
  - 1 byte: record type (FULL, FIRST, MIDDLE, LAST).
- Arbitrary-length logical records are **fragmented** across one or more physical records.
- CRC32C is computed with per-type seeds (`type_crc`) and then masked (as in upstream LevelDB) to reduce risk of embedded length/CRC confusion.
- Safe public API, with internal use of `unsafe` only for pointer-to-slice bridging and foreign CRC primitives.

The crate is intended primarily as a **building block**: a log writer that can be wired into a Bitcoin/LevelDB-compatible storage engine, custom databases, or replication layers.

## Core Types

### `LogWriter`

```rust
#[derive(Builder)]
#[builder(setter(into))]
pub struct LogWriter {
    dest:         Rc<RefCell<dyn WritableFile>>,
    block_offset: i32,
    type_crc:     [u32; LOG_MAX_RECORD_TYPE as usize + 1],
}
```

`LogWriter` owns the logic for writing LevelDB-format log records into an abstract `WritableFile` sink. Block alignment, header construction, CRC computation, and record fragmentation are all handled internally.

The `dest` is reference-counted and interior-mutably borrowed using `Rc<RefCell<..>>` so that a single underlying file-like object can be shared across multiple components if desired.

### `MockWritableFile*`

Three mock file implementations are provided for testing and integration harnesses:

- `MockWritableFileCore`
- `MockWritableFileAddRecord`
- `MockWritableFileEmit`

All of them store written bytes in an in-memory `Vec<u8>` and implement various `WritableFile*` traits (`WritableFileAppend`, `WritableFileFlush`, `WritableFileClose`, `WritableFileSync`, and the combined `WritableFile`).

`MockWritableFileAddRecord` and `MockWritableFileEmit` additionally support deterministic fault injection via `fail_append_after`, allowing you to test error handling paths in `LogWriter`.

## Public API

Below is a condensed guide to the major methods. Many functions are `pub` primarily to assist deterministic testing and allow explicit control in advanced users.

### Construction

```rust
impl LogWriter {
    /// Create a writer that will append data to `*dest`.
    ///
    /// `*dest` must have initial length `dest_length`.
    /// `*dest` must remain live while this LogWriter is in use.
    pub fn new(dest: Rc<RefCell<dyn WritableFile>>, dest_length: u64) -> Self;
}
```

`new` initializes the block offset based on the existing length of the destination file and precomputes the per-record-type CRC seeds:

- `initial_block_offset_from_length(dest_length)` computes `dest_length % LOG_BLOCK_SIZE`.
- `build_type_crc_table()` invokes `init_type_crc` to fill `type_crc` with the CRC over the one-byte record type tags.

### Adding logical records

```rust
impl LogWriter {
    /// Add a logical record, fragmenting it into one or more physical records.
    pub fn add_record(&mut self, slice: &Slice) -> Status;
}
```

`add_record` is the main entry point. It accepts a `Slice` view of arbitrary length and writes it as one or more physical records:

- If the record fits into the remaining space of the current block, a single `FULL` record is written.
- Otherwise, the record is split into a sequence of `FIRST`, zero or more `MIDDLE`, and a final `LAST` fragment.

Internally, `add_record` delegates to:

```rust
pub fn add_record_internal(&mut self, slice: &Slice) -> Status;
```

`add_record_internal` performs a loop that:

1. Checks whether there is room left for at least a header using `should_start_new_block`.
2. If not, calls `write_trailer_padding_if_necessary` to emit zero-padding in the block trailer and resets the block offset.
3. Computes the available data capacity in the current block via `block_available_data_bytes`.
4. Determines the fragment length (`fragment_length`) and fragment record type using `choose_record_fragment_type(begin, end)`.
5. Calls `emit_physical_record` with the fragment.

The loop terminates once the payload has been fully consumed or a write error is returned.

### Emitting physical records

```rust
impl LogWriter {
    pub fn emit_physical_record(
        &mut self,
        t:      LogRecordType,
        ptr:    *const u8,
        length: usize,
    ) -> Status;
}
```

`emit_physical_record` is responsible for writing a single physical record into the current block:

1. Validates the record length (`validate_record_length`) — must fit into a 16-bit field (`<= 0xffff`).
2. Ensures that the header + payload fits into the current block (`validate_record_fits_in_block`).
3. Computes CRC32C via `crc32c_for_record`.
4. Builds the header with `build_record_header`.
5. Delegates to `append_header_and_payload` to perform the actual writes and flush.
6. Advances `block_offset` by `LOG_HEADER_SIZE + length` using `advance_block_offset`.

### Header construction & CRC

```rust
impl LogWriter {
    /// Compute the CRC32C for the given record type and payload, including masking.
    pub fn crc32c_for_record(
        &self,
        t:      LogRecordType,
        ptr:    *const u8,
        length: usize,
    ) -> u32;

    /// Build the physical record header for the given type, length and CRC.
    pub fn build_record_header(
        &self,
        t:      LogRecordType,
        length: usize,
        crc:    u32,
    ) -> [u8; LOG_HEADER_SIZE as usize];
}
```

The CRC is computed as:

```text
extended_crc = crc32c_extend(type_crc[t], payload_ptr, payload_len)
masked_crc   = crc32c_mask(extended_crc)
```

This matches LevelDB’s scheme, where `type_crc[t]` is the CRC of the single-byte record type tag, and `crc32c_mask` applies a bit-rotation and constant XOR to decorrelate the CRC space.

The header layout (indices are byte positions):

- `buf[0..4]`: masked CRC32C, little-endian, written via `encode_fixed32`.
- `buf[4]`: low byte of length.
- `buf[5]`: high byte of length.
- `buf[6]`: `LogRecordType` discriminant as `u8`.

### Block management

```rust
impl LogWriter {
    /// Remaining bytes in the current block.
    pub fn block_trailer_bytes_remaining(&self) -> i32;

    /// Bytes available for payload in the current block, after accounting for the header.
    pub fn block_available_data_bytes(&self) -> usize;

    /// Whether we need to start a new block because the remaining bytes are too
    /// small to hold a header.
    pub fn should_start_new_block(&self) -> bool;

    /// Write zero-padding in the current block trailer if there are leftover bytes
    /// that are too small for a header, then reset the block offset to zero.
    pub fn write_trailer_padding_if_necessary(&mut self) -> Status;
}
```

LevelDB’s log format guarantees that there are never fewer than `LOG_HEADER_SIZE` bytes left unused in a block. If the remaining trailer is too short to hold a header, it is zero-padded and the next record begins at offset 0 of the next block.

`write_trailer_padding_if_necessary` appends a small zero-filled trailer (up to 6 bytes in the provided implementation, due to `LOG_HEADER_SIZE == 7`) and then resets `block_offset` to 0.

### Fragment type selection

```rust
impl LogWriter {
    /// Choose the physical record type for the current fragment.
    pub fn choose_record_fragment_type(begin: bool, end: bool) -> LogRecordType;
}
```

Mapping:

- `(true, true)`   → `LogRecordType::Full`
- `(true, false)`  → `LogRecordType::First`
- `(false, false)` → `LogRecordType::Middle`
- `(false, true)`  → `LogRecordType::Last`

This logic is key for reconstructing logical records during replay.

### Accessors & validation

```rust
impl LogWriter {
    pub fn type_crc_for(&self, record_type: LogRecordType) -> u32;
    pub fn dest_handle(&self) -> &Rc<RefCell<dyn WritableFile>>;

    pub fn block_offset_value(&self) -> i32;
    pub fn set_block_offset_value(&mut self, value: i32);
    pub fn advance_block_offset(&mut self, delta: i32);

    pub fn validate_record_length(&self, length: usize);
    pub fn validate_record_fits_in_block(&self, length: usize);

    pub fn build_type_crc_table() -> [u32; LOG_MAX_RECORD_TYPE as usize + 1];
    pub fn initial_block_offset_from_length(dest_length: u64) -> i32;
}
```

These methods primarily exist to facilitate careful testing, custom integration, and invariant reasoning. `validate_record_fits_in_block` asserts that `block_offset + LOG_HEADER_SIZE + length <= LOG_BLOCK_SIZE` and panics if violated, which helps catch logic errors in block management.

## Mock Writable Files

All mock files share a common pattern:

```rust
#[derive(Getters)]
#[getset(get = "pub")]
pub struct MockWritableFileCore {
    buffer: Vec<u8>,
}

impl MockWritableFileCore {
    pub fn new() -> Self { Self { buffer: Vec::new() } }
    pub fn recorded_bytes(&self) -> &[u8] { &self.buffer }
}

impl WritableFileAppend for MockWritableFileCore { /* append into buffer */ }
impl WritableFileFlush  for MockWritableFileCore { /* no-op */ }
impl WritableFileClose  for MockWritableFileCore { /* no-op */ }
impl WritableFileSync   for MockWritableFileCore { /* no-op */ }
impl WritableFile       for MockWritableFileCore {}
```

`MockWritableFileAddRecord` and `MockWritableFileEmit` extend this with counters and fault injection:

```rust
pub struct MockWritableFileAddRecord {
    buffer:            Vec<u8>,
    fail_append_after: Option<usize>,
    append_call_count: usize,
    flush_call_count:  usize,
    close_call_count:  usize,
    sync_call_count:   usize,
}

impl MockWritableFileAddRecord {
    pub fn new() -> Self { /* ... */ }
    pub fn with_fail_append_after(call_index: usize) -> Self { /* ... */ }
    pub fn recorded_bytes(&self) -> &[u8] { &self.buffer }
}
```

The `append` implementation increments `append_call_count` and, if `fail_append_after` is set and the threshold is reached, returns a deterministic `Status::io_error`. This is extremely useful for verifying that `add_record` and `emit_physical_record` propagate I/O failures correctly without corrupting invariants.

All mocks also implement a `Named` trait, returning a stable identifier string (e.g. `"mock_writable_file_add_record"`), which simplifies logging and diagnostics.

## Example Usage

### Basic in-memory logging

```rust
use std::cell::RefCell;
use std::rc::Rc;

use bitcoinleveldb_logwriter::{
    LogWriter,
    MockWritableFileCore,
};
use bitcoinleveldb_core::{Slice, WritableFile}; // hypothetical external traits

fn main() {
    // Prepare an in-memory destination file.
    let dest = Rc::new(RefCell::new(MockWritableFileCore::new()));

    // Existing file length is zero in this example.
    let mut writer = LogWriter::new(dest.clone(), 0);

    // Construct a logical payload.
    let payload = b"hello, leveldb-style log";
    let slice   = Slice::from_ptr_len(payload.as_ptr(), payload.len());

    // Append the logical record.
    let status = writer.add_record(&slice);
    assert!(status.is_ok());

    // Inspect the on-disk (here: in-memory) bytes.
    let bytes = dest.borrow().recorded_bytes();
    println!("total bytes written: {}", bytes.len());
}
```

### Testing error handling with fault injection

```rust
use std::cell::RefCell;
use std::rc::Rc;

use bitcoinleveldb_logwriter::{
    LogWriter,
    MockWritableFileAddRecord,
};
use bitcoinleveldb_core::Slice;

#[test]
fn add_record_stops_on_append_error() {
    let dest = Rc::new(RefCell::new(
        MockWritableFileAddRecord::with_fail_append_after(1),
    ));

    let mut writer = LogWriter::new(dest.clone(), 0);

    let big_payload = vec![0u8; 64 * 1024]; // large enough to fragment
    let slice = Slice::from_ptr_len(big_payload.as_ptr(), big_payload.len());

    let status = writer.add_record(&slice);
    assert!(!status.is_ok());

    let dest_ref = dest.borrow();
    assert_eq!(dest_ref.append_call_count(), 1); // injection point
}
```

These examples assume an external crate or module that exposes `Slice`, `Status`, and the `WritableFile*` traits referenced by this crate. The patterns shown remain valid independent of their concrete definitions, as long as they match the interface used here.

## Error Handling & Status

All write operations return a `Status` value, which is treated as a fallible result but not the standard `Result<T, E>` type. This matches the LevelDB lineage, where `Status` encodes I/O and corruption information compactly.

- Methods such as `append_header_and_payload` and `write_trailer_padding_if_necessary` always propagate the `Status` they receive from `WritableFile::append` / `flush`.
- Logging calls (`trace!`, `debug!`, `info!`, `error!`) are used to annotate the path taken and aid in root-cause analysis; they do not affect control flow.

When integrating into a larger system, you will typically map `Status` into your own error type or propagate it up unchanged.

## Mathematical & Systems Considerations

### Block arithmetic

The writer reasons about block boundaries using simple modular arithmetic:

- For initial offsets:

  ```text
  block_offset = (dest_length mod LOG_BLOCK_SIZE) as i32
  ```

  This ensures that resumes from partially written files respect existing block boundaries without rewriting the prefix.

- Available payload in the current block:

  ```text
  available = LOG_BLOCK_SIZE - block_offset - LOG_HEADER_SIZE
  ```

  This quantity is always non-negative when `should_start_new_block()` returns `false`, and it constitutes an invariant that is asserted in `validate_record_fits_in_block`.

### Fragmentation invariants

Let `L` be the length of a logical record and `B` be `block_available_data_bytes` at the start of a fragment. The loop in `add_record_internal` repeatedly sets:

```text
fragment_len = min(left, B)
```

and then updates:

```text
left := left - fragment_len
```

so that across all fragments, the sum of payload lengths equals `L`. Combined with the header size and block capacity constraints, this yields a tiling of the log file into disjoint valid physical records and occasional zero-padded trailers.

### CRC32C correctness

The CRC design follows the standard LevelDB approach:

1. Precompute `type_crc[t] = CRC32C([t as u8])` for all `t`.
2. For each record, compute `extended = crc32c_extend(type_crc[t], payload)`, which is equivalent to CRC over `[t_byte || payload]` but more efficient.
3. Mask the final CRC using `crc32c_mask` to minimize accidental alignment of data and control information.

This allows a reader to recalculate the CRC for each record and detect bit flips or partial writes.

## Integration Notes

- **Threading model**: `LogWriter` uses `Rc<RefCell<..>>`, which fits single-threaded or externally synchronized contexts. For multi-threaded writers, wrap it in your own synchronization or adapt the internals to `Arc<Mutex<..>>` if you fork the crate.
- **Unsafe**: All `unsafe` operations are localized to `Slice` interop and FFI-backed CRC primitives. The public surface remains safe under the documented invariants (e.g., valid lifetimes of `Slice` and underlying memory).
- **Compatibility**: If your goal is binary compatibility with Bitcoin Core’s LevelDB fork or upstream LevelDB, ensure that your constants (`LOG_BLOCK_SIZE`, `LOG_HEADER_SIZE`, record type enum) and CRC masking functions are identical.

## License, Edition, and Metadata

- **Crate name**: `bitcoinleveldb-logwriter`
- **Version**: `0.1.1`
- **Rust edition**: `2024`
- **License**: MIT
- **Authors**: `YourName <you@example.com>`

This crate is intended for production-grade systems that require precise control over log layout, but it is also suitable as a reference implementation or test fixture for any LevelDB-style log reader or storage engine.
