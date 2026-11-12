# bitcoinleveldb-erroriterator

Constructs an empty iterator carrying a supplied `Status`, for paths where a
table or other iterator source could not be opened.

`new_error_iterator(&Status)` copies the status into an `EmptyIterator`, wraps it
in a `LevelDBIterator`, and returns a newly allocated raw pointer. The iterator
has no entries; callers inspect its status to recover the original error. Pass
a non-OK status when representing failure. The returned pointer is owned by the
caller and must be released through the appropriate iterator-destruction path.

See [the constructor](src/error_iterator.rs),
[`bitcoinleveldb-emptyiterator`](../bitcoinleveldb-emptyiterator/README.md), and
[`bitcoinleveldb-iterator`](../bitcoinleveldb-iterator/README.md).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
