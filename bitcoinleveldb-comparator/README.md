# bitcoinleveldb-comparator

Bytewise key ordering and key-shortening operations for the `bitcoin-rs`
LevelDB port. `SliceComparator` combines comparison, a stable name, shortest
separator, short-successor, and bytewise-comparator access.

`BytewiseComparatorImpl` compares `Slice` values lexicographically by byte.
`bytewise_comparator()` returns a borrowed pointer to its process-lifetime
singleton named `leveldb.BytewiseComparator`; callers must not free that pointer.
The separator and successor operations shorten index keys while preserving their
ordering constraints.

`borrowed_slice_comparator_adapter_box` wraps an existing comparator pointer
without taking ownership and returns `None` for a null pointer. The pointed-to
comparator must outlive the boxed adapter. `MockComparator` and
`MockCorruptComparator` support subsystem tests.

See [the traits](src/interface.rs), [bytewise comparator](src/comparator.rs),
and [borrowed adapter](src/borrowed_slice_comparator_adapter.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
