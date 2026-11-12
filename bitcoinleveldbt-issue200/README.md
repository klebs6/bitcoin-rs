# bitcoinleveldbt-issue200

Iterator direction-change regression for upstream LevelDB issue 200.

The test switches iteration from backward to forward around a newly inserted mutation and checks the resulting key sequence. It opens and cleans up a private test database through the raw-handle API.

Sources: [bitcoinleveldbt_issue200](src/bitcoinleveldbt_issue200.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-issue200`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
