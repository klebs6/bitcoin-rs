# bitcoin-leveldb

Facade for the modular LevelDB port in `bitcoin-rs`. It re-exports selected
component crates; it does not add a separate ergonomic database wrapper.

Start with [the actual re-export list](src/lib.rs), the
[raw-handle database API](../bitcoinleveldb-db/README.md),
[options](../bitcoinleveldb-options/README.md), and
[write batches](../bitcoinleveldb-batch/README.md). Use the component crate
directly when its API is not re-exported here. The API is not the RocksDB
column-family interface.

The database layer provides key/value operations, batches, iterators, snapshots
and compaction. Batches should not be described as general transactions with
arbitrary rollback. Raw-handle calls retain explicit pointer, ownership,
length, error-buffer and destruction contracts; the facade does not turn them
into a memory-safe ownership wrapper or establish a C ABI.

## Status and verification

The recorded Linux/default-feature test inventory is qualified. The source
currently uses nightly Rust features. Publication preparation and known source
limitations are tracked in the [release-readiness record](../doc/release/leveldb-readiness062.md).
In particular, the remaining use of `String` for binary representations requires
correction before claiming general binary-data correctness or release readiness.

Use [check.py](check.py) through the [bounded subsystem testing guide](../doc/testing/leveldb-subsystem.md).
It discovers the LevelDB workspace packages and supports `--crate NAME` and
`--failed-from RECEIPT` for focused verification. Full randomized/stress cases
must run through their enforced resource profiles.

This README intentionally does not advertise an unverified registry version or
an untested application example. The [manifest](Cargo.toml) and
[source](src/lib.rs) define the current workspace integration surface.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](../LICENSE)
