# bitcoinleveldb-posixseqfile

A small, sharply focused crate that provides a POSIX-backed implementation of LevelDB's `SequentialFile` abstraction, suitable for integration into a Bitcoin-oriented LevelDB port or any storage engine that expects a LevelDB-compatible sequential file API.

---

## Overview

`bitcoinleveldb-posixseqfile` exposes a single core type:

```rust
pub struct PosixSequentialFile {
    fd:       i32,
    filename: String,
}
```

It is designed as a low-level adapter from raw POSIX file descriptors (`read`, `lseek`, `close`) to the traits that model LevelDB's **sequential file** interface:

- `SequentialFile`
- `SequentialFileRead`
- `SequentialFileSkip`
- `Named`

This crate does **not** own the higher-level database semantics; it only implements efficient, robust, sequential read access over an already-open file descriptor in a form that is idiomatic to a Rust port of LevelDB.

The design goals are:

- **Strict adherence to LevelDB semantics**: sequential-only reads, explicit skip, and EOF behavior that callers can rely on.
- **POSIX-conformant behavior**: uses `read(2)`, `lseek(2)`, and `close(2)` directly, with correct handling of `EINTR` and I/O errors.
- **Thread-friendly, but not thread-safe**: instances can be used by multiple threads if externally synchronized, but internal methods assume exclusive &mut access as per the `SequentialFile` contract.
- **Minimal overhead**: no allocation in the hot path except what is required to construct `Status` and log messages on error paths.


## Core API

### `PosixSequentialFile`

```rust
pub struct PosixSequentialFile {
    fd:       i32,
    filename: String,
}

impl PosixSequentialFile {
    pub fn new(filename: String, fd: i32) -> Self { /* ... */ }
}

impl SequentialFile for PosixSequentialFile {}
impl SequentialFileRead for PosixSequentialFile {}
impl SequentialFileSkip for PosixSequentialFile {}
impl Named for PosixSequentialFile {}
```

#### Construction

```rust
use bitcoinleveldb_posixseqfile::PosixSequentialFile;

// `fd` must be an open file descriptor positioned at the desired starting offset.
let filename = "/var/lib/blocks/index.leveldb/000123.sst".to_owned();
let fd: i32 = open_file_descriptor_somehow();

let seq = PosixSequentialFile::new(filename, fd);
```

- `filename` is stored only for diagnostics and introspection via `Named`.
- `fd` is assumed to be a valid, open, read-capable POSIX file descriptor.
- Ownership: `PosixSequentialFile` takes ownership of `fd` and will close it in `Drop`.

#### Naming

```rust
use bitcoinleveldb_posixseqfile::PosixSequentialFile;
use your_leveldb_traits::Named;

fn print_name(f: &PosixSequentialFile) {
    println!("sequential file: {}", f.name());
}
```

The `Named` implementation returns a borrowed `Cow<'_, str>` referencing the stored `filename`. This allows logging and error reporting with minimal allocation.


### Sequential Reads

The core of the abstraction is realized via `SequentialFileRead`:

```rust
impl SequentialFileRead for PosixSequentialFile {
    fn read(
        &mut self,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> Status { /* ... */ }
}
```

Semantics:

- Attempts to read **up to** `n` bytes from the current file offset into the caller-provided buffer at `scratch`.
- On success:
  - Sets `*result` to a `Slice` pointing directly into the `scratch` buffer with the actual read size.
  - If `read` returns 0 (EOF), `*result` is set to an empty `Slice`.
  - Returns `Status::ok()`.
- On error (other than `EINTR`):
  - Constructs a `Status::io_error(ctx, detail)` describing the failure.
  - Leaves `*result` as an empty `Slice`.
  - Returns that error `Status`.
- On `EINTR`:
  - Logs and automatically retries the `read` syscall in a loop.

The function is deliberately low-level:

- `result` and `scratch` are raw pointers to avoid any coupling to particular slice or buffer lifetimes in higher layers.
- The caller guarantees that `scratch` points to a writable buffer of at least `n` bytes for the duration of the call.
- `Slice` is a lightweight, non-owning view into arbitrary memory, compatible with typical LevelDB-style slice abstractions.

This design isolates unsafe operations to the implementation, while presenting a deterministic interface to the rest of the LevelDB port.


### Skipping Forward

Sequential skipping is implemented via `lseek(2)` relative to the current offset:

```rust
impl SequentialFileSkip for PosixSequentialFile {
    fn skip(&mut self, n: u64) -> Status { /* ... */ }
}
```

Semantics:

- Moves the file descriptor forward by `n` bytes using `lseek(fd, n, SEEK_CUR)`.
- If `lseek` succeeds:
  - Logs the new offset.
  - Returns `Status::ok()`.
- If `lseek` fails:
  - Captures the OS error and maps it into a `Status::io_error` with:
    - `ctx` set to the file name.
    - `detail` set to the OS error string.
  - Returns that `Status`.

Note that this is not a `pread`-style interface: it mutates the underlying file offset. This matches the expectations of LevelDB's `SequentialFile`, which models a forward-only scan.


### Drop and Resource Management

`PosixSequentialFile` owns its `fd` and is responsible for closing it when dropped:

```rust
impl Drop for PosixSequentialFile {
    fn drop(&mut self) {
        // close(fd) with logging and error reporting
    }
}
```

- On `drop`, it invokes `libc::close(self.fd)`.
- If `close` fails, it logs a warning including the `errno` and OS error text.
- Regardless of success or failure, the Rust type system ensures that the application can no longer use the file descriptor through this object after drop.

