# bitcoin-hash

Deterministic, Bitcoin-Core‑faithful hashing utilities and hash‑centric data structures for the `bitcoin-rs` ecosystem.

This crate provides thin, allocation‑free primitives that closely mirror Bitcoin Core's C++ hashing infrastructure while exposing an idiomatic Rust API. It focuses on correctness with respect to consensus‑relevant behavior and on zero‑cost integration into performance‑sensitive code such as full node implementations, mempools, UTXO sets, and filter indexes.

---

## Features at a Glance

- **Consensus‑compatible digests**
  - `Hash256`: Bitcoin's double‑SHA256 (256‑bit) hash function.
  - `Hash160` and `hash160(..)`: Bitcoin's 160‑bit hash (SHA256 ➜ RIPEMD160).
  - `hash1`, `hash2`, `serialize_hash`: convenient helpers mirroring Bitcoin Core semantics.
- **Tagged hashes (BIP‑340 style)**
  - `tagged_hash(tag: &str) -> HashWriter` and pre‑tagged writers for Taproot:
    - `HASHER_TAPSIGHASH`, `HASHER_TAPLEAF`, `HASHER_TAPBRANCH`, `HASHER_TAPTWEAK`.
- **Specialized hashers for core data structures**
  - `SaltedOutpointHasher`, `SaltedTxidHasher`, `ByteVectorHash`, `SaltedSipHasher`.
  - `SignatureCacheHasher` implementing `bitcoin_cuckoo_cache::EightWayHasher<u256>`.
  - `BlockHasher`, `FilterHeaderHasher` for cheap indexing based on low‑64‑bits.
- **Stream‑based hashing**
  - `HashWriter`: a write‑only, serialization‑style hasher.
  - `HashVerifier<S>`: wraps an `std::io::Read` and hashes all bytes as they are read.
- **Bitcoin data model utilities**
  - `OutPoint`: transaction outpoint (`txid` + `vout` index) with ordering, hashing, and display.
  - `BaseHash<HashType>`: a general fixed‑size hash wrapper mirroring Bitcoin Core's `BaseHash` helpers.
  - `AssumeUtxoHash`, `AssumeUtxoData`, `MapAssumeUtxo` for assumeUTXO snapshot metadata.
- **Low‑level helpers**
  - `murmur_hash3`, `read_le64`, `rotl32` – building blocks for filters, caches, and Bloom‑like structures.

The design goal is to make it trivial to port consensus‑critical code from Bitcoin Core while retaining Rust's type safety and trait ecosystem.

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-hash = "0.1.20"
```

The crate targets Rust 2021 and is licensed under MIT.

---

## Core Types and Functions

### 1. Fundamental hash types

#### `Hash256`

Bitcoin defines its main digest function as `double-SHA256`:

\[
H_{256}(m) = \text{SHA256}(\text{SHA256}(m)).
\]

`Hash256` is a stateful hasher implementing this exact construction:

```rust
use bitcoin_hash::Hash256;

let mut h = Hash256::default();
h.write(b"hello world");
let mut out = [0u8; Hash256::OUTPUT_SIZE];
h.finalize(&mut out);
```

Key properties:

- Streaming interface (`write`, `finalize`, `reset`).
- Zero allocation; works with fixed‑size arrays.
- Matches Bitcoin Core's behavior byte‑for‑byte when fed the same serialized data.

#### `Hash160` and `hash160`

Bitcoin's 160‑bit hash for P2PKH/P2WPKH, address generation, and script templates is defined as:

\[
H_{160}(m) = \text{RIPEMD160}(\text{SHA256}(m)).
\]

`Hash160` is the streaming variant; `hash160` is a convenience helper operating on any `AsRef<[u8]>`:

```rust
use bitcoin_hash::hash160;
use bitcoin_hash::u160; // assumed to come from the same crate or a sibling

let pk_bytes: Vec<u8> = get_public_key_bytes();
let digest: u160 = hash160(&pk_bytes);
```

### 2. High‑level helpers: `hash1`, `hash2`, `serialize_hash`

These functions capture common patterns from Bitcoin Core:

```rust
use bitcoin_hash::{hash1, hash2, serialize_hash, u256};

let a = b"foo";
let b = b"bar";

let h_single: u256 = hash1(a);
let h_concat: u256 = hash2(a, b);

