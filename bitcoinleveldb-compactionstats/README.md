# bitcoinleveldb-compactionstats

Per-level counters for LevelDB compaction work.

`CompactionStats` records elapsed microseconds, bytes read, and bytes written.
It defaults to zero and provides generated accessors, setters, and a builder.
`add` accumulates another record with saturating `i64` addition.

The caller associates records with levels and supplies synchronization; this
crate does not measure time, perform compaction, or update counters automatically.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
