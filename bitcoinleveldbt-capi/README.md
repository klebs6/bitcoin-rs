# bitcoinleveldbt-capi

Integration regression for the LevelDB port's C-API-style Rust functions.

The round-trip test exercises database, option, batch, iterator, snapshot, comparator and filter handles, including explicit result/error cleanup. This is a Rust test of raw-handle functions; it does not establish that this crate exports a linkable C ABI.

Sources: [bitcoinleveldbt_capi](src/bitcoinleveldbt_capi.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-capi`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