// `serialize_hash` exists for API compatibility with the C++ codebase.
let h_ser: u256 = serialize_hash(a, None, None);
```

- `hash1` – hash a single blob of bytes with `Hash256`.
- `hash2` – hash the concatenation of two blobs (`in1 || in2`).
- `serialize_hash` – placeholder for (de)serialization‑aware hashing; in the Rust port, it simply calls `hash1(obj.as_ref())`.

### 3. Tagged hashes (BIP‑340, Taproot)

Tagged hashes are defined as in BIP‑340:

\[
\text{tagged\_hash}(\text{tag}, m) = \text{SHA256}(\text{SHA256}(\text{tag})\,||\,\text{SHA256}(\text{tag})\,||\,m).
\]

`tagged_hash(tag)` returns a `HashWriter` already primed with `SHA256(tag)` written twice. You then feed the payload and finalize via `HashWriter`:

```rust
use bitcoin_hash::{tagged_hash, HashWriter, u256};

let mut w: HashWriter = tagged_hash("TapLeaf");
w.write(&serialized_script);
let tapleaf_hash: u256 = w.get_hash();
```

The crate also exposes pre‑instantiated `lazy_static!` hash writers for the commonly used Taproot tags:

- `HASHER_TAPSIGHASH` – BIP‑341 `TapSighash`.
- `HASHER_TAPLEAF` – `TapLeaf`.
- `HASHER_TAPBRANCH` – `TapBranch`.
- `HASHER_TAPTWEAK` – `TapTweak`.

These mirror the C++ global singletons and allow direct 1‑to‑1 translation of Taproot‑related code.

### 4. `HashWriter`: serialization‑style hashing

`HashWriter` is a SHA256‑based writer that matches Bitcoin Core’s `CHashWriter`:

```rust
use bitcoin_hash::{HashWriter, u256};

// Type and version parameters are for structural compatibility
let mut hw = HashWriter::new(SER_GETHASH as i32, 0);

hw.write(&tx_bytes);
let txid: u256 = hw.get_hash();       // double-SHA256
let cheap: u64 = hw.get_cheap_hash(); // low 64 bits of the double-SHA256
```

Important methods:

- `write(&mut self, pch: &[u8])` – feed bytes.
- `get_hash(&mut self) -> u256` – compute double‑SHA256; invalidates the internal state.
- `getsha256(&mut self) -> u256` – compute a single SHA256; invalidates the state.
- `get_cheap_hash(&mut self) -> u64` – the first 64 bits of `get_hash()`.

The type and version fields (`n_type`, `n_version`) enable future extensibility and parity with C++ serialization code, even though they may not affect the Rust implementation today.

Operator overloads:

```rust
use bitcoin_hash::HashWriter;

let mut w = HashWriter::new(0, 0);
let data = b"payload";

// `<<` style chaining (mirrors `operator<<` in C++)
let mut w2 = w << &data[..];
let h = w2.get_hash();
```

### 5. Stream hashing with `HashVerifier<S>`

`HashVerifier<S>` wraps an `std::io::Read` and forwards all bytes into an internal `HashWriter`:

```rust
use bitcoin_hash::{HashVerifier, u256};
use std::fs::File;
use std::io::Read;

let file = File::open("block.dat")?;
let mut hv = HashVerifier::new(file);

let mut buf = vec![0u8; 1024];
hv.read(&mut buf)?;   // reads and hashes 1024 bytes

// … read or `ignore` more …

// finalize by pulling the hash out of the embedded HashWriter
let hash: u256 = hv.base.get_hash();
# Ok::<(), std::io::Error>(())
```

Additional capabilities:

- `read(&mut self, buf: &mut [u8])` – `Read::read_exact` + hash.
- `ignore(&mut self, n: usize)` – read and hash but discard the bytes.
- `Shr<&mut T>` operator (`>>`‑like) that reads exactly `obj.as_mut().len()` and panics on I/O failure (mimicking C++ stream exceptions).

This is useful for validating that a streamed serialization matches an expected digest without buffering the entire payload.

### 6. `OutPoint`: transaction output locator

`OutPoint` represents a specific output of a transaction:

```rust
use bitcoin_hash::{OutPoint, u256};

let txid: u256 = get_txid();
let op = OutPoint::new(&txid, 0);

