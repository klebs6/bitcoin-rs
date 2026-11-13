# bitcoinleveldb-coding

Fixed-width little-endian integers, 32/64-bit base-128 varints, and
length-prefixed slices for the bitcoin-rs LevelDB port.

Use `put_fixed32_bytes`, `put_fixed64_bytes`, `put_varint32_bytes`,
`put_varint64_bytes`, and `put_length_prefixed_slice_bytes` with `Vec<u8>`
outputs. They append exact encoded bytes, including bytes outside UTF-8. The
length-prefixed byte helper copies its input before growing the destination,
accepts a Slice into the previous destination allocation, and returns `false`
without appending when its length exceeds `u32::MAX`.

The legacy `String` output helpers remain pending migration. Arbitrary binary
encodings do not satisfy Rust's `String` UTF-8 invariant; do not use those
helpers as general binary owners. Their presence remains a release blocker.

The pointer-range `encode_*`, `decode_*`, and `get_*` functions require live,
sufficiently sized caller storage. `get_varint32`, `get_varint64`, and
`get_length_prefixed_slice` consume a mutable Slice on success; decoded slices
borrow the original payload. See [byte append contracts](tests/byte_append_contracts.rs)
and [length-prefixed encoding](src/put_length_prefixed_slice.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT; see the [repository license](../LICENSE).
