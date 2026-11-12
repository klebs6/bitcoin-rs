# bitcoinleveldbt-mockversionset

Mock version-set support used by the LevelDB port's fixtures.

`MockVersionSet` supplies a version-set interface adapter for controlled tests and constructors. Some current library dependencies use this support crate through normal dependency edges, so its test-oriented name alone is not authority to omit it from a release dependency closure.

Sources: [mock_version_set](src/mock_version_set.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-mockversionset`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
