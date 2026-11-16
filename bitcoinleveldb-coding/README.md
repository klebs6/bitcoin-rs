# bitcoinleveldb-coding

Fixed-width little-endian integers, 32/64-bit base-128 varints, and
length-prefixed slices for the bitcoin-rs LevelDB port.

The `put_*_bytes` helpers append exact encoded bytes to `&mut Vec<u8>`. The
corresponding `put_fixed32`, `put_fixed64`, `put_varint32`, `put_varint64`, and
`put_length_prefixed_slice` functions take `*mut Vec<u8>` and assert that the
destination is nonnull before dereferencing it. Nonnull destinations must be
live, writable byte vectors. The encoding append APIs have no String destination.
Encoded bytes need not be UTF-8.

Length-prefixed append copies its source before growing the destination, so a
Slice may borrow the previous destination allocation. Lengths above `u32::MAX`
leave the destination unchanged: the `_bytes` helper returns `false`, while the
raw-pointer wrapper returns no value.

The pointer-range `encode_*`, `decode_*`, and `get_*` functions require live,
sufficiently sized caller storage. Decoded slices borrow their source payload.
See [append contracts](tests/byte_append_contracts.rs) and
[length-prefixed encoding](src/put_length_prefixed_slice.rs).

Part of the [LevelDB subsystem](../bitcoin-leveldb/README.md).
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT; see the [repository license](LICENSE).

[Upstream notices](NOTICE)
