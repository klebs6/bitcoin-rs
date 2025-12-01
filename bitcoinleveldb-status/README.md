# bitcoinleveldb-status

A small, C++-compatible `Status` implementation extracted from the Bitcoin Core LevelDB fork.

This crate mirrors the error-handling semantics of LevelDB's C++ `Status` type as used in Bitcoin Core, while providing an idiomatic Rust API. It is designed to be binary-layout compatible with the original implementation so that Rust code can interoperate cleanly with existing C/C++ LevelDB bindings and on-disk encodings.

---

## Design overview

### StatusCode

The `StatusCode` enum is a direct translation of LevelDB's internal status codes:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCode {
    Ok,             // 0
    NotFound,       // 1
    Corruption,     // 2
    NotSupported,   // 3
    InvalidArgument,// 4
    IOError,        // 5
}
``

These codes are encoded as a single byte in the serialized `Status` layout, matching the C++ implementation (with the byte value at offset 4).

### Status

```rust
#[derive(Debug, Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct Status {
    state: Option<Box<[u8]>>, // None => OK, Some => error with encoded payload
}
```

Semantics:

- `Status::ok()` or `Status::default()` represent success and carry no payload (`state: None`).
- Non-OK statuses hold an encoded byte slice with:
  - bytes `[0..4]`: little-endian `u32` length of the message
  - byte `[4]`: the `StatusCode` as a raw `u8`
  - bytes `[5..5+len]`: UTF-8 error message data (typically from one or two `Slice` arguments, concatenated in the C++ implementation)

This layout is intentionally low-level so that FFI and on-disk compatibility with Bitcoin's LevelDB fork can be maintained.

The crate provides move- and copy-like constructors/assignments modeled after the C++ interface, but in safe Rust form.

---

## Core API

### Construction

```rust
use bitcoinleveldb_status::{Status, StatusCode};
use bitcoinleveldb_slice::Slice; // or equivalent, depending on your stack

// OK status
let s_ok = Status::ok();
assert!(s_ok.is_ok());

// Error statuses
let msg = Slice::from_str("block index missing");
let st_not_found = Status::not_found(&msg, None);

let extra = Slice::from_str(" for hash abc123...");
let st_corrupt = Status::corruption(&msg, Some(&extra));
```

Available constructors (all static methods on `Status`):

- `Status::ok()`
- `Status::not_found(msg: &Slice, msg2: Option<&Slice>)`
- `Status::corruption(msg: &Slice, msg2: Option<&Slice>)`
- `Status::not_supported(msg: &Slice, msg2: Option<&Slice>)`
- `Status::invalid_argument(msg: &Slice, msg2: Option<&Slice>)`
- `Status::io_error(msg: &Slice, msg2: Option<&Slice>)`

Each method encodes the `StatusCode` plus the concatenated message into the internal `state` buffer.

### Introspection

```rust
if st_not_found.is_not_found() {
    // handle missing key / file / record
}

match st_corrupt.code() {
    StatusCode::Ok => { /* never for non-OK */ }
    StatusCode::Corruption => { /* data corruption path */ }
    StatusCode::IOError => { /* disk / fs error path */ }
    _ => { /* other errors */ }
}

println!("status: {}", st_corrupt.to_string());
```

Provided predicates:

- `is_ok(&self) -> bool`
- `is_not_found(&self) -> bool`
- `is_corruption(&self) -> bool`
- `is_io_error(&self) -> bool`
- `is_not_supported_error(&self) -> bool`
- `is_invalid_argument(&self) -> bool`

Plus the low-level code extractor:

- `code(&self) -> StatusCode`

### Copy and move semantics

This crate emulates C++ copy/move semantics in a way that is convenient in FFI-heavy or ported codebases:

```rust
let original = Status::io_error(&Slice::from_str("fs full"), None);

// Copy-construction equivalent
let copy = Status::new_from_other_copy(&original);
assert_eq!(copy.to_string(), original.to_string());

// Move-construction equivalent
let original_moved = Status::new_from_other(original);
// `original` is now logically empty (OK) because its state was `take()`n.

// Copy assignment
let mut a = Status::ok();
let b = Status::invalid_argument(&Slice::from_str("height < 0"), None);
a.assign_from_other_copy(&b);

// Move assignment
let mut c = Status::ok();
let mut d = Status::not_supported(&Slice::from_str("compaction style"), None);
c.assign_from_other_move(&mut d);
```

