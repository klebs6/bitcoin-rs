# bitcoinleveldb-emptyiterator

An always-invalid LevelDB iterator with a stored status.

`EmptyIterator::new(status)` represents either an empty successful result or a
terminal error. Seeking and movement leave it invalid; `status()` returns the
stored result. `new_empty_iterator()` allocates a `LevelDBIterator` wrapper with
an OK status and returns its owned raw pointer.

An empty result and an error are distinguished by status, not by `valid()`.
Callers receiving the raw factory result must arrange its destruction through
the matching iterator ownership path. Use the normal iterator contract rather
than treating an invalid iterator's key/value accessors as data.

See [the iterator and factory](src/empty_iterator.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
