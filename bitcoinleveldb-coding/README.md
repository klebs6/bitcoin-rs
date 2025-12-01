# bitcoinleveldb-coding

Low-level, allocation-conscious encoders and decoders for LevelDB-style binary formats used in `bitcoin-rs`. This crate exposes pointer-based primitives for:

- Fixed-width little-endian integers (`u32`, `u64`)
- Varint-encoded integers (`u32`, `u64`)
- Length-prefixed slices
- Conversions between `Slice` and `String`/UTF‑8

The implementation is intentionally close to the original LevelDB C++ code, with Rust idioms where they do not compromise layout compatibility or performance.

## Design goals

- **Bit-level compatibility with LevelDB**: Encodings are little-endian and follow LevelDB's varint and length-prefix conventions so data can be shared with existing LevelDB implementations.
- **Zero extra allocation in hot paths**: Pointer-based APIs allow writing directly into preallocated buffers and reading from raw memory without intermediate copies.
- **Predictable performance**: Varint encoders use simple branch patterns, and decoders operate in tight loops amenable to inlining and optimization.
- **Logging-friendly**: Functions are instrumented with `trace!`, `debug!`, and `warn!` calls (using the `log` facade or `tracing`-style macros, depending on the parent crate) to aid in debugging complex storage issues.

The crate is primarily intended as an internal component of the `bitcoin-rs` LevelDB port, but it can be used independently wherever LevelDB-like encodings are needed.

## Encoding primitives

### Fixed-width little-endian integers

These functions read/write 32-bit and 64-bit integers in little-endian order directly to/from raw pointers:

```rust
use bitcoinleveldb_coding::{
    encode_fixed32, encode_fixed64,
    decode_fixed32, decode_fixed64,
};

// Write a 32-bit value into an 8-byte buffer
let mut buf = [0u8; 8];
unsafe {
    encode_fixed32(buf.as_mut_ptr(), 0x11223344);
}
assert_eq!(buf[..4], [0x44, 0x33, 0x22, 0x11]);

// Read it back
let v = unsafe { decode_fixed32(buf.as_ptr()) };
assert_eq!(v, 0x11223344);
```

APIs:

- `fn encode_fixed32(dst: *mut u8, value: u32)`
- `fn encode_fixed64(dst: *mut u8, value: u64)`
- `fn decode_fixed32(ptr: *const u8) -> u32`
- `fn decode_fixed64(ptr: *const u8) -> u64`

These functions perform **no bounds checking** and are `unsafe` to call in a memory-safety sense. Callers must guarantee that `dst`/`ptr` points to at least 4 (for 32-bit) or 8 (for 64-bit) valid bytes.

### Varint encoding

Varint encoding represents an integer using a base-128 scheme:

- Each byte carries 7 bits of payload in the low bits.
- The high bit (bit 7) is a continuation flag: `1` means another byte follows, `0` terminates the varint.

This is identical to the scheme used in LevelDB and many other storage systems. Values in `[0, 2^7)` fit in 1 byte, `[2^7, 2^14)` in 2 bytes, etc.

#### Pointer-based varint encoding

```rust
use bitcoinleveldb_coding::{encode_varint32, encode_varint64};

let mut buf = [0u8; 10];
let start = buf.as_mut_ptr();

let end32 = unsafe { encode_varint32(start, 300) };
let len32 = unsafe { end32.offset_from(start) as usize };

let end64 = unsafe { encode_varint64(start, 1234567890123) };
let len64 = unsafe { end64.offset_from(start) as usize };

assert!(len32 <= 5);
assert!(len64 <= 10);
```

APIs:

- `fn encode_varint32(dst: *mut u8, v: u32) -> *mut u8`
- `fn encode_varint64(dst: *mut u8, v: u64) -> *mut u8`

Both functions:

- Assume `dst` points to a buffer with enough capacity (`≤ 5` bytes for `u32`, `≤ 10` bytes for `u64`).
- Return a pointer to the first byte *after* the encoded value.

The helper `fn varint_length(v: u64) -> i32` computes the length (in bytes) of the varint encoding of `v`. This is useful when pre-sizing buffers:

```rust
use bitcoinleveldb_coding::varint_length;

let v: u64 = 1_000_000;
let len = varint_length(v);
assert!(len >= 1 && len <= 10);
```

### String-backed varint and fixed-width encoding

Instead of working with raw pointers, you can append encodings directly into `String` buffers. This matches the original LevelDB design, where `std::string` served as a generic byte buffer.

