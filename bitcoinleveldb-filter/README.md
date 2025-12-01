# bitcoinleveldb-filter

A low-level implementation of LevelDB-compatible filter blocks (Bloom filters) for `bitcoin-rs`, written in Rust.

This crate models the filter subsystem used by LevelDB to accelerate point lookups in SSTables (sorted string tables). It provides the primitives necessary to build and query filter blocks, while delegating the concrete filter policy (e.g., Bloom filter configuration) to user-defined implementations.

---

## Overview

LevelDB uses *filter blocks* (typically Bloom filters) to quickly determine whether a key **cannot** be present in a particular data block, substantially reducing I/O for negative lookups. Each SSTable has:

- A **filter block**, consisting of one filter region per data block range.
- A small trailer describing where the filter block lives and how to interpret it.

This crate focuses on the **filter block** part:

- **`FilterBlockBuilder`**: constructs an in-memory filter block given a user-supplied filter policy and a stream of keys grouped by block offsets.
- **`FilterBlockReader`**: parses a filter block and uses the policy to answer `key_may_match` queries for a given block offset.

The design is intentionally low-level and close to the original C++ LevelDB layout, so that filter blocks are bitwise-compatible with LevelDB and can be used in interoperable storage formats (such as Bitcoin Core’s LevelDB-backed chainstate and index databases).

You are expected to plug in a concrete filter policy (generally a Bloom filter) via the `FilterPolicy`, `CreateFilter`, and `KeyMayMatch` traits.

## Core Traits

### `FilterPolicy`

```rust
pub trait FilterPolicy {}
```

`FilterPolicy` is a marker trait representing a particular filter strategy.

In practice you will implement additional traits (`CreateFilter`, `KeyMayMatch`) for the same type, so that a single policy type both *creates* filters and *queries* them. The crate’s `FilterBlockBuilder` and `FilterBlockReader` store a `Box<dyn FilterPolicy>`; your type must be downcast or enriched to also implement the necessary behavior.

### `CreateFilter`

```rust
pub trait CreateFilter {
    fn create_filter(
        &self,
        keys:  *const Slice,
        n:     i32,
        dst:   &mut Vec<u8>
    );
}
```

This trait encapsulates the construction of a single filter region:

- `keys`: pointer to an array of `Slice` values (each slice is a key).
- `n`: number of keys in the array.
- `dst`: output buffer where the concrete filter representation is appended.

Typical implementation for a Bloom filter will:

1. Hash each key multiple times.
2. Set bits in a bitset sized according to `bits_per_key * n`.
3. Serialize the bitset (and possibly some metadata, such as number of probes) into `dst`.

### `KeyMayMatch`

```rust
pub trait KeyMayMatch {
    fn key_may_match(&self, key: &Slice, filter: &Slice) -> bool;
}
```

This trait encapsulates querying a single filter region:

- `key`: the key to test.
- `filter`: serialized filter region for some key range.

Return semantics:

- `false`: the key is *definitely not* present in this region.
- `true`: the key *may* be present (false positives allowed; false negatives are **not** allowed for a correct Bloom filter implementation).

In Bloom-filter terms, `key_may_match` checks that all probe bits corresponding to the key are set in the underlying bitset.

## Filter Block Construction

### `FilterBlockBuilder`

```rust
#[derive(Getters, Setters)]
pub struct FilterBlockBuilder  {
    policy:         Box<dyn FilterPolicy>,
    keys:           Vec<u8>,
    start:          Vec<usize>,
    result:         Vec<u8>,
    tmp_keys:       Vec<Slice>,
    filter_offsets: Vec<u32>,
}
```

The builder aggregates keys per logical data block and finally emits a single contiguous filter block buffer.

#### Layout

The produced filter block has the same high-level layout as in LevelDB:

```text
[filter_0][filter_1]...[filter_(n-1)][offset_array][array_offset][base_lg]
```

- `filter_i`: opaque bytes produced by the `FilterPolicy` for range `i`.
- `offset_array`: `n` little-endian `u32` values; `offset_array[i]` is the start offset of `filter_i` within the block.
- `array_offset`: `u32` little-endian, byte offset where `offset_array` begins.
- `base_lg`: `u8`, log2 of the filter base (block alignment in bytes) used to compute region indices.

Because the policy is policy-specific, the crate does not prescribe the inner layout of each `filter_i` region; it merely provides the surrounding indexing structure.

#### Constructor

```rust
impl FilterBlockBuilder {
    pub fn new(policy: Box<dyn FilterPolicy>) -> Self { ... }
}
```

Creates a new builder bound to a specific filter policy.

#### `start_block`

