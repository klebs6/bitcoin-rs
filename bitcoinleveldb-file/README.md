# bitcoinleveldb-file

File interfaces, database filename helpers, and file metadata for LevelDB.

The crate defines traits for sequential reads, random-access reads, writable
files, directory operations, file creation/removal, and locking. Environment
implementations provide the concrete I/O. Filename helpers construct and parse
LevelDB log, table, manifest, CURRENT, LOCK, temporary, and information-log names.
`FileMetaData` carries table-file bookkeeping used by versions and compaction.

This crate supplies interfaces and naming rules; it is not itself a POSIX or
portable filesystem implementation. Read/write interfaces use `Slice` views,
caller-provided buffers, and raw output pointers. Implementors and callers must
honor the specific buffer and ownership contracts.

See [sequential files](src/sequential_file.rs),
[writable files](src/writable_file.rs), [filename parsing](src/parse_file_name.rs),
and [metadata](src/metadata.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
