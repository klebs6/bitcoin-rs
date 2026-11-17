# bitcoinleveldb-blockhandle

Block locations and block-reading helpers for LevelDB SSTables. A `BlockHandle`
records a byte offset and payload size. `encode_to(*mut Vec<u8>)` appends their
varint64 encodings to byte storage, preserving any existing prefix. Initialize
both fields first: `u64::MAX` is the uninitialized sentinel. `decode_from`
consumes a borrowed Slice and returns a Status.

`read_block` reads payload and trailer through RandomAccessFile, checks the
checksum when requested, and handles uncompressed or supported Snappy data.
The compressed-read regression explicitly records whether its Snappy backend
was available. An unsupported early return is not enabled-backend qualification.
The BlockContents ownership flags distinguish borrowed data from allocated
storage; keep backing storage alive for borrowed contents. Raw pointers must
refer to live storage for each operation.

See [handles](src/block_handle.rs), [encoding](src/block_handle_encode_to.rs),
[reading](src/read_block.rs), and [byte contracts](tests/binary_handle_contracts.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Upstream notices](NOTICE)
