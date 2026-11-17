# bitcoinleveldb-blockcontents

The byte view and ownership metadata passed between LevelDB block readers and
block objects.

`BlockContents::new(data, cachable, heap_allocated)` stores a `Slice` and two
flags. `cachable` indicates whether the block may be cached;
`heap_allocated` tells the receiving block whether it must reclaim the buffer.
Generated accessors expose these fields. The default is an empty, non-cachable,
non-heap-allocated view.

This type does not copy the bytes or free them on drop. Callers must keep borrowed
storage live and accurately describe allocation ownership before passing it to
an owning block.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
