# bitcoin-golombrice

A minimal, BIP‑158–oriented implementation of Golomb‑Rice coding and Golomb‑coded sets (GCS) for Bitcoin compact block filters.

---

## Overview

`bitcoin-golombrice` implements the core primitives required for [BIP‑158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki) Golomb‑coded filters:

- **Golomb‑Rice scalar codec**
  - `golomb_rice_encode` – bit‑level encoder over an arbitrary `Write`.
  - `golomb_rice_decode` – bit‑level decoder over an arbitrary `Read`.
- **GCSFilter** – a compact, probabilistic set representation for membership tests with tunable false‑positive rate.
- **GcsFilterParams** – filter parameterization (SipHash keys, Golomb parameter `p`, and scaling factor `m`).

The implementation is aimed at interoperability with Bitcoin Core’s `GCSFilter` (parameterization and serialization layout), making it suitable for SPV/light‑client workflows and Bitcoin indexers.

The crate focuses on:

- Close tracking of the BIP‑158 construction.
- Deterministic encoding/decoding via bit‑precise I/O.
- Efficient membership checks with predictable false‑positive bounds.

## Core Concepts

### Golomb‑Rice coding

A non‑negative integer \( x \) is mapped to a pair \((q, r)\):

- \( q = \lfloor x / 2^p \rfloor \) – quotient
- \( r = x \bmod 2^p \) – remainder (\(p\) bits)

Encoding:

1. Write `q` as *unary* code: `q` ones followed by a zero.
2. Write `r` as `p` raw bits.

Decoding inverts this process. This is exactly what

```rust
pub fn golomb_rice_encode<OStream>(
    bitwriter: &mut BitStreamWriter<OStream>,
    p: u8,
    x: u64,
) where
    OStream: Default + Write
{ /* … */ }

pub fn golomb_rice_decode<IStream>(
    bitreader: &mut BitStreamReader<IStream>,
    p: u8,
) -> u64
where
    IStream: Default + Read
{ /* … */ }
```

implements. The parameter `p` trades off code length and quotient distribution. BIP‑158 uses a fixed `p` per filter type.

### Golomb‑coded sets (GCS)

A GCS over domain elements \(E\) uses:

- A keyed hash \( h: E \rightarrow [0, f) \), implemented here via SipHash with keys `siphash_k0`, `siphash_k1`.
- Sorting of hashed values and Golomb‑Rice coding of their **deltas**.

Membership queries are probabilistic:

- If `match_` / `match_any` returns `false`, the element is *definitely not* in the set.
- If it returns `true`, the element is in the set **or** it is a false positive with probability bounded by \(1/m\).

BIP‑158 chooses `m` to obtain a target false‑positive rate (e.g. \(1/784931\) for block filters).

---

## Data Types

```rust
pub type GcsFilterElement = Vec<u8>;

pub type GcsFilterElementSet = std::collections::HashSet<GcsFilterElement, ByteVectorHash>;
```

Elements are arbitrary byte vectors; the set requires a `ByteVectorHash` hasher (typically a byte‑wise hash over `Vec<u8>` to use as `HashSet` key).

### `GcsFilterParams`

```rust
#[derive(Debug, Clone, Getters, Builder)]
#[builder(setter(into), default)]
#[getset(get = "pub")]
pub struct GcsFilterParams {
    siphash_k0: u64,
    siphash_k1: u64,
    p:          u8,
    m:          u32,
}
```

- `siphash_k0`, `siphash_k1`: SipHash‑2‑4 keys used to hash elements to `[0, n * m)`.
- `p`: Golomb‑Rice parameter (number of remainder bits).
- `m`: scaling factor; false‑positive probability is ~`1 / m`.

Construction:

```rust
impl GcsFilterParams {
    pub fn new(
        siphash_k0: Option<u64>,
        siphash_k1: Option<u64>,
        p:          Option<u8>,
        m:          Option<u32>,
    ) -> Self { /* … */ }
}

impl Default for GcsFilterParams { /* … */ }
```

`new` mirrors the C++ `GCSFilter::Params` constructor: each argument is optional, with defaults `(0, 0, 0, 1)` for `(k0, k1, p, m)`.

Using the derived builder:

```rust
use bitcoin_golombrice::GcsFilterParamsBuilder;

let params = GcsFilterParamsBuilder::default()
    .siphash_k0(0x1234_5678_9abc_def0)
    .siphash_k1(0xfedc_ba98_7654_3210)
    .p(20u8)
    .m(784_931u32)
    .build()
    .unwrap();
```

