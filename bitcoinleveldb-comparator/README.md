# bitcoinleveldb-comparator

A minimal, faithful Rust reimplementation of LevelDB's bytewise comparator and separator logic, extracted for use in `bitcoin-rs`' LevelDB-compatible storage layer.

This crate provides the comparison and key-shortening primitives that LevelDB relies on for ordering keys in its sorted string tables (SSTables) and memtables. It preserves the C++ semantics while exposing a Rust-friendly interface over byte slices.

---

## Design intent

LevelDB orders keys using a `Comparator` interface. The default implementation, `BytewiseComparator`, orders keys lexicographically by raw bytes and provides two additional operations that are essential for index and block metadata compaction:

- **`FindShortestSeparator`**: Given a start key and an upper bound `limit`, mutates `start` in-place to an abbreviated key that is still within `[start, limit)` but lexicographically shorter, reducing index size.
- **`FindShortSuccessor`**: Given a key, mutates it in-place to a short lexicographic successor `>= key`, again minimizing key length while preserving order constraints.

This crate mirrors that behavior exactly, but for Rust, and is tailored to be binary-compatible and behaviorally aligned with Bitcoin's use of LevelDB.

---

## Crate features

- **Trait-based API** that abstracts three distinct concerns:
  - `Compare` — three-way comparison of slices.
  - `FindShortestSeparator` — in-place key shortening between `[start, limit)`.
  - `FindShortSuccessor` — in-place minimal lexicographic successor.
- **`SliceComparator` unifying trait**, matching LevelDB's `Comparator` role.
- **`BytewiseComparatorImpl`**:
  - Implements all traits with LevelDB-compatible semantics.
  - Uses lexicographic byte ordering (`memcmp`-style) via an underlying `Slice` abstraction.
- **Global singleton access** via `bytewise_comparator()` that mimics LevelDB's `static Comparator*` pattern.

This is a low-level crate intended for engines that need precise control over on-disk key ordering and compatibility with existing LevelDB datasets.

---

## Core traits

### `Compare`

```rust
pub trait Compare {
    /// Returns:
    ///  - < 0 if `a < b`
    ///  -   0 if `a == b`
    ///  - > 0 if `a > b`
    fn compare(&self, a: &Slice, b: &Slice) -> i32;
}
```

`Compare` is the fundamental three-way comparator, analogous to LevelDB's `Comparator::Compare`. The implementation for `BytewiseComparatorImpl` delegates to `Slice::compare`, which is expected to perform lexicographic comparison of two byte sequences.

### `FindShortestSeparator`

```rust
pub trait FindShortestSeparator {
    /// Mutate `start` in-place to a shorter key in [start, limit)
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]);
}
```

Semantics (mirroring LevelDB):

1. Compute the length of the common prefix between `start` and `limit`.
2. At the first differing byte `i`, attempt to increment `start[i]` by 1 such that:
   - `start[i] < 0xFF`, and
   - `start[i] + 1 < limit[i]`.
3. If possible, set `start[i] = start[i] + 1` and truncate `start` to length `i + 1`.
4. If not possible, leave `start` unchanged.

This produces a key that still lies in `[start_original, limit)` but is often much shorter, which is highly advantageous for index blocks.

Because Rust `String` must be UTF-8 but LevelDB keys are arbitrary bytes, this API operates on `Vec<u8>`/`[u8]` instead of `String`.

### `FindShortSuccessor`

```rust
pub trait FindShortSuccessor {
    /// Mutate `key` in-place to a short successor >= original key
    fn find_short_successor(&self, key: &mut Vec<u8>);
}
```

Semantics:

1. Scan from the beginning of `key` for the first byte `!= 0xFF`.
2. Increment that byte by 1.
3. Truncate `key` to this position + 1.
4. If all bytes are `0xFF`, leave `key` unchanged.

This produces a compact successor key that remains a valid upper bound.

### `SliceComparator`

```rust
pub trait SliceComparator {
    /// Return a pointer to a global bytewise comparator.
    fn bytewise_comparator(&self) -> *const dyn SliceComparator;
}
```

