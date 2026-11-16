# bitcoinleveldb-db

C-API-style database handles and Rust functions for the `bitcoin-rs` LevelDB
implementation. This crate exposes raw-pointer operations such as `leveldb_open`,
`leveldb_put`, `leveldb_get`, `leveldb_write`, iterator/snapshot operations, and
the corresponding close/destroy/free functions. These are Rust functions; this
crate does not declare a C ABI or exported `extern "C"` entry points.

Create option handles before opening a database and inspect the error pointer
returned by operations. Input buffers must be live for their declared lengths;
database names follow the NUL-terminated input convention. Retrieved value/error
buffers must be released with `leveldb_free`. Destroy iterator and option handles
with their matching functions, and release snapshots through their owning
database before closing it. These pointer/ownership obligations are not encoded
in Rust lifetimes.

`leveldb_get` returns literal value bytes with their explicit length. A found
empty value has a non-null, freeable allocation of length zero; `NotFound`
returns null and length zero. Both results keep the existing Rust function
signature; this crate does not add C ABI exports.

Snapshot wrappers forward their inner read capabilities. Their sequence shadow
is reconstructed through the snapshot interface; it does not copy another
implementation's object layout or intrusive links.

Start with the [end-to-end handle test](src/lib.rs),
[open](src/leveldb_open.rs), [get](src/leveldb_get.rs), and
[buffer release](src/leveldb_free.rs). The lower-level database trait is in
[`bitcoinleveldb-dbinterface`](../bitcoinleveldb-dbinterface/README.md).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
