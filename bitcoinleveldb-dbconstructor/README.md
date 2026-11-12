# bitcoinleveldb-dbconstructor

Database-backed construction fixtures for the LevelDB comparison harness.

`DBConstructor` owns a database instance and shares its comparator through an
`Arc`-backed adapter. It rebuilds the database from a fixture `KVMap` and supplies
iterators so the harness can compare database behavior with block and table
constructors. Iterators borrow database state and must be disposed of before the
constructor closes or rebuilds that database.

The crate also exports `destroydb`, which deletes recognized database files using
the configured `Env`. This is a destructive storage operation, separate from
constructing test data. These helpers are primarily test infrastructure; callers
looking for the database API should start with
[bitcoinleveldb-db](../bitcoinleveldb-db/README.md).

See [the fixture](src/db_constructor.rs) and [database destruction](src/destroy_db.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
