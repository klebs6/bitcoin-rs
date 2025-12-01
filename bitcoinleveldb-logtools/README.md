# bitcoinleveldb-logtools

Low-level logging and encoding utilities extracted from a LevelDB-style write-ahead log, adapted for use in Bitcoin-oriented storage stacks. This crate focuses on precise control of log record types, safe(ish) manipulation of raw byte slices via a `Slice` abstraction, and deterministic textual encoding/decoding of numeric and binary data.

---

## Overview

`bitcoinleveldb-logtools` provides building blocks for implementing LevelDB-compatible log readers/writers and related tooling:

- **Record typing primitives** matching LevelDB's physical log format:
  - `LogRecordType` for base record kinds (`Full`, `First`, `Middle`, `Last`, `Zero`).
  - `ExtendedRecordTypes` bitflags for synthetic states (`Eof`, `BadRecord`).
- **Logging traits** that mirror the C++ LevelDB logging surface:
  - `Logger` as a marker trait for log sinks.
  - `Logv` for `printf`-style structured logging.
- **Textual encoding helpers**
  - `number_to_string` / `append_number_to` for fast, predictable integer formatting.
  - `escape_string` / `append_escaped_string_to` for ASCII-safe string rendering with hex escapes for non-printables.
  - `consume_decimal_number` for robust, overflow-aware parsing of decimal `u64` from a `Slice`.
- **Binary utilities**
  - `append_slice_bytes` for efficiently copying from a raw-pointer-backed `Slice` into a `Vec<u8>`.
- **CRC precomputation**
  - `init_type_crc` to precompute CRC32C values for each log record type, accelerating integrity checks.

The crate deliberately exposes low-level, pointer-driven operations to track the original LevelDB semantics. It is intended for developers implementing storage engines, Bitcoin database tooling, or compatibility layers that must replicate LevelDB's on-disk layout and error handling precisely.

---

## When to use this crate

Use `bitcoinleveldb-logtools` if you need:

- **Binary compatibility** with LevelDB-style logs, especially in Bitcoin Core–adjacent systems.
- Tight control over **physical records** and their classification into `Full`/`First`/`Middle`/`Last` fragments.
- Deterministic, allocation-conscious helpers for converting between bytes and human-readable monitoring/debugging output.
- Precomputation of **CRC32C** for log record types at startup to reduce per-record hashing cost.

It is *not* a full log implementation; instead, it is an internal toolkit you would combine with your own I/O and higher-level record framing.

---

## Log record types

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogRecordType {
    Zero,
    Full,
    First,
    Middle,
    Last,
}
```

These correspond to the physical record types used in LevelDB's log file format:

- `Full`: the logical record fits entirely within one physical record.
- `First`: the first physical fragment of a multi-fragment logical record.
- `Middle`: an intermediate fragment.
- `Last`: the final fragment.
- `Zero`: reserved; often treated as padding or invalid.

This classification allows a reader to reconstruct logical records across block boundaries.

### Extended record types

```rust
bitflags! {
    pub struct ExtendedRecordTypes: i32 {
        const Eof       = LOG_MAX_RECORD_TYPE as i32 + 1;
        const BadRecord = LOG_MAX_RECORD_TYPE as i32 + 2;
    }
}
```

`ExtendedRecordTypes` adds higher-level sentinel states used during scanning:

- `Eof`: Logical end-of-file encountered.
- `BadRecord`: A physically invalid record was found. The provided comment indicates three classes of invalid records:
  - Invalid CRC.
  - Zero-length record.
  - Record earlier than the constructor's `initial_offset`.

These are useful in a log reader state machine to distinguish between *intentional termination* and *data corruption or protocol mismatch*.

---

## Logging traits

### `Logger`

```rust
/// An interface for writing log messages.
pub trait Logger {}
```

`Logger` is currently a marker trait: it does not impose any methods. In practice you implement it for structs that represent a logging sink (file logger, stderr logger, in-memory collector, etc.). Other components can then be parameterized over `T: Logger` to remain abstract over the specific logging backend.

### `Logv`

```rust
pub trait Logv {
    fn logv(&mut self, format: *const u8, ap: &[&str]);
}
```

`Logv` models a variadic `printf`-style logging interface, keeping close fidelity with the original C++ implementation:

- `format`: a raw `*const u8` pointer to a format string (typically a NUL-terminated C-style string).
- `ap`: a slice of string parameters.

This trait is intentionally low-level. Callers should ensure that the `format` pointer is valid for the duration of the call and that the argument list semantically matches the format string. Implementations typically perform format string parsing and write to their underlying logger or sink.

---

## String and number utilities

### `append_number_to` and `number_to_string`

```rust
pub fn append_number_to(str_: *mut String, num: u64) { /* ... */ }

