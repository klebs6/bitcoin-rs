# bitcoinleveldb-skiplist

Arena-backed ordered set used by the LevelDB memtable. `SkipList<K, C>`
supports insertion, membership tests, ordered search, and traversal with
`SkipListIterator`. Keys satisfy `Debug + Copy + Default`; comparisons are
provided by `SkipListComparator<K>`.

Construct a list with a live `*mut Arena` and a comparator. The arena owns node
storage and must outlive the list and every iterator. Do not insert a key that
compares equal to an existing entry. The implementation does not remove
individual nodes. Readers use atomic link loads and insertions publish links
with atomic stores; the raw arena pointer and these primitives do not themselves
provide a general safe cross-thread container. Preserve the database's writer
serialization and lifetime discipline when integrating it.

See [list construction/insertion/search](src/skip_list.rs),
[iteration](src/skip_list_iterator.rs), and [node links](src/skip_list_node.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
