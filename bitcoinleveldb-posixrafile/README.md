# bitcoinleveldb-posixrafile

POSIX positioned reads for LevelDB random-access files.

`PosixRandomAccessFile::new(filename, fd, limiter)` takes ownership of an open
file descriptor. If a limiter slot is available, the instance retains that
descriptor; otherwise it closes the incoming descriptor and opens a temporary
descriptor for each read. The limiter must outlive the file object.

The `RandomAccessFileRead` implementation uses `pread` and returns a `Slice`
over the caller's scratch storage. Keep that storage valid while using the
result, and check the returned `Status`; short reads and end-of-file can return
fewer bytes than requested. The crate also exposes helpers used by this path.

This is a low-level POSIX backend with raw pointers and descriptor ownership,
not an owning Rust byte-buffer abstraction.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
