# bitcoinleveldb-logreader

Logical-record reader for LevelDB's block-framed write-ahead log format.

`LogReader` owns a `SequentialFile` and `LogReaderReporter`. It reassembles
fragmented physical records, optionally verifies checksums, reports dropped or
corrupt data through the reporter, and supports starting at an initial byte
offset. `last_record_offset` identifies the last returned logical record.

`read_record` writes a non-owning `Slice` into its output and uses a caller-owned
`Vec<u8>` for fragmented-record scratch space. Consume or copy the record before
the next mutating reader operation or mutation of the scratch vector. A `false`
return ends the read loop; consult reporter observations to distinguish clean EOF
from I/O or corruption encountered while reading. The reader is mutable replay
state, not a concurrent ingestion queue.

See [construction](src/log_reader_create.rs),
[logical-record assembly](src/log_reader_read_record.rs), and
[physical-record validation](src/log_reader_read_physical_record.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
