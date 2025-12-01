# bitcoinleveldb-bloom

LevelDB-compatible Bloom filter implementation and test harness used by the `bitcoin-rs` LevelDB port. This crate focuses on byte-level compatibility with the original C++ LevelDB `BuiltinBloomFilter2` policy while providing a safe Rust façade.

---

## Overview

This crate implements a Bloom filter policy and supporting utilities mirroring LevelDB's built-in Bloom filter (`leveldb.BuiltinBloomFilter2`). It is intended to be plugged into a LevelDB-compatible storage engine within the `bitcoin-rs` project, but it can also be used as a standalone Bloom filter implementation over arbitrary byte slices.

Key properties:

- **LevelDB-compatible format**: Filters produced here are intended to be interoperable with LevelDB implementations using `BuiltinBloomFilter2` (same hashing scheme, layout, and trailing `k` marker).
- **Kirsch–Mitzenmacher double hashing**: Efficient computation of multiple hash probes from a single 32-bit hash, as in the original LevelDB design.
- **Configurable bits-per-key**: Construct policies tuned for a target false positive rate / memory footprint trade-off.
- **Low-level slice interop**: Can construct filters from raw `Slice` pointers (for FFI / LevelDB core) or from standard `&[u8]` slices.
- **Instrumented** with `tracing` for detailed diagnostics in performance and correctness testing.

The crate exposes two main abstractions:

- `BloomFilterPolicy`: a `FilterPolicy` / `CreateFilter` / `KeyMayMatch` implementation compatible with the LevelDB APIs provided elsewhere in `bitcoin-rs`.
- `BloomTest`: a stateful helper to accumulate keys, build a filter, and empirically measure false positive rates.

---

## Crate Integration

This crate lives inside the `bitcoin-rs` monorepo:

- Repository: <https://github.com/klebs6/bitcoin-rs>

It assumes the existence of several traits and types from the surrounding LevelDB port, particularly:

