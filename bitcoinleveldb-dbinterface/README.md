# bitcoinleveldb-dbinterface

Operation traits for LevelDB-style databases.

`DB` combines the traits for opening a database, put/delete/batch writes, get,
iteration, snapshots, properties, approximate sizes, and compaction. Separate
traits let implementations and callers use the operation they need without
importing a concrete storage engine.

The interface preserves low-level ownership conventions: `DBOpen::open` writes
a heap-allocated database through an output pointer, and `DBNewIterator` returns
a caller-owned iterator that starts invalid. Position it before reading entries,
and destroy it before its database. A snapshot obtained with `DBGetSnapshot`
must be released through the corresponding database implementation.

Implementing `DB` does not itself add synchronization or durability; those
properties depend on the implementation and correct use of its contracts.

`DBGet::get_bytes` reads arbitrary value bytes into a `Vec<u8>` output. Its
default returns `NotSupported` until an implementation opts in. The default
text `get` adapter validates UTF-8 and preserves its output on errors; existing
implementations may override that legacy text method.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
