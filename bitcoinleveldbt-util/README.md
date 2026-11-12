# bitcoinleveldbt-util

Shared test helpers for the LevelDB regression crates.

The modules provide deterministic data generation, seeds, temporary database paths, comparator probes, environment adapters, delay functions and a small test harness. These helpers are designed for controlled tests; they are not cryptographic random generators or a production application runtime.

Sources: [lib](src/lib.rs), [unique_db_path](src/unique_db_path.rs), [random_seed](src/random_seed.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-util`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
