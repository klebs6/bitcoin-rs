# bitcoinleveldbt-snapshot

Snapshot behavior, lifetime and dispatch regressions for the LevelDB port.

The tests compare model and filesystem-backed snapshot views across writes, release, flush and reopen. Typed snapshot bridges use supported semantic capabilities and reject unsupported objects. Helpers for collecting rows and building read options are test infrastructure, not a general snapshot serialization API.

Sources: [lib](src/lib.rs), [snapshot_dispatch_boundary_contracts](tests/snapshot_dispatch_boundary_contracts.rs), [snapshot](tests/snapshot.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-snapshot`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
