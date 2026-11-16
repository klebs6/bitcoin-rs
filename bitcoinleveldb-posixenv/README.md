# bitcoinleveldb-posixenv

POSIX-backed operating-system services for the LevelDB storage engine.

`PosixEnv` implements the `Env` file, directory, lock, logger, clock, scheduling,
and thread operations. `posix_default_env` and `PosixEnv::shared` return shared
environment handles as `Rc<RefCell<dyn Env>>`. The environment coordinates file
descriptor/mapping limits and a background-work queue.

File-creation APIs return boxed trait-object allocations through raw output
pointers, following `bitcoinleveldb-file` conventions. Keep shared environment
handles live while their dependent operations run, and use the wrapper's
serialization rules when forwarding through `Rc<RefCell<_>>`.

This implementation uses POSIX/libc services; this README does not claim a
portable backend for non-POSIX targets.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT (as declared in the manifest).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
