# bitcoinleveldb-posixlogger

POSIX diagnostic logger for the `bitcoin-rs` LevelDB environment.
`PosixLogger` implements the subsystem's `Logger`/`Logv` traits and writes lines
with time and thread labels to a C runtime `FILE*`.

`PosixLogger::new` takes ownership of a non-null `*mut libc::FILE`; dropping the
logger calls `fclose`. Do not close that stream independently while the logger
is alive. Formatting uses a stack buffer for short lines and a heap buffer when
the required line is larger, and ensures a trailing newline. This is a
POSIX/libc implementation rather than a portable application logging facade.

See [stream ownership](src/posix_logger.rs),
[two-phase buffering](src/posix_logger_emit_log_line_with_two_phase_buffering.rs),
and [stream writes](src/posix_logger_flush_buffer_to_log_file.rs).

Part of the [bitcoin-rs LevelDB subsystem](../bitcoin-leveldb/README.md).
For bounded verification, see the [subsystem testing guide](../doc/testing/leveldb-subsystem.md).

License: MIT. See the [repository license](LICENSE).

[Repository and support](https://github.com/klebs6/bitcoin-rs/issues)

[Upstream notices](NOTICE)
