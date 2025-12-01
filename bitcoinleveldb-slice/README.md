# bitcoinleveldb-slice

A low-level, pointer-based slice type used internally by the `bitcoin-rs` LevelDB bindings. It mirrors the semantics of LevelDB's C++ `leveldb::Slice`: a cheap, non-owning view into externally managed bytes with explicit lifetime and safety responsibilities.

> **Safety disclaimer**
>
> `Slice` is a thin wrapper around a raw pointer (`*const u8`) and a length. It **does not** manage memory, and it performs no lifetime tracking beyond debug assertions. The caller is responsible for ensuring that the referenced memory remains valid and immutable for the entire lifetime of the `Slice` value.

---

## Overview

`bitcoinleveldb-slice` provides a single core type:

```rust
pub struct Slice {
    data: *const u8,
    size: usize,
}
```

This type is:

- **Non-owning** – it never allocates or frees; it just points at memory owned by something else.
- **Copy-free** – constructing a `Slice` from a string or byte slice does not copy the bytes.
- **C-FFI friendly** – can be constructed from raw C strings (`*const u8` interpreted as `char*` / null-terminated) and plain pointers plus explicit length.
- **Comparable** – supports equality, prefix checks, and three-way byte-wise ordering compatible with typical database and key-value store semantics.

This design is suitable for high-performance storage engines where the cost of copying data dominates and where ownership is controlled at a higher abstraction layer.

## Features at a Glance

- `Slice::from_ptr_len(*const u8, usize)` – construct from a raw pointer plus a byte length.
- `impl From<&[u8]> for Slice` – construct from a Rust byte slice without copying.
- `impl From<&String> for Slice` – construct from a `String` without copying.
- `impl From<*const u8> for Slice` – construct from a C-style null-terminated string.
- `Slice::empty()` – constant-time check for zero length.
- `Slice::clear()` – reset to an empty slice.
- `Slice::remove_prefix(n)` – logically drop the first `n` bytes (pointer bump + length decrement).
- `Slice::starts_with(&Slice)` – constant-time prefix check (`O(prefix_len)` byte-wise).
- `Slice::compare(&Slice)` – three-way lexicographic comparison, returning an `i32` (<0, 0, >0).
- `Slice::to_string()` – copy data into a `String` (lossy UTF-8 conversion).
- `Index<usize>` – `slice[i]` returns the `i`-th byte with bounds checking via assertion.
- `PartialEq`/`Eq` – equality defined by size and byte-wise content.

Logging hooks (`info!`, `debug!`, `trace!`, `warn!`) are embedded throughout; the crate expects a `log`-compatible backend to be configured by the consumer for runtime diagnostics.

## Core Semantics

`Slice` behaves conceptually like:

```rust
struct Slice<'a> {
    data: &'a [u8];
}
```

but implemented as a pointer and length to interface cleanly with C APIs and avoid Rust lifetime generics in public FFI signatures. This mirrors the classic design of low-level storage engines:

- **Aliasing is allowed**: many `Slice` instances may point to overlapping regions.
- **Immutability is assumed**: callers are expected not to mutate the underlying bytes while they are observed through `Slice` (the type does not enforce this).
- **Ordering**: `compare` implements lexicographic ordering over the byte sequence, using a `memcmp`-like primitive (`compare_bytes`). This is critical for ordered key spaces in LevelDB-style LSM trees.

In mathematical terms, consider `Slice` as representing a word over the finite alphabet `{0, 1, ..., 255}`. The `compare` function implements the standard lexicographic order on words induced by the usual order on bytes.

## Usage

### Constructing Slices

#### From a Rust `String`

```rust
use bitcoinleveldb_slice::Slice;

let s = String::from("hello");
let slice = Slice::from(&s);

assert!(!slice.empty());
assert_eq!(slice.to_string(), "hello");
```

**Lifetimes**: `slice` borrows the contents of `s` via a raw pointer; dropping or mutating `s` while `slice` is used is undefined behavior.

#### From a byte slice `&[u8]`

```rust
let bytes: &[u8] = &[0x01, 0x02, 0x03];
let slice = Slice::from(bytes);

assert_eq!(slice.empty(), false);
assert_eq!(slice[0], 0x01);
```

If `bytes` is empty, `Slice::from(bytes)` produces a canonical empty slice pointing at a static empty buffer.

#### From a C-style string pointer

```rust
use bitcoinleveldb_slice::Slice;
use std::ffi::CString;

let c = CString::new("leveldb").unwrap();
let ptr = c.as_ptr() as *const u8;

// Interprets ptr as a null-terminated string.
let slice = Slice::from(ptr);

assert_eq!(slice.to_string(), "leveldb");
```

If the pointer is null, an empty `Slice` is returned.

#### From raw pointer and length

```rust
use bitcoinleveldb_slice::Slice;

unsafe {
    let buf: [u8; 4] = *b"test";
    let slice = Slice::from_ptr_len(buf.as_ptr(), buf.len());
    assert_eq!(slice.to_string(), "test");
}
```

This is the most direct constructor for interoperation with foreign code where you already have explicit length metadata.

