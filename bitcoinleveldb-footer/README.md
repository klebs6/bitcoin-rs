# bitcoinleveldb-footer

Encoding and decoding of the fixed-size footer at the end of a LevelDB
SSTable. `Footer` contains the metaindex and index `BlockHandle` values; the
serialized footer includes padding and the table magic number.

`encode_to` appends the footer to a destination buffer. `decode_from` validates
the available length and magic, decodes both handles, and advances the supplied
`Slice` on success; malformed input returns a corruption `Status`.
`FOOTER_ENCODED_LENGTH` is the format's fixed footer size. Raw destination/input
pointers must refer to live storage for each operation.

See [the representation](src/footer.rs),
[encoding](src/footer_encode_to.rs), and [decoding](src/footer_decode_from.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
