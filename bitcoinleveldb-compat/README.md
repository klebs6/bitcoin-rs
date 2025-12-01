# bitcoinleveldb-compat

Low-level Rust bindings that emulate the original C++ LevelDB `port` and Snappy interfaces used by Bitcoin Core, with tracing, safety checks, and modern Rust infrastructure under the hood.

This crate is intentionally narrow in scope: it exists to make it straightforward to plug Bitcoin's LevelDB-dependent code into Rust with minimal semantic drift. It provides

- C/FFI-friendly Snappy compression helpers with LevelDB-style contracts
- CRC32C acceleration in the same shape as LevelDB's `crc32c` helpers
- A `port` module implementing LevelDB-compatible mutexes and condition variables on top of Rust primitives
- Stubs for heap profiling to satisfy link-time expectations without adding heavy dependencies

All interfaces are designed to be close to the C++ originals (pointer-based, explicit contracts, and boolean success flags) while being implemented in Rust with logging and modern concurrency primitives.

---

## Features at a Glance

- **Snappy compatibility**
  - `snappy_compress(input, length, output) -> bool`
  - `snappy_uncompress(input, length, output) -> bool`
  - `snappy_get_uncompressed_length(input, length, result) -> bool`
  - Behavior gated behind the `leveldb_snappy` Cargo feature; without it, the functions are present but always return `false`, matching LevelDB's conditional Snappy support.

- **CRC32C acceleration**
  - `acceleratedcrc32c(crc, buf, size) -> u32` wraps a performant CRC32C implementation and mirrors LevelDB's accelerated CRC API.

- **Synchronization primitives (`port` module)**
  - `port::Mutex`: thin wrapper over `parking_lot::RawMutex`, with explicit `lock()`, `unlock()`, and `assert_held()` semantics aligned with LevelDB's `port::Mutex`.
  - `port::CondVar`: condition variable that works with `port::Mutex`, mirroring LevelDB's `CondVar` semantics (`wait`, `signal`, `signal_all`).

- **Heap profiling stub**
  - `get_heap_profile(func, arg) -> bool` satisfies LevelDB's expected symbol but always returns `false`, indicating that heap profiling is not supported in this build.

