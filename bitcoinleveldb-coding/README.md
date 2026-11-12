# bitcoinleveldb-coding

Binary encoding primitives used by the `bitcoin-rs` LevelDB port: fixed-width
little-endian integers, 32/64-bit varints, and length-prefixed slices.

The `encode_*`/`decode_*` and pointer-range `get_*` functions operate on caller
storage. `get_varint32`, `get_varint64`, and `get_length_prefixed_slice` consume
the beginning of a mutable `Slice` on success. `put_*` helpers append encoded
bytes to the subsystem's `String`-backed buffers; these buffers are used as byte
containers, not as a promise that arbitrary encoded data is UTF-8 text.

Raw input/output pointers, lengths, and limits must describe live, sufficiently
sized memory. A decoded `Slice` borrows its original payload. The length prefix
for `put_length_prefixed_slice` is limited to `u32`; an oversized length is
rejected without appending data.

See [varint decoding](src/get_varint.rs),
[length-prefixed output](src/put_length_prefixed_slice.rs), and
[tests](tests/put_length_prefixed_slice.rs). Encoding helpers alone do not
establish interoperability of a complete database with another implementation.

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](../LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
