# bitcoinleveldb-blockiter

Iteration over the prefix-compressed entries of a LevelDB block.

`BlockIter` implements the LevelDB iterator traits: seeking uses the restart
array, forward iteration reconstructs keys, and reverse iteration searches from
a preceding restart point. Keys are reconstructed as bytes; values are
non-owning `Slice` views into the block. Decoding failures are exposed through
the iterator's `Status`.

This is an internal iterator implementation. Its constructor receives raw block
storage and a comparator pointer; both must remain valid while the iterator is
used. The constructor requires at least one restart point. Check `valid()` before
using the current key or value, and do not retain borrowed views across movement
or destruction of their backing storage.

See [the iterator state](src/block_iter.rs) and
[entry decoding](src/block_iter_parse_next_key.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