assert!(!op.is_null());
println!("{}", op); // e.g. "OutPoint(0123abc..., 0)"
```

Salient traits and methods:

- `new(hash_in: &u256, n_in: u32) -> Self`.
- `set_null`, `is_null` – sentinel semantics with `OUT_POINT_NULL_INDEX`.
- `Display`/`to_string` – prints `OutPoint(<first 10 hex chars>, n)` (matching Bitcoin Core’s `ToString`).
- `Ord`, `Eq`, `Hash` – uses `(hash, n)` ordering and a stable Hash implementation by writing `hash` bytes and `n` in little‑endian.

The `Hash` impl makes it suitable as a key in `HashMap<OutPoint, _>` or `HashSet<OutPoint>` using default hashers or the provided salted variants.

### 7. Salted hashers and indexing strategies

The crate includes several SipHash‑based and structure‑aware hashers intended for use in high‑performance containers. Salting randomizes hash functions per process, mitigating collision attacks and certain denial‑of‑service vectors.

#### `SaltedOutpointHasher`

Implements both `BuildHasher` and `Hasher`, enabling use as a custom hasher for `HashMap`/`HashSet` keyed by `OutPoint`:

```rust
use bitcoin_hash::{OutPoint, SaltedOutpointHasher};
use std::collections::HashMap;

let map: HashMap<OutPoint, u64, SaltedOutpointHasher> = HashMap::default();
```

Additionally, it provides an explicit `invoke` helper mirroring Bitcoin Core's `SipHashUint256Extra`:

```rust
use bitcoin_hash::{OutPoint, SaltedOutpointHasher};

let hasher = SaltedOutpointHasher::default();
let op = get_outpoint();
let idx: usize = hasher.invoke(&op);
```

The implementation uses two 64‑bit keys (`k0`, `k1`) seeded from `rand::rngs::OsRng` and SipHash‑2‑4 internally.

#### `SaltedTxidHasher`

Specialized SipHash‑based hasher for 256‑bit transaction IDs (`u256`):

```rust
use bitcoin_hash::{SaltedTxidHasher, u256};

let txid: u256 = get_txid();
let h = SaltedTxidHasher::default();
let bucket: usize = h.invoke(&txid);
```

#### `ByteVectorHash`

A general SipHash‑2‑4 based `BuildHasher` and `Hasher` intended for arbitrary byte‑vector types used as keys:

```rust
use bitcoin_hash::ByteVectorHash;
use std::collections::HashSet;

let mut set: HashSet<Vec<u8>, ByteVectorHash> = HashSet::default();
set.insert(b"hello".to_vec());
```

Includes a convenience `invoke(&self, input: &[u8]) -> usize` for ad‑hoc hashing.

#### `SaltedSipHasher`

A lighter helper using SipHash with random keys for generic byte slices:

```rust
use bitcoin_hash::SaltedSipHasher;

let s = SaltedSipHasher::default();
let idx = s.invoke(b"some script bytes");
```

#### `SignatureCacheHasher`

Implements `bitcoin_cuckoo_cache::EightWayHasher<u256>`.

Mathematically, this decomposes a `u256` (32 bytes) into eight disjoint 32‑bit words, each producing one of the eight hash values required by the cuckoo cache structure:

```rust
use bitcoin_hash::{SignatureCacheHasher, u256};
use bitcoin_cuckoo_cache::EightWayHasher;

let key: u256 = get_sigcache_key();
let h = SignatureCacheHasher {};
let buckets: [u32; 8] = h.hashes(&key);
```

This is designed specifically for Bitcoin's signature cache implementation, avoiding recomputation and extra passes over the data.

#### `BlockHasher` and `FilterHeaderHasher`

Both use `read_le64(hash.as_ref())` to derive a `usize` index from the low 64 bits of a 256‑bit hash.

These are useful for cheap indexing where full cryptographic collision resistance is not required (e.g., block/file bucketing, compact filter headers).

### 8. AssumeUTXO structures

The AssumeUTXO mechanism allows nodes to bootstrap validation by accepting an externally supplied UTXO snapshot, identified by a consensus‑critical hash and height/tx‑count metadata.

This crate models the relevant pieces:

- `AssumeUtxoHash { base: BaseHash<u256> }` – wrapper for the hash of the serialized UTXO snapshot.
- `AssumeUtxoData { hash_serialized: AssumeUtxoHash, n_chain_tx: u32 }` – configuration for UTXO snapshot load/validation.
- `MapAssumeUtxo = HashMap<i32, AssumeUtxoData>` – map from block height (or similar integer index) to snapshot data.

Usage example:

```rust
use bitcoin_hash::{AssumeUtxoHash, AssumeUtxoData, MapAssumeUtxo, u256};

let snapshot_hash: u256 = compute_snapshot_hash();
let au_hash = AssumeUtxoHash::new(&snapshot_hash);

let data = AssumeUtxoData {
    hash_serialized: au_hash,
    n_chain_tx: 1_000_000,
};