```rust
use bitcoinleveldb_coding::{
    put_varint32, put_varint64,
    put_fixed32, put_fixed64,
};

let mut s = String::new();

unsafe {
    put_varint32(&mut s as *mut String, 1000);
    put_fixed64(&mut s as *mut String, 0x0102_0304_0506_0708);
}

let bytes = s.into_bytes();
// ``bytes`` now begins with the varint-encoded 1000, followed by 8 LE bytes
```

APIs:

- `fn put_varint32(dst: *mut String, v: u32)`
- `fn put_varint64(dst: *mut String, v: u64)`
- `fn put_fixed32(dst: *mut String, value: u32)`
- `fn put_fixed64(dst: *mut String, value: u64)`

These functions:

- Treat `String` as an opaque byte buffer via `String::as_mut_vec`.
- Append encoded bytes; they do not clear or truncate existing data.
- Expose a raw `*mut String` interface because they are designed to be called from unsafe internals where borrowing rules are already enforced at a higher level.

## Decoding primitives with `Slice`

The crate interoperates with a `Slice` abstraction that behaves like a non-owning byte span with a cursor.

### Varint decoding from pointer ranges

These functions decode varints from `[p, limit)` and either return a pointer to the first byte after the value or `null()` on failure.

```rust
use bitcoinleveldb_coding::{
    get_varint_32ptr,
    get_varint_64ptr,
};

let mut buf = [0u8; 10];
let start = buf.as_mut_ptr();

unsafe {
    let end = bitcoinleveldb_coding::encode_varint64(start, 999_999);
    let limit = end;

    let mut out: u64 = 0;
    let p = get_varint_64ptr(start as *const u8, limit as *const u8, &mut out as *mut u64);

    assert!(!p.is_null());
    assert_eq!(out, 999_999);
}
```

APIs:

- `fn get_varint_32ptr(p: *const u8, limit: *const u8, value: *mut u32) -> *const u8`
- `fn get_varint_32ptr_fallback(p: *const u8, limit: *const u8, value: *mut u32) -> *const u8`
- `fn get_varint_64ptr(p: *const u8, limit: *const u8, value: *mut u64) -> *const u8`

`get_varint_32ptr` uses a fast path for single-byte varints, then falls back to the more general `get_varint_32ptr_fallback` for multi-byte values.

### Varint decoding from `Slice`

These functions parse a varint at the beginning of a `Slice` and advance the slice on success.

```rust
use bitcoinleveldb_coding::{get_varint32, get_varint64};
use bitcoinleveldb_types::Slice; // pseudoname; use the actual path in the repo

let mut storage = String::new();
unsafe { bitcoinleveldb_coding::put_varint32(&mut storage as *mut String, 12345); }

let bytes = storage.into_bytes();
let mut slice = Slice::from_ptr_len(bytes.as_ptr(), bytes.len());

let mut out: u32 = 0;
let ok = unsafe { get_varint32(&mut slice as *mut Slice, &mut out as *mut u32) };

assert!(ok);
assert_eq!(out, 12345);
// ``slice`` has been advanced past the varint
```

APIs:

- `fn get_varint32(input: *mut Slice, value: *mut u32) -> bool`
- `fn get_varint64(input: *mut Slice, value: *mut u64) -> bool`

Semantics:

- On success, return `true`, write the decoded value to `*value`, and call `input.remove_prefix(consumed_bytes)`.
- On failure (overflow or not enough bytes), return `false` and leave `input` unchanged.

## Length-prefixed slices

Length-prefixed slices are encoded as:

1. A `u32` length `L` encoded as varint32.
2. Followed by `L` raw bytes.

This format is omnipresent in LevelDB metadata (keys, values, and other structures).

### Encoding length-prefixed slices

```rust
use bitcoinleveldb_coding::put_length_prefixed_slice;
use bitcoinleveldb_types::Slice; // adjust path to actual crate

let mut s = String::new();
let data = b"hello world";
let slice = unsafe { Slice::from_ptr_len(data.as_ptr(), data.len()) };

unsafe {
    put_length_prefixed_slice(&mut s as *mut String, &slice);
}

// s now holds: varint32(len=11) + b"hello world"
```

API:

- `fn put_length_prefixed_slice(dst: *mut String, value: &Slice)`

Behavior:

- Panics are avoided: if length exceeds `u32::MAX`, the function logs an error and returns early.
- For zero-length slices, only the length varint (0) is written.

### Decoding length-prefixed slices

From a mutable `Slice` cursor:

```rust
use bitcoinleveldb_coding::get_length_prefixed_slice;
use bitcoinleveldb_types::Slice;

// suppose ``input`` points at a length-prefixed slice
let mut input: Slice = /* ... */;
let mut out: Slice = Slice::default(); // or uninitialized according to actual API

let ok = unsafe { get_length_prefixed_slice(&mut input as *mut Slice, &mut out as *mut Slice) };
if ok {
    // ``out`` is a view into the original data; ``input`` is advanced past it
}
```

From raw pointers with an explicit limit:

```rust
use bitcoinleveldb_coding::get_length_prefixed_slice_with_limit;
use bitcoinleveldb_types::Slice;

let buf: &[u8] = /* ... */;
let mut out: Slice = Slice::default();

let next = unsafe {
    get_length_prefixed_slice_with_limit(
        buf.as_ptr(),
        unsafe { buf.as_ptr().add(buf.len()) },
        &mut out as *mut Slice,
    )
};

if !next.is_null() {
    // success; ``next`` points past the slice
}
```

APIs:

- `fn get_length_prefixed_slice(input: *mut Slice, result: *mut Slice) -> bool`
- `fn get_length_prefixed_slice_with_limit(p: *const u8, limit: *const u8, result: *mut Slice) -> *const u8`

Both validate that the declared length does not exceed the available bytes.

## Slice to UTF‑8 conversion

For debugging or higher-level string handling, `slice_to_utf8` converts a `Slice` into an owned `String` using `from_utf8_lossy` semantics:

```rust
use bitcoinleveldb_coding::slice_to_utf8;
use bitcoinleveldb_types::Slice;

let bytes = b"example";
let slice = unsafe { Slice::from_ptr_len(bytes.as_ptr(), bytes.len()) };
let s = slice_to_utf8(&slice);
assert_eq!(s, "example");
```

API:

- `fn slice_to_utf8(slice: &Slice) -> String`

Behavior:

- If the slice is empty or has a null data pointer, returns an empty `String`.
- Invalid UTF‑8 sequences are replaced with the Unicode replacement character; this is deliberate to avoid panics in low-level diagnostics.

## Safety and invariants

Almost all functions in this crate are `unsafe` to use indirectly because they operate on raw pointers or manipulate `String` internals.

Callers must ensure:

- Pointers (`*const u8` / `*mut u8`) point to valid, appropriately sized memory.
- `limit` pointers in decoding functions delimit the actual readable range; `p <= limit` and the region `[p, limit)` must remain valid for the duration of the call.
- `Slice` values obey their own invariants: `data()` and `size()` reflect a valid contiguous region.
- No concurrent mutable aliasing of the same `String` or `Slice` occurs across threads without synchronization.

The crate itself does not attempt to enforce Rust's aliasing rules; it assumes that higher-level abstractions (e.g., the LevelDB table code) orchestrate these invariants.

## Relationship to mathematics and bit-level representation

Varint encoding is effectively a representation of a non-negative integer in base 128 with a self-delimiting prefix code:

- Let `v` be a non-negative integer.
- Repeatedly emit `v mod 128` (7 bits) and set the continuation bit to `1` while `v >= 128`.
- For the final byte, emit `v mod 128` with continuation bit `0`.

This yields a prefix-free code over `u64` with the following length function:

\[
\ell(v) = 1 + \left\lfloor \log_{128} v \right\rfloor \quad (v > 0), \quad \ell(0) = 1.
\]

By encoding smaller integers with fewer bytes, storage layouts benefit significantly when keys and lengths are typically small (common in LevelDB metadata and in many Bitcoin-related indices).

## Integration within `bitcoin-rs`

This crate lives in the `bitcoin-rs` monorepo and is designed to be used by the LevelDB-compatible storage layer that underpins components such as block indexes, UTXO sets, or other key-value stores.

Typical usage pattern:

1. **Serialize** structured metadata into a `String` or `Vec<u8>` using `put_*` APIs.
2. **Store** that byte sequence in LevelDB or a LevelDB-compatible backend.
3. **Deserialize** on load using `get_*` pointer or `Slice`-based APIs.

Because the encodings match the canonical C++ LevelDB representation, databases can be shared between Rust and C++ nodes without reindexing.

## Crate metadata

- **Name:** `bitcoinleveldb-coding`
- **Version:** `0.1.19`
- **Edition:** `2021`
- **License:** MIT
- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **Authors:** `klebs <none>`

This crate is intended for advanced users who are comfortable reasoning about memory safety, binary layout, and cross-language interoperability.
