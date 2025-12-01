# bitcoinleveldb-hash

A minimal, `no_std`-friendly implementation of LevelDB's 32‑bit hash primitive, extracted for use in Bitcoin-related and embedded contexts.

---

## Overview

`bitcoinleveldb-hash` implements the 32‑bit hash function used internally by Google's LevelDB, exposed as a single low‑level function that operates on raw pointers. This hash is used by LevelDB for:

- Hash table bucketing
- Bloom filter key hashing
- Lightweight checksumming / sharding logic

The implementation in this crate is a faithful Rust port of the original C++ routine, preserving:

- Exact 32‑bit wrapping arithmetic
- Little‑endian decoding semantics
- Tail‑byte mixing behavior
- Seed‑dependent initialization

The function signature:

```rust
pub fn leveldb_hash(data: *const u8, n: usize, seed: u32) -> u32
```

This is intentionally low level and unsafe‑adjacent (raw address, explicit length). The interface is suitable for high‑performance code where you already manipulate buffers as `(ptr, len)` pairs (e.g. database internals, FFI, or custom allocators).

The algorithm is structurally similar to a 32‑bit Murmur‑style hash: it performs block‑wise mixing using a fixed multiplier and shift, then folds remaining bytes with a small tail routine. Specifically, it uses constant `M = 0xc6a4a793` and a right shift `R = 24` during finalization, closely mirroring LevelDB's original implementation.

---

## Features

- **Deterministic LevelDB‑compatible hash**: Intended to produce the same output as LevelDB's C++ `Hash()` function for identical inputs and seed.
- **Low‑level pointer interface**: Accepts `*const u8` and a byte length; ideal where copying or slice conversion would be overhead.
- **`no_std` friendly core**: The fundamental logic relies only on `core` and raw pointers; logging uses macros from the `log` ecosystem but can be compiled out.
- **Seeded hashing**: The `seed` parameter allows domain separation and mitigation of trivial collision patterns.

---

## Safety model

The function is declared `pub fn` (not `unsafe fn`), but *internally* uses `unsafe` blocks to read from `data`. As a user, you **must** uphold the following invariants:

1. `data` must be a valid, non‑null pointer to at least `n` bytes of readable memory.
2. The region `[data, data.add(n))` must be fully initialized.
3. The memory region may be aliased, but only read (no concurrent mutation that would cause undefined behavior with raw reads).

Violating these invariants results in undefined behavior. If you prefer a safe, slice‑based wrapper, add one in your own crate, for example:

```rust
use bitcoinleveldb_hash::leveldb_hash;

pub fn leveldb_hash_slice(data: &[u8], seed: u32) -> u32 {
    // Safety: &[u8] guarantees a contiguous, initialized region.
    unsafe { leveldb_hash(data.as_ptr(), data.len(), seed) }
}
```

---

## Usage

Add to `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-hash = "0.1.19"
```

### Basic example

```rust
use bitcoinleveldb_hash::leveldb_hash;

fn main() {
    let key = b"example-key";
    let seed: u32 = 0x1234_5678;

    let hash = unsafe { leveldb_hash(key.as_ptr(), key.len(), seed) };

    println!("hash = 0x{hash:08x}");
}
```

### Using as a bucket index

```rust
use bitcoinleveldb_hash::leveldb_hash;

fn bucket_index(key: &[u8], seed: u32, buckets: usize) -> usize {
    assert!(buckets.is_power_of_two(), "buckets should be a power of two");

    let h = unsafe { leveldb_hash(key.as_ptr(), key.len(), seed) };
    (h as usize) & (buckets - 1)
}
```

This mirrors LevelDB's use of a 32‑bit hash for indexing into small hash tables.

### Bloom‑filter style application

For a Bloom filter, one can derive multiple indices from a single hash by simple linear transformations:

