# bitcoinleveldb-merger

Merge cursor for sorted LevelDB child iterators.

`new_merging_iterator` combines sorted children under one comparator. The result
supports forward and reverse movement and seeks. It does not suppress duplicate
keys: the same key in multiple children is returned multiple times. With no
children it returns an empty OK iterator; with one child it returns that child.

The factory transfers ownership of the child iterators to the returned iterator.
For multiple children, the implementation owns boxed child wrappers and releases
them on destruction. All children must be sorted according to the supplied
comparator, and callers must not retain independent ownership after transfer.
Key/value slices remain borrowed from the active child.

See [factory and ownership contract](src/new_merging_iterator.rs),
[merge state](src/merger.rs), and [direction changes](src/next.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
