# bitcoinleveldb-posixseqfile

POSIX file-descriptor implementation of LevelDB's sequential-file interface.
`PosixSequentialFile::new(filename, fd)` takes ownership of an already-open file
descriptor; it does not open the named path. Dropping it closes the descriptor.

`SequentialFileRead::read` reads into caller-provided scratch storage, retries
`EINTR`, and returns a `Slice` borrowing that storage. A successful zero-byte
read represents EOF. The scratch buffer must hold the requested byte count and
the result pointer must be writable. `SequentialFileSkip::skip` advances the
descriptor position with `lseek`; failures become a `Status`.

Operations change the shared file position and require coordinated use of the
descriptor. See [construction](src/posix_sequential_file.rs), [reading](src/read.rs),
[skipping](src/skip.rs), and [close-on-drop](src/drop.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