This RAII discipline reduces the risk of descriptor leaks when used from higher-level code that may early-return on error or panic.


## Threading Model

`PosixSequentialFile` is described as:

> Instances of this class are thread-friendly but not thread-safe, as required by the SequentialFile API.

Interpretation:

- **Thread-friendly**: The underlying POSIX syscalls `read`, `lseek`, and `close` are thread-safe *for independent file descriptors*. Multiple `PosixSequentialFile` instances that wrap distinct `fd`s can be used concurrently without interference.
- **Not thread-safe**: A single `PosixSequentialFile` should not be accessed from multiple threads without synchronization. This is enforced at the Rust level by the use of `&mut self` in `read` and `skip`.

In other words, concurrency is possible at the database level through sharding or independent files, but this object models a single-threaded cursor over one file.


## Error Handling and Status Mapping

All public trait methods return a `Status` object consistent with LevelDB conventions. While the exact `Status` type is defined elsewhere, this crate uses it as follows:

- `Status::ok()` for successful operations.
- `Status::io_error(ctx: &Slice, detail: Option<&Slice>)` for OS-level I/O failures.

For both `read` and `skip`:

1. The OS error is captured via `io::Error::last_os_error()`.
2. The raw `errno` is obtained via `raw_os_error().unwrap_or(0)` for logging.
3. The context is the file name; detail is the human-readable error string.

This allows higher layers to distinguish between logical database errors (e.g., corruption) and environmental errors (e.g., disk full, permission denied), which is essential for robust storage systems.


## Logging

The implementation uses structured logging macros (`trace!`, `debug!`, `warn!`) to emit:

- Entry/exit traces for `new`, `read`, `skip`, and `drop`.
- Error paths including `errno`, file name, and human-readable error text.
- EOF conditions (read of 0 bytes) and new offsets after `skip`.

The macros are assumed to be provided by an external logging facade (e.g., `tracing` or `log`-compatible macros). This crate focuses on ensuring that meaningful, structured context is always available for observability and diagnostics.


## Integration in a LevelDB Port

`PosixSequentialFile` is typically not used directly by application code. Instead, it is instantiated by an **environment** or **file system adapter** layer that implements the broader LevelDB `Env` API, for example:

```rust
use bitcoinleveldb_posixseqfile::PosixSequentialFile;
use your_leveldb_traits::{SequentialFile, SequentialFileRead, SequentialFileSkip, Slice, Status};

fn open_sequential_file(path: &str) -> Result<Box<dyn SequentialFile>, Status> {
    use std::ffi::CString;
    use libc::{open, O_RDONLY};

    let c_path = CString::new(path).unwrap();
    let fd = unsafe { open(c_path.as_ptr(), O_RDONLY) };
    if fd < 0 {
        // Map to Status::io_error via your environment layer
        return Err(/* ... */);
    }

    Ok(Box::new(PosixSequentialFile::new(path.to_owned(), fd)))
}
```

Higher-level database components (iterators, table readers, compaction code) then interact only with the trait objects, not with raw file descriptors or POSIX primitives.


## Safety Considerations

All direct interaction with POSIX syscalls is encapsulated in `unsafe` blocks inside this crate. The public API remains entirely safe **under the following preconditions**:

- The caller:
  - Supplies a valid, open file descriptor (`fd`) that remains unique to this `PosixSequentialFile` instance.
  - Avoids using the same file descriptor elsewhere after passing it to `new`.
  - Provides a `scratch` buffer of length at least `n` bytes for `read`, with a lifetime sufficient for the duration of the call.
- The crate:
  - Never dereferences `scratch` or `result` beyond the bounds specified by `n` and the `read` return value.
  - Does not expose any raw pointers to the caller, only `Slice` values.

When used through the intended LevelDB environment layer, these invariants are typically enforced at a single integration point.


## Performance Considerations

While the crate itself is small, its behavior matters for large sequential scans common in database workloads:

- **No internal buffering**: the caller controls buffering entirely, deciding `n` and the buffer strategy.
- **Minimal copies**: data is read directly into the caller's buffer; `Slice` is just a view.
- **EINTR handling**: transient interruptions are retried, avoiding spurious failures under load when signals are present.

For high-throughput workloads (e.g., LevelDB table scans in block validation), you can tune performance by:

- Choosing a large `n` and reusing a fixed-size `scratch` buffer across calls.
- Aligning I/O sizes with filesystem or storage hardware characteristics.


## Example: Manual Sequential Scan

The following example shows how you might manually loop over a file sequentially using the traits. This is mostly illustrative; real applications will likely use higher-level iterators.

```rust
use bitcoinleveldb_posixseqfile::PosixSequentialFile;
use your_leveldb_traits::{SequentialFileRead, Slice, Status};

fn scan_file(mut file: PosixSequentialFile) -> Status {
    const BUF_SIZE: usize = 64 * 1024;
    let mut buf = vec![0u8; BUF_SIZE];

    loop {
        let mut slice = Slice::default();
        let status = file.read(BUF_SIZE, &mut slice as *mut Slice, buf.as_mut_ptr());
        if !status.is_ok() {
            return status;
        }

        if slice.is_empty() {
            // EOF
            break;
        }

        // Process `slice` as a view into `buf[0..slice.len()]`.
        process_chunk(&slice);
    }

    Status::ok()
}
```


## License

This crate is licensed under the **MIT** license. See the `LICENSE` file for details.


## Rust Edition

This crate targets **Rust 2024 edition**, leveraging its language semantics while remaining compatible with stable tooling.
