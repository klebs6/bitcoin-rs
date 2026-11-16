# bitcoinleveldbt-issue44

Level-zero compaction regressions for upstream LevelDB issue 44.

Two database fixture cases construct the key/compaction layouts that exposed the original issue and check the resulting reads. This package is a regression suite, not a separate storage implementation.

Sources: [bitcoinleveldbt_issue44](src/bitcoinleveldbt_issue44.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-issue44`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
