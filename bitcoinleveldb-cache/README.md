# bitcoinleveldb-cache

A faithful, test-focused Rust port of the cache subsystem used by Bitcoin Core's LevelDB fork. It provides an internal, concurrency-safe caching layer with explicit control over memory charge, eviction, and lifecycle semantics, suitable for embedding under higher-level storage engines.

---

## Overview

`bitcoinleveldb-cache` exposes a low-level cache abstraction that mimics the behavior and semantics of the original C++ LevelDB/Bitcoin cache implementation. The primary design goals are:

- **Deterministic memory accounting** via explicit `charge` for each value.
- **Capacity-bounded caching** with eviction when the total charge exceeds the configured capacity.
- **Opaque values** (`*mut c_void`) for maximal flexibility and interoperability with FFI-heavy or legacy code.
- **Reference-counted handles** so clients can pin entries and manage lifetimes precisely.
- **LRU-like eviction** implemented by tracking a logical clock and last-use timestamps.

The crate is primarily concerned with providing the cache core that Bitcoin's LevelDB adapter depends upon. It is not a general high-level Rust cache with ownership-safe value types; instead, it mirrors the original memory and pointer semantics used by Bitcoin's tests, including custom deleters and test fixtures.

---

## Core Concepts

### 1. Cache Interface and Traits

The cache API is specified as a trait composition. Any type implementing all the following traits automatically implements the marker trait `CacheInterface`:

- `CacheInsert`
- `CacheLookup`
- `CacheRelease`
- `CacheValue`
- `CacheErase`
- `CacheNewId`
- `CachePrune`
- `CacheTotalCharge`

This enables alternative cache implementations (e.g., scan-resistant or different eviction policies) while maintaining a common API boundary.

#### Trait Summary

```rust
pub trait CacheInsert {
    fn insert(
        &mut self,
        key_:    &Slice,
        value:   *mut c_void,
        charge:  usize,
        deleter: CacheDeleterFn,
    ) -> *mut CacheHandle;
}

pub trait CacheLookup {
    fn lookup(&mut self, key_: &Slice) -> *mut CacheHandle;
}

pub trait CacheRelease {
    fn release(&mut self, handle: *mut CacheHandle);
}

pub trait CacheValue {
    fn value(&mut self, handle: *mut CacheHandle) -> *mut c_void;
}

pub trait CacheErase {
    fn erase(&mut self, key_: &Slice);
}

pub trait CacheNewId {
    fn new_id(&mut self) -> u64;
}

pub trait CachePrune {
    fn prune(&mut self);
}

pub trait CacheTotalCharge {
    fn total_charge(&self) -> usize;
}

pub type CacheDeleterFn = fn(key_: &Slice, value: *mut c_void);
```

`CacheInterface` is then defined as:

```rust
impl<T> CacheInterface for T where
    T: CacheInsert
        + CacheLookup
        + CacheRelease
        + CacheValue
        + CacheErase
        + CacheNewId
        + CachePrune
        + CacheTotalCharge {}
```

### 2. Built-in LRU Cache

The main production implementation is the `Cache` type backed by a `CacheRep` struct. It is instantiated via:

```rust
pub fn new_lru_cache(capacity: usize) -> *mut Cache
```

- `capacity` is expressed in abstract **charge units**, not entry count.
- Each entry inserted with `charge` contributes to the aggregate `usage` of the cache.
- When `usage > capacity`, entries are evicted according to an approximate **least-recently-used** policy.

Internally:

- `Cache` stores `Rc<RefCell<CacheRep>>`, making it cloneable and internally synchronized at the Rust level.
- `CacheRep` manages a `HashMap<Vec<u8>, *mut CacheHandle>` that maps serialized keys to handle pointers.
- Eviction uses a monotonically increasing logical `clock` (u64) and `last_use` timestamps inside each `CacheHandle`. Candidates with `refs == 1` and the oldest `last_use` are selected for eviction.

### 3. Cache Handles and Reference Semantics

Each cache lookup/insert returns an opaque pointer to a `CacheHandle`:

```rust
#[derive(Getters, Setters, Builder)]
pub struct CacheHandle {
    key:      Vec<u8>,
    value:    *mut c_void,
    deleter:  CacheDeleterFn,
    charge:   usize,
    refs:     u32,
    in_cache: bool,
    last_use: u64,
}
```

Important invariants:

