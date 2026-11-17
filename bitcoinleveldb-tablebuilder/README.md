# bitcoinleveldb-tablebuilder

Writes immutable sorted LevelDB SSTables: data, index, filter, metaindex, and
footer blocks, with block checksums and the configured compression policy.

Construct TableBuilder with live Options and a valid `*mut dyn WritableFile`.
The file is borrowed and is not closed by the builder. Add keys in strictly
increasing comparator order, optionally flush data blocks, then finish or
abandon construction. Check returned Status values and the builder status;
the caller remains responsible for closing and syncing the file. Keep borrowed
file and policy objects alive while used.

Data/index blocks, previous keys, serialized block handles and footer output
use byte storage. Compression scratch also owns bytes as a vector. Snappy is an optional backend
provided by bitcoinleveldb-compat; unavailable or insufficient compression falls
back to raw block storage. Default results and explicit Snappy-enabled results
are qualified separately; neither implies an all-features/platform matrix.
`num_entries` and `file_size` expose progress.

See [construction](src/table_builder.rs), [entries](src/table_builder_add.rs),
[finalization and binary metadata test](src/table_builder_finish.rs), and
[block writing](src/table_builder_write_raw_block.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Upstream notices](NOTICE)
