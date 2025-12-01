# bitcoinleveldb-lru

A faithful, low-level Rust port of LevelDB's sharded LRU cache implementation, as used inside the `bitcoin-rs` project. This crate exposes the internal cache primitives that back a LevelDB-compatible block cache, including sharded LRU caches, intrusive list nodes, and a custom hash table.

---

## Overview

`bitcoinleveldb-lru` implements the exact cache semantics of LevelDB's C++ `ShardedLRUCache`, translated into Rust with careful attention to:

- **Intrusive LRU lists** implemented as circular doubly linked lists of `LRUHandle` nodes.
- **Sharding** across a fixed number of shards (`NUM_SHARDS`) to reduce contention in multi-threaded workloads.
- **Custom handle table** (`HandleTable`) that acts as a compact open-hashing structure keyed by LevelDB `Slice` values.
- **Precise reference-counting semantics** identical to LevelDB: an entry is freed when `refs == 0` *and* `in_cache == false`.
- **Explicit manual memory management**, using `libc::malloc`/`free` and flexible array members that inline the key bytes adjacent to the `LRUHandle` header.

The design is intentionally low-level and unsafe in places, because it aims to be wire-compatible with LevelDB expectations and to interoperate with foreign code (e.g., C and C++) with minimal overhead.

The crate is not a general-purpose Rust cache with ergonomic types; instead, it is the *internal engine* for a LevelDB-style block cache used by higher-layer crates in the `bitcoin-rs` repository.

---

## Core Data Structures

### `LRUHandle`

`LRUHandle` represents a single cache entry. It is allocated on the heap with a flexible trailing key byte array:

```rust
#[repr(C)]
pub struct LRUHandle {
    value:      *mut c_void,
    deleter:    fn(_0: &Slice, value: *mut c_void) -> c_void,
    next_hash:  *mut LRUHandle,
    next:       *mut LRUHandle,
    prev:       *mut LRUHandle,
    charge:     usize,
    key_length: usize,
    in_cache:   bool,
    refs:       u32,
    hash:       u32,
    key_data:   [u8; 1],
}
```

Semantically:

- `key_data` stores `key_length` key bytes inline; this minimizes allocations.
- `next`/`prev` form a circular doubly linked list for the LRU and in-use lists.
- `next_hash` links handles within a hash bucket in `HandleTable`.
- `charge` is the logical size/weight of the entry (e.g., bytes of a block), used to constrain total cache usage.
- `refs` is a reference count; the cache itself holds one reference while the entry is in the cache, and clients hold additional references while they are using the entry.

Important invariants (matching LevelDB):

- An entry is destroyed when `refs == 0` and `in_cache == false`.
- When the only remaining reference is held by the cache (`refs == 1 && in_cache == true`), the entry resides on the LRU list.
- When additional references exist, the entry is instead on the `in_use` list.

### `HandleTable`

`HandleTable` is a custom hash table over `*mut LRUHandle` values, tuned for speed and to avoid C++-porting complications:

```rust
#[derive(Getters,Setters)]
pub struct HandleTable {
    length: u32,
    elems:  u32,
    list:   *mut *mut LRUHandle,
}
```

- `length` is the number of buckets, always a power of two.
- `elems` counts entries.
- `list` points to a C-style array of `length` buckets, where each bucket is a linked list via `LRUHandle::next_hash`.

The core operations:

- `insert(&mut self, h: *mut LRUHandle) -> *mut LRUHandle` – insert handle, possibly replacing an existing one with the same key and hash.
- `lookup(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle` – locate handle by key and precomputed hash.
- `remove(&mut self, key_: &Slice, hash_: u32) -> *mut LRUHandle` – detach handle from table without altering its refcount.

The table dynamically grows via `resize` when `elems > length`, rehashing all handles into a new bucket array. Allocation and deallocation are performed with `libc::malloc`/`free`.

### `LRUCacheInner`

`LRUCacheInner` contains the core state of a non-sharded LRU cache:

```rust
pub struct LRUCacheInner {
    usage:  usize,
    lru:    Box<LRUHandle>,
    in_use: Box<LRUHandle>,
    table:  HandleTable,
}
```

- `usage` tracks total `charge` of all entries currently in the cache.
- `lru` and `in_use` are sentinel nodes that serve as list heads for the LRU and in-use lists, respectively.
- `table` indexes entries by `(key, hash)`.

The constructor `LRUCacheInner::new()` creates sentinel nodes via `LRUHandle::make_sentinel()` and links them into empty circular lists.