pub fn number_to_string(num: u64) -> String { /* ... */ }
```

- `append_number_to` appends the decimal representation of `num` to an existing `String` via a raw `*mut String` pointer. This is designed to mirror the original implementation's pointer-based API.
- `number_to_string` constructs a new `String`, calls `append_number_to`, and returns it.

Usage pattern:

```rust
use bitcoinleveldb_logtools::{append_number_to, number_to_string};

let mut s = String::from("height=");
append_number_to(&mut s as *mut String, 840_000);
assert_eq!(s, "height=840000");

let s2 = number_to_string(42);
assert_eq!(s2, "42");
```

**Safety note**: Only pass valid pointers to `append_number_to`. Passing a null or dangling pointer results in undefined behavior beyond the crate's explicit null check.

### `append_escaped_string_to` and `escape_string`

```rust
pub fn append_escaped_string_to(str_: *mut String, value: &Slice) { /* ... */ }

pub fn escape_string(value: &Slice) -> String { /* ... */ }
```

These functions encode arbitrary bytes from a `Slice` into a printable ASCII representation:

- Printable characters in the interval `[' ', '~']` are appended as-is.
- All other bytes are appended as `"\\xNN"` hex escapes.

This is ideal for:

- Logging keys/values that may contain non-UTF-8 data.
- Emitting diagnostics for corrupted log entries or internal binary keys.

Example (assuming a compatible `Slice` type):

```rust
use bitcoinleveldb_logtools::escape_string;
use some_slice_crate::Slice; // Replace with your actual Slice implementation

let data = b"hello\nworld";
let slice = Slice::from(data);
let escaped = escape_string(&slice);
assert_eq!(escaped, "hello\\x0aworld");
```

As with other pointer-heavy functions, `append_escaped_string_to` requires that `str_` is a valid, non-null pointer to a `String`.

### `consume_decimal_number`

```rust
pub fn consume_decimal_number(in_: *mut Slice, val: *mut u64) -> bool { /* ... */ }
```

This function parses a non-negative decimal integer from the *beginning* of the `Slice` referenced by `in_` and writes the result to `*val`.

Semantics:

- On **success**:
  - Leading decimal digits `[0-9]+` are consumed.
  - The parsed value is stored into `*val`.
  - The `Slice` is advanced by the number of consumed bytes (`remove_prefix(digits_consumed)`).
  - Returns `true`.
- On **overflow** or invalid state:
  - If parsing would exceed `u64::MAX`, it logs a warning and returns `false`.
  - If there are **no leading digits**, returns `false` and leaves the `Slice` unspecified.
  - If `in_` or `val` is null, logs an error and returns `false`.

Algorithmically, this is a base-10 accumulation with explicit overflow checks:

\[
\text{value}_{n+1} = 10 \cdot \text{value}_n + d_{n+1}, \quad d_{n+1} \in \{0,\dots,9\}
\]

and it refuses to proceed once `value > u64::MAX / 10` or the final digit would exceed `u64::MAX % 10`.

Example sketch:

```rust
use bitcoinleveldb_logtools::consume_decimal_number;
use some_slice_crate::Slice; // Replace with your actual Slice implementation

let mut slice = Slice::from(b"12345 rest");
let mut value: u64 = 0;
let ok = consume_decimal_number(&mut slice as *mut Slice, &mut value as *mut u64);
assert!(ok);
assert_eq!(value, 12345);
// `slice` now points at " rest"
```

This function is especially appropriate for parsing configuration numbers or internal sequence counters encoded as ASCII in a LevelDB / log context.

---

## Binary helpers

### `append_slice_bytes`

```rust
pub fn append_slice_bytes(source: &Slice, destination: &mut Vec<u8>) { /* ... */ }
```

Copies the raw bytes referenced by `source` into `destination` using the `Slice`'s internal data pointer and length.

- If the length is zero, it returns immediately.
- If the `Slice`'s data pointer is null, it logs a warning and does nothing.
- Otherwise it constructs a `from_raw_parts` view and `extend_from_slice`s into `destination`.

Usage:

```rust
use bitcoinleveldb_logtools::append_slice_bytes;
use some_slice_crate::Slice;

