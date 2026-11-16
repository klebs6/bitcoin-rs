# bitcoinleveldb-blockutil

Low-level decoding of a LevelDB table-block entry header.

`decode_entry` reads the shared-key length, unshared-key length, and value length
from the range `[p, limit)`. It handles one-byte headers and varint32 headers,
checks that the key suffix and value fit, and returns a pointer to the key suffix.
A null return indicates a detected malformed or truncated entry.

The API uses raw pointers even though the function is not declared `unsafe`.
Input pointers must delimit one valid allocation, and all three output pointers
must be writable. The returned pointer borrows the input bytes; the function
does not allocate or reconstruct the complete key.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
