# bitcoinleveldb-compat

Platform-port helpers used by the `bitcoin-rs` LevelDB implementation.

The crate exports C++-style mutex/condition-variable wrappers, platform capability
constants, CRC32C extension, heap-profile hooks, and optional Snappy helpers.
These are low-level compatibility interfaces with explicit pointer and buffer
contracts, not a general safe FFI layer or a stable C ABI.

Snappy helpers are enabled by the `leveldb_snappy` Cargo feature; without it they
return `false`. The enabled compression path currently places compressed bytes
in a `String` using unchecked UTF-8 conversion. Compressed data need not be UTF-8:
this is a representation limitation requiring review before enabling that path,
not an assurance that `String` is an arbitrary-byte container. Default-feature
qualification does not qualify the optional feature.

`acceleratedcrc32c` currently performs software CRC32C extension; its name does
not promise hardware acceleration. Capability constants and feature switches
should be read from the [manifest](Cargo.toml) and
[port configuration](src/port_config.rs), rather than treated as runtime probing.

See [synchronization wrappers](src/port_stdcxx.rs),
[compression](src/snappy_compress.rs), and [CRC implementation](src/accelerated_crc32c.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
