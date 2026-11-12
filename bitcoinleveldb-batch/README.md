# bitcoinleveldb-batch

Ordered write batches for the `bitcoin-rs` LevelDB implementation.

`WriteBatch` collects `put` and `delete` operations, preserving their insertion
order. `clear` resets a batch; `append` copies another batch's operations;
`iterate` passes decoded operations to a `WriteBatchHandler` and reports malformed
records through `Status`.

The encoded representation starts with an eight-byte sequence number and a
four-byte operation count. `write_batch_internal` supplies sequence/count access,
serialized contents, and insertion into a memtable. These are storage-engine
interfaces with raw-pointer contracts; the batch object itself does not perform
I/O or commit a transaction. Database-level atomic application belongs to the
caller that writes the batch.

The current implementation stores its binary representation in a `String` buffer.
This is a representation limitation requiring review for arbitrary bytes, not a
guarantee that the encoded contents are valid UTF-8.

See [the batch representation](src/write_batch.rs),
[record decoding](src/iterate.rs), and [internal operations](src/internal.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
