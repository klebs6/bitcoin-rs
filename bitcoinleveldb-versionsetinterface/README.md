# bitcoinleveldb-versionsetinterface

Traits connecting LevelDB versions, compaction, and file-number allocation.

`VersionSetInterface` combines the operations used by version-set clients:
options/comparator access, sequence tracking, live-file enumeration, file and
level accounting, recovery, manifest reuse, table-cache access, and file-number
allocation. `VersionSetLevelSummaryStorage` holds a formatted level summary.

This crate defines the boundary; it does not implement persistent version-set
management. `DummyVersionSet` is a limited test double: several operations return
empty/default values and comparator access panics. Use the concrete
[version-set implementation](../bitcoinleveldb-versionset/README.md) for engine
behavior. Raw-pointer outputs and returned references retain the lifetime
requirements of the underlying engine objects.

See [the traits](src/interface.rs), [file-number operations](src/file_number_interface.rs),
and [test double](src/dummy.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
