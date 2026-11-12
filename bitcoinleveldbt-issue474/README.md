# bitcoinleveldbt-issue474

Logging/flush regression for upstream LevelDB issue 474.

`db_test_fflush_issue474` exercises the database behavior associated with the upstream fflush regression. It uses the shared database fixture and a private test directory.

Sources: [bitcoinleveldbt_issue474](src/bitcoinleveldbt_issue474.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-issue474`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
