# bitcoin-blob

A small, no-nonsense crate providing fixed-size opaque byte blobs, modeled after Bitcoin Core's `base_blob` utility types. It focuses on:

- **Deterministic, fixed-width byte arrays** (8, 64, 128, 160, 256 bits)
- **Efficient comparison and ordering** semantics
- **Hex encoding/decoding** with Bitcoin-style conventions
- **Serialization/deserialization** via `std::io::{Read, Write}`
- **Iterator and slice access** over the underlying bytes

---

## Overview

`bitcoin-blob` defines a macro-based implementation for a family of fixed-size byte containers and then instantiates:

- `BaseBlob8`   – 8 bits  (1 byte)
- `BaseBlob64`  – 64 bits (8 bytes)
- `BaseBlob128` – 128 bits (16 bytes)
- `BaseBlob160` – 160 bits (20 bytes)
- `BaseBlob256` – 256 bits (32 bytes)

Each type is a thin wrapper around `[u8; N]` with a consistent set of operations:

- construction from bytes or small primitives
- zero/one constants
- pointer, slice and iterator access
- total ordering and equality
- hex string encoding/decoding
- simple binary serialization to/from any `Read` / `Write`

The design follows typical Bitcoin low-level primitives such as hashes and IDs, which are represented as fixed-size, little-endian byte arrays but usually *displayed* as big-endian hex strings.

---

## Features at a Glance

### Types

The crate exposes the following concrete blob types:

```rust
use bitcoin_blob::{
    BaseBlob8,
    BaseBlob64,
    BaseBlob128,
    BaseBlob160,
    BaseBlob256,
};
```

Each type is:

- `Clone`, `Debug`, `Hash`
- `Send` + `Sync` (via `unsafe impl` mirroring Bitcoin Core assumptions)
- `Eq`, `Ord`, `PartialEq`, `PartialOrd`
- `Default` (all bytes set to zero)

### Core API Surface

For any blob type `B` (e.g. `BaseBlob256`), the following methods are available:

#### Construction

```rust
impl B {
    /// Const constructor from an exact-width byte array.
    pub const fn from_bytes(arr: [u8; BYTES]) -> Self;

    /// All zeros.
    pub fn zero() -> Self;

    /// Value with the first byte set to 1, others 0.
    pub fn one() -> Self;
}

impl From<u8> for B;
impl From<&Vec<u8>> for B; // enforces exact length == BYTES
```

`From<&Vec<u8>>` panics if the vector length does not equal the blob width, helping detect protocol-level length errors early.

#### Byte Access and Iteration

```rust
impl B {
    /// Immutable slice view.
    pub fn as_slice(&self) -> &[u8];

    /// Mutable slice view.
    pub fn as_slice_mut(&mut self) -> &mut [u8];

    /// Alias for `as_slice_mut`.
    pub fn as_mut_slice(&mut self) -> &mut [u8];

    /// Returns `true` if all bytes are zero.
    pub fn is_null(&self) -> bool;

    /// Sets all bytes to zero.
    pub fn set_null(&mut self);

    /// Signed lexicographic comparison helper: -1, 0, 1.
    pub fn compare(&self, other: &B) -> i32;

    /// Raw pointer accessors (const and mut) to the underlying buffer.
    pub fn data_ptr(&self) -> *const u8;
    pub fn data_ptr_mut(&mut self) -> *mut u8;

    /// C-style begin/end pointers (const and mut); `end` is one-past-last.
    pub fn begin(&self) -> *const u8;
    pub fn end(&self) -> *const u8;
    pub fn begin_mut(&mut self) -> *mut u8;
    pub fn end_mut(&mut self) -> *mut u8;

    /// Number of bytes in the blob.
    pub fn size(&self) -> u32;
}

impl B {
    /// Immutable iterator over bytes.
    pub fn iter(&self) -> core::slice::Iter<'_, u8>;

    /// Mutable iterator over bytes.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<'_, u8>;
}

impl IntoIterator for B {
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, BYTES>;
}

impl<'a> IntoIterator for &'a B {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;
}

impl<'a> IntoIterator for &'a mut B {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;
}
```

This means you can write idiomatic Rust loops over the blob contents:

```rust
let blob = BaseBlob256::one();

// By reference
for b in &blob {
    println!("{}", b);
}

// By value
for b in blob {
    // consumes blob
    println!("{}", b);
}
```

#### Serialization

Serialization is intentionally minimalistic and explicit – there is no `serde` integration in this crate; instead, a raw byte protocol is implemented:

```rust
impl B {
    pub fn serialize<Stream>(&self, s: &mut Stream)
    where
        Stream: std::io::Write;

    pub fn unserialize<Stream>(&mut self, s: &mut Stream)
    where
        Stream: std::io::Read;
}
```

Both methods are fixed-width: `serialize` writes exactly `BYTES` bytes and `unserialize` reads exactly `BYTES` bytes, panicking on I/O failure. This matches wire formats where these blobs represent fields of static length (e.g. block hashes, txids).

#### Hex Encoding / Decoding

Hex conversion follows the traditional crypto convention: the *display* string is big-endian (highest-order byte first), even though the internal representation is little-endian.

```rust
impl B {
    /// Returns the big-endian hex representation (two lowercase chars per byte).
    pub fn get_hex(&self) -> String;

    /// Equivalent to `get_hex`.
    pub fn to_string(&self) -> String;

    /// Parse from a C-style `char *` (ASCII hex, optional 0x prefix).
    pub fn set_hex(&mut self, psz: *const u8);

    /// Safe Rust wrapper around `set_hex` taking an `&str`.
    pub fn set_hex_from_str(&mut self, str_: &str);
}
```

