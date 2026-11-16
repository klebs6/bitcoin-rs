# bitcoinleveldb-tableconstructor

In-memory SSTable construction fixtures for LevelDB tests. `TableConstructor`
builds a table into a byte-backed sink, retains its source and backing bytes,
and opens iterators over that table. Dispose of iterators before reset, rebuild
or destruction; they borrow constructor state.

`finish_impl_bytes` accepts the shared `ConstructorByteMap` and presents entries
to TableBuilder in the configured comparator order. Comparator-equivalent
input duplicates are rejected before rebuilding. Use the base `Constructor`
byte API when insertion order must select the last value for equivalent keys.
Comparator names and behavior must agree with Options. The existing text-map
`finish_impl` remains an adapter; it copies valid String bytes without transcoding.

See [construction](src/table_constructor_finish_impl.rs),
[backing ownership](src/table_constructor.rs), and
[shared fixture APIs](../bitcoinleveldb-blockconstructor/README.md).
[Manifest](Cargo.toml) · [Bounded verification](../doc/testing/leveldb-subsystem.md)
License: MIT. [Repository license](LICENSE) · [Support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