### `LRUCache`

`LRUCache` is a thread-safe, single-shard LRU cache, wrapping `LRUCacheInner` in a `Mutex` and a `RefCell`:

```rust
#[derive(Getters,Setters)]
pub struct LRUCache {
    capacity: usize,
    mutex:    RefCell<Mutex<LRUCacheInner>>,
}
```

Key operations (per shard):

- `insert(&mut self, key_: &Slice, hash_: u32, value: *mut c_void, charge: usize, deleter: fn(&Slice, *mut c_void) -> c_void) -> *mut CacheHandle`
- `lookup(&mut self, key_: &Slice, hash_: u32) -> *mut CacheHandle`
- `release(&mut self, handle: *mut CacheHandle)`
- `erase(&mut self, key_: &Slice, hash_: u32)`
- `prune(&mut self)` – aggressively drop all LRU entries whose `refs == 1`.
- `total_charge(&self) -> usize`

The public API follows LevelDB's `Cache` interface style:

- `insert` returns a `CacheHandle` pointer for the client; the caller is responsible for calling `release` when done.
- `lookup` returns a handle with an incremented refcount, or null if not found.
- `release` decrements the refcount and triggers deletion if the entry is no longer cached.
- `erase` removes an entry from the cache; it will be freed once no client holds it.

Internally, the helper functions `ref_inner`, `unref_inner`, `finish_erase_inner`, `lru_append_node`, and `lru_remove_node` enforce list and refcount consistency.

### `ShardedLRUCache`

`ShardedLRUCache` composes multiple `LRUCache` instances to reduce lock contention:

```rust
pub struct ShardedLRUCache {
    base:     Cache,
    shard:    [LRUCache; NUM_SHARDS],
    id_mutex: parking_lot::RawMutex,
    last_id:  u64,
}
```

- The total capacity is divided across `NUM_SHARDS` by `per_shard = (capacity + NUM_SHARDS - 1)/NUM_SHARDS`.
- `hash_slice(s: &Slice) -> u32` computes a 32-bit hash (via `leveldb_hash`) and is used to derive the shard index.
- `shard(hash_: u32) -> u32` maps a hash to a shard index by taking the upper `NUM_SHARD_BITS` bits.

Public operations:

- `new(capacity: usize) -> Self` – allocate shards and configure per-shard capacities.
- `insert(&mut self, key_: &Slice, value: *mut c_void, charge: usize, deleter: fn(&Slice, *mut c_void) -> c_void) -> *mut CacheHandle`
- `lookup(&mut self, key_: &Slice) -> *mut CacheHandle`
- `release(&mut self, handle: *mut CacheHandle)` – determines the shard from the handle's hash and forwards to that shard.
- `erase(&mut self, key_: &Slice)` – calculates hash from key and erases in the appropriate shard.
- `value(&mut self, handle: *mut CacheHandle) -> *mut c_void` – obtains the underlying stored pointer.
- `new_id(&mut self) -> u64` – monotonically increasing ID generation protected by `RawMutex`.
- `prune(&mut self)` – runs `prune()` on each shard.
- `total_charge(&self) -> usize` – aggregate `total_charge` across all shards.

The sharded design is crucial when this cache is used in high-concurrency database workloads, such as a Bitcoin node replaying or validating large blockchains.

---

## Safety and Memory Model

This crate heavily uses raw pointers and manual allocation. Safety is established by a set of invariants enforced through assertions and debug checks:

- All `LRUHandle` pointers must be properly aligned (verified via `lru_debug_verify_list` and `unref_inner_list_contains`).
- Circular list invariants are asserted: for each node, `next.prev == node` and `prev.next == node`.
- `unref_inner` asserts that `refs_before > 0` before decrementing.
- `LRUCache::prune`, `LRUCache::drop`, and `HandleTable::resize` contain additional checks and safety breaks to avoid undefined behavior in case of corruption.

Because the interface exposes raw `*mut CacheHandle` and `*mut c_void`, it is intended for controlled internal usage inside `bitcoin-rs` and LevelDB-compatible infrastructure. It is not designed as an ergonomic public API for general Rust applications.

When using it directly, you must:

- Ensure that each acquired handle is released exactly once.
- Ensure that `value` pointers are valid for the lifetime expected by the cache, and that the provided `deleter` correctly deallocates or manages them.
- Treat `Slice` values as immutable while in use as keys in the cache.

---

## API Sketch and Usage

