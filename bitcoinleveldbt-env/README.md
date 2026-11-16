# bitcoinleveldbt-env

An environment adapter used by LevelDB tests.

`TestEnv` forwards the environment surface while allowing selected test behavior to be controlled. It is test infrastructure; use the production environment implementations for application I/O and follow the wrapped environment's lifetime contract.

Sources: [testenv](src/testenv.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-env`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