```rust
pub fn start_block(&mut self, block_offset: u64)
```

Indicates that keys subsequently added belong to the data block whose *file-level* offset is `block_offset`.

- `FILTER_BASE` (constant, external to this snippet) defines the granularity at which filters are grouped. The filter index is computed as:

  ```rust
  let filter_index = (block_offset / FILTER_BASE as u64) as usize;
  ```

- If `filter_index` skips ahead relative to the current number of filters, empty filters are generated for intermediate indices via `generate_filter()`.

This mirrors LevelDB’s logic where multiple neighboring data blocks may share the same filter region.

#### `add_key`

```rust
pub fn add_key(&mut self, key_: &Slice)
```

Adds a key to the current, not-yet-flushed filter batch.

- The raw key bytes are appended to `self.keys`.
- `self.start` records the start index of each key within that buffer.

`Slice` is assumed to be a lightweight pointer+length structure (from the surrounding codebase), and here it is reinterpreted as a byte slice using `from_raw_parts`.

#### `finish`

```rust
pub fn finish(&mut self) -> Slice
```

Finalizes all filters and returns a `Slice` view over the resulting buffer.

Process:

1. If there are unflushed keys (tracked via `self.start`), the builder calls `generate_filter()` once more to produce the last `filter_i` region.
2. Append all `filter_offsets` as little-endian `u32` values.
3. Append `array_offset` (the index where `filter_offsets` begins) as `u32`.
4. Append `FILTER_BASE_LG` as `u8`.

The returned `Slice` points into `self.result`. It is your responsibility to ensure `self.result` outlives this slice.

### Low-level Utilities

#### `put_fixed32`

```rust
pub fn put_fixed32(dst: &mut Vec<u8>, value: u32)
```

Appends a 32-bit little-endian integer to `dst`. Used to encode offsets and `array_offset`.

#### `decode_fixed32`

```rust
pub fn decode_fixed32(src: &[u8]) -> u32
```

Decodes a 32-bit little-endian integer from the first 4 bytes of `src`. Caller must ensure at least 4 bytes are available.

## Filter Block Querying

### `FilterBlockReader`

```rust
#[derive(Getters, Setters)]
pub struct FilterBlockReader  {
    policy:  Box<dyn FilterPolicy>,
    data:    Arc<[u8]>,
    offset:  usize,
    num:     usize,
    base_lg: usize,
    valid:   bool,
}
```

The reader parses a serialized filter block and exposes `key_may_match` per block offset.

- `policy`: same policy implementation used at build time (or at least compatible with the serialized format).
- `data`: reference-counted backing storage for the filter block bytes.
- `offset`: byte offset of the offsets array.
- `num`: number of filter regions.
- `base_lg`: log2 of the filter base (i.e., `FILTER_BASE = 1 << base_lg`).
- `valid`: whether parsing was successful.

#### Constructor

```rust
impl FilterBlockReader {
    pub fn new(policy: Box<dyn FilterPolicy>, contents: &Slice) -> Self { ... }
}
```

Parsing algorithm:

1. Copy `contents` into an `Arc<[u8]>`.
2. If total length `n < 5`, mark invalid (insufficient space for `array_offset + base_lg`).
3. Read `base_lg` as the last byte (`data[n - 1]`).
4. Read `last_word = decode_fixed32(&data[n - 5..n - 1])` as the start offset of the offsets array.
5. Validate `last_word <= n - 5`; otherwise mark invalid.
6. Let `offset = last_word` and `num = (n - 5 - last_word) / 4`.
7. Mark as valid.

This reconstructs the same metadata the builder appended during `finish`.

#### `key_may_match`

```rust
pub fn key_may_match(&self, block_offset: u64, key: &Slice) -> bool
```

Evaluates whether `key` may be present in the filter associated with the given `block_offset`.

Process:

1. If `self.valid` is `false`, conservatively return `true` (cannot safely refute membership).
2. Compute the filter index:

   ```rust
   let index = (block_offset >> self.base_lg) as usize;
   ```

3. If `index >= self.num`, return `true` (out-of-range; treat as potential match).
4. Read the start and limit offsets for this index from the offsets array:

   ```rust
   let idx1 = self.offset + index * 4;
   let start = decode_fixed32(&self.data[idx1..idx1 + 4]) as usize;
   let limit = decode_fixed32(&self.data[idx1 + 4..idx1 + 8]) as usize;
   ```

5. If `start == limit`, region is empty ⇒ return `false`.
6. If `start <= limit && limit <= self.offset`, the filter region is `data[start..limit]`.
7. Wrap this region in a `Slice` and delegate to `self.policy.key_may_match(key, &filter_slice)`.

