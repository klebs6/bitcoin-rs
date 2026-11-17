# bitcoinleveldbt-faultinjection

Filesystem fault-injection and recovery tests for the LevelDB port.

`FaultInjectionTestEnv`, `TestWritableFile`, and `FileState` track writes and synchronization so the tests can simulate lost unsynced data and recovery with or without log reuse. The model is an injected test filesystem behavior, not a claim to cover every hardware or power-failure mode.

Sources: [fault_injection_test_env](src/fault_injection_test_env.rs), [test_writable_file](src/test_writable_file.rs), [fault_injection](tests/fault_injection.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-faultinjection`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
