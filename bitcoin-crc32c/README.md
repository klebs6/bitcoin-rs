# bitcoin-crc32c

CRC32C (Castagnoli) checksums and incremental extension for byte buffers.
The public entry points in [src/interface.rs](src/interface.rs) include
`crc32c_value`, `crc32c_extend` and the safe `&str` convenience input
`crc32c_with_str`.

The pointer-based checksum functions require a pointer valid for the supplied
byte count. Callers of low-level load/backend helpers must also uphold those
helpers' pointer and CPU-feature requirements. CRC32C detects accidental
corruption; it is not a cryptographic authentication mechanism.

The dispatcher selects a compiled x86_64 SSE4.2 or AArch64 backend when its
runtime check succeeds, otherwise using the portable implementation. See
[platform checks](src/platform.rs) and [module selection](src/lib.rs).
Source availability is not a cross-platform qualification claim: the current
LevelDB evidence records the measured Linux x86_64 environment.

The source uses nightly Rust features. The [manifest](Cargo.toml) records
dependencies; [upstream notices](NOTICE) preserve the Google CRC32C and
LevelDB license texts alongside the repository notice.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](LICENSE) · [Upstream notices](NOTICE)
