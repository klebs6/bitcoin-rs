# bitcoinleveldb-blockconstructor

Block-construction fixtures for LevelDB iterator and table tests.

`Constructor` accumulates a string-keyed `KVMap`; `BlockConstructor` serializes
that data using the block builder, retains the resulting block storage, and
creates `LevelDBIterator` instances over it. The constructor owns its comparator
and block. Finish construction before requesting an iterator, and keep the
constructor and its backing bytes alive while an iterator borrows them.

This is test infrastructure, not an API for opening a database. The helper's
`String`/`KVMap` conversions are not a general arbitrary-byte key/value interface.

See [construction](src/block_constructor.rs) and
[block finalization](src/block_constructor_finish_impl.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
