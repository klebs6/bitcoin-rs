# bitcoinleveldb-posixwfile

Buffered POSIX writes for LevelDB's `WritableFile` interface.

`PosixWritableFile::new(filename, fd)` takes responsibility for the supplied
descriptor and uses a 65536-byte buffer for appends. `flush` drains buffered
bytes; `sync` also synchronizes the file, and for manifest files first
synchronizes the containing directory. Check each returned `Status` for I/O
failure.

`close` flushes and closes the descriptor. Dropping a still-open instance calls
`close`, but discards its error, so explicitly close when error reporting matters.
Buffer flushing alone is not a durability guarantee.

This crate uses POSIX/libc calls and raw byte views. It does not open the file
for the caller or transfer ownership of appended input buffers.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
