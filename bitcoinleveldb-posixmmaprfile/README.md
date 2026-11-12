# bitcoinleveldb-posixmmaprfile

Read-only memory-mapped implementation of LevelDB's `RandomAccessFile` traits.

`PosixMmapReadableFile` takes ownership of an existing mapping. A successful read
returns a `Slice` directly into that mapping; the scratch-buffer argument is
unused. Reads beyond the mapped length return an I/O error and an empty result.
Dropping the object unmaps the region and releases its mmap-limiter slot when a
limiter was supplied.

Construction requires a valid mapping and, for a non-null limiter, an already
acquired slot whose limiter outlives the file object. The mapping must remain
valid until destruction, and returned views must not outlive the mapping. The
raw pointers and low-level read contract do not provide compiler-enforced
lifetimes or a general shared-thread safety guarantee. This is POSIX-specific
infrastructure normally constructed by the environment layer.

See [ownership transfer](src/create.rs), [reads](src/read.rs), and
[resource release](src/drop.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)
