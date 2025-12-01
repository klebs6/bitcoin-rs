# bitcoinleveldb-posixlogger

A minimal, POSIX-focused logging backend for `bitcoinleveldb`-style systems, implemented as a thin, unsafe wrapper around `libc::FILE` with deterministic buffering and timestamped log formatting.

## Overview

`bitcoinleveldb-posixlogger` provides a `PosixLogger` implementation designed to mirror the behavior of LevelDB's POSIX logger in C++. It writes log lines directly to a `*mut libc::FILE` using `fwrite`/`fflush`, with explicit control over:

- Timestamp formatting down to microseconds
- Thread identifier labelling
- Two-phase buffering: first on a fixed-size stack buffer, then (if needed) on a heap buffer
- Truncation semantics when log lines exceed the available buffer
- Ensuring a trailing `\n` without double-newlines

The logger is intended for environments where you either already have a C FILE* (e.g., interoperating with C/C++ code or legacy logging subsystems) or you want deterministic, minimal-overhead logging without imposing an opinionated Rust logging facade.

The type implements the `Logv` and `Logger` traits from the surrounding ecosystem (likely another crate in the `bitcoinleveldb` family). It is **not** a generic Rust logging facade by itself; instead, it is the low-level sink to which higher-level logging abstractions can write.

## Design Goals

- **POSIX-level control**: Use `gettimeofday`, `localtime_r`, `fwrite`, and `fflush` directly.
- **Deterministic buffering**: Two-phase buffering strategy that first attempts a stack allocation, then falls back to a correctly-sized heap buffer.
- **Safety via encapsulation**: All unsafe operations are concentrated inside `PosixLogger` methods; the public API is safe (except for the fact that you must pass a valid `FILE*`).
- **Minimal dependencies**: Built around `libc` and core/std primitives.

## Core Data Type

```rust
#[derive(Getters)]
#[getset(get = "pub")]
pub struct PosixLogger {
    fp: *const libc::FILE,
}
```

- The logger holds a raw pointer to a C `FILE`. Ownership is taken at construction (`PosixLogger::new`) and released in `Drop` via `fclose`.
- The generated getter `fp()` returns `*const libc::FILE`, but internally `PosixLogger` casts to `*mut libc::FILE` when writing.

## Safety Model

This crate is intentionally low-level and uses unsafe code:

- The constructor `PosixLogger::new(fp: *mut libc::FILE)` asserts that `fp` is not null.
- The `Drop` impl calls `libc::fclose(self.fp as *mut libc::FILE)` if the pointer is non-null.
- Callers **must** ensure:
  - The `FILE*` is uniquely owned by the `PosixLogger` (no concurrent closes or writes from other code).
  - No further use of the `FILE*` after the logger is dropped.

All other public methods are safe under the assumption that `PosixLogger` has exclusive logical ownership of the `FILE*`.

## Time and Formatting Semantics

### Timestamps

`capture_current_time_components` uses:

- `gettimeofday` to obtain a `libc::timeval` (`tv_sec`, `tv_usec`).
- `localtime_r` to obtain a `libc::tm` with local time components.

The header format produced by `construct_log_header_prefix` is:

```text
YYYY/MM/DD-HH:MM:SS.UUUUUU <thread-id> 
```

Example:

```text
2025/03/04-17:56:12.123456 ThreadId(7) 
```

The header is a plain ASCII `String`, including a trailing space ready for the log body.

### Thread Identification

`build_thread_identifier_label` converts `std::thread::current().id()` with `format!("{:?}", ...)`, then truncates to `Self::MAX_THREAD_ID_SIZE` if necessary. This yields a concise, printable thread label that plays well with `Debug` formatting semantics of Rust thread IDs.

## Log Body Construction

`build_log_body_from_format_and_arguments` adapts a C-style format string and a Rust slice of string arguments into a single `String` body.

Signature:

```rust
pub fn build_log_body_from_format_and_arguments(
    &self,
    format: *const u8,
    arguments: &[&str],
) -> Option<String>
```

Behavior:

