# bitcoinleveldb-posixtools

POSIX locking, resource-limit, error, and environment helpers for LevelDB.

The crate provides `PosixFileLock`, process-local duplicate-lock bookkeeping,
`fcntl` lock/unlock support, POSIX-error conversion to `Status`, and limits used
for read-only descriptors and memory mappings. `SingletonEnv` retains an
`Rc<RefCell<dyn Env>>` handle for environment wiring.

The lock table complements OS locking; it does not replace it. Descriptor and
mmap limit controls are process-wide values, so changing them affects other
users of these helpers. `SingletonEnv`'s reference-counted handle is not a
thread-safe environment-sharing API. Use these primitives through the POSIX
environment unless implementing that environment or its tests.

See [lock bookkeeping](src/posix_lock_table.rs),
[OS locking](src/lock_or_unlock.rs), [limits](src/max_open_files.rs), and
[environment handle](src/singleton_env.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
