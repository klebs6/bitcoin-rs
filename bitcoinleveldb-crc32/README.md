# bitcoinleveldb-crc32

CRC32C checksums and LevelDB checksum masking.

- `crc32c_value(data, len)` computes a checksum from an initial value of zero.
- `crc32c_extend(crc, data, len)` extends a checksum with more bytes.
- `crc32c_mask` and `crc32c_unmask` convert stored LevelDB checksums to and from
  their masked representation.

The current `crc32c_extend` implementation uses the portable table-based path.
The separate `crc32c_can_accelerate` probe does not switch that implementation
to a hardware backend.

Pointer/length functions require valid readable storage for the requested range,
despite their safe Rust signatures. This is a corruption-detection checksum,
not a cryptographic hash or authentication primitive.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
