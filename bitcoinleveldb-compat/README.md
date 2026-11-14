# bitcoinleveldb-compat

Platform-port helpers for the bitcoin-rs LevelDB implementation: mutex/condition
wrappers, capability constants, CRC32C extension, heap-profile hooks and optional
Snappy compression. These low-level pointer interfaces are not a stable C ABI.

With `leveldb_snappy`, `snappy_compress(input, length, *mut Vec<u8>)` replaces
the destination with raw Snappy bytes on success. False leaves the existing
output unchanged. Non-null input must describe readable bytes, and the output
must be a live, exclusively accessible byte vector. Input may borrow the old
output allocation; compression finishes reading before replacing that owner.
Without the feature, Snappy helpers return false without reading the buffers.
Decoded output ports require enough live writable storage; validate the reported
uncompressed length against the caller's allocation policy first.

`acceleratedcrc32c` currently uses software CRC32C; its name does not promise
hardware acceleration. Read the [manifest](Cargo.toml) and
[port configuration](src/port_config.rs) for capabilities and feature switches.

See [synchronization](src/port_stdcxx.rs), [compression](src/snappy_compress.rs),
and [binary codec contracts](tests/binary_snappy_contracts.rs).
Default and named Snappy results are separate qualification profiles.

[LevelDB subsystem](../bitcoin-leveldb/README.md) ·
[Bounded verification](../doc/testing/leveldb-subsystem.md) ·
[Repository and support](https://github.com/klebs6/bitcoin-rs/issues).
License: MIT; see the [repository license](../LICENSE).