### Observing and Manipulating Slices

#### Empty check and clearing

```rust
let mut slice = Slice::from(&String::from("data"));
assert!(!slice.empty());

slice.clear();
assert!(slice.empty());
```

`clear` logically detaches the `Slice` from the previously referenced memory by pointing it at a static empty buffer.

#### Removing a prefix

```rust
let mut slice = Slice::from(&String::from("abcdef"));

slice.remove_prefix(2);  // now represents "cdef"
assert_eq!(slice.to_string(), "cdef");
```

`remove_prefix(n)` advances the internal pointer by `n` bytes and reduces the size by `n`. An assertion guards against `n > size`.

#### Indexing

```rust
let slice = Slice::from(&String::from("xyz"));

assert_eq!(slice[0], b'x');
assert_eq!(slice[1], b'y');
assert_eq!(slice[2], b'z');
```

Indexing performs a runtime assertion that `i < size`. Access is then performed via `unsafe` pointer arithmetic.

#### String conversion

```rust
let bytes = [0xf0, 0x9f, 0x92, 0x96]; // valid UTF-8 sequence
let slice = Slice::from(&bytes[..]);

let s = slice.to_string(); // uses String::from_utf8_lossy
```

`to_string` always allocates and copies the underlying bytes, decoding via `String::from_utf8_lossy`. Invalid UTF-8 is replaced with the Unicode replacement character.

### Comparisons and Ordering

#### Equality

```rust
let s1 = Slice::from(&String::from("abc"));
let s2 = Slice::from(&String::from("abc"));
let s3 = Slice::from(&String::from("abd"));

assert_eq!(s1, s2);
assert_ne!(s1, s3);
```

Equality is defined as equal length and byte-wise identity via `compare_bytes`.

#### Prefix checks

```rust
let base = Slice::from(&String::from("abcdef"));
let prefix = Slice::from(&String::from("abc"));
let non_prefix = Slice::from(&String::from("abd"));

assert!(base.starts_with(&prefix));
assert!(!base.starts_with(&non_prefix));
```

`starts_with` compares only the prefix length using `compare_bytes` and returns `true` when the prefix is identical.

#### Three-way comparison

```rust
let a = Slice::from(&String::from("a"));
let b = Slice::from(&String::from("b"));
let aa = Slice::from(&String::from("aa"));

assert!(a.compare(&b) < 0);
assert!(b.compare(&a) > 0);
assert!(a.compare(&aa) < 0); // "a" is a prefix of "aa", but shorter
```

This is suitable for implementing ordered maps, sorted tables, or delegating to LSM-tree comparators. The semantics align with the standard `memcmp`-then-length rule:

1. Compare the first `min(len(self), len(b))` bytes.
2. If they differ, the sign of the result is the sign of the first differing byte.
3. If they are identical and lengths differ, the shorter slice is considered smaller.

## Safety Considerations

Because `Slice` is fundamentally unsafe, there are important constraints:

- **Memory validity**: The underlying memory must remain allocated and immutable for the entire lifetime of the `Slice`.
- **No internal lifetime tracking**: The compiler cannot help you with dangling pointers here. Treat `Slice` as you would treat FFI pointers.
- **Thread safety**: Whether a `Slice` is `Send` or `Sync` depends on its definition and Rust's auto-trait derivation. Conceptually, as a raw pointer plus length, it is safe to move between threads only if the backing memory is itself safe to access concurrently.
- **Logging in unsafe paths**: Many methods log, including ones that use `unsafe`. Ensure that logging backends are well-behaved and do not introduce reentrancy issues for your use case.

In contexts such as a LevelDB implementation, `Slice` typically points to memory controlled by the storage engine (e.g., table blocks, memtable allocations). Higher-level logic enforces correct lifetimes; this crate deliberately stays minimal.

## Integration with `bitcoin-rs` / LevelDB

This crate is designed to live inside a broader ecosystem (`bitcoin-rs`) and to work closely with LevelDB-style bindings. Typical patterns include:

- Representing keys and values from on-disk blocks without copying.
- Passing around temporary views during compaction, write batching, and iteration.
- Using `compare` to define a canonical ordering for keys in SSTables and memtables.

For arbitrarily large data sets, avoiding unnecessary copies significantly improves throughput and reduces memory pressure, particularly in log-structured merge tree architectures.

## Logging Behavior

The implementation uses standard `log` macros:

- `info!` on construction and state-transition operations.
- `debug!` on index operations and comparisons.
- `trace!` on lightweight checks and conversions.
- `warn!` when encountering anomalous situations (e.g., null pointer passed to `From<*const u8>`).

To see these logs, configure a logger in your binary (e.g., `env_logger`, `tracing-log`, or any other `log`-compatible backend) and set the appropriate log level.

## License

This crate is licensed under the MIT License. See the `LICENSE` file in the repository for details.

## Repository

The source code for this crate is hosted in the monorepo:

- <https://github.com/klebs6/bitcoin-rs>

Issues and pull requests related specifically to `bitcoinleveldb-slice` should reference the crate name and version in their description for clarity.
