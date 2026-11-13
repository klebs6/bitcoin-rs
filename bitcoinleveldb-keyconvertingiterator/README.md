# bitcoinleveldb-keyconvertingiterator

Iterator adapter that exposes user keys from LevelDB internal keys.

`KeyConvertingIterator` delegates movement and values to an internal iterator,
parses the current internal key, and returns its user-key portion. A malformed
internal key is surfaced as a corruption `Status` rather than a valid user key.
The adapter also retains the normal iterator cleanup representation.

Construction transfers ownership of a raw `Box<dyn LevelDBIteratorInterface>`
pointer to the adapter. Do not separately free or continue owning that pointer;
dropping the adapter reclaims it. Key/value `Slice` views remain borrowed from
the underlying iterator and must not outlive movement or destruction.

`seek` builds the exact user-key bytes and maximum sequence/type tag in a
byte buffer before delegation. The recording iterator helpers in this crate
still have textual convenience interfaces; the binary regression exercises
the adapter directly. See the [migration record](../doc/release/leveldb-iterator-bytes066.md).

See [construction](src/create.rs), [key conversion](src/key.rs), and
[ownership cleanup](src/drop.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
