# bitcoinleveldb-harness

Shared comparison harness for LevelDB blocks, tables, memtables and databases.
`Harness::init` selects a constructor and comparator. `add_bytes` and `test_bytes`
provide a byte-preserving fixture path; checks compare literal key/value pairs,
forward/reverse traversal, seeks and database point reads where available.
Repeated finalization replaces prior backing only after its iterators are gone.

Existing String `add` and text-fixture `test` remain available. Their random draw
law is unchanged, and their iterator assertions compare bytes rather than
formatted diagnostic strings. Binary additions require `test_bytes`; the legacy
text-output adapter rejects non-UTF-8 input. Formatting helpers remain diagnostics.

Constructors and iterators have explicit pointer-based ownership; iterators borrow
constructor storage/comparators. Database-backed cases use the shared execution
mutex and private runner test directories. Keep same-named comparator instances
behaviorally equivalent to Options.

The [finite binary target](tests/binary_constructor_contracts.rs) covers all four
constructors in both orders with small literal datasets. Original randomized and
long database targets are separate workloads; use the [bounded subsystem runner](../doc/testing/leveldb-subsystem.md).

[State](src/harness.rs) · [Byte checks](src/binary_fixture.rs) · [Manifest](Cargo.toml)
License: MIT. [Repository license](../LICENSE) · [Support](https://github.com/klebs6/bitcoin-rs/issues)
