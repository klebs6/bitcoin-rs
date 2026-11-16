# bitcoinleveldbt-randomized

Deterministic randomized, replay and failure-prefix regressions for the LevelDB port.

The integration tests compare database state with a model across updates and option configurations. Bounded cases exercise a short recorded workload; replay contracts check operation order and configuration offsets; the bisect path diagnoses a failing prefix. Original workloads require the execution controls enforced by the subsystem runner. A short diagnostic pass is not the original-workload result.

Sources: [randomized](tests/randomized.rs), [bisect](tests/bisect.rs), [bounded_contracts](tests/bounded_contracts.rs), [replay_contracts](tests/replay_contracts.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-randomized`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](LICENSE)

[Upstream notices](NOTICE)