The contract is: reader handles indexing and bounds; the policy handles the semantics of the filter.

## Bloom Filter Policy Stub

```rust
pub fn new_bloom_filter_policy(_bits_per_key_: i32) -> Box<dyn FilterPolicy> {
    unimplemented!("new_bloom_filter_policy is not yet implemented");
}
```

This function is a placeholder intended to produce a concrete Bloom filter policy implementation. In canonical LevelDB, the Bloom filter policy:

- Uses `k ≈ ln(2) * bits_per_key` hash functions.
- Achieves false positive probability approximately:

  \[
  p \approx (1 - e^{-k n / m})^{k}
  \]

  where \(n\) is the number of keys, \(m\) is the total number of bits (\(m = \text{bits\_per\_key} \times n\)).

To make this function functional, you should:

1. Define a type, e.g. `struct BloomFilterPolicy { bits_per_key: i32, k: u8, ... }`.
2. Implement `FilterPolicy` for it (marker).
3. Implement `CreateFilter` and `KeyMayMatch` using a robust hashing scheme (e.g., MurmurHash or a suitable 64-bit hash folded into multiple probes).
4. Pack per-region Bloom filter bits and any parameters necessary to decode them.
5. Return `Box::new(BloomFilterPolicy { ... })` from `new_bloom_filter_policy`.

You must ensure that the encoding/decoding conventions of your policy are self-consistent between builder and reader.

## Example Usage

> The following is conceptual and assumes you supply a concrete `BloomFilterPolicy` type implementing `FilterPolicy + CreateFilter + KeyMayMatch` and exposing a constructor used by `new_bloom_filter_policy`.

### Building a Filter Block

```rust
use bitcoinleveldb_filter::FilterBlockBuilder;
use bitcoinleveldb_filter::new_bloom_filter_policy;
// use crate::Slice; // from the surrounding bitcoin-rs/leveldb implementation

fn build_filter_block(blocks: Vec<(u64, Vec<Slice>)>) -> Slice {
    let policy = new_bloom_filter_policy(10); // e.g., 10 bits per key
    let mut builder = FilterBlockBuilder::new(policy);

    for (block_offset, keys) in blocks {
        builder.start_block(block_offset);
        for key in &keys {
            builder.add_key(key);
        }
    }

    let filter_block = builder.finish();
    filter_block
}
```

### Querying a Filter Block

```rust
use bitcoinleveldb_filter::FilterBlockReader;
use bitcoinleveldb_filter::new_bloom_filter_policy;

fn probe_key(filter_block: &Slice, block_offset: u64, key: &Slice) -> bool {
    let policy = new_bloom_filter_policy(10); // must match the writer
    let reader = FilterBlockReader::new(policy, filter_block);

    reader.key_may_match(block_offset, key)
}
```

In an integrated LevelDB-like system, you will:

1. Build filter blocks while constructing SSTables.
2. Persist filter blocks alongside data and index blocks.
3. Reload the filter block with `FilterBlockReader` when opening the table.
4. Use `key_may_match` as a fast pre-check before binary searching or performing disk I/O for the data block.

## Safety and Invariants

- `add_key` uses `unsafe` to reinterpret a `Slice` as `&[u8]`. The correctness of that operation depends on `Slice` containing a valid pointer and length for the lifetime of the builder.
- `decode_fixed32` assumes that `src.len() >= 4`; misuse is undefined behavior.
- `finish` returns a `Slice` borrowing from `self.result`; do not drop or mutate `FilterBlockBuilder` in ways that invalidate `self.result` while that `Slice` is in use.
- `FilterBlockReader::new` copies the input bytes into an `Arc<[u8]>`, so the `Slice` passed to it does not need to outlive the reader.

## Integration Context

This crate is part of `bitcoin-rs`:

- Repository: <https://github.com/klebs6/bitcoin-rs>
- Likely used by the LevelDB reimplementation or bindings inside that repository to support Bitcoin-compatible storage layouts.

When integrating, keep the following aligned:

- `FILTER_BASE` and `FILTER_BASE_LG` must match between the writer and reader.
- The same `FilterPolicy` implementation (or a wire-compatible version) must be used to create and query filter regions.
- Any on-disk format expectations (e.g., Bitcoin Core’s) must be reflected in the policy’s encoding strategy.

## License and Metadata

- **License**: MIT
- **Edition**: Rust 2021
- **Authors**: `klebs <none>`

This design emphasizes low-level control and compatibility with existing LevelDB-based ecosystems while remaining idiomatic enough to compose nicely with the rest of `bitcoin-rs`.
