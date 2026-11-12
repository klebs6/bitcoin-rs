# bitcoinleveldbt-bench

Benchmark workloads for the LevelDB port, with a separate SQLite comparison implementation.

The source contains workload dispatch, flag parsing, thread coordination, data generation, and statistics. Benchmark entry points can create or destroy database directories and run substantial workloads. Compiling this package or passing its ordinary tests does not measure throughput or establish benchmark results. Review the chosen entry point and its path/size flags before running a benchmark.

Sources: [db_bench](src/db_bench.rs), [db_bench_sqlite3](src/db_bench_sqlite3.rs).

For resource-bounded verification from the repository, use the [subsystem testing guide](../doc/testing/leveldb-subsystem.md). Select this package with `--crate bitcoinleveldbt-bench`; the runner handles its ordinary, ignored and explicitly classified stress cases.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [Manifest](Cargo.toml) · [License](../LICENSE)