- Interprets `format` as a null-terminated `CStr` (`*const libc::c_char`).
- Converts to a Rust string (UTF‑8, lossy for invalid bytes).
- Scans the template, handling:
  - `%s` → substitutes the next entry from `arguments`.
  - `%%` → a literal `%`.
  - Any other `%<char>` combination is copied as-is.
- Extra arguments beyond the number of `%s` placeholders are ignored.
- If `format` is null, logs an error and returns `None`.

This is intentionally a restricted subset of `printf`-style formatting specialized for LevelDB-style logging where only `%s` expansion is required.

## Buffering and Layout

### Sizing

`compute_required_log_bytes(header_bytes, body_bytes)` returns:

```text
required_without_newline = header.len() + body.len()
```

This excludes the trailing `\n`, which may be appended later.

### Stack vs Heap Buffer

`emit_log_line_with_two_phase_buffering` orchestrates the write:

1. Compute `required_without_newline`.
2. Attempt to format into a fixed `STACK_BUFFER_SIZE` stack buffer.
3. If the stack buffer is too small, compute a `dynamic_buffer_size` (at least `required_without_newline + 2`) and retry with a heap-allocated `Vec<u8>`.

The method iterates at most twice: once on the stack, once on the heap.

### Layout

`layout_log_line_in_buffer` decides between full or truncated writes:

```rust
pub fn layout_log_line_in_buffer(
    &self,
    header_bytes:             &[u8],
    body_bytes:               &[u8],
    buffer_ptr:               *mut u8,
    buffer_size:              usize,
    required_without_newline: usize,
    is_first_iteration:       bool,
) -> Result<usize, usize>
```

Policy:

- If `buffer_size == 0`, logs an error and returns `Ok(0)` (no write).
- If `header_bytes.len() >= buffer_size`, logs an error and returns `Ok(0)`.
- If `required_without_newline >= buffer_size - 1`:
  - On the **first** iteration, returns `Err(dynamic_size)` where `dynamic_size = required_without_newline + 2`. The caller will allocate a heap buffer of at least this size and retry.
  - On the **second** iteration, calls `copy_truncated_log_line_into_buffer` and returns `Ok(offset)`, ensuring the buffer is filled as much as possible and ends with a newline if possible.
- Otherwise, calls `copy_full_log_line_into_buffer` and returns `Ok(offset)`.

### Copying and Newline Handling

Both the full and truncated paths eventually call `ensure_trailing_newline_for_buffer`:

```rust
pub fn ensure_trailing_newline_for_buffer(
    &self,
    buffer_ptr:         *mut u8,
    buffer_size:        usize,
    current_offset:     usize,
) -> usize
```

Semantics:

- If `buffer_size == 0`, returns 0.
- If `current_offset == 0`, writes `b'\n'` at position 0 and returns 1.
- If `current_offset > buffer_size`, clamps to `buffer_size`.
- Checks the final written byte:
  - If it is `\n`, returns `current_offset` unchanged.
  - If not and `current_offset < buffer_size`, appends a `\n` and increments the offset.
  - If not and `current_offset == buffer_size`, logs a warning and leaves the buffer as-is.

Thus, writers can assume that, whenever feasible, each emitted log line terminates with exactly one newline.

## Flushing to the Log File

`flush_buffer_to_log_file` writes a buffer into the underlying `FILE*`:

```rust
pub fn flush_buffer_to_log_file(
    &self,
    buffer_ptr:    *mut u8,
    buffer_size:   usize,
    buffer_offset: usize,
)
```

- If `fp()` is null, logs an error and drops the data.
- If `buffer_ptr` is null, logs an error and returns.
- If `buffer_offset == 0`, logs a trace and returns.
- If `buffer_offset > buffer_size`, logs a warning and clamps.
- Calls `libc::fwrite` with `write_len = min(buffer_offset, buffer_size)`.
- If `written != write_len`, logs a warning.
- Calls `libc::fflush` to ensure immediate persistence to the file.

## Logging Entry Point: `logv`

The main interface required by the `Logv` trait is:

```rust
impl Logv for PosixLogger {
    fn logv(&mut self, format: *const u8, arguments: &[&str]) {
        // ...
    }
}
```

Execution path:

