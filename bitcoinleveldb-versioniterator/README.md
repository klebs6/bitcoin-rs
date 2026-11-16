# bitcoinleveldb-versioniterator

An internal iterator over the files in one LevelDB version level.

`VersionLevelFileNumIterator` implements the `LevelDBIteratorInterface` traits
over a caller-supplied file-metadata vector. Each key is the file's largest
internal key. Each value is 16 bytes containing the file number followed by the
file size, both fixed-width little-endian `u64` values.

The iterator starts invalid. Seek or position it before calling `key`, `value`,
`next`, or `prev`. The metadata vector, its file entries, and the comparator
must remain valid for the iterator's lifetime; returned slices borrow metadata
or the iterator's reusable value buffer. This iterator reports file information,
not the key/value records inside those files.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
