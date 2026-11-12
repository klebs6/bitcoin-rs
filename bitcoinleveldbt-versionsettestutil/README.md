# bitcoinleveldbt-versionsettestutil

Reusable version-set fixtures and observations for compaction, recovery and lifecycle tests.

The modules include scenario harnesses, internal-key/file-metadata constructors, point-file sequence observations, a raw-mutex test guard, and recording writable-file state. Several current library paths depend on these fixtures; publication must retain the actual dependency closure rather than excluding packages solely by their names.

Sources: [lib](src/lib.rs), [harness](src/harness.rs), [point_file_layout](src/point_file_layout.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-versionsettestutil`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
