# bitcoinleveldb-sync

Scoped locking helper for LevelDB's raw-mutex call sites.

`MutexLock::new` locks a `parking_lot::RawMutex`; dropping the guard unlocks it,
including on normal early returns or unwinding. The constructor accepts a
mutable raw pointer and rejects null. The guard does not own the mutex.

Keep the mutex allocated and valid until the guard is dropped, and honor the
raw mutex's locking and ownership rules. This is a low-level engine helper with
pointer-based lifetime obligations, not a replacement for a lifetime-checked
Rust mutex guard.

See [the guard implementation](src/mutexlock.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
