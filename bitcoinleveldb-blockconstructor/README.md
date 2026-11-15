# bitcoinleveldb-blockconstructor

Construction fixtures for comparing LevelDB blocks with tables, memtables and
databases. `BlockConstructor` owns encoded block bytes and its comparator;
`new_iterator` borrows that backing storage. Destroy iterators before rebuilding
or dropping the constructor.

The shared `Constructor` owns a scoped `ConstructorByteMap`. `add_bytes` preserves
literal keys/values and coalesces comparator-equivalent keys: the first key stays,
the last value wins. `finish_bytes` returns the map plus keys in comparator order,
then clears the input. The map's own BTreeMap traversal is byte order, not the
configured comparator order. Comparator names and ordering must match Options;
matching names alone do not prove matching behavior.

`finish_impl_bytes` builds a block from a byte map. Direct maps must contain no
comparator-equivalent duplicates; ambiguous input is rejected. Existing text
`add`, `data`, `finish` and `finish_impl` signatures remain. Text views and outputs
validate every key/value; `try_data` is optional, while `data`/`finish` panic for
binary data rather than converting it. Use the byte APIs for arbitrary values.
The unrelated global text `KVMap` alias is unchanged.

Source: [shared fixture](src/constructor.rs) · [block construction](src/block_constructor_finish_impl.rs).
[Manifest](Cargo.toml) · [Bounded verification](../doc/testing/leveldb-subsystem.md)
License: MIT. [Repository license](../LICENSE) · [Support](https://github.com/klebs6/bitcoin-rs/issues)
