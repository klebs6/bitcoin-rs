# bitcoinleveldbt-corruption

Database corruption, repair and recovery regressions.

The `CorruptionTest` fixture modifies log, table and descriptor data and checks recovery, sequence numbers, write errors and compaction error handling. These are destructive operations on private test databases, not a repair utility for a user's database.

Sources: [bitcoinleveldbt_corruption](src/bitcoinleveldbt_corruption.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-corruption`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
