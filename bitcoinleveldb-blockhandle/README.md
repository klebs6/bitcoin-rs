# bitcoinleveldb-blockhandle

Block locations and block-reading helpers for LevelDB SSTable files.
`BlockHandle` records an offset and payload size, encodes them as varints, and
decodes them with a `Status` result. `BLOCK_TRAILER_SIZE` describes the trailing
compression-type byte and checksum.

`read_block` reads a handle's payload and trailer through `RandomAccessFile`,
applies the requested checksum policy, and dispatches uncompressed or Snappy
contents into `BlockContents`. Its output pointer must be valid. The returned
contents' ownership flags distinguish borrowed data from allocated storage; keep
the backing file or buffer alive for borrowed contents.

See [handles](src/block_handle.rs), [the read path](src/read_block.rs), and
[`BlockContents`](../bitcoinleveldb-blockcontents/src/block_contents.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