The intended high-level usage pattern closely matches LevelDB's `Cache` interface.

### Creating a sharded cache

```rust
use bitcoinleveldb_lru::{ShardedLRUCache, new_lru_cache};
use core::ffi::c_void;

// Capacity in some logical units (often bytes)
let capacity: usize = 64 * 1024 * 1024; // 64 MiB

// High-level: construct directly
let mut cache = ShardedLRUCache::new(capacity);

// Or, for FFI-style usage: obtain a raw Cache pointer
let raw_cache: *mut Cache = new_lru_cache(capacity);
```

### Inserting and retrieving entries

Assuming you have a `Slice` type compatible with LevelDB semantics and a buffer stored elsewhere:

```rust
use core::ffi::c_void;

fn my_deleter(key: &Slice, value: *mut c_void) -> c_void {
    unsafe {
        // Example: value is a Box<u8> cast to *mut c_void
        let _ = Box::from_raw(value as *mut u8);
        core::mem::zeroed()
    }
}

// `key` is your LevelDB-style Slice
let key: Slice = ...;

// Example value
let value: *mut c_void = Box::into_raw(Box::new(42u8)) as *mut c_void;
let charge: usize = 1; // cost of this entry

let handle = cache.insert(&key, value, charge, my_deleter);
assert!(!handle.is_null());

// Later: lookup by key
let handle2 = cache.lookup(&key);
if !handle2.is_null() {
    unsafe {
        let ptr = cache.value(handle2) as *mut u8;
        assert_eq!(*ptr, 42u8);
    }

    // Release the lookup reference
    cache.release(handle2);
}

// Release the original insert reference when you're done
cache.release(handle);
```

### Erasing and pruning

```rust
// Erase a specific key; entry will be freed once no client holds it
cache.erase(&key);

// Manually prune all LRU entries that are only referenced by the cache
cache.prune();

// Measure usage
let total = cache.total_charge();
```

---

## Concurrency

`ShardedLRUCache` is designed for concurrent access:

- Each `LRUCache` shard owns a `Mutex<LRUCacheInner>`; operations on distinct shards can proceed in parallel.
- Shard selection is deterministic based on the high bits of the hash of the key (via `hash_slice` and `shard`).
- `new_id` uses a `parking_lot::RawMutex` to serialize ID allocation without interfering with cache operations.

Note that the API presented here mostly uses `&mut self`; in the broader `bitcoin-rs` ecosystem, the cache is typically wrapped and shared in a way that hides mutability details while preserving correctness.

---

## Diagnostics and Debugging

The crate includes a rich set of debug utilities:

- `lru_debug_verify_inner(inner: &mut LRUCacheInner)` – verifies structural invariants of the LRU and in-use lists.
- `lru_debug_verify_list(head: *mut LRUHandle, list_name: &str)` – checks pointer alignment and next/prev consistency.
- `unref_inner_list_contains(head: *mut LRUHandle, target: *mut LRUHandle, list_name: &str) -> bool` – linear search with alignment checks and cycle protection.

Additionally, many functions emit trace/debug/warn/error messages for instrumentation, which can be routed through the `log` ecosystem.

These facilities are useful for diagnosing subtle reference-counting or pointer errors when integrating with external code or when porting from other LevelDB variants.

---

## Integration Notes

- **Repository**: This crate lives inside the `bitcoin-rs` monorepo: <https://github.com/klebs6/bitcoin-rs>
- **License**: MIT
- **Edition**: Rust 2021

It is primarily intended as an internal component of a LevelDB-compatible storage subsystem for Bitcoin-related applications. While it can be reused elsewhere, you should treat it as a low-level, expert-only primitive: misuse can easily produce memory unsafety.

If you need a safe, generic LRU cache for application-level code, consider using a different crate that exposes a high-level, type-safe API.

---

## Caveats

- The public interface is pointer-centric and `unsafe`-oriented; misuse can cause undefined behavior.
- The semantics explicitly mirror LevelDB, including details like `charge`, double lists (`lru_` vs `in_use_`), and deletion rules.
- The crate assumes the existence of `Slice`, `Cache`, `CacheHandle`, `NUM_SHARDS`, `NUM_SHARD_BITS`, and `leveldb_hash`, provided elsewhere in the `bitcoin-rs` ecosystem.

Despite these constraints, when used correctly within its intended environment, `bitcoinleveldb-lru` offers a highly efficient, production-grade LRU cache engine suitable for heavy blockchain and database workloads.
