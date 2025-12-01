# bitcoinleveldb-env

Bindings and utilities for LevelDB's `Env` abstraction in Rust, specialized for the `bitcoinleveldb` port. This crate exposes the environment layer used by LevelDB (files, logging, scheduling, time) and provides thin, allocation-safe wrappers around the original C++-style API.

---

## Overview

LevelDB isolates all OS interaction behind the `Env` interface. In the original C++ implementation, this allows the database to run against POSIX, custom filesystems, injected schedulers, or test harnesses without re‑architecting the core engine.

`bitcoinleveldb-env` provides a Rust translation of that environment layer, suitable for:

- Embedding LevelDB semantics into a Rust application or library
- Substituting custom environments (in‑memory, sandboxed, or instrumented)
- Intercepting and tracing filesystem and timing operations used by LevelDB

The design mirrors the C++ API very closely:

- Methods use raw pointers (e.g. `*mut Box<dyn SequentialFile>`) where the C++ API allocates and transfers ownership via out‑parameters.
- `Status` is used instead of `Result` to remain compatible with the LevelDB ecosystem.
- `Rc<RefCell<dyn Env>>` is used to model shared, dynamically dispatched environments, preserving mutability where LevelDB expects it.

This crate intentionally favors fidelity to the original semantics over idiomatic Rust ergonomics. It is aimed at consumers who need low‑level control, binary compatibility with existing logic, or precise behavioral matching with LevelDB.

---

## Features

- `Env` trait: marker trait representing a LevelDB environment implementation.
- Extended environment traits providing the operational surface:
  - `NewLogger`
  - `NowMicros`
  - `SleepForMicroseconds`
  - `GetTestDirectory`
  - `StartThread`
  - `Schedule`
- `EnvWrapper`:
  - Delegating wrapper around an underlying `Env` (via `Rc<RefCell<dyn Env>>`).
  - Forwards filesystem, scheduling, timing, and logging calls to an inner environment.
  - Ideal for partial overrides (e.g. wrap an existing OS env, override only logging or scheduling).
- `RcWritableFileAdapter`:
  - Adapter implementing the `WritableFile*` traits for an `Rc<RefCell<dyn WritableFile>>`.
  - Preserves shared ownership while conforming to the LevelDB file API.
- File utility functions:
  - `read_file_to_string` – read an entire file into a `String` via an `Env`'s `SequentialFile` interface.
  - `write_string_to_file` – write a `Slice` to a file (no `Sync`).
  - `write_string_to_file_sync` – write and `Sync()` a file.
  - `set_current_file` – atomically update the `CURRENT` file for a LevelDB database directory.
- Logging utility:
  - `log` – forward formatted messages to a `Logger` implementation, with diagnostics around pointer safety and UTF‑8 validity.

All key operations are instrumented with `trace!`/`debug!` logging, making it straightforward to inspect and profile LevelDB's interaction with the environment.

---

## When to use this crate

Use `bitcoinleveldb-env` if you:

- Are integrating the `bitcoinleveldb` Rust port and need to control its environment (filesystem bindings, clocks, threads, tests).
- Need a test harness that simulates disk failures, latency, crashes, or concurrency patterns by wrapping an underlying `Env`.
- Want to intercept or instrument file I/O, logging, or scheduling at the LevelDB boundary for observability and debugging.

If you are looking for a high‑level, idiomatic Rust database API, this crate is likely too low‑level and specialized.

---

## Core traits

### `Env`

```rust
pub trait Env {}
```

`Env` is a marker trait; the operational methods live in additional traits (`NewSequentialFile`, `DeleteFile`, `Schedule`, etc.) that are implemented for your environment type. The environment is typically used through `Rc<RefCell<dyn Env>>` combined with those additional traits.

The original C++ comment applies here as design intent:

> Return a default environment suitable for the current operating system. Sophisticated users may wish to provide their own Env implementation instead of relying on this default environment. The result of Default() belongs to leveldb and must never be deleted.

A concrete environment type normally implements:

- `Env` (marker)
- File traits: `NewSequentialFile`, `NewRandomAccessFile`, `NewWritableFile`, `NewAppendableFile`, `FileExists`, `GetChildren`, `GetFileSize`, `CreateDir`, `DeleteDir`, `DeleteFile`, `RenameFile`, `LockFile`, `UnlockFile`
- Time / threading traits: `Schedule`, `StartThread`, `NowMicros`, `SleepForMicroseconds`
- Logging / test traits: `NewLogger`, `GetTestDirectory`

The exact trait set is provided by the broader `bitcoinleveldb_*` crates, but `EnvWrapper` demonstrates the forwarding surface.

### Time and scheduling traits

These traits abstract the clock and task execution model:

