# bitcoinleveldb-harness

Shared comparison harness for LevelDB blocks, tables, and databases.

`Harness` constructs the selected fixture, adds key/value pairs, and checks
forward scans, backward scans, seeks, and randomized access against a `KVMap`.
It includes comparator adapters and constructor-pointer cleanup used to run the
same checks across different representations. This is test infrastructure, not
a database facade for applications.

Constructor and iterator state is pointer-based; initialization and disposal are
part of the harness lifecycle. Database-backed cases use the shared execution
mutex exported by this crate. Full randomized cases are substantial workloads:
use the [bounded subsystem runner](../doc/testing/leveldb-subsystem.md), which
classifies them explicitly and gives each case private test data.

See [harness state](src/harness.rs), [initialization](src/init.rs), and
[comparison execution](src/test.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
