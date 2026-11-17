# bitcoinleveldb-memenv

An in-memory `Env` implementation for LevelDB tests and transient databases.

`new_mem_env(base_env)` returns an `Rc<RefCell<dyn Env>>` whose file contents are
held in memory. `InMemoryEnv::new` constructs the concrete environment. File
state is reference-counted across sequential, random-access, and writable file
handles; timing and background execution delegate to the retained base
environment.

This is not a persistent filesystem. Directory creation/deletion are no-ops,
and its file-lock objects do not provide POSIX process-lock exclusion. Use the
POSIX environment when real filesystem persistence and locking are required.
The raw file-state pointers and output parameters follow the workspace's
explicit lifetime and ownership conventions.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