```rust
pub trait NowMicros {
    fn now_micros(&mut self) -> u64;
}

pub trait SleepForMicroseconds {
    fn sleep_for_microseconds(&mut self, micros: i32);
}

pub trait Schedule {
    fn schedule(&mut self, function: fn(arg: *mut c_void) -> c_void, arg: *mut c_void);
}

pub trait StartThread {
    fn start_thread(&mut self, function: fn(arg: *mut c_void) -> c_void, arg: *mut c_void);
}
```

They decouple LevelDB from the host runtime's clock, sleep primitive, and background thread pool. Implementations can:

- Use the system monotonic clock for `now_micros`.
- Map `sleep_for_microseconds` to high‑resolution timers or cooperative schedulers.
- Implement `schedule` and `start_thread` on top of a custom executor or thread pool.

### Logging and test directory traits

```rust
pub trait NewLogger {
    fn new_logger(&mut self, fname: &String, result: *mut *mut Box<dyn Logger>) -> Status;
}

pub trait GetTestDirectory {
    fn get_test_directory(&mut self, path: *mut String) -> Status;
}
```

`NewLogger` creates a `Logger` bound to a specific file. `GetTestDirectory` returns a per‑process or per‑user directory where tests can create temporary files and subdirectories.

---

## `EnvWrapper`: delegated environments

`EnvWrapper` forwards all trait calls to an inner `Env` stored as `Rc<RefCell<dyn Env>>`:

```rust
pub struct EnvWrapper {
    target: Rc<RefCell<dyn Env>>, 
}

impl Env for EnvWrapper {}

impl EnvWrapper {
    /// Initialize an EnvWrapper that delegates all calls to `t`.
    pub fn new(t: Rc<RefCell<dyn Env>>) -> Self { /* ... */ }

    /// Return the target to which this Env forwards all calls.
    pub fn target(&self) -> Rc<RefCell<dyn Env>> { /* ... */ }
}
```

For nearly every environment trait, `EnvWrapper` implements a method that logs the call and then delegates:

```rust
impl DeleteFile for EnvWrapper {
    fn delete_file(&mut self, f: &String) -> Status {
        trace!(file = %f, "EnvWrapper::delete_file forwarding to target Env");
        let status = self.target.borrow_mut().delete_file(f);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::delete_file completed");
        status
    }
}
```

This pattern is repeated for:

- File operations: `CreateDir`, `DeleteDir`, `NewSequentialFile`, `NewRandomAccessFile`, `NewWritableFile`, `NewAppendableFile`, `FileExists`, `GetChildren`, `GetFileSize`, `RenameFile`, `LockFile`, `UnlockFile`.
- Time / threads: `Schedule`, `StartThread`, `NowMicros`, `SleepForMicroseconds`.
- Logging / tests: `GetTestDirectory`, `NewLogger`.

### Typical usage: instrumentation and partial overrides

Wrap an existing environment and override specific behaviors:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use bitcoinleveldb_env::EnvWrapper;

// Suppose `OsEnv` implements Env + the required traits.
let base: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(OsEnv::default()));
let mut env = EnvWrapper::new(base.clone());

// `env` can now be passed into higher‑level bitcoinleveldb components.
// You can later wrap `EnvWrapper` again or intercept calls via the logging macros.
```

To customize behavior, you can implement your own type that internally holds an `Rc<RefCell<dyn Env>>` and conditionally forwards (similar to `EnvWrapper`), or embed an `EnvWrapper` and hook the relevant traits.

---

## `RcWritableFileAdapter`

`RcWritableFileAdapter` adapts an `Rc<RefCell<dyn WritableFile>>` to the LevelDB file traits that expect &mut self receivers:

```rust
pub struct RcWritableFileAdapter {
    inner: Rc<RefCell<dyn WritableFile>>, 
}

impl WritableFile for RcWritableFileAdapter {}

impl Named for RcWritableFileAdapter {
    fn name(&self) -> Cow<'_, str> { /* ... */ }
}

impl WritableFileAppend for RcWritableFileAdapter {
    fn append(&mut self, data: &Slice) -> Status { /* forwards to inner */ }
}

