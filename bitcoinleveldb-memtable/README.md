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

`encode_key` takes a `*mut Vec<u8>` scratch destination; iterator seek scratch
and returned slices preserve encoded bytes. Point reads preserve bytes through `get_bytes`; the checked text `get` adapter
retains a `String` output. See the [migration record](../doc/release/leveldb-get-bytes081.md).

`get_bytes` preserves its output on a tombstone or miss. The legacy `get`
adapter validates UTF-8 and returns `Corruption` for an invalid value; a direct
legacy tombstone lookup still clears the text output.

`MemTableConstructor::finish_impl_bytes` accepts a `ConstructorByteMap` for
binary comparison fixtures. Its existing text-map entry point remains an adapter.
The constructor retains its comparator and rejects equivalent duplicate input
keys; its iterators must die before rebuilding or destruction.

See [lifetime management](src/memtable.rs), [insertions](src/memtable_add.rs),
and [lookups](src/memtable_get.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
