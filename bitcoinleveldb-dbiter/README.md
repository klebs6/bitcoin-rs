# bitcoinleveldb-dbiter

User-visible database iteration over LevelDB internal keys. `DBIter` filters
entries by a snapshot sequence number, hides deletion records and older values,
and supports forward and reverse traversal across user keys.

`DBIter::new` receives the underlying `LevelDBIterator`, a user comparator, a
`DBIterReadSample` callback owner, the visible sequence number, and a random seed.
`new_db_iterator` provides the subsystem's iterator wrapper. Position the iterator
with a seek operation before reading `key` or `value`, check validity when moving,
and inspect `status` for an error after traversal. Returned `Slice` values borrow
iterator-managed storage and should be consumed before advancing or destroying
the iterator.

`DBIter` owns saved keys, saved values and seek/skip scratch as byte vectors.
The iterator storage preserves binary bytes without a UTF-8 requirement.
The [byte-owner migration record](../doc/release/leveldb-iterator-bytes066.md)
documents the raw helper signature changes and remaining subsystem limits.

See [construction](src/lifecycle.rs), [iterator state](src/iter.rs),
[accessors](src/access.rs), and [visibility logic](src/find_next_user_entry.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
