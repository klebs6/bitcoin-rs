# bitcoinleveldbt-dbtest

Shared database fixture and core database regressions for the LevelDB port.

`DBTest`, `DBTestOptionConfig`, and the fixture helpers coordinate open/reopen, options, snapshots, reads, writes and compaction. Other regression crates depend on this scaffolding. The tests cover multiple option configurations; callers of raw-pointer helpers must follow their source ownership and teardown contracts.

Sources: [dbtest](src/dbtest.rs), [fixture](src/fixture.rs), [probes](src/probes.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-dbtest`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
