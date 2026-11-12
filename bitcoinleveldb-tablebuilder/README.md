# bitcoinleveldb-tablebuilder

Writes immutable sorted LevelDB SSTable files. `TableBuilder` assembles data,
index, filter, metaindex, and footer blocks, with checksums and the configured
compression policy.

Construct it with `&Options` and a valid `*mut dyn WritableFile`; the file is
borrowed and is not closed by the builder. Add keys in strictly increasing
comparator order, optionally `flush` a data block, and call `finish` to complete
the table or `abandon` to stop building it. Check the returned `Status` and the
builder's `status`; the caller remains responsible for file close/sync policy.
Keep borrowed file and policy objects valid while they are used.

`num_entries` and `file_size` expose progress. See [construction](src/table_builder.rs),
[entry handling](src/table_builder_add.rs), [finalization](src/table_builder_finish.rs),
and [raw block writing](src/table_builder_write_raw_block.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