Internally:

- `new_from_other(rhs: Status) -> Self` takes ownership of `rhs.state` using `Option::take`, leaving `rhs` as `OK`.
- `new_from_other_copy(rhs: &Status) -> Self` deep-copies the internal buffer, preserving C++ copy constructor semantics.
- `assign_from_other_copy(&mut self, rhs: &Status) -> &mut Status` deep-copies unless both statuses share the same internal pointer.
- `assign_from_other_move(&mut self, rhs: &mut Status) -> &mut Status` drops any existing data and takes `rhs.state`.

These helpers are primarily valuable when porting C++ LevelDB/Bitcoin code, where copy/move semantics are explicit in the original source.

### String representation

```rust
let st = Status::not_found(&Slice::from_str("key"), None);
assert_eq!(st.to_string(), "NotFound: key");

let ok = Status::ok();
assert_eq!(ok.to_string(), "OK");
```

Format rules:

- `OK` statuses stringify to exactly `"OK"`.
- Error statuses stringify to a canonical prefix based on `StatusCode`:
  - `NotFound: `
  - `Corruption: `
  - `Not implemented: ` (for `NotSupported`)
  - `Invalid argument: `
  - `IO error: `
- The encoded UTF-8 message bytes are appended to this prefix.

---

## Logging behavior

Several methods emit log lines through the `log` facade (or an equivalent logging backend), e.g.:

- `Status::default()` logs when constructing an OK status.
- `Drop for Status` logs whether an OK or non-OK status is being dropped.
- Copy/move helpers log invocation, and move-assign logs when dropping old state.
- `code()` logs a warning if it encounters an unknown numeric code.

For deterministic behavior in production, configure a `log`-compatible backend (such as `env_logger`, `flexi_logger`, or a custom implementation) upstream in your application.

---

## Interoperability and use cases

This crate is intended for:

1. **Bitcoin Core / LevelDB porting**: When mechanically translating C++ code using `leveldb::Status` into Rust, this type allows nearly one-to-one structural translation, preserving behavior and string formats.
2. **FFI boundaries**: When Rust code must interface with existing C/C++ LevelDB implementations that expect the binary layout of the original `Status`. The internal `[u8]` layout is kept as a simple, contiguous representation compatible with C.
3. **Deterministic error semantics**: Bitcoin and similar consensus systems often depend on extremely stable error texts and classifications. Matching the C++ implementation minimizes divergence.

If you are building a purely idiomatic Rust system, prefer using `Result<T, E>` with a structured error type instead. `Status` is particularly valuable where compatibility and fidelity to legacy behavior matter more than typical Rust ergonomics.

---

## Example: wrapping a LevelDB-like operation

```rust
use bitcoinleveldb_status::Status;
use bitcoinleveldb_slice::Slice;

fn db_get(key: &Slice) -> (Status, Option<Vec<u8>>) {
    // In a real implementation, this would call into LevelDB / Bitcoin's DB layer
    if key.as_bytes().is_empty() {
        return (
            Status::invalid_argument(&Slice::from_str("empty key"), None),
            None,
        );
    }

    // pretend: key not present
    let not_found_status = Status::not_found(&Slice::from_str("key not found"), None);
    (not_found_status, None)
}

fn main() {
    let key = Slice::from_str("");
    let (st, value) = db_get(&key);

    if !st.is_ok() {
        eprintln!("db_get failed: {}", st.to_string());
        return;
    }

    println!("value: {:?}", value.unwrap());
}
```

---

## Feature expectations

While the exact public API may evolve, this crate is conceptually constrained by the needs of the Bitcoin/LevelDB ecosystem:

- Preserve binary compatibility with C++ LevelDB `Status` encoding.
- Provide predicates for all relevant error classes.
- Emit consistent, stable string representations.
- Offer copy/move helpers for mechanical porting of existing C++ code.

If you require additional predicates or status codes, consider whether they exist in the upstream C++ implementation before extending the API, in order to avoid semantic drift.

---

## License and contribution

- License: MIT (matching the broader `bitcoin-rs` repository license).
- Repository: <https://github.com/klebs6/bitcoin-rs>

This crate lives inside a larger ecosystem of Bitcoin-related Rust libraries. Please submit issues and pull requests to that repository, referencing `bitcoinleveldb-status` explicitly in your description.

By contributing, you agree to license your contributions under the MIT license of the project.
