# bitcoinleveldb-log

Convenience re-exports for the `bitcoin-rs` LevelDB logging components.
This crate's library contains no separate log implementation; it re-exports:

- [`bitcoinleveldb-logtools`](../bitcoinleveldb-logtools/README.md): record format,
  logger traits, and formatting helpers.
- [`bitcoinleveldb-logreader`](../bitcoinleveldb-logreader/README.md): reading and
  reconstructing log records.
- [`bitcoinleveldb-logwriter`](../bitcoinleveldb-logwriter/README.md): writing
  logical records as framed physical records.
- [`bitcoinleveldb-posixlogger`](../bitcoinleveldb-posixlogger/README.md): the
  diagnostic logger backed by a POSIX `FILE*`.

Binary recovery logs and human-readable diagnostic logs have different roles.
Choose the component matching the operation; log-record writing alone is not a
database transaction or durability policy. See [the re-exports](src/lib.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
