# bitcoinleveldb-cache

Handle-based caching used by the `bitcoin-rs` LevelDB storage engine.

`CacheInterface` combines insertion, lookup, release, value access, erasure,
pruning, charge accounting, and cache-local numeric IDs. `new_lru_cache` returns
an owned raw `Cache` pointer. The current implementation serializes its backing
state with a mutex and selects unused entries for eviction using recency stamps.

Each successful insertion or lookup returns a handle that must be released
exactly once to the same cache. A live handle pins its entry. Erasing a key removes
the cache's reference; it does not invalidate an outstanding client handle. The
insertion callback manages final cleanup of the opaque value. Release all handles
before destroying the cache. Pinned entries can prevent charge from falling to
the configured capacity; charge is an application-supplied accounting measure,
not a measurement of total process memory.

The legacy `lru_remove` and `lru_append` methods are currently no-ops; callers must
not use them to infer list mutation. This crate's implementation is distinct from
the sharded primitives in [bitcoinleveldb-lru](../bitcoinleveldb-lru/README.md).

See [handle contracts](src/cache_interface.rs) and
[eviction/accounting](src/cache_rep.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