This matches the conceptual role of LevelDB's `Comparator` base class and allows access to a stable, static instance.

---

## BytewiseComparatorImpl

```rust
#[derive(Debug)]
pub struct BytewiseComparatorImpl {}
```

Implements:

- `Default`
- `Compare`
- `FindShortestSeparator`
- `FindShortSuccessor`
- `SliceComparator`
- `Named` (from the surrounding `bitcoin-rs` ecosystem), returning
  `"leveldb.BytewiseComparator"`.

### Global singleton

```rust
pub fn bytewise_comparator() -> *const dyn SliceComparator {
    static BYTEWISE_COMPARATOR: OnceLock<BytewiseComparatorImpl> = OnceLock::new();

    let reference = BYTEWISE_COMPARATOR.get_or_init(|| {
        BytewiseComparatorImpl::default()
    });
    reference as *const BytewiseComparatorImpl as *const dyn SliceComparator
}
```

This behaves like the C++ `static NoDestructor<BytewiseComparatorImpl>` pattern. Consumers who need raw pointers (for FFI or structural compatibility) can store and pass this pointer directly.

Note: The use of `*const dyn SliceComparator` is deliberate to align with LevelDB's pointer-based comparator design.

---

## Usage

This crate is primarily intended for internal use by `bitcoin-rs`, but it can be consumed directly in any project that needs LevelDB-style lexicographic comparators over arbitrary byte keys.

Assume you already have the `Slice` type from `bitcoinleveldb` or the surrounding repository:

```rust
use bitcoinleveldb_comparator::{
    BytewiseComparatorImpl,
    Compare,
    FindShortestSeparator,
    FindShortSuccessor,
};
use bitcoinleveldb_core::Slice; // hypothetical path

fn demo_compare(cmp: &BytewiseComparatorImpl) {
    let a = Slice::from(b"foo" as &[u8]);
    let b = Slice::from(b"bar" as &[u8]);

    let ordering = cmp.compare(&a, &b);
    assert!(ordering > 0); // "foo" > "bar" lexicographically
}

fn demo_shortest_separator(cmp: &BytewiseComparatorImpl) {
    let mut start = b"foobar".to_vec();
    let limit = b"fooz";

    cmp.find_shortest_separator(&mut start, limit);
    // `start` is now a shortened key between original start and limit
}

fn demo_short_successor(cmp: &BytewiseComparatorImpl) {
    let mut key = b"foo\x7fzzz".to_vec();
    cmp.find_short_successor(&mut key);
    // `key` is now a compact successor >= original
}
```

If you need the global comparator pointer (e.g., for FFI):

```rust
use bitcoinleveldb_comparator::{bytewise_comparator, SliceComparator};

fn get_global_comparator() -> *const dyn SliceComparator {
    bytewise_comparator()
}
```

---

## Ordering semantics and correctness

All operations work at the level of raw bytes, not UTF-8 code points. This mirrors LevelDB's behavior and is essential for compatibility with existing LevelDB databases, including those used in Bitcoin implementations.

Lexicographic byte ordering is equivalent to treating keys as base-256 big-endian integers for ordering purposes. The `find_shortest_separator` and `find_short_successor` algorithms exploit this ordering to construct shorter representative keys while preserving the following invariants:

- `start_original <= start_modified < limit` (for `find_shortest_separator` when modification occurs).
- `key_original <= key_modified` (for `find_short_successor`).

These invariants are critical for correctly delimiting data blocks and index entries; violating them would break search correctness and could corrupt logical key ranges.

---

## Logging

The implementation uses the `log` crate macros (`trace!`, `debug!`, `info!`) to provide fine-grained instrumentation:

- `trace!` for entry/exit of operations and raw comparison values.
- `debug!` for mutated key states.
- `info!` for one-time initialization and naming.

To observe these logs, configure a logger such as `env_logger` or `tracing-log` in your application.

---

## Repository and maintenance

This crate lives in the `bitcoin-rs` monorepo:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Issues, bug reports, and pull requests should be filed against that repository.

---

## License

This crate is distributed under the MIT license.