```rust
use bitcoinleveldb_hash::leveldb_hash;

fn bloom_positions(key: &[u8], seed: u32, bits: u32, k: usize) -> Vec<u32> {
    let base = unsafe { leveldb_hash(key.as_ptr(), key.len(), seed) };
    let delta = (base >> 17) | (base << 15); // simple rotation

    (0..k)
        .map(|i| base.wrapping_add((i as u32).wrapping_mul(delta)) & (bits - 1))
        .collect()
}
```

This pattern parallels the construction used in LevelDB's own Bloom filter implementation.

---

## Algorithmic notes

The internal structure of `leveldb_hash` is:

1. **Initialization**
   ```rust
   const M: u32 = 0xc6a4a793;
   const R: u32 = 24;

   let mut h: u32 = seed ^ ((n as u32).wrapping_mul(M));
   ```
   The input length is mixed with a fixed multiplier and XORed with the seed. This introduces sensitivity to both size and seed at the beginning of the chain.

2. **Block processing** (4 bytes at a time, little‑endian):
   ```rust
   while offset + 4 <= n {
       let mut word_bytes = [0u8; 4];
       unsafe {
           let chunk_ptr = data.add(offset);
           core::ptr::copy_nonoverlapping(chunk_ptr, word_bytes.as_mut_ptr(), 4);
       }
       let w = u32::from_le_bytes(word_bytes);

       h = h.wrapping_add(w);
       h = h.wrapping_mul(M);
       h ^= h >> 16;

       offset += 4;
   }
   ```
   Each 32‑bit word is added, multiplied by `M`, and subjected to a right‑shift XOR. This creates diffusion and avalanche behavior typical of non‑cryptographic hash functions.

3. **Tail processing** (1–3 bytes):
   Remaining bytes are folded into `h` with left shifts by 8 and 16, then multiplied and mixed again:
   ```rust
   match remaining {
       3 => { /* add bytes << 16, 8, 0; mul M; h ^= h >> R; */ }
       2 => { /* add bytes << 8, 0;  mul M; h ^= h >> R; */ }
       1 => { /* add byte;        mul M; h ^= h >> R; */ }
       _ => unreachable!(),
   }
   ```

All arithmetic uses `wrapping_*` operations to match the C++ semantics on 32‑bit unsigned integers.

This hash is **not cryptographically secure**. It is suitable for internal indexing, partitioning, and approximate data structures (Bloom filters, count‑min sketches), but **not** for adversarial settings where second‑preimage or collision resistance is required.

---

## Logging and diagnostics

The implementation calls `trace!`, `debug!`, and `error!` macros. If you enable the `log` crate in your application and configure a logger (e.g. `env_logger`), you can observe the internal processing for debugging or verification:

```rust
fn main() {
    env_logger::init();

    let data = b"debug-hash";
    let seed = 0xdead_beef;

    let _ = unsafe { bitcoinleveldb_hash::leveldb_hash(data.as_ptr(), data.len(), seed) };
}
```

With `RUST_LOG=trace`, you will see per‑chunk trace output, including offsets and intermediate hash values.

In production builds, you can disable logging or set higher log levels to eliminate runtime logging overhead.

---

## Interoperability with LevelDB and Bitcoin code

Because this crate targets bit‑for‑bit compatibility with LevelDB's C++ hash algorithm, it can be used to:

- Reproduce LevelDB hash table indices computed by other implementations.
- Port filter policies (e.g. Bloom filters) from C++ LevelDB / Bitcoin Core to Rust.
- Validate that a Rust reimplementation of storage logic produces identical layout to an existing LevelDB store.

When interoperating, ensure you:

- Use the same byte order (little‑endian, as encoded in LevelDB keys/values).
- Match the seed exactly.
- Hash the same serialized key representation, including prefixes and varint‑encoded fields if present.

---

## Repository and maintenance

This crate lives inside the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Issues and pull requests should be filed there, under the appropriate module or crate directory.

---

## License

`bitcoinleveldb-hash` is distributed under the MIT license.

See the `LICENSE` file in the repository for full terms.
