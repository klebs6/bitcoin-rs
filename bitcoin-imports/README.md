# bitcoin-imports

Shared prelude, re-exports and compatibility helpers for `bitcoin-rs`.
[src/lib.rs](src/lib.rs), [module utilities](src/util.rs) and the
[manifest](Cargo.toml) define the actual surface and dependency selection.

This crate re-exports a broad set of standard-library and third-party items
and provides workspace macros such as `x!`. Consumers inherit this broad
dependency surface. Cargo manifests and the lockfile control resolution;
the prelude does not establish API compatibility between dependency versions.

The source uses nightly Rust features. Some unrelated compatibility helpers
remain placeholders: `Broken`'s dereference and `BrokenDelegate` methods
currently call `todo!()`. They are not operational Bitcoin node APIs.

[build.rs](build.rs) may scan the parent workspace through
`PersistentWorkspaceTypeMap`. It skips that scan when `CARGO_PUBLISH` is set
or when the parent manifest is absent. Detached consumer behavior remains
a release check. The repository inherits its selected libevent Git fork. Detached LevelDB
release packaging explicitly selects published `libevent =0.2.0` and
`libevent-sys =0.4.0`; the public re-exports remain. The readable
[release transformation](https://github.com/klebs6/bitcoin-rs/blob/master/contrib/release/leveldb-registry-projection096.py)
records this dependency/API version change. The isolated current-host library
consumer check passed; wider Core, other platforms and runtime compatibility
are separate qualifications.

[Repository and support](https://github.com/klebs6/bitcoin-rs) · [License](LICENSE) · [Upstream notices](NOTICE)
