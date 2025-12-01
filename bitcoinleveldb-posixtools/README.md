# bitcoinleveldb-posixtools

Low-level POSIX tooling extracted from a LevelDB port, focused on deterministic file locking, mmap / file‑descriptor limits, and a debuggable singleton environment abstraction. Intended for embedding in storage engines, databases, and performance‑sensitive systems that need explicit control over POSIX runtime behavior.

---

## Table of Contents

- [Overview](#overview)
- [Design Goals](#design-goals)
- [Features](#features)
- [Crate Layout](#crate-layout)
- [Usage](#usage)
  - [Configuring global POSIX limits](#configuring-global-posix-limits)
  - [Using `SingletonEnv` for global environment wiring](#using-singletonenv-for-global-environment-wiring)
  - [Tracking process‑local POSIX file locks](#tracking-process-local-posix-file-locks)
  - [Locking files with `lock_or_unlock`](#locking-files-with-lock_or_unlock)
  - [Mapping POSIX errors to `Status`](#mapping-posix-errors-to-status)
- [Concurrency and Memory Model](#concurrency-and-memory-model)
- [Caveats](#caveats)
- [License](#license)

---

## Overview

`bitcoinleveldb-posixtools` provides the POSIX‑oriented substrate for a LevelDB‑style storage engine:

- A **singleton environment** wrapper (`SingletonEnv`) which hides the lifetime of the underlying `Env` implementation while enforcing a single global instance in debug builds.
- Thread‑safe globals controlling:
  - maximum number of **read‑only memory mappings** (`max_mmaps`, `MMAP_LIMIT`), and
  - maximum number of **open read‑only file descriptors** (`max_open_files`, `OPEN_READ_ONLY_FILE_LIMIT`).
- A **POSIX file lock tracker** (`PosixLockTable` / `PosixFileLock`) that works around the fact that `fcntl(F_SETLK)` does not protect against re‑locking from the same process.
- A thin wrapper over **`fcntl` locking** (`lock_or_unlock`) for write locks and unlocks.
- A convenience conversion from **POSIX errno values to LevelDB‑style `Status`** (`posix_error`).

Conceptually, this crate isolates the non‑portable POSIX behavior behind narrow, ergonomically wrapped functions and data structures. Higher‑level database code depends only on the `Env` trait, `Status`, and these helpers, leaving all detailed OS interaction here.


## Design Goals

- **Deterministic behavior under load.** Explicit control of mmap and open‑file limits avoids accidental resource exhaustion and gives predictable scaling characteristics.
- **Process‑local lock accounting.** While `fcntl` provides kernel‑level inter‑process locking, it does not prevent a single process from violating its own lock discipline. `PosixLockTable` maintains an internal `HashSet` to guard against such logic errors.
- **Debug‑enforced singleton environment.** Many LevelDB/engine configurations assume a single global environment instance. `SingletonEnv` defends that assumption in debug builds while remaining zero‑cost in release builds.
- **Minimal unsafe surface area.** All unsafe interaction is localized around POSIX FFI calls (`libc::fcntl`, `libc::getrlimit`, `libc::strerror`), wrapped in safe and well‑documented functions.


## Features

At a high level, the crate exposes:

- `SingletonEnv<EnvType>`
  - Wraps a concrete `Env` implementation and exposes an `Rc<RefCell<dyn Env>>` handle.
  - Asserts in debug builds that the singleton has not been previously initialized.

- Global POSIX resource controls
  - `MMAP_LIMIT: AtomicI32` and `max_mmaps() -> i32`
  - `OPEN_READ_ONLY_FILE_LIMIT: AtomicI32` and `max_open_files() -> i32`

- POSIX lock coordination
  - `PosixLockTable` / `PosixLockTableInner` for process‑local tracking.
  - `PosixFileLock` implementing `FileLock` and `Named`.
  - `lock_or_unlock(fd: i32, lock: bool) -> i32` wrapping `fcntl(F_SETLK, ...)`.

- Error translation
  - `posix_error(context: &String, error_number: i32) -> Status`

- Stubbed environment factory
  - `posix_default_env() -> Rc<RefCell<dyn Env>>` (currently `todo!()` and intended to be wired to OS‑specific code).


## Crate Layout

Relevant public items (simplified):

```rust
lazy_static! {
    pub static ref OPEN_READ_ONLY_FILE_LIMIT: std::sync::atomic::AtomicI32;
    pub static ref MMAP_LIMIT: std::sync::atomic::AtomicI32;
}

pub struct SingletonEnv<EnvType> {
    env_rc:  Rc<RefCell<dyn Env>>,
    _marker: std::marker::PhantomData<EnvType>,
}

pub struct PosixLockTable {
    mu: Mutex<PosixLockTableInner>,
}

pub struct PosixLockTableInner {
    locked_files: HashSet<String>,
}

pub struct PosixFileLock {
    fd:       i32,
    filename: String,
}

pub fn posix_default_env() -> Rc<RefCell<dyn Env>>;
pub fn max_mmaps() -> i32;
pub fn max_open_files() -> i32;
pub fn lock_or_unlock(fd: i32, lock: bool) -> i32;
pub fn posix_error(context: &String, error_number: i32) -> Status;
```

The actual traits/types `Env`, `Status`, `Slice`, `FileLock`, `Named`, and the `Mutex` implementation are expected to live in the surrounding LevelDB/Bitcoin codebase and are not redefined here.


## Usage

### Configuring global POSIX limits

Two global atomics regulate resource usage:

- `MMAP_LIMIT` affects `max_mmaps()`.
- `OPEN_READ_ONLY_FILE_LIMIT` affects `max_open_files()`.

By default, `MMAP_LIMIT` is initialized to `DEFAULT_MMAP_LIMIT`, and `OPEN_READ_ONLY_FILE_LIMIT` is `-1`, meaning **"not yet computed."**

`max_open_files()` performs the following algorithm:

1. If `OPEN_READ_ONLY_FILE_LIMIT >= 0`, return the cached value.
2. Otherwise, call `getrlimit(RLIMIT_NOFILE, ...)`.
   - On error, fall back to `50` descriptors.
   - On `RLIM_INFINITY`, use `i32::MAX`.
   - Otherwise, compute `cur / 5` (20% of currently allowed descriptors), with lower bound `50` and upper bound `i32::MAX`.
3. Store the result in `OPEN_READ_ONLY_FILE_LIMIT` and return it.

Example: overriding the defaults for tests or specialized deployments:

```rust
use std::sync::atomic::Ordering;
use bitcoinleveldb_posixtools::{MMAP_LIMIT, OPEN_READ_ONLY_FILE_LIMIT, max_mmaps, max_open_files};

fn configure_limits_for_benchmarking() {
    // Hard cap mmaps (e.g., to avoid page table blowup under synthetic load).
    MMAP_LIMIT.store(1024, Ordering::SeqCst);

    // Hard cap read‑only file descriptors.
    OPEN_READ_ONLY_FILE_LIMIT.store(2048, Ordering::SeqCst);

    assert_eq!(max_mmaps(), 1024);
    assert_eq!(max_open_files(), 2048);
}
```


### Using `SingletonEnv` for global environment wiring

`SingletonEnv` provides a thin, well‑typed wrapper around an `Env` implementation that is intended to be globally shared:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use bitcoinleveldb_posixtools::SingletonEnv;

// Suppose you have a concrete Env implementation:
#[derive(Default)]
struct PlatformEnv;
impl Env for PlatformEnv { /* ... */ }

// Type alias matching the C++ pattern `using PlatformSingletonEnv = SingletonEnv<PlatformEnv>`.
type PlatformSingletonEnv = SingletonEnv<PlatformEnv>;

fn configure_env() {
    // In debug builds, this will assert if a singleton has already been constructed.
    PlatformSingletonEnv::assert_env_not_initialized();

    // Here you might set global flags that influence Env construction
    // (e.g., custom path prefixes, I/O throttling, etc.).
}

fn global_env() -> Rc<RefCell<dyn Env>> {
    static PLATFORM_ENV: PlatformSingletonEnv = PlatformSingletonEnv::default();
    PLATFORM_ENV.env()
}
```

Semantic behavior:

- `SingletonEnv::default()` constructs an `EnvType::default()`, wraps it in `Rc<RefCell<dyn Env>>`, and records that the singleton is now initialized (debug‑only `AtomicBool`).
- `SingletonEnv::env()` clones the `Rc`, returning a shared handle without exposing the concrete type.
- `SingletonEnv::assert_env_not_initialized()` fails fast in debug builds if a singleton has already been created, catching mis‑ordered configuration code.


### Tracking process‑local POSIX file locks

`PosixLockTable` is a mutex‑protected table of file names currently considered locked by the process. It is **not** a replacement for kernel locks, but a guardrail around them.

Typical pattern:

```rust
use bitcoinleveldb_posixtools::PosixLockTable;

fn acquire_process_local_lock(table: &mut PosixLockTable, path: &str) -> bool {
    // Returns true if "path" was not previously recorded as locked.
    table.insert(path)
}

fn release_process_local_lock(table: &mut PosixLockTable, path: &str) {
    table.remove(path);
}
```

Properties:

- Internally, `PosixLockTable` holds a `Mutex<PosixLockTableInner>`, where `PosixLockTableInner` owns a `HashSet<String>`.
- `insert()` acquires the mutex and performs `HashSet::insert`, returning whether the file was newly inserted.
- `remove()` acquires the mutex and removes the file, logging whether it was present.

This structure enables higher‑level code to:

- Assert that no two logical `FileLock` objects in the same process reference the same path concurrently.
- Mirror C++ LevelDB behavior when validating locking semantics.


### Locking files with `lock_or_unlock`

`lock_or_unlock` is a direct wrapper around `fcntl(F_SETLK, ...)` with a fully initialized `libc::flock` structure:

```rust
use bitcoinleveldb_posixtools::lock_or_unlock;

fn try_lock(fd: i32) -> std::io::Result<()> {
    let rc = lock_or_unlock(fd, true);
    if rc == 0 { Ok(()) } else { Err(std::io::Error::last_os_error()) }
}

fn try_unlock(fd: i32) -> std::io::Result<()> {
    let rc = lock_or_unlock(fd, false);
    if rc == 0 { Ok(()) } else { Err(std::io::Error::last_os_error()) }
}
```

Details:

- For `lock = true`, `l_type` is set to `F_WRLCK`.
- For `lock = false`, `l_type` is set to `F_UNLCK`.
- `l_whence = SEEK_SET`, `l_start = 0`, `l_len = 0` → lock/unlock the entire file.
- Return value is exactly the `fcntl` return value (0 on success, −1 on error, with `errno` set).

This function is usually combined with `PosixFileLock` instances and `PosixLockTable` to coordinate both kernel and process‑local state.


### Mapping POSIX errors to `Status`

`posix_error` bridges from a POSIX errno (`i32`) and a context string into a LevelDB‑style `Status` value:

```rust
use bitcoinleveldb_posixtools::posix_error;

fn open_maybe_missing(path: &str) -> Status {
    // Hypothetical error path where open() fails with errno.
    let errno = unsafe { *libc::__errno_location() }; // example only, not recommended API
    let ctx = format!("open {}", path);
    posix_error(&ctx, errno)
}
```

Behavior:

- Obtains a human‑readable error message via `libc::strerror`.
- Handles the rare case where `strerror` returns null with a fallback string `"<unknown-posix-error>"`.
- If `error_number == ENOENT`, returns `Status::not_found(ctx, msg)`; otherwise, returns `Status::io_error(ctx, msg)`.

This classification mirrors the LevelDB convention of distinguishing *not found* from generic I/O errors.


## Concurrency and Memory Model

The crate uses atomics and mutexes to expose deterministic, thread‑safe behavior:

- `MMAP_LIMIT` and `OPEN_READ_ONLY_FILE_LIMIT` are `AtomicI32` values:
  - They are read and written with `Ordering::SeqCst` in the limit functions, providing a straightforward and conservative memory model.
- `SINGLETON_ENV_INITIALIZED` (debug‑only) is an `AtomicBool` set and read with `Ordering::Relaxed` because it is used only as a *program‑order* debugging check, not for data synchronization.
- `PosixLockTable` uses a `Mutex` around the internal `HashSet<String>`; all reads/writes to the table go through this mutex.
- `SingletonEnv` exposes an `Rc<RefCell<dyn Env>>`:
  - `Rc` and `RefCell` are not thread‑safe; this matches the underlying LevelDB semantics where the environment is typically used from a thread‑safe wrapper or from a single logical owner.
  - If you need cross‑thread sharing, you should wrap the `Env` in `Arc<Mutex<...>>` or equivalent in your own code, or ensure the surrounding code enforces single‑threaded access.


## Caveats

- **`posix_default_env()` is currently a stub** and calls `todo!()`. It is intended to be wired to a platform‑specific `Env` implementation (e.g., using `PlatformEnv` for POSIX, a Windows env, or a test/mock env). Do not call it in production code until the wiring is implemented.
- The crate assumes the existence of external traits/types (`Env`, `Status`, `Slice`, `Named`, `FileLock`, `Mutex`) from the broader Bitcoin/LevelDB integration. This crate is not currently intended as a stand‑alone, general‑purpose library.
- File locking semantics are those of `fcntl(F_SETLK)`:
  - Advisory locks only.
  - No protection against a single process opening the same file multiple times and failing to coordinate; `PosixLockTable` exists to address the latter at the logical level.


## License

This crate is distributed under the MIT license.

See the `LICENSE` file for full text.
