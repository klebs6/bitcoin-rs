# bitcoinleveldb-memtable

The in-memory sorted write table for the `bitcoin-rs` LevelDB port. `MemTable`
stores encoded entries in an arena-backed skiplist, ordered by internal key.
`add` records a sequence number, value/deletion type, key, and value; `get`
performs a sequence-aware lookup, and `new_iterator` exposes internal entries.

Memtables use explicit `ref_`/`unref` lifetime management. A new table starts with
zero references. Instances managed through `unref` must have been heap-allocated
with the ownership expected by that method; the final `unref` destroys the table.
An iterator borrows table storage, so destroy it before releasing the last table
reference. Caller/database locking governs concurrent mutation and access.

`approximate_memory_usage` reports arena accounting above the construction
baseline. The database's write-buffer setting controls when a memtable is
flushed; there is no separate write-buffer container inside this crate.

See [lifetime management](src/memtable.rs), [insertions](src/memtable_add.rs),
and [lookups](src/memtable_get.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
