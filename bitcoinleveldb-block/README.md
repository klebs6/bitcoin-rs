# bitcoinleveldb-block

Read-only LevelDB table data blocks and their iterators.

`Block::new` reads the restart-array trailer from `BlockContents`;
`Block::new_iterator` produces a `LevelDBIterator` using the supplied comparator.
The crate also supplies entry decoding and block/cache deletion callbacks.

`BlockContents` carries a borrowed byte view and an ownership flag. A non-owned
buffer must remain live while the block and its iterators use it. An owned block
reclaims the buffer on drop, so the allocation must match the deallocation
contract in `Block::drop`; setting the flag does not copy or validate ownership.
Keep the block alive for its iterators.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
