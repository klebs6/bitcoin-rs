# bitcoinleveldb-logtools

Shared LevelDB log-format definitions and diagnostic formatting helpers.

The crate defines `LogRecordType`, extended reader result values,
`LOG_BLOCK_SIZE` (32768 bytes), and `LOG_HEADER_SIZE` (7 bytes), plus initial
per-record-type CRC values. Log readers and writers are implemented in separate
crates.

Other helpers append/parse decimal numbers, escape non-printable bytes, append
slice bytes, and define the `Logger`/`Logv` message interface. These message
logging traits are distinct from the database's write-ahead record format.

Several helpers accept raw `String`, `Slice`, or output-array pointers. Their
storage and length requirements remain the caller's responsibility.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
