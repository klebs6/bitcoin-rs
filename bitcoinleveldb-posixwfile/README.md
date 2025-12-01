# bitcoinleveldb-posixwfile

A small, tightly scoped crate providing a POSIX-backed implementation of a LevelDB-style `WritableFile` for the Bitcoin-derived LevelDB port. It encapsulates buffered appends, durable syncing, and careful error handling on UNIX-like systems.

---

## Overview

`bitcoinleveldb-posixwfile` implements a `PosixWritableFile` type that:

- Wraps a Unix file descriptor (`fd: i32`) with a fixed-size write buffer.
- Implements traits like `WritableFile`, `WritableFileAppend`, `WritableFileFlush`, `WritableFileSync`, and `WritableFileClose` used by the Bitcoin-flavored LevelDB core.
- Uses low-level POSIX syscalls (`open`, `write`, `fsync` / `fdatasync`, `close`, `fcntl(F_FULLFSYNC)` on Apple targets) via `libc` for predictable behavior.
- Differentiates manifest files (`MANIFEST*`) from regular files and applies Bitcoin/LevelDB’s durable-fsync discipline: syncing the directory containing the manifest before syncing the manifest itself.
- Mirrors the semantics of the original C++ LevelDB/Bitcoin implementation as closely as possible, including treatment of error codes and destructor behavior.

This crate is intentionally minimal and low-level. It is not a general-purpose file API but a precise building block in the Bitcoin-leveldb storage stack, with durability and crash semantics tuned to LevelDB’s expectations.

---

## Core Data Structure: `PosixWritableFile`

```rust
pub struct PosixWritableFile {
    buf:           [u8; WRITABLE_FILE_BUFFER_SIZE],
    pos:           usize,
    fd:            i32,
    is_manifest:   bool,
    filename:      String,
    dirname:       String,
    ever_valid_fd: bool,
}
```

Key invariants and behavior:

- `buf` + `pos`: in-memory staging buffer for appends. Writes are batched to reduce syscall overhead and improve I/O throughput.
- `fd`:
  - If `fd >= 0`, the OS file descriptor is considered open and owned by this instance.
  - If `fd < 0`, the descriptor is treated as closed. A negative descriptor that was once valid is handled idempotently on `close()`.
- `ever_valid_fd` distinguishes "never opened" from "already closed"; this allows `close()` to return a realistic `EBADF`-style error when the file was never valid.
- `is_manifest` and `dirname` are precomputed:
  - `is_manifest` is true when the basename starts with `"MANIFEST"`.
  - `dirname` holds the directory portion of `filename` (or `"."` if none).

Construction is explicit:

```rust
impl PosixWritableFile {
    pub fn new(filename: String, fd: i32) -> Self { /* ... */ }
}
```

`new` does not open the file; the caller is responsible for obtaining an `fd` with the desired flags and mode. This separation keeps POSIX concerns localized and predictable.

---

## Buffering and Append Semantics

### `WritableFileAppend` implementation

```rust
impl WritableFileAppend for PosixWritableFile {
    fn append(&mut self, data: &Slice) -> Status { /* ... */ }
}
```

Algorithm outline:

1. Trivial case: zero-length appends immediately return `Status::ok()`.
2. Fill remaining space in `buf` until either `data` is exhausted or the buffer becomes full.
3. If all data fit, return `Ok` without a syscall (purely buffered).
4. Otherwise, flush the current buffer (`flush_buffer()`). On error, propagate the `Status`.
5. For remaining data:
   - If `write_size < WRITABLE_FILE_BUFFER_SIZE`, copy into an empty buffer and return `Ok`.
   - If `write_size >= WRITABLE_FILE_BUFFER_SIZE`, bypass the buffer and call `write_unbuffered` directly.

This strategy approximates a batching policy: many small appends are batched in memory; large writes do not incur extra buffering overhead.

### `flush_buffer`

```rust
impl PosixWritableFile {
    pub fn flush_buffer(&mut self) -> crate::Status { /* ... */ }
}

impl WritableFileFlush for PosixWritableFile {
    fn flush(&mut self) -> crate::Status { self.flush_buffer() }
}
```

- If `pos == 0`, it is a no-op and returns `Status::ok()`.
- Otherwise, it calls `write_unbuffered` with the buffer content.
- It **always** resets `pos` to 0, even if the underlying write fails, to match the C++ LevelDB semantics.

### `write_unbuffered`

```rust
impl PosixWritableFile {
    pub fn write_unbuffered(&mut self, data: *const u8, size: usize) -> crate::Status { /* ... */ }
}
```

Low-level loop:

