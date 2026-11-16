# bitcoinleveldb-filter

Filter-policy interfaces and the filter blocks stored in LevelDB table files.

`FilterPolicy` combines `Named`, `CreateFilter`, and `KeyMayMatch`. A policy's
name identifies its filter encoding; incompatible encodings need distinct
names. The concrete Bloom policy is supplied by `bitcoinleveldb-bloom`.

`FilterBlockBuilder::new` takes a boxed policy. Feed it nondecreasing block
offsets with `start_block`, add keys, then call `finish`. The returned `Slice`
borrows the builder's result buffer and must not outlive or be invalidated by it.
Filters are grouped by 2-KiB data-offset regions.

`FilterBlockReader::new` takes an `Arc<dyn FilterPolicy>` and copies the encoded
block bytes into owned shared storage. `key_may_match` consults the policy for
the relevant region; a possible match is not proof that a key exists.
Pointer-backed input slices must describe valid readable storage.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
