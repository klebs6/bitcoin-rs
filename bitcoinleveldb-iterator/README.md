# bitcoinleveldb-iterator

Owning wrapper for the LevelDB iterator trait interface.

`LevelDBIterator` contains an optional boxed `LevelDBIteratorInterface` and
forwards validity, movement, seek, key/value, and status operations. It caches
current iterator state and retains the cleanup representation used by storage
components. `set`/`reset_iterator` replace the wrapped implementation; the old
implementation is dropped. An unattached wrapper is invalid.

This is LevelDB's cursor-style interface, not Rust's standard `Iterator` trait.
Check `valid()` before reading key or value. Returned `Slice` values borrow
underlying iterator storage: movement, replacement, or destruction may invalidate
them. Owning the iterator implementation does not automatically own the database
or block storage that implementation references.

See [wrapper state](src/iterator.rs), [delegation](src/prev_next.rs), and
[destruction](src/drop.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