### `GCSFilter`

```rust
#[derive(Builder, Debug, Clone, Getters, Default)]
#[getset(get = "pub")]
#[builder(setter(into))]
pub struct GCSFilter {
    params:  GcsFilterParams,
    n:       u32,
    f:       u64,
    encoded: Vec<u8>,
}
```

- `params`: filter parameterization.
- `n`: element count in the set.
- `f`: equal to `n as u64 * m as u64`; hash range upper bound.
- `encoded`: serialized form, starting with Bitcoin CompactSize `n`, followed by Golomb‑Rice encoded deltas.

Helper accessors:

```rust
impl GCSFilter {
    pub fn getn(&self) -> u32 { /* … */ }
    pub fn get_params(&self) -> &GcsFilterParams { /* … */ }
    pub fn get_encoded(&self) -> &Vec<u8> { /* … */ }
}
```

A `From<Option<GcsFilterParams>>` implementation allows constructing an *empty* filter encoding `CompactSize(0)` with chosen or default parameters:

```rust
let filter: GCSFilter = None::<GcsFilterParams>.into();
assert_eq!(filter.getn(), 0);
```

---

## Creating Filters

### From an element set

```rust
impl GCSFilter {
    /// Build a filter from a concrete element set.
    pub fn new_with_element_set(
        params: &GcsFilterParams,
        elements: &GcsFilterElementSet,
    ) -> Self { /* … */ }
}
```

Usage example:

```rust
use std::collections::HashSet;
use bitcoin_golombrice::{
    GCSFilter,
    GcsFilterParams,
    GcsFilterElement,
    GcsFilterElementSet,
};

fn build_filter(params: &GcsFilterParams) -> GCSFilter {
    let mut set: GcsFilterElementSet = HashSet::default();

    let key1: GcsFilterElement = b"scriptpubkey-1".to_vec();
    let key2: GcsFilterElement = b"scriptpubkey-2".to_vec();

    set.insert(key1);
    set.insert(key2);

    GCSFilter::new_with_element_set(params, &set)
}
```

Semantics:

1. Compute `n = elements.len()`, `f = n * m`.
2. Serialize `CompactSize(n)`.
3. Hash each element via SipHash with `siphash_k0`, `siphash_k1` into `[0, f)`.
4. Sort the hashed values, delta‑encode them, then Golomb‑Rice encode each delta with parameter `p`.

### From an already‑encoded filter

```rust
impl GCSFilter {
    pub fn new_with_encoded_filter(
        params: &GcsFilterParams,
        encoded_filter: Vec<u8>,
    ) -> Self { /* … */ }
}
```

This constructor validates consistency:

- Decodes `CompactSize(n)`.
- Replays Golomb‑Rice decoding for `n` values.
- Ensures no surplus bytes remain (`panic!`s otherwise).

Useful when you obtain the encoded filter from the network or from Bitcoin Core and want a typed wrapper and query interface.

---

## Membership Queries

### Hashing into the filter range

```rust
impl GCSFilter {
    /// Hash a data element to an integer in the range [0, N * M).
    pub fn hash_to_range(&self, element: &GcsFilterElement) -> u64 { /* … */ }

    pub fn build_hashed_set(
        &self,
        elements: &GcsFilterElementSet,
    ) -> Vec<u64> { /* … */ }
}
```

- `hash_to_range` applies keyed SipHash and projects into `[0, f)` via `map_into_range`.
- `build_hashed_set` hashes all elements, reserves capacity, and sorts.

### Single‑element membership

```rust
impl GCSFilter {
    /// Checks if the element may be in the set.
    /// False positives are possible with probability 1/M.
    pub fn match_(&self, element: &GcsFilterElement) -> bool { /* … */ }
}
```

Usage:

```rust
let element: GcsFilterElement = b"scriptpubkey-1".to_vec();
if filter.match_(&element) {
    // Element is either present or a false positive.
}
```

### Multi‑element membership (batched)

```rust
impl GCSFilter {
    /// Checks if any of the given elements may be in the set.
    /// False positives are possible with probability 1/M per element.
    /// More efficient than calling `match_` repeatedly.
    pub fn match_any(&self, elements: &GcsFilterElementSet) -> bool { /* … */ }
}
```

This path hashes and sorts queries once, then streams through the encoded filter via `match_internal` using a classic two‑pointer merge technique.

Example:

```rust
let mut queries: GcsFilterElementSet = HashSet::default();
queries.insert(b"candidate-1".to_vec());
queries.insert(b"candidate-2".to_vec());

if filter.match_any(&queries) {
    // At least one query may match the filter.
}
```

### Unsafe, pointer‑based core matcher

```rust
impl GCSFilter {
    pub fn match_internal(&self, element_hashes: *const u64, size: usize) -> bool { /* … */ }
}
```

`match_internal` operates on a raw pointer to a sorted array of query hashes and performs a streaming comparison against the decoded delta sequence. It:

- Reads `n` from the prefix.
- Iteratively decodes deltas using `golomb_rice_decode`.
- Maintains a cumulative `value` and an index `idx` into the query array.
- Returns early when it finds a match or when the query sequence is exhausted.

For most use cases, prefer `match_` and `match_any`, which wrap this safely.

---

## Serialization and Interop

The filter encoding layout matches Bitcoin Core’s `GCSFilter`:

1. `CompactSize(n)` – encoded with `write_compact_size` / `read_compact_size` helpers.
2. Golomb‑Rice encoded deltas of sorted hashes.

This makes the crate suitable for interoperating with:

- Block filters (`basic`, `extended`) as defined in BIP‑158.
- External Bitcoin libraries that expect the exact same wire format.

The implementation uses `VectorReader` / `VectorWriter` and `BitStreamReader` / `BitStreamWriter` abstractions from the surrounding codebase (`bitcoin-rs`) to guarantee bit‑accurate encoding.

---

## Parameter Selection and False Positives

Let:

- \( n \) = number of elements in the filter
- \( m \) = scaling factor in `GcsFilterParams`
- \( f = n \cdot m \) = hash range size

Then:

- Each element is hashed uniformly into `[0, f)` (modulo SipHash properties and `map_into_range`).
- BIP‑158’s analysis shows a per‑query false‑positive probability approximately `1 / m`.

Operational guidance:

- Choose `m` large enough for your acceptable false‑positive rate.
- Choose `p` to balance:
  - Code length (larger `p` yields shorter unary quotient, longer remainder).
  - Decoding CPU cost (more bits per value vs more unary runs).

For exact Bitcoin filter parameters, mirror the constants from BIP‑158 or from Bitcoin Core’s implementation and feed them into `GcsFilterParams`.

---

## Safety and Panics

This crate uses a few invariants:

- `new_with_encoded_filter` will `panic!` if the encoded filter contains trailing data beyond what decoding `n` elements consumes.
- `new_with_element_set` and parameter constructors assume `n < 2^32` (enforced via `try_from`).
- `match_internal` is `unsafe` in spirit (raw pointer input), though its signature is safe; incorrect `element_hashes` / `size` will cause undefined behavior. Use the safe wrappers instead.

When integrating into production systems, ensure that untrusted filters are validated at higher layers or handled with error propagation if you adapt this code.

---

## Example End‑to‑End Flow

```rust
use std::collections::HashSet;
use bitcoin_golombrice::{
    GCSFilter,
    GcsFilterParams,
    GcsFilterElement,
    GcsFilterElementSet,
};

fn main() {
    // 1. Define parameters (example values; use BIP‑158 constants for Bitcoin deployment).
    let params = GcsFilterParams::new(
        Some(0x1234_5678_9abc_def0),
        Some(0xfedc_ba98_7654_3210),
        Some(20),        // p
        Some(784_931),   // m – target false positive rate ~ 1/784931
    );

    // 2. Build an element set.
    let mut elems: GcsFilterElementSet = HashSet::default();
    elems.insert(b"elem-1".to_vec());
    elems.insert(b"elem-2".to_vec());

    // 3. Construct the filter.
    let filter = GCSFilter::new_with_element_set(&params, &elems);

    // 4. Serialize to bytes (e.g., for network transmission or storage).
    let encoded = filter.get_encoded().clone();

    // 5. Reconstruct a filter from bytes.
    let filter2 = GCSFilter::new_with_encoded_filter(&params, encoded);

    // 6. Query membership.
    let q: GcsFilterElement = b"elem-1".to_vec();
    assert!(filter2.match_(&q));
}
```

---

## License and Repository

- **License:** MIT
- **Repository:** <https://github.com/klebs6/bitcoin-rs>

This crate is part of a broader Bitcoin‑related Rust codebase. Consult the repository for example usage, integration points, and parameter constants matching Bitcoin Core.
