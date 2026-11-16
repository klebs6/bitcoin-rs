# bitcoinleveldb-test

Workspace dependency aggregate for LevelDB test crates.

The manifest gathers the `bitcoinleveldbt-*` regression, fixture, environment,
and utility crates. The current library root exports no test-runner API and
contains no tests of its own. Depending on this package does not automatically
execute the tests belonging to those dependency packages.

Use the [bounded subsystem runner](../doc/testing/leveldb-subsystem.md) to discover
and run the individual packages. Application users should start with the
[database facade](../bitcoin-leveldb/README.md), not this test dependency aggregate.

See [the aggregate manifest](Cargo.toml) and [library root](src/lib.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
