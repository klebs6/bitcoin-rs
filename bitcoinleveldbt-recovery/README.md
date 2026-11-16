# bitcoinleveldbt-recovery

Log and manifest recovery regressions for the LevelDB port.

`RecoveryTest` checks log reuse, missing and multiple logs, manifest reuse/compaction, and recovery across memtable flushes. Tests manage temporary databases and file handles through shared fixtures.

Sources: [bitcoinleveldbt_recovery](src/bitcoinleveldbt_recovery.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-recovery`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
