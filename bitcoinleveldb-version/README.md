# bitcoinleveldb-version

An SSTable-file view of the LevelDB database, used by
[`bitcoinleveldb-versionset`](../bitcoinleveldb-versionset/README.md). `Version`
records files per level, links to neighboring versions, reference counts, and
compaction-selection state. Older versions remain available while live iterators
still reference their file view.

The implementation supplies lookup, overlapping-range selection, iterator
construction, read-sample accounting, and debug output. `ref_`/`unref` manage
version lifetime, while file metadata and the owning version set are represented
by raw pointers. Access requires the owning database's external synchronization;
this crate is not a standalone MVCC database or a user snapshot manager.

`get_bytes` reads values into byte vectors. The legacy `get` adapter validates
UTF-8 and returns `Corruption` for an invalid value, preserving its String output.

See [representation](src/version.rs), [reference management](src/ref_unref.rs),
[lookup](src/get.rs), and [overlap selection](src/get_overlapping_inputs.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