- `Slice`: a lightweight view over `*const u8` + length (mirroring LevelDB's `Slice`).
- `FilterPolicy`, `CreateFilter`, `KeyMayMatch`, `Named`: traits defining the filter behavior and metadata.

If you are using this crate outside the monorepo, you will typically consume it indirectly via the LevelDB layer that defines these traits and the `Slice` abstraction.

---

## Installation

In your `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-bloom = "0.1.19"
```

This crate uses Rust 2021 edition and is licensed under MIT.

---

## Core Types

### `BloomFilterPolicy`

```rust
#[derive(Clone, Debug, Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct BloomFilterPolicy {
    bits_per_key: usize,
    k: usize,
}
```

`BloomFilterPolicy` encapsulates the Bloom filter configuration:

- `bits_per_key`: approximate number of bits in the filter per inserted key.
- `k`: number of hash probes per key (automatically derived from `bits_per_key`).

Construction:

```rust
impl BloomFilterPolicy {
    pub fn new(bits_per_key_: i32) -> Self { /* ... */ }
}

pub fn new_bloom_filter_policy(bits_per_key_: i32) -> Box<dyn FilterPolicy> { /* ... */ }
```

Behavior:

- Negative `bits_per_key_` is clamped to `0`, then coerced to at least `1` bit per key.
- `k` is computed as `k = floor(bits_per_key * ln(2)) ≈ floor(bits_per_key * 0.69)`, which approximates the theoretically optimal number of probes and is then clamped to `1 ≤ k ≤ 30`.

This matches the LevelDB design: choosing `k = ln(2) * m/n` where `m` is the number of bits and `n` is the number of keys minimizes false positives for a given filter size.

Interfaces implemented (from the LevelDB API layer):

- `Named` — returns `"leveldb.BuiltinBloomFilter2"` for wire compatibility.
- `FilterPolicy` — marker trait for LevelDB filter policies.
- `CreateFilter` — builds filters from `*const Slice`.
- `KeyMayMatch` — checks potential membership using a `Slice` key and filter.

#### Byte-Slice Interface

The policy exposes a fully byte-based API for independent use:

```rust
impl BloomFilterPolicy {
    pub fn create_filter_from_bytes(&self, keys: &[&[u8]], dst: &mut Vec<u8>);
    pub fn key_may_match_bytes(&self, key: &[u8], bloom_filter: &[u8]) -> bool;
}
```

`create_filter_from_bytes`:

- Input: `keys: &[&[u8]]` — array of byte-slice keys.
- Output: appends a filter to `dst`:
  - A fresh region of `bytes` zero-initialized bytes for the bitset.
  - A single trailing byte encoding `k`.

Filter layout:

- Let `num_keys = keys.len()`.
- Bits per key: `bpk = self.bits_per_key`.
- Raw bits: `bits = max(64, num_keys * bpk)` — small filters are padded to at least 64 bits to avoid pathological false positive rates.
- Bytes: `bytes = (bits + 7) / 8` and `bits` is rounded up to a multiple of 8.
- `dst` is extended by `bytes` zero bytes, then a trailing byte `k` is pushed.

Hashing (`Kirsch–Mitzenmacher` double hashing):

- Base hash: `h = leveldb_hash(key, seed = 0xbc9f1d34)`.
- Delta: `delta = (h >> 17) | (h << 15)` (rotate right by 17 bits).
- For each `probe` from `0` to `k-1`:
  - `bitpos = (h % bits)`.
  - `array[bitpos / 8] |= 1 << (bitpos % 8)`.
  - `h = h.wrapping_add(delta)`.

This is exactly the double-hashing construction described by Kirsch and Mitzenmacher, reducing the cost of `k` independent hashes to a single base hash plus linear updates.

`key_may_match_bytes`:

- Ensures the filter has at least length 2 and decodes:
  - `bits = (len(filter) - 1) * 8`.
  - `k = filter[len(filter)-1]`.
- If `k > 30`, the function treats the filter as a match for compatibility with potential future encodings (matching LevelDB's semantics).
- Otherwise performs the same double hashing scheme and returns `false` on the first missing bit, `true` if all `k` bits are present.

This provides a canonical, constant-time-per-probe membership test with standard Bloom filter semantics.

### `BloomTest`

```rust
#[derive(Debug, Getters, Setters, Builder)]
#[getset(get = "pub", set = "pub")]
pub struct BloomTest {
    policy: BloomFilterPolicy,
    filter: Vec<u8>,
    keys: Vec<Vec<u8>>,
}
```

`BloomTest` is a harness around `BloomFilterPolicy` used to:

- Accumulate keys.
- Build a filter from them.
- Probe membership.
- Empirically estimate false positive rate.

Key methods:

```rust
impl BloomTest {
    pub fn reset(&mut self);
    pub fn add_key_slice(&mut self, key: &[u8]);
    pub fn add_key_str(&mut self, key: &str);
    pub fn add_key_slice_object(&mut self, s: &Slice);
    pub fn add(&mut self, s: &Slice);

    pub fn build(&mut self);
    pub fn filter_size(&self) -> usize;
    pub fn dump_filter(&self);

    pub fn matches_slice(&mut self, key: &[u8]) -> bool;
    pub fn matches_str(&mut self, key: &str) -> bool;

    pub fn false_positive_rate(&mut self) -> f64;
}
```

Behavior:

- `Default` builds a policy with `bits_per_key = 10` (roughly ~1% false positive rate in classical Bloom filter analysis).
- `add_*` methods stage keys in `self.keys`.
- `build` converts all staged keys into a single Bloom filter (using `create_filter_from_bytes`), stores it in `self.filter`, and clears `keys`.
- `matches_*` lazily builds the filter if `keys` is not empty before checking membership.
- `false_positive_rate` tests 10,000 new keys encoded via `encode_fixed32_into` into a 4-byte buffer, offset by `1_000_000_000` to avoid overlap with typical test data, counts how many yield `true` under `matches_slice`, and returns the observed fraction.

`dump_filter` logs a textual visualization of the filter bits, ignoring the trailing `k` byte, with bits rendered as `1` (set) or `.` (unset) and space-separated per byte.

---

## Low-Level Utilities

### Hashing and Encoding

```rust
pub fn bloom_hash(key_: &Slice) -> u32 { /* ... */ }
```

- Computes the LevelDB Bloom hash for a `Slice` using `leveldb_hash` with seed `0xbc9f1d34`.
- Used as the foundation of `BloomFilterPolicy`'s hashing scheme.

```rust
pub fn encode_fixed32_to_bytes(value: u32) -> [u8; 4];
pub fn encode_fixed32_into(value: u32, buffer: &mut [u8; 4]);
```

- Encode `u32` values into 4-byte little-endian representation, mirroring LevelDB's fixed-width integer encoding.

### Synthetic Keys and Length Progression

```rust
pub fn key(i: i32, buffer: *mut u8) -> Slice { /* ... */ }
```

- Encodes `i` with `encode_fixed32_to_bytes` into `buffer` and returns a `Slice` over it.
- If `buffer` is null, returns an empty `Slice` built from a null pointer and zero length (for defensive usage patterns).

```rust
pub fn next_length(length: i32) -> i32 { /* ... */ }
```

- Generates a simple sequence for test sizes: increments by 1 below 10, by 10 below 100, by 100 below 1000, then by 1000.
- Useful for iterating over key counts in scalability benchmarks.

---

## Usage Examples

### Basic: Build a Bloom Filter from Byte Slices

```rust
use bitcoinleveldb_bloom::BloomFilterPolicy;

fn main() {
    // Aim for ~10 bits per key (typical LevelDB default).
    let policy = BloomFilterPolicy::new(10);

    // Keys as raw byte slices.
    let key1 = b"alpha";
    let key2 = b"beta";
    let key3 = b"gamma";
    let keys: [&[u8]; 3] = [key1.as_ref(), key2.as_ref(), key3.as_ref()];

    let mut filter = Vec::new();
    policy.create_filter_from_bytes(&keys, &mut filter);

    assert!(policy.key_may_match_bytes(b"alpha", &filter));
    assert!(policy.key_may_match_bytes(b"beta", &filter));

    // Typical Bloom filter behavior: non-members should usually be rejected,
    // but could be false positives.
    let might_contain = policy.key_may_match_bytes(b"delta", &filter);
    println!("delta may be in the set: {might_contain}");
}
```

### Using `BloomTest` to Empirically Check False Positives

```rust
use bitcoinleveldb_bloom::BloomTest;

fn main() {
    let mut tester = BloomTest::default();

    // Add some keys.
    tester.add_key_str("a");
    tester.add_key_str("b");
    tester.add_key_str("c");

    // Build the filter explicitly (matches_* will also build lazily).
    tester.build();

    assert!(tester.matches_str("a"));
    assert!(tester.matches_str("b"));
    assert!(tester.matches_str("c"));

    let fp_rate = tester.false_positive_rate();
    println!("Observed false positive rate: {fp_rate:.4}");
}
```

### Integrating via `FilterPolicy` Trait Objects

```rust
use bitcoinleveldb_bloom::new_bloom_filter_policy;
// use crate::leveldb::{FilterPolicy, Options, DB}; // from bitcoin-rs LevelDB layer

fn configure_leveldb() {
    // Create a boxed policy with 10 bits per key.
    let bloom_policy = new_bloom_filter_policy(10);

    // This example is schematic; actual LevelDB options will be defined by the
    // surrounding `bitcoin-rs` LevelDB wrapper.
    //
    // let mut options = Options::default();
    // options.filter_policy = Some(bloom_policy);
    // let db = DB::open(options, "/path/to/db").unwrap();
}
```

---

## Mathematical and Algorithmic Notes

The Bloom filter implemented here follows the classical analysis:

- A Bloom filter is a bit vector of length `m` with `k` hash functions.
- After inserting `n` keys, the probability of a false positive is approximately:

\[
  p \approx \left(1 - e^{-kn/m}\right)^k.
\]

Choosing `m = bits_per_key * n` and `k ≈ ln(2) * m/n` minimizes `p` for fixed `m`, and yields:

\[
  p_{min} \approx (0.5)^{bits\_per\_key \cdot \ln 2} \approx (0.6185)^{bits\_per\_key}.
\]

Thus:

- `bits_per_key = 10` gives `p ≈ 0.8%`.
- Smaller `bits_per_key` trades memory for increased false positives.

The Kirsch–Mitzenmacher double hashing used in this crate preserves asymptotic false positive characteristics while reducing hash computation to a single base hash plus linear updates.

The filter construction additionally enforces `bits ≥ 64` for small `n`, preventing degenerate configurations where `m` becomes so small that the asymptotic approximation no longer yields reasonable behavior.

---

## Tracing and Diagnostics

All core operations (construction, hashing, filter building, membership testing) are instrumented with the `tracing` crate, using log levels:

- `info!` for high-level events (policy construction, false positive statistics),
- `debug!` for structural details (filter sizes, reset actions),
- `trace!` for per-key and per-bit actions (probing, bit-setting),
- `warn!` and `error!` for anomalous conditions (invalid parameters, null pointers where unexpected).

Enable appropriate `tracing` subscribers in your application to inspect filter behavior, verify interoperability, or benchmark performance.

---

## Safety and FFI Considerations

The crate assumes a `Slice` type akin to LevelDB's, consisting of a raw pointer and length. The unsafe blocks are localized and guarded:

- Null pointers from `Slice::data()` with zero length are handled gracefully, treated as empty keys.
- `create_filter` converts `*const Slice` + `n` into a slice with `from_raw_parts`, only when `n > 0` and `keys` is non-null.
- Byte slices are always reconstructed with lengths derived from the `Slice` metadata.

When integrating with C or C++:

- Ensure `Slice` lifetime guarantees match those expected by the Rust side.
- Ensure that any `*mut u8` passed into `key` points to at least 4 bytes of valid writable memory.

---

## License

This crate is distributed under the MIT License. See the repository for full licensing information.
