# bitcoinleveldb-dumpfile

Text inspection of individual LevelDB storage files.

`dump_file` uses an `Env` to read a named file and appends a textual rendering to
a caller-supplied `WritableFile`. It dispatches log files, descriptor/manifest
files, and table files to their corresponding readers and returns a `Status`.
It does not export a whole database as a consistent snapshot.

The library also exposes `dbleveldbutil_main`, a C-style argument entry point for
the `dump files...` command. This manifest defines a library, so this function
does not by itself install a `leveldbutil` executable.

Raw destination and argument pointers must refer to valid live objects. Use
the relevant `Status` result to distinguish an unknown file type, I/O failure,
or parsing failure from a successful dump.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
