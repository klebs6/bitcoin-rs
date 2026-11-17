# bitcoinleveldb-env

Operating-system services and file helpers used by the LevelDB implementation.

`Env` combines file creation/reading, directory operations, locking, logging,
scheduling, thread creation, and clock/sleep traits. Concrete environments live
in other crates, including `bitcoinleveldb-posixenv` and `bitcoinleveldb-memenv`.

`EnvWrapper` retains an `Rc<RefCell<dyn Env>>` and forwards operations through a
shared serialization gate for that target. `read_file_to_bytes` reads arbitrary
file contents into a byte vector. `read_file_to_string` validates the complete
read buffer as UTF-8, including characters split across reads; invalid text
returns corruption without replacement characters. An I/O error takes precedence,
and only a valid UTF-8 prefix is exposed through the text helper on failure.

Write helpers take a `Slice`, with optional syncing, and update `CURRENT`.
`*_with_env` variants operate through a caller-owned mutable environment view.

The interfaces include raw output pointers and explicit allocation conventions.
Honor those contracts and the target's lifetime/synchronization requirements;
the use of an `Env` trait object alone does not establish Rust `Send` or `Sync`.

Source: [crate modules](src/lib.rs) · [manifest](Cargo.toml).

For repository verification, use the [bounded LevelDB test workflow](../doc/testing/leveldb-subsystem.md).

License: MIT AND BSD-3-Clause. See [LICENSE](LICENSE) and [NOTICE](NOTICE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
