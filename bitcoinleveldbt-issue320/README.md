# bitcoinleveldbt-issue320

Bounded and original-workload regressions for upstream LevelDB issue 320.

The tests exercise large randomized database updates and reads, with explicit execution profiles, progress and cleanup. The original case requires verified resource containment; the diagnostic profile does not replace it. Contract tests check the profile and guard predicates. Use the subsystem runner rather than launching the original workload directly.

Sources: [bitcoinleveldbt_issue320](src/bitcoinleveldbt_issue320.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-issue320`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
