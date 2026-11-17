# bitcoinleveldb-slice

Non-owning byte views used throughout the LevelDB storage interfaces.

`Slice` stores a raw pointer and byte length. Construction from a string, byte
slice, or pointer does not copy or retain the backing allocation. `as_bytes`,
indexing, bytewise comparison, prefix checks, and `remove_prefix` operate on that
view. `to_string` makes a lossy UTF-8 conversion; use `as_bytes` when byte identity
matters. `Range` holds inclusive-start and exclusive-limit key slices.

The type has no lifetime parameter. Keep backing storage valid and suitably
immutable for every use of the view, including copies of `Slice`; do not create
a view from a temporary `String` or vector. Raw-pointer construction additionally
requires a valid readable region, and C-string conversion requires a live
NUL-terminated string. Bounds checks do not establish allocation validity.
The crate currently enables Rust nightly's `core_intrinsics` feature.

See [the implementation and contracts](src/slice.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