- **Observability**
  - All public operations are instrumented using [`tracing`](https://docs.rs/tracing), producing detailed spans and events at `trace`, `debug`, and `error` levels for diagnosis of concurrency and compression behavior.

---

## Installation

```toml
[dependencies]
bitcoinleveldb-compat = "0.1.19"

# Optional: to enable Snappy support
[features]
# In your own crate, just enable this feature for bitcoinleveldb-compat
# bitcoinleveldb-compat = { version = "0.1.19", features = ["leveldb_snappy"] }
```

This crate targets Rust **edition 2021** and is licensed under **MIT**.

The code lives in the `bitcoin-rs` monorepo:

- Repository: <https://github.com/klebs6/bitcoin-rs>

---

## Snappy API

The Snappy interface is deliberately low-level and pointer-based to approximate the C++ implementation.

### Feature gating

All Snappy functions are compiled in both with and without Snappy support:

- With `leveldb_snappy` feature enabled:
  - Functions perform real Snappy compression/decompression using the `snap` crate.
- Without `leveldb_snappy` feature:
  - Functions are no-ops that return `false` and log that Snappy is disabled, mirroring LevelDB's optional Snappy integration.

### `snappy_compress`

```rust
#[cfg(feature = "leveldb_snappy")]
pub fn snappy_compress(input: *const u8, length: usize, output: *mut String) -> bool
```

- **Input**: `input[0..length)` is treated as a raw byte slice.
- **Output**: On success, `*output` is overwritten with a `String` whose underlying bytes are the raw compressed Snappy buffer.
- **Return**: `true` on success, `false` on failure or invalid pointers.

Important properties:

- The function treats `String` purely as an owned byte container, using `String::from_utf8_unchecked`. The compressed Snappy stream is *not* guaranteed to be UTF-8. This is intentional to mimic the original `std::string` usage in LevelDB, which is a generic byte container.
- Callers must never rely on UTF-8 validity of the resulting `String`; treat it as opaque binary data.

**Safety**: This function is `unsafe` to call from a strict Rust perspective because it takes and dereferences raw pointers. In idiomatic Rust code, you should wrap it with a safe abstraction that ensures:

- `input` is non-null and references at least `length` bytes
- `output` is non-null and points to a valid `String` instance

Example usage inside `unsafe` context:

```rust
use bitcoinleveldb_compat::snappy_compress;

unsafe {
    let data = b"hello leveldb";
    let mut out = String::new();
    let ok = snappy_compress(data.as_ptr(), data.len(), &mut out as *mut String);
    assert!(ok);
    // `out` now holds the compressed Snappy payload as bytes in a String
}
```

### `snappy_uncompress`

```rust
#[cfg(feature = "leveldb_snappy")]
pub fn snappy_uncompress(input: *const u8, length: usize, output: *mut u8) -> bool
```

- **Input**: `input[0..length)` is a Snappy-compressed buffer.
- **Output**: Raw bytes are written starting at `output`.
- **Return**: `true` on successful decompression, `false` on invalid or corrupt input.

Constraints:

- The caller must ensure that `output` points to a writable buffer of at least `n` bytes, where `n` is the value returned by a successful `snappy_get_uncompressed_length` call on the same `input`.
- The function uses `ptr::copy_nonoverlapping` to write the decompressed data.

Example pattern:

```rust
use bitcoinleveldb_compat::{
    snappy_compress,
    snappy_uncompress,
    snappy_get_uncompressed_length,
};

unsafe {
    let data = b"block index payload";

    // 1. Compress
    let mut compressed = String::new();
    assert!(snappy_compress(data.as_ptr(), data.len(), &mut compressed as *mut String));

    // 2. Query uncompressed length
    let mut out_len: usize = 0;
    let ok = snappy_get_uncompressed_length(
        compressed.as_ptr(),
        compressed.len(),
        &mut out_len as *mut usize,
    );
    assert!(ok);

    // 3. Allocate output buffer
    let mut out = vec![0u8; out_len];

    // 4. Decompress
    let ok = snappy_uncompress(
        compressed.as_ptr(),
        compressed.len(),
        out.as_mut_ptr(),
    );
    assert!(ok);
    assert_eq!(&out[..], data);
}
```

### `snappy_get_uncompressed_length`

```rust
#[cfg(feature = "leveldb_snappy")]
pub fn snappy_get_uncompressed_length(
    input: *const u8,
    length: usize,
    result: *mut usize,
) -> bool
```

This function attempts to uncompress the buffer internally to determine the output length and writes it into `*result`.

- **Success path**: returns `true` and sets `*result` to the decompressed length.
- **Failure**: returns `false` and leaves `*result` unspecified.

Note that the implementation currently does *full* decompression to compute the length instead of using Snappy's cheap-length-query primitive. This is acceptable for compatibility but may be less efficient; consult the repository if efficiency is critical and consider contributing an optimized implementation.

---

## CRC32C Acceleration

```rust
pub fn acceleratedcrc32c(crc: u32, buf: *const u8, size: usize) -> u32
```

- If `buf` is null or `size == 0`, the function returns the original `crc` unchanged.
- Otherwise, it computes an extended CRC32C over `buf[0..size)` via `crc32c_extend(crc, data)` and returns the new CRC.

This corresponds to the mathematical operation:

\[
\text{new_crc} = \operatorname{CRC32C\_extend}(\text{crc}, \text{buf})
\]

where CRC32C is the Castagnoli polynomial-based CRC with well-defined compositional properties (you can extend a prefix CRC with suffix bytes without recomputing over the entire concatenated buffer).

Example:

```rust
use bitcoinleveldb_compat::acceleratedcrc32c;

unsafe {
    let part1 = b"header";
    let part2 = b"body";

    let mut crc = 0u32;
    crc = acceleratedcrc32c(crc, part1.as_ptr(), part1.len());
    crc = acceleratedcrc32c(crc, part2.as_ptr(), part2.len());

    // `crc` is now the CRC32C of b"headerbody".
}
```

---

## `port` Module: Mutex and Condition Variable

The `port` module re-implements the LevelDB `port::Mutex` and `port::CondVar` APIs, but on top of Rust's synchronization primitives (`parking_lot::RawMutex` and `std::sync::Condvar`), preserving the original semantics.

### `port::Mutex`

```rust
#[LOCKABLE]
pub struct Mutex {
    mu: parking_lot::RawMutex,
    is_locked: AtomicBool,
}

impl Mutex {
    pub fn new() -> Self;
    pub fn lock(&mut self);
    pub fn unlock(&mut self);
    pub fn assert_held(&mut self);
}
```

Key behaviors:

- `lock()` blocks until the mutex is obtained, then flips `is_locked` to `true`.
- `unlock()` sets `is_locked` to `false` and releases the underlying `RawMutex`.
- `assert_held()` issues a `debug_assert!` that `is_locked` is `true`, used for internal invariants consistent with LevelDB's expectations.

This mutex is marked `Send` and `Sync`, so it can be shared across threads. It is explicitly *not* a RAII-guarded mutex; you must call `lock()`/`unlock()` manually, which is closer to how the C++ code interacts with `port::Mutex`.

Example:

```rust
use bitcoinleveldb_compat::port::Mutex;

let mut mu = Mutex::new();

mu.lock();
// critical section
mu.assert_held();
mu.unlock();
```

### `port::CondVar`

```rust
pub struct CondVar {
    cv:      std::sync::Condvar,
    mu:      *const Mutex,
    waiters: std::sync::Mutex<usize>,
}

impl CondVar {
    pub fn new(mu: *mut Mutex) -> Self;
    pub fn wait(&mut self);
    pub fn signal(&mut self);
    pub fn signal_all(&mut self);
}
```

Semantics mirror LevelDB:

- `CondVar::new(mu)` associates the condition variable with a specific `port::Mutex` pointer. The pointer is assumed to outlive the `CondVar`.
- `wait()` must be called while holding `mu`:
  - Registers the caller as a waiter (increments waiter count).
  - Releases `mu`.
  - Blocks on the internal `Condvar`.
  - On wakeup, decrements waiter count and re-acquires `mu` before returning.
- `signal()` wakes a single waiting thread (if any).
- `signal_all()` wakes all waiters.

There is a small internal `StdMutex<usize>` tracking waiters; poisoning is detected and logged but recovered from by taking the inner state.

Example usage:

```rust
use bitcoinleveldb_compat::port::{Mutex, CondVar};
use std::sync::Arc;
use std::thread;

let mu = Arc::new(Mutex::new());
let cond = Arc::new(std::sync::Mutex::new(CondVar::new(Arc::as_ptr(&mu) as *mut _)));
let ready = Arc::new(std::sync::atomic::AtomicBool::new(false));

// Producer
{
    let mu = mu.clone();
    let cond = cond.clone();
    let ready = ready.clone();
    thread::spawn(move || {
        let mut mu_ref = mu; // explicit binding
        mu_ref.lock();
        ready.store(true, std::sync::atomic::Ordering::SeqCst);
        cond.lock().unwrap().signal_all();
        mu_ref.unlock();
    });
}

// Consumer
mu.lock();
while !ready.load(std::sync::atomic::Ordering::SeqCst) {
    cond.lock().unwrap().wait();
}
mu.unlock();
```

This pattern closely tracks the LevelDB idiom: check predicate under lock, call `wait()` to sleep if not satisfied, and re-evaluate after wake-up.

---

## Heap Profiling Stub

```rust
pub fn get_heap_profile(
    func: fn(_0: *mut c_void, _1: *const u8, _2: i32) -> c_void,
    arg: *mut c_void,
) -> bool
```

This function always logs that heap profiling is not supported and returns `false`. It exists solely to satisfy link-time expectations from LevelDB/Bitcoin code that may conditionally call into `get_heap_profile`.

If you need real heap profiling, you should layer a dedicated solution over or in place of this stub and adjust your integration accordingly.

---

## Tracing and Diagnostics

All public functions are annotated with `#[instrument]` from the `tracing` ecosystem. This means:

- Every call generates a span with structured fields (e.g., `length`, `size`, pointer values).
- Errors and exceptional conditions are logged with `debug!`, `warn!`, or `error!` levels.

In production systems, this makes it significantly easier to diagnose concurrency issues, mis-sized buffers, or misconfigured features (e.g., Snappy disabled when expected).

To consume these logs, wire up a `tracing` subscriber in your binary:

```rust
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // your application using bitcoinleveldb-compat
}
```

---

## Safety and FFI Considerations

This crate presents a C-like API surface for the sake of Bitcoin/LevelDB compatibility:

- Raw pointers (`*const u8`, `*mut u8`, `*mut String`, `*mut usize`)
- Manual memory management responsibilities for the caller
- No lifetime tracking in the type system

When used from Rust, prefer to encapsulate these calls behind safe wrappers that:

- Own and manage the input/output buffers
- Avoid null pointers
- Guarantee length invariants

When used from C/C++ via FFI, ensure that the ABI and calling conventions are matched, and that the Rust side is compiled with a stable layout and visibility for the symbols in question.

---

## License

This crate is distributed under the terms of the **MIT license**.

