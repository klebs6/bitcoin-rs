# bitcoinleveldb-modeldb

An in-memory reference database for LevelDB comparison tests. It provides
byte-preserving writes, deletes, point reads, iterators, and copied snapshots
through the database operation traits. It is test support, not persistent storage.

`ModelDB` owns byte keys and values. Its configured comparator controls key
equivalence, replacement, deletion, ordering, and seeking. Lookup and equivalent-key
updates scan the reference map; this is not an application performance model.
`DBGet::get_bytes` returns exact bytes. The inherited text `get` adapter accepts
valid UTF-8 values and reports `Corruption` otherwise, preserving output on error
or a missing key. Empty keys and values are supported.

Iterators own their data and comparator and can outlive the model, read options,
and source snapshot. Model snapshots expose an immutable byte-map capability;
unsupported snapshot kinds return an error instead of reading live state.
`ModelSnapshot::byte_map_ref` works for arbitrary bytes. Its legacy `map_ref`
requires a fully UTF-8 map and panics otherwise; use `try_map_ref` for an optional
strict text view. `new_from_map` and legacy `ModelIter` retain their text-map APIs.
The legacy iterator still requires its caller to honor its raw-pointer ownership
contract; ModelDB itself no longer uses that borrowed-map path.

Properties are unavailable, approximate sizes are zero, and compaction is a no-op.
Use the persistent database implementation for application storage.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
