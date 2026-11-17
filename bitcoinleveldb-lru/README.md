# bitcoinleveldb-lru

Low-level sharded LRU cache primitives from the LevelDB implementation.

`LRUCache` manages one shard; `ShardedLRUCache` selects shards by key hash and
provides insertion, lookup, release, erase, pruning, and charge accounting.
`LRUHandle` and `HandleTable` support intrusive lists, reference counts, and
hash-table lookup. This implementation is separate from the mutex-backed cache
facade in [bitcoinleveldb-cache](../bitcoinleveldb-cache/README.md).

Values are opaque pointers paired with deleter callbacks. Returned handles pin
entries and must be released to the originating cache. Do not free an entry or
reuse a released handle. Charge is supplied by the caller; pinned entries can
remain above capacity. The public surface contains raw-pointer operations and
methods requiring mutable access; internal synchronization does not by itself
make arbitrary sharing or overlapping mutable access valid in Rust.

See [sharded operations](src/sharded_lru_cache.rs),
[handle representation](src/lru_handle.rs), and [shard state](src/lru_cache.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
