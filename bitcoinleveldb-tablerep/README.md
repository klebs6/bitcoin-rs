# bitcoinleveldb-tablerep

Shared representation of an opened LevelDB SSTable, used by
[`bitcoinleveldb-table`](../bitcoinleveldb-table/README.md). `TableRep` holds
table-read options and status, a shared random-access file handle, cache identity,
the metaindex handle, index block, and optional filter state.

This is internal reader state rather than a database or table-opening API.
`TableRep::new` accepts the already prepared options, file, cache ID, metaindex
handle, and index-block pointer. The representation owns the index-block
allocation and any installed filter-data allocation; its drop path reconstructs
and releases those allocations. Do not populate ownership-bearing raw fields
with borrowed or allocator-incompatible storage. Keep the shared file available
for all operations that use the representation.

See [the fields, constructor, and drop implementation](src/table_rep.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
