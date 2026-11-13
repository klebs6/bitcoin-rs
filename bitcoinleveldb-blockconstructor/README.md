# bitcoinleveldb-blockconstructor

Block-construction fixtures for LevelDB iterator and table tests.

`Constructor` accumulates a string-keyed `KVMap`; `BlockConstructor` serializes
that map in comparator order and owns the encoded block bytes as `Vec<u8>`.
`data_bytes` exposes those bytes. It owns its comparator and block, and creates
LevelDB iterators that borrow them. Finish construction before requesting an
iterator, and drop all such iterators before rebuilding or dropping the
constructor or modifying its backing storage.

This is test infrastructure. The string-keyed `KVMap` and its conversions remain
outside a general arbitrary-byte input interface. Byte storage for the encoded
block does not remove that fixture limitation.

See [construction](src/block_constructor.rs),
[finalization](src/block_constructor_finish_impl.rs), and
[binary block/iterator contracts](tests/binary_block_contracts.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT; see the [repository license](../LICENSE).