let slice = Slice::from(b"block data");
let mut buf = Vec::new();
append_slice_bytes(&slice, &mut buf);
assert_eq!(&buf[..], b"block data");
```

This is exactly what you want when re-materializing a logical record from multiple fragmented physical slices.

---

## CRC32C initialization

### `init_type_crc`

```rust
pub fn init_type_crc(type_crc: *mut u32) { /* ... */ }
```

Precomputes CRC32C values for each valid log record type in `[0, LOG_MAX_RECORD_TYPE]` and stores them in a table at `type_crc`.

For each `i` in that range, the function computes:

\[
\text{crc}[i] = \text{CRC32C}([i \text{ as u8}])
\]

and writes it to `*type_crc.add(i)`.

Assumptions/requirements:

- `type_crc` must point to an array of at least `LOG_MAX_RECORD_TYPE + 1` `u32` entries.
- `LOG_MAX_RECORD_TYPE` is an external constant (likely equal to the largest `LogRecordType` discriminant).
- `crc32c_value` is an external function, presumably providing the CRC32C primitive.

Usage sketch:

```rust
use bitcoinleveldb_logtools::init_type_crc;

const MAX_TYPE: usize = bitcoinleveldb_logtools::LOG_MAX_RECORD_TYPE as usize; // if exported
let mut table = vec![0u32; MAX_TYPE + 1];

init_type_crc(table.as_mut_ptr());
// `table[i]` now holds the CRC32C of the i-th record type byte
```

This table can then be combined with the CRC of the payload to construct or verify the complete record CRC in O(1) per record.

---

## Safety and invariants

This crate deliberately uses raw pointers (`*mut String`, `*mut Slice`, `*const u8`) to parallel a C++ codebase. Consequently:

- Many functions are **memory-unsafe** if called with invalid pointers.
- The crate performs some null checks and logs diagnostic messages but cannot defend against all misuse.

When integrating:

1. Ensure that all pointers passed into these functions are:
   - Non-null.
   - Properly aligned.
   - Pointing to valid, mutable data for the duration of the call.
2. Ensure that `Slice`'s `data()` and `size()` methods uphold their documented contracts:
   - `size()` must return a pointer to a valid length `usize` or `usize`-compatible.
   - `data()` must return a pointer to an inner data pointer that is either null or points to at least `size()` bytes of readable memory.
3. Respect integer overflow checks in `consume_decimal_number`. Do not ignore its `false` return; treat it as parse failure.

If you require a safer abstraction, consider building a thin, fully safe wrapper around these primitives that validates invariants up front.

---

## Example: Implementing a simple log printer

Below is a conceptual sketch integrating several APIs from this crate. It assumes a `Slice` implementation compatible with the function signatures.

```rust
use bitcoinleveldb_logtools::{
    escape_string,
    number_to_string,
    append_slice_bytes,
    LogRecordType,
};
use some_slice_crate::Slice;

pub struct LogPrinter;

impl LogPrinter {
    pub fn print_record(typ: LogRecordType, payload: &Slice) {
        let human_type = format!("{:?}", typ);
        let human_payload = escape_string(payload);
        println!("[type={}] payload=\"{}\"", human_type, human_payload);
    }

    pub fn accumulate_payload(chunks: &[Slice]) -> Vec<u8> {
        let mut buf = Vec::new();
        for c in chunks {
            append_slice_bytes(c, &mut buf);
        }
        buf
    }
}

// Usage (assuming `Slice::from` and data):
// let slice = Slice::from(b"hello\x00world");
// LogPrinter::print_record(LogRecordType::Full, &slice);
```

This demonstrates how the crate's utilities help transform binary log payloads into something auditable and loggable while keeping control over the underlying binary representation.

---

## Crate metadata

- **Name**: `bitcoinleveldb-logtools`
- **Version**: `0.1.1`
- **Edition**: `2024`
- **License**: `MIT`
- **Intended domain**: LevelDB-style logging, Bitcoin database/log tooling, low-level storage infrastructure.

---

## Status and caveats

- The exposed API is low-level and mirrors an external codebase; it is primarily suitable as an **internal dependency** of a larger system that already uses a compatible `Slice` and CRC32C implementation.
- The README intentionally describes only the components provided in the excerpt. Additional items in the crate (if any) may not be documented here.
- Because this document was produced by an AI model from partial source context, verify semantics against the actual crate source before deploying in production-critical paths.