1. Check that `fp()` and `format` pointers are non-null.
2. Capture the current timestamp and time components via `capture_current_time_components`.
3. Build the thread identifier (`build_thread_identifier_label`).
4. Construct the log body from the C-style format and arguments (`build_log_body_from_format_and_arguments`). If this fails, skip the line.
5. Construct the header prefix (`construct_log_header_prefix`).
6. Emit the combined header + body through `emit_log_line_with_two_phase_buffering`.

This method is designed for integration with a higher-level logging wrapper that provides the `format: *const u8` and `arguments: &[&str]` slices, often mirroring how LevelDB collects its arguments.

## Usage

### Basic Construction

```rust
use bitcoinleveldb_posixlogger::PosixLogger;
use std::ffi::CString;

fn main() {
    unsafe {
        // Open a C FILE* in append mode
        let path = CString::new("/var/log/bitcoinleveldb.log").unwrap();
        let mode = CString::new("a").unwrap();
        let fp = libc::fopen(path.as_ptr(), mode.as_ptr());

        if fp.is_null() {
            panic!("failed to open log file");
        }

        let mut logger = PosixLogger::new(fp);

        // Prepare a C-style format string: "%s: %s" with two string args
        let fmt = CString::new("%s: %s").unwrap();
        let format_ptr = fmt.as_ptr() as *const u8;

        let args = ["INFO", "database opened"]; // &[&str]

        logger.logv(format_ptr, &args);

        // PosixLogger::drop will fclose(fp) automatically
    }
}
```

### Using as a Backend in a Larger System

Typical integration with a broader `bitcoinleveldb` logging framework might look as follows (sketch):

```rust
struct DbEnvironment {
    logger: PosixLogger,
}

impl DbEnvironment {
    fn log_info(&mut self, msg: &str) {
        use std::ffi::CString;

        // Single %s placeholder
        let fmt = CString::new("%s").unwrap();
        let format_ptr = fmt.as_ptr() as *const u8;
        let args = [msg];

        self.logger.logv(format_ptr, &args);
    }
}
```

The environment or database wrapper is responsible for holding the `PosixLogger` as a member and wiring it into the rest of the system.

## Error Reporting and Diagnostics

The implementation uses logging macros such as `trace!`, `debug!`, `info!`, `warn!`, and `error!`. These macros must be provided by your surrounding project (e.g., through a global logging facade or a local macro re-export). The crate assumes these macros are available and will not compile if they are not defined.

These internal logs can be helpful when validating the behavior of the logger:

- Buffer sizing problems
- Null pointers
- Partial writes via `fwrite`
- Time acquisition failures (`gettimeofday` / `localtime_r`)

In production, you may route these macros into a more global logging system for visibility.

## Performance Considerations

- **Stack buffer**: The common path uses a fixed-size stack buffer (`STACK_BUFFER_SIZE`) and avoids heap allocations for typical log lines.
- **Heap buffer fallback**: On rare large messages, `emit_log_line_with_two_phase_buffering` allocates a single `Vec<u8>` sized precisely for the needed capacity.
- **System calls**: Each log line ends with `fwrite` + `fflush`. This trades throughput for durability: logs are forced out promptly. Integrators optimizing for high-volume logging may wish to relax flushing frequency at a higher level.
- **Copying**: Header and body bytes are copied once into the final buffer (`copy_full_log_line_into_buffer` or `copy_truncated_log_line_into_buffer`), and then written with a single system write call.

## Safety and Concurrency

`PosixLogger` does not itself perform any synchronization around the underlying `FILE*`. You must ensure that:

- Either:
  - Each `PosixLogger` has exclusive ownership of its `FILE*` and is used from a single thread; or
  - External synchronization (e.g., a `Mutex<PosixLogger>`) is used when logging from multiple threads.

The internal thread identifier labeling assumes multiple threads may log, but the implementation does not attempt to serialize writes; serialization must be handled by the higher-level system.

## License

This crate is licensed under the MIT License.

## Caveats

- The `Logv` and `Logger` traits, as well as the logging macros, are assumed to be provided externally (e.g., another crate in the same workspace). This crate focuses on the concrete POSIX implementation.
- The implementation assumes a POSIX-like environment with `libc` and the usual C runtime facilities available.