- Calls `libc::write(fd, data, size)` repeatedly until all bytes are written.
- Handles partial writes by advancing `data` and decreasing `size`.
- On `EINTR`, logs and retries.
- On other errors, converts the OS error into a `Status` via `posix_error` and returns immediately.

This design ensures that higher layers see a simple all-or-error contract, hiding the complexities of partial POSIX writes.

---

## Durability, Syncing, and Manifest Directories

Durability semantics are vital for LevelDB’s correctness under power loss. This crate implements a conservative, Bitcoin-inspired policy.

### `sync_fd`

```rust
impl PosixWritableFile {
    pub fn sync_fd(fd: i32, fd_path: &String, syncing_dir: bool) -> crate::Status { /* ... */ }
}
```

Behavior:

1. On macOS / iOS
   - Try `fcntl(fd, F_FULLFSYNC)` first, which provides stronger guarantees than `fsync()` with respect to power failures.
   - On success, return `Status::ok()`.
2. On other platforms
   - Prefer `fdatasync` where available (Linux, *BSD, Android) for metadata-light syncing.
   - Fall back to `fsync` when `fdatasync` is not available.
3. On success (`sync_result == 0`), return `Status::ok()`.
4. On error
   - Extract `errno`. If `syncing_dir == true` and `errno == EINVAL`, treat it as a **non-fatal** success. Some filesystems do not support directory fsync; Bitcoin explicitly tolerates this.
   - Otherwise, log and return an error `Status` via `posix_error(fd_path, err)`.

This function centralizes platform-conditional behavior and error normalization.

### `sync_dir_if_manifest`

```rust
impl PosixWritableFile {
    pub fn sync_dir_if_manifest(&mut self) -> crate::Status { /* ... */ }
}
```

- If `!is_manifest`, it returns `Status::ok()` without making syscalls.
- For manifest files:
  1. Convert `dirname` to `CString`. If the directory name contains an embedded NUL, return `io_error` with a descriptive message.
  2. Open the directory with `open(dir, O_RDONLY)`.
  3. On success, call `sync_fd(dir_fd, dirname, true)` and then `close(dir_fd)`.
  4. On failure to open, convert `errno` into a `Status` via `posix_error`.

This mirrors Bitcoin’s strategy: ensure that directory entries referencing newly created files are durable before the manifest referencing them is considered persistent.

### `sync`

```rust
impl WritableFileSync for PosixWritableFile {
    fn sync(&mut self) -> crate::Status { /* ... */ }
}

// Rough order of operations inside sync():
// 1. sync_dir_if_manifest();
// 2. flush_buffer();
// 3. sync_fd(fd, filename, false);
```

- For manifest files, syncs the parent directory first.
- Flushes any buffered data to the OS via `flush_buffer()`.
- Performs the final durable sync via `sync_fd` on the file descriptor itself.

The sequencing is deliberate: LevelDB’s metadata consistency and recovery logic assume that files referenced by the manifest are already on disk by the time the manifest is reported as synced.

---

## Closing and Drop Semantics

### `close`

```rust
impl WritableFileClose for PosixWritableFile {
    fn close(&mut self) -> Status { /* ... */ }
}
```

Behavior:

- If `fd < 0`:
  - If `ever_valid_fd` is true, treat this as a no-op, returning `Status::ok()` (idempotent close).
  - Otherwise, behave like `close(-1)` would and return an `EBADF`-style error using `posix_error`.
- If `fd >= 0`:
  1. Call `flush_buffer()` first.
  2. Call `libc::close(fd)`.
  3. If `close()` fails and the flush was successful, propagate the close error as `Status`.
  4. Set `fd` to `-1` regardless of error, reflecting ownership transfer back to the OS.

This yields a pragmatic mix of realism (EBADF when the file was never valid) and ergonomic idempotence (multiple `close()` calls on a previously valid file are safe).

### `Drop` implementation

```rust
impl Drop for PosixWritableFile {
    fn drop(&mut self) {
        if *self.fd() >= 0 {
            let _ = self.close();
        }
    }
}
```

- If `fd >= 0`, `drop` calls `close()` and discards any resulting `Status`.
- If `fd < 0`, it simply logs that the fd is already closed.

This mirrors the C++ LevelDB destructor: best-effort closure on drop, with errors only observable if you call `close()` explicitly.

---

## Path Utilities

`PosixWritableFile` includes static helpers for file path analysis:

### `dirname_static`

```rust
impl PosixWritableFile {
    pub fn dirname_static(filename: &String) -> String { /* ... */ }
}
```

- Returns the directory component of `filename` using the last `'/'`.
- If no `'/'` is found, returns `"."`.
- Includes an internal `debug_assert!` that the basename does not itself contain `/`.

### `basename`

```rust
impl PosixWritableFile {
    pub fn basename(filename: &String) -> Slice { /* ... */ }
}
```

