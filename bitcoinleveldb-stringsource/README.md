# bitcoinleveldb-stringsource

In-memory `RandomAccessFile` implementation used by the LevelDB table and
block tests. `StringSource::new(&Slice)` copies the supplied bytes into owned
storage; the original input only needs to remain valid during construction.

`read` copies bytes from an offset into caller-provided scratch storage and
returns a `Slice` borrowing that scratch buffer. It clamps a valid read at the
end of the contents; an offset at or beyond the stored length returns an
invalid-argument `Status`. The output and scratch pointers must be non-null,
and scratch must have room for the requested read. `size` reports the stored
byte count.

See [the implementation and behavior tests](src/string_source.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
