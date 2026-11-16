# bitcoinleveldb-dbimplwriter

Per-writer queue state for LevelDB's database write path.

`DBImplWriter` stores a batch pointer, completion `Status`, sync request, done
flag, and condition variable. `new` initializes a waiting writer with no batch,
no sync request, and an unset completion flag. The mutex argument is retained in
the constructor signature; the condition variable is constructed independently.

This is internal coordination state, not a writer thread, transaction API, or
independent owner of the batch. The database implementation supplies locking,
queue ordering, batch lifetime, and wake-up behavior.

See [the writer state](src/bitcoinleveldb_dbimplwriter.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