- `refs` counts strong references, including the internal cache reference if `in_cache == true`.
- User code must call `CacheRelease::release` when done with a handle, decrementing `refs`.
- When `refs` reaches 0, the following occur:
  - The associated `deleter` is invoked with the `Slice`-view of `key` and the stored `value` pointer.
  - The `CacheHandle` itself is deallocated.

This closely replicates the original C++ pointer-based design while making it explicit in Rust. Misuse (forgetting `release` or double-releasing) will generally show up as incorrect reference counts and log messages.

### 4. Key Encoding & Bitcoin/LevelDB Compatibility

A number of helpers are provided for converting between integer keys and the `Slice`/byte representation used internally:

```rust
pub fn encode_key(k: i32) -> Vec<u8>;
pub fn decode_key(k: &Slice) -> i32;
```

- Encoding uses **little-endian fixed32** semantics, matching LevelDB's `PutFixed32`.
- `decode_key` asserts that the key size is exactly 4 bytes and then reverses the encoding.

For values, two helpers exist primarily for tests:

```rust
pub fn decode_value(v: *mut c_void) -> i32;
pub fn encode_value(v: uintptr_t) -> *mut c_void;
```

`decode_value` expects a pointer to a `CacheTestValue` struct, which is test-only payload carrying both the owning fixture and a logical `i32` value.

### 5. Deleter Semantics

The cache never assumes ownership of your values beyond calling the `deleter` you provide at insertion time. This enables complex lifetime management across FFI boundaries.

In tests, the deleter is:

```rust
impl CacheTest {
    pub fn deleter(key_: &Slice, v: *mut c_void) {
        // Cast `v` to `Box<CacheTestValue>`, record deleted key/value on fixture,
        // then drop the box exactly once.
    }
}
```

In production use, you will typically wrap your data structures in a `Box` and provide a deleter that reconstructs the box, performs any side effects, and then allows Rust to drop the object.

---

## Usage

### Adding the Dependency

```toml
[dependencies]
bitcoinleveldb-cache = "0.1.19"
```

### Constructing a Cache

```rust
use bitcoinleveldb_cache::{new_lru_cache, CacheInterface, CacheInsert, CacheLookup, CacheRelease, CacheValue, CacheErase};
use std::ffi::c_void;

fn example_usage() {
    // Capacity in arbitrary charge units; choose according to your accounting model.
    let capacity: usize = 1024 * 1024; // e.g. 1 MiB equivalent

    // Allocate a new LRU cache on the heap.
    let cache_ptr = unsafe { new_lru_cache(capacity) };

    // SAFETY: `cache_ptr` is owned here; ensure you eventually reconstruct the Box and drop it.
    let cache: &mut bitcoinleveldb_cache::Cache = unsafe { &mut *cache_ptr };

    // Application-specific: encode a key into bytes then into the `Slice` type used by the cache.
    let key_bytes = bitcoinleveldb_cache::encode_key(42);
    let key_slice = bitcoinleveldb_cache::Slice::from_ptr_len(key_bytes.as_ptr(), key_bytes.len());

    // Suppose we have some heap-allocated payload.
    let payload = Box::new(MyPayload { /* ... */ });
    let payload_ptr: *mut c_void = Box::into_raw(payload) as *mut c_void;

    // Define a deleter that recovers the Box and drops it.
    fn my_deleter(_key: &bitcoinleveldb_cache::Slice, value: *mut c_void) {
        if value.is_null() {
            return;
        }
        unsafe {
            let _: Box<MyPayload> = Box::from_raw(value as *mut MyPayload);
            // Dropped on scope exit.
        }
    }

    // Insert into the cache with an explicit charge.
    let charge = 128usize; // e.g. approximate size in bytes
    let handle = cache.insert(&key_slice, payload_ptr, charge, my_deleter);

    // Immediately release our reference if we only want the cache to own it.
    cache.release(handle);

    // Later: look up the entry.
    let lookup_handle = cache.lookup(&key_slice);
    if !lookup_handle.is_null() {
        let value_ptr = cache.value(lookup_handle);
        // SAFETY: You must know the actual type; here we re-cast to `MyPayload`.
        let payload_ref: &mut MyPayload = unsafe { &mut *(value_ptr as *mut MyPayload) };
        // Use `payload_ref`...

        // Release when done.
        cache.release(lookup_handle);
    }

    // When completely finished with the cache, reconstruct the Box and drop it.
    unsafe {
        let boxed_cache = Box::from_raw(cache_ptr);
        drop(boxed_cache); // triggers Cache::drop, which clears entries and calls deleters
    }
}

struct MyPayload {
    // your data
}
```

