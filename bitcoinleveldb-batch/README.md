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

The encoded representation is owned as `Vec<u8>` and preserves arbitrary key and
value bytes, including NUL and invalid UTF-8. The generated `rep`, `rep_mut`, and
`set_rep` accessors expose this byte buffer; direct mutations must preserve the
header and record format.

See [the batch representation](src/write_batch.rs),
[record decoding](src/iterate.rs), and [internal operations](src/internal.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

License: MIT. See the [repository license](LICENSE).

[Upstream notices](NOTICE)
