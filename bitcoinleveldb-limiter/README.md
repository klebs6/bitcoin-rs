# bitcoinleveldb-limiter

A small, focused concurrency primitive for bounding resource consumption in the Bitcoin LevelDB stack. It provides a lock-free, atomic counter–based limiter intended for constraining scarce process resources such as file descriptors and mmapped regions.

---

## Overview

`bitcoinleveldb-limiter` exposes a single type, `Limiter`, which enforces a hard upper bound on the number of concurrently held resources. It is designed for scenarios where allocation is cheap but oversubscription has catastrophic or non-linear system cost (e.g., exhausting file descriptors, virtual address space, or triggering kernel pathologies under massive FD counts).

The limiter is:

- **Lock-free**: uses an `Atomic<i32>` with relaxed ordering for minimal contention.
- **Deterministic**: enforces a fixed maximum concurrent acquisition count.
- **Non-blocking**: acquisition attempts are fail-fast (`bool`), leaving blocking/retry policy to the caller.
- **Symmetric**: each successful `acquire()` must be paired with exactly one `release()`.

Conceptually, `Limiter` is an integer-valued resource semaphore with non-blocking acquisition and a compile-time known capacity.

---

## Core API

```rust
use bitcoinleveldb_limiter::Limiter;

/// Construct a limiter with a maximum of `max_acquires` concurrent resources.
/// Negative values are clamped to 0.
pub fn new(max_acquires: i32) -> Limiter;

/// Attempt to acquire one unit of capacity.
///
/// Returns `true` if a resource was successfully acquired, `false` if the
/// limiter is at capacity and no additional resource can be granted.
pub fn acquire(&self) -> bool;

/// Release one unit of capacity previously acquired by `acquire()`.
///
/// Must only be called after a successful `acquire()`. Over-release is
/// detected in debug builds via `debug_assert!`.
pub fn release(&self);
```

### Semantics

- `new(max_acquires)`
  - `max_acquires >= 0`: the limiter starts with exactly `max_acquires` available slots.
  - `max_acquires < 0`: a warning is logged and capacity is clamped to `0`.
  - The maximum capacity is immutable for the lifetime of the limiter.

- `acquire()`
  - Performs an atomic `fetch_sub(1, Relaxed)`.
  - If the *previous* value was strictly greater than 0, the call succeeds and returns `true`.
  - If the *previous* value was `<= 0`, capacity is immediately restored via a compensating `fetch_add(1, Relaxed)` and the call returns `false`.
  - In debug builds, exceeding the configured maximum (indicating mismatched `release()` calls) triggers a `debug_assert!`.

- `release()`
  - Performs an atomic `fetch_add(1, Relaxed)`.
  - In debug builds, if the pre-increment value is already at or above `max_acquires`, a `debug_assert!` fires to signal over-release.

This yields a simple invariant in debug mode:

\[
0 \leq \text{acquires\_allowed} \leq \text{max\_acquires}
\]

when all clients pair acquisitions and releases correctly.

---

## Usage Patterns

### Limiting Open File Descriptors

```rust
use bitcoinleveldb_limiter::Limiter;
use std::fs::File;
use std::io;
use std::path::Path;

fn open_bounded<P: AsRef<Path>>(limiter: &Limiter, path: P) -> io::Result<Option<File>> {
    if !limiter.acquire() {
        // At capacity: caller decides whether to block, backoff, or degrade.
        return Ok(None);
    }

    let file = File::open(path);

    // Ensure capacity is returned even on error paths.
    if file.is_err() {
        limiter.release();
    }

    Ok(file.ok())
}
```

### Integrating Into a Pool or Handle Type

Wrap the limiter into a higher-level RAII guard so you cannot forget to release:

```rust
use bitcoinleveldb_limiter::Limiter;
use std::sync::Arc;

pub struct Permit {
    limiter: Arc<Limiter>,
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.limiter.release();
    }
}

impl Permit {
    pub fn try_acquire(limiter: Arc<Limiter>) -> Option<Self> {
        if limiter.acquire() {
            Some(Permit { limiter })
        } else {
            None
        }
    }
}
```

This pattern is particularly valuable under complex control flow, where manual bookkeeping is error-prone.

### Backpressure and Retry Strategies

Because `acquire()` is non-blocking, the caller can implement any policy for handling saturation:

- Immediate failure with a typed error.
- Bounded retry with exponential backoff.
- Parking the current thread or task until capacity becomes available.

This design cleanly separates *resource accounting* (the concern of `Limiter`) from *scheduling and latency* (the concern of the caller or surrounding framework).

---

## Concurrency & Memory Ordering

`Limiter` employs `Atomic<i32>` with `Ordering::Relaxed`. This is adequate because:

- The only shared state is the integer count of permits.
- Callers do not rely on `Limiter` to publish or synchronize additional data beyond the availability of capacity.
- Correctness depends solely on arithmetic invariants of the counter, not on happens-before relations for external memory.

If you use the limiter as part of a synchronization protocol that also coordinates additional shared state, you must introduce the appropriate memory ordering or separate synchronization primitives outside of `Limiter`.

As with any non-blocking primitive, `Limiter` does **not** provide fairness guarantees: under heavy contention, some threads may experience repeated acquisition failures even while others succeed.

---

## Error Detection & Debugging

`Limiter` uses `debug_assert!` for internal consistency checks:

- Over-release (`release()` called more times than successful `acquire()` calls) is detected.
- Misbehavior in `acquire()` / `release()` symmetry manifests early in debug builds.

These checks are removed in `--release` builds, so you should validate your usage under debug configuration during development.

Logging (`trace!`, `debug!`, `warn!`) is employed for observability and can be integrated with a structured logging backend. In production, you typically reduce the log level to keep overhead minimal.

---

## Integration in the `bitcoin-rs` Repository

This crate originates from the `bitcoin-rs` project and mirrors the resource limiting semantics used in Bitcoin Core's LevelDB integration. Its primary application is preventing pathological resource usage in large on-disk databases while keeping the limiting primitive self-contained and composable.

Repository: <https://github.com/klebs6/bitcoin-rs>

---

## When to Use `Limiter`

`Limiter` is appropriate when:

- You are protecting a small, fixed, global resource pool (FDs, mmaps, cache entries).
- You want **non-blocking**, low-overhead checks on resource availability.
- Debug-time detection of misuse is desirable.

You might prefer a different abstraction when:

- You require blocking behavior or async integration out of the box (consider semaphores in async runtimes).
- You need fairness, prioritization, or complex admission control (consider queues or more sophisticated schedulers).

`bitcoinleveldb-limiter` deliberately focuses on a single, precise responsibility: safe, low-latency enforcement of a global concurrency bound.
