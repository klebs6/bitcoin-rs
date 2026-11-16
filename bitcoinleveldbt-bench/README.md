# bitcoinleveldbt-bench

Benchmark workloads for the LevelDB port, with a separate SQLite comparison
implementation. Source includes workload dispatch, flags, thread coordination,
data generation and statistics. Compression output buffers use byte vectors.

Entry points can create or destroy database directories and execute substantial
workloads. Review the entry point and path/size flags before launching a
benchmark. Compilation alone neither executes workloads nor measures throughput.
The focused compression070 qualification compiles this consumer without running
its benchmark or native test entry points.

See [LevelDB workloads](src/db_bench.rs), [SQLite comparison](src/db_bench_sqlite3.rs),
and the [bounded testing guide](../doc/testing/leveldb-subsystem.md).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues) ·
[Manifest](Cargo.toml) · [License](LICENSE).

[Upstream notices](NOTICE)
