# bitcoinleveldb-duplex

Two-level iteration for LevelDB tables: an index iterator identifies blocks,
and a caller-supplied `BlockFunction` opens each block's data iterator.
`TwoLevelIterator` presents their entries as one forward/reverse sequence,
skipping empty blocks and retaining encountered errors in its status.

`new_two_level_iterator` takes ownership of a boxed index iterator and returns a
boxed `LevelDBIteratorInterface`. The block callback returns an optional owned
data iterator. Its opaque `arg` remains caller-owned and must stay valid for
every callback. Seek before reading entries and use the iterator interface's
validity/status checks. The crate provides no general thread-safety guarantee
for arbitrary callbacks or their underlying data.

See [the callback and constructor](src/two_level_iterator.rs),
[block initialization](src/init_data_block.rs), and
[empty-block traversal](src/skip_empty_data_blocks.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