- Returns a `Slice` pointing into the original string’s buffer.
- If a `/` exists, it returns the substring after the final slash as a borrowed slice.
- If no `/` is found, returns a slice over the entire filename.
- The lifetime ties directly to the `filename` value; callers must ensure the string outlives the `Slice` use.

### `is_manifest_static`

```rust
impl PosixWritableFile {
    pub fn is_manifest_static(filename: &String) -> bool { /* ... */ }
}
```

- Uses `basename` to obtain the last path component.
- Interprets any basename starting with `"MANIFEST"` as a manifest file.
- This classification is used internally to determine whether to sync the containing directory in `sync_dir_if_manifest`.

---

## Logging and Diagnostics

Throughout the implementation, logging macros such as `trace!` and `debug!` are invoked to record:

- File names, directory names, and file descriptors.
- Entry/exit points of critical methods (`append`, `flush_buffer`, `write_unbuffered`, `sync`, `close`, `drop`).
- System call outcomes and OS error codes.

This crate assumes a logging facade compatible with those macros (e.g., `tracing`). When integrated with the rest of the `bitcoinleveldb` ecosystem, these logs provide high-resolution introspection of I/O behavior under load and during error conditions.

---

## Integration Expectations

This crate is designed to be used as part of a larger `bitcoinleveldb` workspace or dependency graph. Typical integration points include:

- Plugging `PosixWritableFile` into a higher-level environment or factory that returns concrete `WritableFile` trait objects to LevelDB’s table and log writers.
- Passing a pre-opened file descriptor for new files; for example, using `libc::open` or Rust’s `std::os::fd::AsRawFd` interfaces from higher layers, then wrapping that descriptor in `PosixWritableFile::new`.
- Letting the rest of the LevelDB logic call `append`, `flush`, `sync`, and `close` following LevelDB’s state machine.

The provided behavior is intentionally conservative: it may favor correctness and persistence guarantees over absolute peak throughput.

---

## Safety and `unsafe` Usage

The implementation uses `unsafe` blocks for:

- Raw pointer arithmetic during buffer copies.
- Exposing string data as raw pointers (`Slice::from_ptr_len`).
- FFI calls into `libc` (`open`, `write`, `fsync`, `fdatasync`, `fcntl`, `close`).

These usages are localized and guarded by consistent invariants:

- Buffer operations respect bounds via explicit size computations.
- Loops over `write` errors treat `EINTR` as retryable.
- All C string interactions ensure NUL termination and report errors if an input path contains internal NULs.

Consumers of the crate interact through safe Rust APIs (`append`, `flush`, `sync`, `close`) and do not need to touch the underlying unsafe elements.

---

## Example (Conceptual)

Below is a schematic example. Exact types like `Slice`, `Status`, and various `WritableFile*` traits come from the surrounding `bitcoinleveldb` ecosystem and are not redefined here.

```rust
use bitcoinleveldb_posixwfile::PosixWritableFile;
use bitcoinleveldb_core::{Slice, Status};
use std::ffi::CString;

fn open_posix_writable(path: &str) -> Status { // pseudo-signature
    let c_path = CString::new(path).unwrap();
    let fd = unsafe { libc::open(c_path.as_ptr(), libc::O_CREAT | libc::O_WRONLY, 0o644) };
    if fd < 0 {
        // convert to Status via posix_error in the real integration
        unimplemented!();
    }

    let mut wf = PosixWritableFile::new(path.to_string(), fd);

    let data = Slice::from("hello world");
    wf.append(&data)?;
    wf.sync()?;
    wf.close()?;

    Ok(())
}
```

In a real system the `Status` type provides rich error reporting and the traits allow polymorphic substitution of other `WritableFile` backends (e.g., in-memory or testing variants).

---

## Crate Metadata

- **Name:** `bitcoinleveldb-posixwfile`
- **Version:** `0.1.1`
- **Edition:** `2024`
- **License:** MIT
- **Intended platforms:** POSIX-style systems (Linux, *BSD, macOS, iOS, Android, etc.) with a `libc`-compatible C library.

---

## Reliability Considerations

This crate exists to deliver robust on-disk behavior under adverse conditions:

- Power-loss durability is emphasized via `fsync`/`fdatasync` and `F_FULLFSYNC` (where available).
- Certain filesystem idiosyncrasies, specifically directory `fsync` returning `EINVAL`, are tolerated in a manner consistent with Bitcoin Core.
- Error reporting attempts to be explicit and structured via the `Status` type rather than panicking.

Systems relying on Bitcoin-leveldb can use this component as the final stage of their persistence pipeline, trusting that it adheres to the same invariants as the C++ implementation it mirrors.
