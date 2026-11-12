# bitcoinleveldb-dbiterstate

Iterator cleanup state for retained LevelDB memtables and versions.

`IterState` groups a database mutex pointer, mutable and immutable memtable
pointers, and a version pointer. `cleanup_iterator_state` is the iterator cleanup
callback: it takes ownership of the heap-allocated state, locks the supplied
mutex when present, releases the retained non-null references, and frees the
state. A null callback argument is a no-op.

This is an engine-internal callback contract. A non-null state must come from the
matching boxed allocation, refer to valid retained objects, and be cleaned up
only once. The callback does not establish those references or own the database
mutex. The crate currently enables Rust's `ptr_metadata` nightly feature.

See [state and callback implementation](src/iterstate.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
