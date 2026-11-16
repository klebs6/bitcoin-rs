# bitcoinleveldbt-table

Table-construction and approximate-offset regressions for the LevelDB port.

The integration target constructs sorted tables and checks approximate offsets for plain and conditionally compressed data. The compressed branch is skipped when its feature is disabled; a default-feature pass is not evidence that the compressed branch ran.

Sources: [table_test](tests/table_test.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-table`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