Parsing rules:

- Accepts optional `0x` / `0X` prefix
- Ignores non-hex characters when counting hex digits
- Fills the internal array with the least significant bytes of the parsed value (matching Bitcoin Core semantics)

There is also a helper:

```rust
pub fn nibble_from_hexchar(ch: char) -> u8;
```

which maps `0-9a-fA-F` to the corresponding 4-bit value, logging a warning and returning `0` for invalid characters.

---

## Ordering and Equality Semantics

Ordering is defined lexicographically on the internal byte array via `compare()`:

- `Eq` / `PartialEq` delegate to `compare() == 0`
- `Ord` and `PartialOrd` are total and consistent, making blobs usable as keys in ordered maps and sets when deterministic ordering of identifiers is required.

For a blob type `B`:

```rust
let a = B::zero();
let b = B::one();

assert!(a < b);
assert!(a != b);
```

This is particularly useful when these blobs represent domain objects such as transaction IDs, block hashes, or arbitrary protocol-level identifiers.

---

## SimpleRng Utility

The crate also includes a lightweight pseudo-random generator intended primarily for testing and fuzzing comparisons:

```rust
pub struct SimpleRng(u64);

impl SimpleRng {
    pub fn new(seed: u64) -> Self;

    /// Linear congruential step, returning a new pseudo-random `u64`.
    pub fn next_u64(&mut self) -> u64;

    /// Fill a buffer with pseudo-random bytes using repeated `next_u64` calls.
    pub fn fill_bytes(&mut self, buf: &mut [u8]);
}
```

`SimpleRng` uses a standard 64‑bit linear congruential generator (LCG):

\[
X_{n+1} = a X_n + c \pmod{2^{64}}
\]

with `a = 6364136223846793005` and `c = 1`. This is not cryptographically secure; it is purely for deterministic testing.

Example:

```rust
use bitcoin_blob::{BaseBlob256, SimpleRng};

let mut rng = SimpleRng::new(0xDEADBEEF);
let mut buf = [0u8; 32];
rng.fill_bytes(&mut buf);

let blob = BaseBlob256::from_bytes(buf);
assert!(!blob.is_null());
```

---

## Usage Examples

### Constructing and Inspecting Blobs

```rust
use bitcoin_blob::BaseBlob256;

// From a fixed array
let data = [0xABu8; 32];
let blob = BaseBlob256::from_bytes(data);
assert_eq!(blob.size(), 32);

// Zero and one
let z = BaseBlob256::zero();
let o = BaseBlob256::one();
assert!(z.is_null());
assert!(!o.is_null());

// Slice access
let slice: &[u8] = blob.as_slice();
assert_eq!(slice.len(), 32);

// Mutable access
let mut mutable = BaseBlob256::zero();
mutable.as_slice_mut()[0] = 0xFF;
```

### Hex Conversion

```rust
use bitcoin_blob::BaseBlob160;

let mut blob = BaseBlob160::zero();
blob.set_hex_from_str("0x00112233445566778899aabbccddeeff00112233");

let hex = blob.get_hex();
println!("blob as hex: {}", hex);

// `to_string` is the same as `get_hex`.
assert_eq!(hex, blob.to_string());
```

### Serialization to a Stream

```rust
use bitcoin_blob::BaseBlob64;
use std::io::Cursor;

let original = BaseBlob64::one();

let mut buf = Vec::new();
original.serialize(&mut buf); // write 8 bytes

let mut cursor = Cursor::new(buf);
let mut restored = BaseBlob64::zero();
restored.unserialize(&mut cursor);

assert_eq!(original, restored);
```

### Interoperating with `Vec<u8>`

```rust
use bitcoin_blob::BaseBlob128;

let v = vec![0x42u8; 16];
let blob = BaseBlob128::from(&v);

assert_eq!(blob.as_slice(), &v[..]);
```

> Note: constructing from a `Vec<u8>` whose length is not exactly the blob width will panic. This enforces length invariants at runtime.

---

## Design Considerations

- **Fixed width as a type-level contract**: using distinct concrete types (`BaseBlob256` etc.) encodes protocol width invariants in the type system while still being lightweight.
- **Predictable memory layout**: a blob is just a `[u8; N]` wrapper with no indirection, ideal for FFI, on-disk structures, and unsafe code that needs raw pointers.
- **Bitcoin-style semantics**: the hex formatting, little-endian internal representation, and total ordering semantics are chosen to align with typical Bitcoin Core usage.
- **No allocations in core paths**: apart from hex conversion and explicit constructs like `Vec<u8>` interop, operations are allocation-free.

---

## Logging

The implementation uses log macros such as `trace!`, `debug!`, and `warn!`. To see these messages, you must provide a logger implementation (e.g. via the `log` crate ecosystem) in your application or tests.

---

## Safety

- `Send` and `Sync` are implemented as `unsafe impl` on the blob types, which is correct because they only contain plain bytes.
- Raw pointer accessors (`data_ptr`, `begin`, `end`, etc.) are offered for low-level interop. Their use is inherently `unsafe` in the Rust sense: the caller must ensure correct lifetimes and bounds.
- `unserialize` and `serialize` use `expect` on I/O failures, which will panic on error; if you require recoverable error handling, you should wrap these in your own fallible abstraction.

---

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
bitcoin-blob = "0.1.19"
```

This crate targets Rust 2021 edition and is licensed under MIT.

---

## Repository and Maintenance

The crate is developed in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Please refer to that repository for issues, pull requests, and broader context in which these blob types are used.
