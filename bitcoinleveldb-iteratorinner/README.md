# bitcoinleveldb-iteratorinner

The operation traits and cleanup nodes used by LevelDB iterators.

`LevelDBIteratorInterface` combines validity, seek, movement, key/value, and
status traits. Seek before reading an initially invalid iterator. `next`, `prev`,
`key`, and `value` require a valid position; returned byte slices are valid only
until the iterator is modified.

`LevelDBIteratorCleanupNodeList` registers function/argument triples and invokes
them when the list is dropped. Callback arguments are raw pointers: their
lifetime and ownership must remain valid until cleanup runs.

The owning iterator wrapper is in `bitcoinleveldb-iterator`; this crate defines
the underlying contracts and cleanup machinery, not a database iterator source.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
