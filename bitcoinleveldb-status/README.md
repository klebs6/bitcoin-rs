# bitcoinleveldb-status

Result codes and diagnostic messages for LevelDB operations.

`Status::ok()` and `Status::default()` represent success. Error constructors
cover not found, corruption, unsupported operation, invalid argument, and I/O
error. Callers inspect predicates or `StatusCode`; `to_string` supplies a
human-readable message. The type owns its error state and supports copying and
explicit move-style helpers.

A non-OK status stores a length, code, and message bytes internally. This internal
representation is not a stable Rust/C ABI or a promise about consensus-critical
error wording. Prefer the typed code/predicates for control flow. Creating a
status from a `Slice` copies the message; the source view must be valid during
construction.

See [constructors, predicates, and formatting](src/status.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
