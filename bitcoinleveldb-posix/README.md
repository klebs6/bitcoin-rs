# bitcoinleveldb-posix

POSIX environment facade and file-descriptor test helpers for LevelDB.

This crate re-exports the POSIX environment, logger, mapped/random/sequential
readers, writable files, and supporting utilities from their component crates.
It also exposes the environment test helper and close-on-exec checks used to
inspect file-descriptor inheritance across helper processes.

The concrete filesystem environment lives in
[bitcoinleveldb-posixenv](../bitcoinleveldb-posixenv/README.md). This facade and its
helper entry points depend on Unix/POSIX facilities; re-exports do not establish
support for every Unix variant or for Windows. Process-spawning and descriptor
checks belong under the bounded test workflow, not inside application startup.

See [re-exports](src/lib.rs) and
[close-on-exec checks](src/check_close_on_exec_does_not_leak_fds.rs).

[Source](src/lib.rs) · [Manifest](Cargo.toml) · [Bounded subsystem verification](../doc/testing/leveldb-subsystem.md)

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
