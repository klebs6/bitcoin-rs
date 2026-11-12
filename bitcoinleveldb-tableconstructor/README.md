# bitcoinleveldb-tableconstructor

An in-memory table-construction fixture for LevelDB iterator tests.

`TableConstructor` builds a table from a `KVMap`, retains a `Constructor` base,
stores the resulting bytes behind a `StringSource`, and reopens the
table for iteration and approximate-offset checks. Its public methods include
`new`, `finish_impl`, `new_iterator`, `approximate_offset_of`, and `reset`.

The fixture owns the table and retains its backing source/bytes. Dispose of
iterators before calling `reset`, rebuilding, or dropping the fixture, since
those actions invalidate the table state they use. The sink/source construction
helpers are crate-private.

This is test infrastructure, not an application database or an on-disk table
creation command.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