impl WritableFileClose for RcWritableFileAdapter { /* ... */ }
impl WritableFileFlush for RcWritableFileAdapter { /* ... */ }
impl WritableFileSync for RcWritableFileAdapter { /* ... */ }
```

This is useful when a file object must be shared across multiple owners but the API assumes unique mutable access. The adapter centralizes the `Rc<RefCell<...>>` indirection.

---

## File utility routines

These helpers operate strictly in terms of the `Env` traits, not raw `std::fs` APIs. They mirror the LevelDB C++ helpers to preserve semantics.

### `read_file_to_string`

```rust
pub fn read_file_to_string(
    env: Rc<RefCell<dyn Env>>,
    fname: &String,
    data: *mut String,
) -> Status
```

- Clears the target `String`.
- Uses `env.new_sequential_file` to obtain a `SequentialFile`.
- Reads the file in fixed‑size chunks (8 KiB) via `file.read` into a `Slice` backed by a scratch buffer.
- Appends UTF‑8 decoded data into `*data`.
- Stops on EOF or error and returns a `Status`.

Ownership and safety mirror the C++ pattern: the `SequentialFile` is allocated indirectly via a `*mut Box<dyn SequentialFile>` out‑parameter, reboxed on Rust side (`Box::from_raw`), and then dropped when leaving scope.

### `write_string_to_file` / `write_string_to_file_sync`

```rust
pub fn write_string_to_file(
    env: Rc<RefCell<dyn Env>>,
    data: &Slice,
    fname: &String,
) -> Status

pub fn write_string_to_file_sync(
    env: Rc<RefCell<dyn Env>>,
    data: &Slice,
    fname: &String,
) -> Status
```

Both forward to `do_write_string_to_file`, which:

- Allocates a `WritableFile` via `env.new_writable_file`.
- Calls `file.append(data)`.
- Optionally calls `file.sync()` when `should_sync` is `true`.
- Calls `file.close()` on success.
- On failure, attempts to delete the file via `env.delete_file`.

`write_string_to_file_sync` is appropriate when durability constraints require that the file data reaches stable storage before the call returns.

### `set_current_file`

```rust
pub fn set_current_file(
    env: Rc<RefCell<dyn Env>>,
    dbname: &String,
    descriptor_number: u64,
) -> Status
```

`set_current_file` updates the `CURRENT` file in a LevelDB database directory to reference a new descriptor file (`MANIFEST-<descriptor_number>`), following the LevelDB protocol:

1. Construct the manifest file name via `descriptor_file_name(dbname, descriptor_number)`.
2. Convert it to a `Slice` and assert it starts with `"<dbname>/"`.
3. Remove the `dbname` prefix from the slice so that only `"MANIFEST-<n>"` remains.
4. Create a temporary file name `TempFileName(dbname, descriptor_number)`.
5. Write the manifest name + `"\n"` to this temporary file using `write_string_to_file_sync`.
6. Atomically rename the temporary file to the `CURRENT` file name via `env.rename_file`.
7. If anything fails, attempt to delete the temporary file.

This two‑phase update (write temp file, then rename) ensures that `CURRENT` is either valid before or valid after the operation, avoiding torn writes, which is critical for LevelDB's recovery invariants.

---

## Logging utility: `log`

```rust
pub fn log(
    info_log: Rc<RefCell<dyn Logger>>,
    format: *const u8,
    args: &[&str],
)
```

- Validates `format` as a non‑null C string and tries to interpret it as UTF‑8.
- Emits trace/debug information about the format string and argument count.
- Forwards to `Logger::logv(format, args)` on the underlying logger.

This is a direct analogue of the C++ `Log` helper, preserving the vararg format semantics through a `&[&str]` argument slice.

---

## Safety and invariants

This crate utilizes raw pointers and `unsafe` blocks in a controlled manner to match the C++ LevelDB ABI. When using or extending it, respect the following invariants:

- Out‑parameters like `*mut *mut Box<dyn SequentialFile>` or `*mut *mut Box<dyn Logger>` must:
  - Either be set to a valid heap‑allocated `Box<dyn Trait>` by the callee on success, and left unmodified or null on error.
  - Never alias memory that will be freed elsewhere.
- Ownership transfers implied by `Box::from_raw` must match allocations performed by the environment implementation.
- `Rc<RefCell<...>>` is **not** thread‑safe. If the environment is accessed across threads, it must itself provide internal synchronization, or you must use a thread‑safe strategy (e.g. `Arc<Mutex<...>>` at a higher layer) and carefully encapsulate it behind the required traits.
- Logging and tracing macros assume the global logger configuration is initialized before heavy usage.

Failure to maintain these invariants can cause memory unsafety. This crate is designed for expert use where ABI and behavioral fidelity outweigh total safety encapsulation.

---

## Integration with `bitcoin-rs`

This crate lives in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

It is typically consumed alongside other `bitcoinleveldb-*` crates (e.g. file, table, db, util) to provide a complete LevelDB port for Bitcoin node implementations. The environment abstraction here is the glue between that port and the host operating system.

---

## License

Licensed under the MIT license. See the repository for full license text.

---

## Status

- Crate version: `0.1.19`
- Rust edition: `2021`
- Author(s): `klebs <none>`

This documentation is intended to be accurate and practically useful, but you should always treat the source code as the final specification of behavior.