### Capacity, Charge, and Eviction Model

Mathematically, you can think of the cache as maintaining:

- A non-negative capacity \( C \in \mathbb{N} \).
- A multiset of entries \( E = \{e_i\} \), each with an associated non-negative charge \( c_i \).
- A usage \( U = \sum_i c_i \).

The invariant is that ideally \( U \leq C \). When an insert causes \( U > C \), the cache repeatedly selects eviction candidates according to an approximate LRU order until \( U \leq C \) or no evictable entries remain.

Eviction candidates must satisfy:

- `in_cache == true`, i.e., the cache still owns a reference.
- `refs == 1`, meaning no other external user currently holds the handle.

Among such candidates, the one with smallest `last_use` (oldest) is removed first, implementing a discrete-time LRU policy.

This behavior matches a widely used pattern in storage engines: approximate LRU policies are usually sufficient to avoid pathological reuse patterns while keeping implementation complexity low.

### Pruning Unused Entries

The `CachePrune` trait allows manual compaction of entries that are not actively pinned by clients:

```rust
use bitcoinleveldb_cache::CachePrune;

fn prune(cache: &mut bitcoinleveldb_cache::Cache) {
    cache.prune();
}
```

`prune` will remove all entries where `in_cache == true` and `refs == 1`, freeing associated memory and invoking deleters. This is particularly useful for tests and memory-pressure simulations.

### Total Charge Introspection

You can inspect the total accumulated charge at any time:

```rust
use bitcoinleveldb_cache::CacheTotalCharge;

fn current_usage(cache: &bitcoinleveldb_cache::Cache) -> usize {
    cache.total_charge()
}
```

This is typically used by higher layers to monitor memory consumption and tune capacity or charge models.

---

## Testing Infrastructure

The crate includes a dedicated test harness with the following auxiliary types:

- `CacheTestValue` — an internal payload type storing:
  - `fixture: *mut CacheTest`
  - `value: i32`

- `CacheTest` — a fixture that owns a cache pointer and records deleted keys/values.

These are used to validate the correctness of deletion, eviction, and reference counting by:

- Inserting boxed `CacheTestValue`s into the cache.
- Using the test-only deleter to record when entries are dropped.

Utility methods such as `CacheTest::insert`, `CacheTest::lookup`, `CacheTest::erase`, and `CacheTest::insert_and_return_handle` are provided to reproduce the original C++ test matrix as closely as possible. Production users should not rely on these types, but they are illustrative of correct integration with the low-level API.

---

## Safety and FFI Considerations

This crate intentionally operates at a low level:

- Values are **untyped raw pointers** (`*mut c_void`).
- Lifetime management depends on correct implementation of deleters and disciplined usage of `lookup` / `release` pairs.
- All casting to and from `*mut c_void` is `unsafe` and must be validated by the caller.

When embedding in a larger Rust system, consider:

- Encapsulating all pointer casting behind a safe wrapper type that owns the cache and enforces correct use.
- Modeling `charge` as the approximate heap footprint (e.g., length of byte buffers, plus metadata) to keep eviction behavior aligned with real memory usage.
- Performing fuzzing or property-based testing on your wrapper layer if you rely on complex deleter logic.

In FFI contexts (e.g., when interoperating with C or C++ code derived from Bitcoin Core), this crate's API is intentionally close to the original patterns and should integrate straightforwardly.

---

## Logging

The implementation uses the `log` crate macros (`trace!`, `debug!`, `info!`, `warn!`, `error!`) extensively to describe internal state transitions:

- Insertions, lookups, erasures, and reference count changes.
- Eviction decisions and internal invariants.
- Error conditions such as invalid reference counts or inconsistent `in_cache` flags.

For meaningful diagnostics during integration or testing, configure a logger (e.g., `env_logger`, `tracing-log`) in your binary or test harness.

---

## Relationship to `bitcoin-rs`

This crate lives within the `https://github.com/klebs6/bitcoin-rs` repository and is primarily designed to support the LevelDB-esque storage components there. It aims for behavioral fidelity with the upstream Bitcoin Core cache implementation while exposing a Rust interface suitable for systems-level work on UTXO sets, block indices, and related structures.

---

## License

This crate is licensed under the **MIT** license, consistent with the broader `bitcoin-rs` project.

See the repository for the full license text and attribution details.
