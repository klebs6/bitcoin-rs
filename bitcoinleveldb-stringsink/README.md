# bitcoinleveldb-stringsink

An in-memory `WritableFile` used by LevelDB table construction and tests.

`StringSink::new` starts with an empty byte vector; `StringSink::with` starts
with the UTF-8 bytes of a string. `append` copies a `Slice` into that vector,
preserving arbitrary bytes. Generated `contents` accessors expose the buffer.

`close`, `flush`, and `sync` return success without filesystem I/O. This sink
provides no persistent storage or crash durability. Appended pointer-backed
slices must be readable for their stated length during the call.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
