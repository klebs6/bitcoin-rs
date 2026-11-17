# bitcoinleveldb-table

Reading immutable LevelDB table files.

`Table::open` reads a table footer and index from a shared `RandomAccessFile`,
returning a boxed `Table` or a `Status` error. `new_iterator` exposes the table's
entries; `internal_get` performs the internal-key lookup callback path, and
`approximate_offset_of` estimates a key's file position. Supporting modules
read metadata and filter blocks and integrate data-block caching.

Returned iterators are caller-owned raw pointers and start invalid. Keep the
table alive while its iterators are in use, and respect the backing-file,
comparator, cache, and callback lifetimes used by the table representation.

Table construction is in `bitcoinleveldb-tablebuilder`; block construction and
iteration types live in their own crates. This crate does not re-export all of
those implementations as a general database facade.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