let mut map: MapAssumeUtxo = MapAssumeUtxo::new();
map.insert(700_000, data);
```

Because this configuration is security‑critical, any changes must be coordinated with consensus rules and deployment strategy.

### 9. `BaseHash<HashType>`: generic fixed‑size hash wrapper

`BaseHash<HashType>` encapsulates a fixed‑width hash type (e.g., `u256`) while exposing a pointer‑centric API like Bitcoin Core's `BaseHash` helpers.

Constraints on `HashType`:

- `Clone + Default + Debug + AsRef<[u8]> + AsMut<[u8]> + Ord + Display`.

Key methods:

```rust
use bitcoin_hash::BaseHash;
use bitcoin_hash::u256;

let inner: u256 = get_u256();
let mut b = BaseHash::new(&inner);

let size = b.size();       // length in bytes
let ptr = b.data();        // *const u8
let s = b.to_string();     // hex string via inner Display

// Transparent deref
let as_inner: &u256 = &*b;
```

Pointers and ranges:

- `begin`, `end`, `begin_mut`, `end_mut`, `data`, `data_mut` – all mirror C++ semantics.
- Ordering (`Ord`, `Eq`, `PartialOrd`) forwards to the inner type.
- `Into<Vec<u8>>` clones the underlying bytes.

This type is primarily aimed at ease of porting C++ code that expects raw pointer access, while preserving Rust ownership and borrowing rules.

### 10. Low‑level primitives: `murmur_hash3`, `read_le64`, `rotl32`

#### `murmur_hash3`

Implements 32‑bit MurmurHash3 over a byte slice with an explicit seed:

```rust
use bitcoin_hash::murmur_hash3;

let seed: u32 = 0x1234_5678;
let val = b"key";
let h: u32 = murmur_hash3(seed, val);
```

This is the same primitive used in Bitcoin Core for Bloom filters, compact filters, and certain probabilistic data structures. The function operates in little‑endian and performs finalization mixing steps as in the reference design.

#### `read_le64`

Reads the first eight bytes of a slice as a little‑endian `u64`:

```rust
use bitcoin_hash::read_le64;

let bytes = [1u8, 0, 0, 0, 0, 0, 0, 0];
let x = read_le64(&bytes);
assert_eq!(x, 1);
```

Used for cheap hashing and indexing where full collision resistance is unnecessary.

#### `rotl32`

A small wrapper over `u32::rotate_left`:

```rust
use bitcoin_hash::rotl32;

let x = 0x1234_5678u32;
let y = rotl32(x, 5); // rotate left by 5 bits
```

Primarily used inside hashing primitives; supplied to keep ported code structurally identical.

---

## Error Handling and Invariants

- Many methods are annotated with `debug_assert!` checks rather than returning `Result`. These are designed to catch internal misuse during development without impacting release performance.
- `HashVerifier::read` and `ignore` surface `std::io::Result<()>`, but the `Shr` operator implementation panics on I/O failure to emulate C++ stream exceptions.
- Hashing functions expect correctly sized inputs; where necessary they copy fixed‑length slices with explicit bounds checks.

Downstream consumers should treat these primitives as low‑level building blocks with the expectation that higher‑level components handle user‑facing validation and error reporting.

---

## Example: hashing a serialized transaction

```rust
use bitcoin_hash::{HashWriter, u256};

fn txid_from_serialized_bytes(tx_bytes: &[u8]) -> u256 {
    let mut hw = HashWriter::new(SER_GETHASH as i32, 0);
    hw.write(tx_bytes);
    hw.get_hash()
}
```

If you already have a single, contiguous slice of the serialized transaction, you may instead use `hash1(tx_bytes)`.

---

## Example: using custom hashers for collections

```rust
use bitcoin_hash::{OutPoint, SaltedOutpointHasher, u256};
use std::collections::HashMap;

let mut utxo: HashMap<OutPoint, u64, SaltedOutpointHasher> = HashMap::default();

let txid: u256 = get_txid();
let op = OutPoint::new(&txid, 1);
utxo.insert(op.clone(), 42);

assert_eq!(utxo.get(&op), Some(&42));
```

Because `SaltedOutpointHasher` implements `BuildHasher`, `HashMap::default()` yields a map with that hasher and fresh per‑process SipHash keys.

---

## Repository, License, and Contributions

- **Crate**: `bitcoin-hash` `0.1.20`
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **License**: MIT
- **Rust edition**: 2021
- **Author**: `klebs <none>`

Contributions should respect consensus compatibility with Bitcoin Core. Any change to hashing behavior, serialization, or snapshot verification logic must be carefully reviewed, as subtle divergences can lead to consensus splits or security regressions.
