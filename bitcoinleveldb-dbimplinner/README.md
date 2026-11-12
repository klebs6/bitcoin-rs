# bitcoinleveldb-dbimplinner

Internal background-compaction and coordination helpers for the `bitcoin-rs`
LevelDB database implementation. This crate separates scheduling predicates,
background-error recording, waiter notification, manual-compaction boundaries,
and compaction-state preparation from the main database implementation.

It also supplies compatibility extension traits, including `CondvarSignalExt`
(`signal`/`signal_all` forward to parking_lot notifications). Helpers that accept
raw database, compaction, version, or mutex pointers require the owning
implementation's lifetime and locking invariants; they are not an application
database-opening interface.

See [the helper implementations](src/bitcoinleveldb_dbimplinner.rs),
[compatibility traits](src/compat.rs), and
[condition-variable wiring](src/condvar_signal_ext.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
