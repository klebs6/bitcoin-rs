# bitcoinleveldb-hash

The seeded 32-bit hash used by LevelDB's internal data structures.

`leveldb_hash(data, len, seed)` mixes little-endian four-byte words and remaining
tail bytes with wrapping arithmetic. It returns the same kind of compact,
non-cryptographic hash used for internal lookup and filtering decisions.

The pointer must identify readable storage for `len` bytes. The function neither
owns nor copies that storage, and its safe signature does not validate the
pointer/length pair. It is not suitable for signatures, authentication, or
cryptographic commitments.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
