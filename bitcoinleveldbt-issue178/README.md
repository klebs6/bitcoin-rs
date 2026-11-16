# bitcoinleveldbt-issue178

Regression for deleted data reappearing after manual compaction (upstream LevelDB issue 178).

The original `issue178_test` keeps two ranges of 1,100,000 keys, batch writes/deletes, compaction bounds and the final live-key scan. `issue178_bounded_diagnostic` supplies a smaller diagnostic workload. Run both through the bounded subsystem runner; the original case needs the explicitly classified stress envelope.

Sources: [bitcoinleveldbt_issue178](src/bitcoinleveldbt_issue178.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-issue178`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
