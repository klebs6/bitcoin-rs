# bitcoinleveldb-footer

The fixed footer at the end of a LevelDB SSTable: metaindex and index block
handles, zero padding, and the eight-byte table magic number.

`Footer::encode_to(*mut Vec<u8>)` appends exactly `FOOTER_ENCODED_LENGTH` bytes
after any existing binary prefix. Both handles must be initialized before
encoding. `decode_from` validates length and magic, decodes the handles, and
advances its borrowed Slice on success; malformed input returns a corruption
Status. Raw pointers must describe live storage for each operation.

See [representation](src/footer.rs), [encoding](src/footer_encode_to.rs),
[decoding](src/footer_decode_from.rs), and
[byte contracts](tests/binary_footer_contracts.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Upstream notices](NOTICE)
