# bitcoinleveldb-blockbuilder

Builds the prefix-compressed data and index blocks used in LevelDB SSTables.
`BlockBuilder` stores shared-key prefixes, periodic restart keys, and a restart
offset array; it does not write an SSTable file itself.

Construct it with a live `*const Options`, then call `add` with keys in strictly
increasing comparator order. `finish` appends the restart trailer and returns a
`Slice` borrowing the builder's buffer. Copy or consume that data before `reset`
or dropping the builder. Do not add entries after `finish` without resetting.

See [construction](src/block_builder.rs), [entry encoding](src/block_builder_add.rs),
and [finalization](src/block_builder_finish.rs). Whole-file writing lives in
[`bitcoinleveldb-tablebuilder`](../bitcoinleveldb-tablebuilder/README.md).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
