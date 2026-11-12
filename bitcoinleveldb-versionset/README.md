# bitcoinleveldb-versionset

Manages LevelDB's current and retained SSTable versions, MANIFEST edits,
file-number/sequence bookkeeping, and compaction selection. `VersionSet` owns
the linked version list and connects it to the table cache and environment.

`VersionSet::new` returns a boxed value: its version-list sentinel and related
objects use stable-address raw pointers. The supplied options, comparator, and
table-cache dependencies must remain valid for the version set's use. Database
locking governs recovery, edit application, and compaction state changes.
`VersionSetBuilder` applies `VersionEdit` records to construct a version's file
set; it is storage-engine machinery rather than an application transaction API.

The crate enables the nightly `ptr_metadata` feature. See
[construction and ownership](src/version_set.rs),
[MANIFEST application](src/version_set_log_and_apply.rs),
[recovery](src/version_set_recover.rs), and
[compaction selection](src/version_set_pick_compaction.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
