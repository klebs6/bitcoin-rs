# bitcoinleveldbt-mt

Multithreaded database read/write regression across fixture option configurations.

Four workers read and write a 1,000-key space and check writer/counter consistency. Each configuration runs for ten seconds before coordinated shutdown; the unchanged overall case therefore exceeds the ordinary 30-second envelope. The subsystem runner defers it to the bounded stress profile.

Sources: [mt_test](src/mt_test.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-mt`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
