# bitcoinleveldb-modeldb

An in-memory reference database used by LevelDB comparison tests.

`ModelDB` applies `WriteBatch` operations to a `KVMap` and exposes iteration and
copied-map snapshots through the database operation traits. Iteration without a
snapshot owns a copied map; iteration with a model snapshot borrows that
snapshot's map, so retain the snapshot while the iterator uses it.

This is test support, not a persistent storage engine or a complete alternate
`DB` implementation. In the current source `DBGet::get` is unimplemented: it
triggers a debug assertion and otherwise returns `NotFound`. Properties are not
provided, approximate sizes are zero, and compaction is a no-op.

Use the persistent database implementation for application storage. The model
is useful only for the operation paths its tests actually exercise.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
