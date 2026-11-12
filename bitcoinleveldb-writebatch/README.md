# bitcoinleveldb-writebatch

C-API-style write-batch handles for the `bitcoin-rs` LevelDB port.
`LevelDBWriteBatch` wraps the binary `WriteBatch` implementation in
[`bitcoinleveldb-batch`](../bitcoinleveldb-batch/README.md).

The Rust functions `leveldb_writebatch_create`, `clear`, `put`, `delete`,
`iterate`, `append`, and `destroy` manage batches through raw handles (each
operation uses the `leveldb_writebatch_` prefix). Put/delete entries record a
batch; they do not write a database until the batch is submitted through the
database API. `iterate` invokes supplied put/delete callbacks with an opaque
caller state pointer.

Destroy each created handle once with `leveldb_writebatch_destroy`. Input
pointers must describe live buffers of their declared lengths. Callback key/value
pointers borrow batch data during the callback and must not be retained as owned
storage. This crate declares Rust functions, not an exported C ABI.

See [the handle and operations](src/writebatch.rs) and
[memtable round-trip test](tests/writebatch.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
