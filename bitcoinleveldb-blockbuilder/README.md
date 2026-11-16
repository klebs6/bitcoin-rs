# bitcoinleveldb-blockbuilder

Builds the prefix-compressed data and index blocks used in LevelDB SSTables:
entry lengths, shared-key prefixes, periodic restart keys, and a little-endian
restart-offset trailer. It does not write an SSTable file itself.

Construct `BlockBuilder` with a live `*const Options`, then call `add` with keys
in strictly increasing comparator order. Keys, values, and encoded block storage
are bytes; the builder owns its buffer and previous key as `Vec<u8>`.
`buffer_bytes` exposes the current encoded bytes. `finish` appends the restart
trailer and returns a Slice borrowing that storage. Consume or copy it before
`reset`, another buffer mutation, or dropping the builder. Reset before adding
entries after finish.

See [construction](src/block_builder.rs), [entry encoding](src/block_builder_add.rs),
[finalization](src/block_builder_finish.rs), and
[wire-format regressions](tests/binary_block_contracts.rs).
Whole-file writing lives in [bitcoinleveldb-tablebuilder](../bitcoinleveldb-tablebuilder/README.md).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT; see the [repository license](LICENSE).

[Upstream notices](NOTICE)
