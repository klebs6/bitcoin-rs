# bitcoinleveldb-dbimpl

The persistent LevelDB database implementation behind the `DB` operation traits.

`DBImpl` implements reads, writes and batches, user iterators, snapshots,
properties, approximate sizes, and manual compaction. Its supporting modules
coordinate the write-ahead log, mutable/immutable memtables, table cache,
version edits, recovery, and background compaction. New-database MANIFEST
records use byte vectors; filenames remain text.

`DBImpl::new` constructs implementation state; opening and recovery are performed
through `DBOpen::open`. Returned database and iterator pointers follow the
ownership contracts in `bitcoinleveldb-dbinterface`. Release iterators and
snapshots before destroying their database.

This is an implementation layer with raw pointers, explicit locking, and
environment ownership contracts. Its public state and test hooks are not a
substitute for the database operation interface.

Point reads expose byte-preserving `DBGet::get_bytes`. The default text `get`
adapter validates UTF-8 and returns `Corruption` for an invalid value.
Unsuccessful DB reads leave the output unchanged. Property outputs remain text.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

Tests using the repository-only `DBConstructor` fixture require a checkout.
Cargo omits that path-only development dependency from the published manifest.
Standalone packaged execution of those tests is not qualified.

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
