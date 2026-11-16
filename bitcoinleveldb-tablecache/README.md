# bitcoinleveldb-tablecache

Cache of opened LevelDB table files and their readers.

`TableCache` locates tables by file number and size, serves point lookups,
creates table iterators, and evicts cached entries. `build_table` and
`build_table_with_env` construct table files from sorted iterator input and
record their metadata.

A table iterator pins its cache entry until its cleanup callback releases the
handle. Any optional raw table pointer returned with that iterator is borrowed
from the cache: do not delete it or retain it after the iterator is destroyed.
The cache also retains its environment handle and uses raw-pointer adapters for
file and iterator integration. Supply valid options, file sizes, comparator
ordering, and the documented iterator/environment lifetimes.

See [cache state](src/table_cache.rs),
[iterator/handle lifetime](src/table_cache_new_iterator.rs), and
[table construction](src/build_table.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
