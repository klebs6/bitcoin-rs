# bitcoinleveldb-repair

Best-effort recovery of LevelDB database files.

`repairdb` drives a `Repairer`: it discovers database files, converts recoverable
log records into tables, scans tables for metadata, and writes a replacement
manifest and CURRENT file. The replacement MANIFEST record is encoded into a
byte vector. Filenames remain text. Files replaced or rejected by the process are moved
into an archive directory through the repair workflow.

Repair changes the database directory and may discard data that cannot be
recovered. Work on a copy when preserving the original files matters, and keep
the database closed while repairing it. An OK status means the repair workflow
completed; it is not proof that every original record was recovered. This is a
storage recovery utility, not a substitute for backups or normal database open.

See [public entry point](src/repairdb.rs), [workflow](src/run.rs), and
[manifest reconstruction](src/write_descriptor.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
