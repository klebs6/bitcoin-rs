# bitcoin-sync

High‑fidelity Rust reimplementation of Bitcoin Core’s low‑level synchronization and threading primitives.

This crate mirrors the semantics of Bitcoin Core’s C++ locking, semaphore, and thread‑interrupt machinery while exposing an idiomatic Rust API built on top of `parking_lot` and `tracing`. It is intended for building Bitcoin‑style concurrent runtimes, particularly where lock‑order correctness and deterministic behaviour matter more than maximal abstraction.

---

## Design goals

- **Semantic compatibility with Bitcoin Core**
  - Similar behaviour to `CCriticalSection`, `CThreadInterrupt`, `TraceThread`, and related helpers.
  - No poisoning semantics on mutex failure, matching Core’s model (panic ≠ permanent lock poisoning).
- **Strong debugging & lock‑order diagnostics**
  - Optional early deadlock detection via a global lock‑graph (when compiled with `DEBUG_LOCKORDER`).
  - Assertions that verify which locks are held/not‑held at specific call sites.
- **Minimal, explicit primitives**
  - Thin wrappers around `parking_lot` raw mutexes and condition variables.
  - RAII helpers (`UniqueLock`, `ReverseLock`, `SemaphoreGrant`, etc.) instead of opaque higher‑level frameworks.
- **Predictable threading behaviour**
  - Structured thread launch wrapper that logs lifecycle and propagates panics.
  - Interruptible sleeper abstraction (`ThreadInterrupt`) with well‑defined timing semantics.

This crate is deliberately low level. It is intended for experts who need precise control over lock ordering and cross‑thread coordination rather than generic application‑level concurrency.

---

## Features overview

### 1. Lock abstraction: `LockApi` and `AnnotatedMixin`

```rust
pub trait LockApi {
    fn lock(&self);
    fn unlock(&self);
    fn try_lock(&self) -> bool;
}
```

`LockApi` represents the minimal interface expected from a raw, **non‑poisoning** mutex. Any type implementing this trait can plug into the higher‑level RAII guards provided by this crate.

`AnnotatedMixin<Parent>` wraps a `Parent: LockApi` and:

- Forwards all lock operations.
- Provides a stable address and hook point for lock‑order tracking and future instrumentation.

```rust
pub struct AnnotatedMixin<Parent: LockApi> {
    parent: Parent,
}

impl<Parent: LockApi> LockApi for AnnotatedMixin<Parent> { /* forwards */ }
```

The primary concrete aliases are:

```rust
/// Recursive lock (reentrant), no waiting (raw mutex semantics).
pub type RecursiveMutex<T> = AnnotatedMixin<parking_lot::ReentrantMutex<T>>;

/// Non‑recursive raw mutex supporting waiting.
pub type Mutex = AnnotatedMixin<parking_lot::RawMutex>;
```

`AnnotatedMixin<parking_lot::RawMutex>` implements `Default` with `RawMutex::INIT` so it can be used as a field with default construction.

### 2. RAII locking: `UniqueLock` and `ReverseLock`

#### `UniqueLock`

`UniqueLock<'a, M>` is a RAII guard analogous to C++ `std::unique_lock` specialized for `M: LockApi + ?Sized`:

```rust
pub struct UniqueLock<'a, M: LockApi + ?Sized> {
    mutex: &'a M,
    owns:  bool,
    name:  &'static str,
    file:  &'static str,
    line:  u32,
}
```

Key operations:

- `new(mutex, name, file, line, try_: Option<bool>)` – constructs and either blocks (`lock`) or attempts `try_lock` depending on `try_`.
- `enter()` – acquire the lock if not already owned.
- `try_enter() -> bool` – try to acquire; returns whether ownership was gained.
- `unlock()` – explicitly release if currently owned.
- `owns_lock() -> bool` – mirror of C++’s `owns_lock`.
- `Drop` – automatically unlocks if still owned.

It also implements `From<&UniqueLock<…>> for bool`, so you can write:

```rust
let locked: bool = (&guard).into();
```

The struct records `name`, `file`, and `line`, allowing detailed logging of lock acquisition and release events via `tracing` macros inside the implementation.

#### `ReverseLock`

`ReverseLock` is a scoped inversion helper:

```rust
pub struct ReverseLock<'guard, 'lock, M: LockApi + ?Sized> {
    guard:    &'guard mut UniqueLock<'lock, M>,
    relocked: bool,
}
```

- `ReverseLock::new(&mut guard)` unlocks the underlying mutex immediately.
- On `Drop`, it re‑enters the lock (if not already re‑locked).

This pattern is essential in Core’s codebase for temporarily releasing a lock around a potentially blocking or re‑entrant operation without losing the association with the guard variable.

### 3. Macros for critical sections and lock assertions

The crate exposes several macros to express critical sections, multi‑lock scopes, and assertions that a lock is (or is not) held.

#### Scope guards

```rust
lock!(cs);      // creates an unnamed UniqueLock bound to `cs`
lock2!(cs1, cs2); // lock two mutexes in sequence

try_lock!(cs, guard);  // guard: UniqueLock with try=Some(true)
wait_lock!(cs, guard); // guard: UniqueLock with blocking lock

with_lock!(cs, {
    // critical region
    do_something();
});
```

The `lock!` and `lock2!` macros use the `paste` crate to synthesize a unique guard name linked to the source line, ensuring RAII destruction at scope exit.

`with_lock!` is syntactic sugar which:

1. Acquires a `UniqueLock` on `$cs` via `lock!`.
2. Executes `$code` while the lock is held.

#### Critical section tracking

Two macros integrate with the lock‑order tracking internals (see the `debug_lockorder` module):

```rust
enter_critical_section!(cs);
leave_critical_section!(cs);
```

- `enter_critical_section!(cs)` records metadata about the acquisition (name, file, line, address) and then calls `cs.lock()`.
- `leave_critical_section!(cs)` verifies that `cs` is the most recent critical section lock before unlocking and notifying the lock‑order system.

These macros allow the `DEBUG_LOCKORDER` logic to build a global **partial order** on lock acquisition, enabling early detection of potential deadlocks by identifying cycles in the lock graph.

#### Lock assertions

```rust
assert_lock_held!(cs);
assert_lock_not_held!(cs);
```

These macros call into `assert_lock_held_internal` / `assert_lock_not_held_internal`, which, under `DEBUG_LOCKORDER`, verify that `cs` is (or is not) present in the thread‑local lock stack. On failure they emit detailed diagnostics and either abort or panic depending on configuration.

### 4. Semaphore & permits: `Semaphore` and `SemaphoreGrant`

`Semaphore` is a simple counting semaphore built on top of a `Condvar` and a raw `Mutex<i32>`:

```rust
#[derive(Default)]
pub struct Semaphore {
    cv:    Condvar,
    count: Mutex<i32>,
}
``

Operations:

- `Semaphore::new(init: i32)` – create with an initial permit count (must be ≥ 0).
- `wait(&self)` – block until `count > 0`, then decrement.
- `try_wait(&self) -> bool` – non‑blocking attempt; `true` if a permit was consumed.
- `post(&self)` – increment `count` and wake one waiter.

`SemaphoreGrant` is a RAII wrapper around a single permit:

```rust
#[derive(Clone)]
pub struct SemaphoreGrant {
    sem:        Arc<Semaphore>,
    have_grant: bool,
}
```

Semantics:

- `SemaphoreGrant::new(sema: Arc<Semaphore>, try_: Option<bool>)` – constructs a grant and either blocks or tries to acquire immediately.
- `acquire(&mut self)` – blocking acquisition if not already holding a permit.
- `try_acquire(&mut self) -> bool` – attempt acquisition without blocking.
- `release(&mut self)` – return the permit if currently held.
- `move_to(&mut self, target: &mut SemaphoreGrant)` – transfer ownership of a permit to `target`.
- `Drop` – automatically releases any held permit.

Like `UniqueLock`, `SemaphoreGrant` implements `From<&SemaphoreGrant> for bool` so it can be treated as a truth value indicating permit ownership.

`WaitTimedOut(bool)` is a thin wrapper with `.timed_out() -> bool` to express timeout results in a self‑documenting fashion.

### 5. Thread management: `launch_traced_thread!` and `trace_thread`

#### `launch_traced_thread!`

```rust
launch_traced_thread!("indexer", || {
    // your thread body
});
```

Expands to `std::thread::Builder::new().name(...).spawn(...)` and delegates the body to `trace_thread`, panicking immediately if thread creation fails.

#### `trace_thread`

```rust
pub fn trace_thread<F>(thread_name: &str, thread_func: F)
where
    F: FnOnce() + Send + 'static,
{ /* ... */ }
```

- Creates a `tracing` span named `"thread"` with field `name = thread_name`.
- Logs `"<name> thread start"` and `"<name> thread exit"` at `INFO` level.
- Wraps the user closure in `std::panic::catch_unwind`. If it panics, it logs the panic and re‑raises, preserving default Rust semantics while still emitting structured telemetry.

This is a direct analogue of Bitcoin Core’s `TraceThread` helper.

### 6. Interruptible sleeping: `ThreadInterrupt`

`ThreadInterrupt` models `CThreadInterrupt` from Bitcoin Core:

```rust
#[derive(Default)]
pub struct ThreadInterrupt {
    cond: Condvar,
    gate: Mutex<()>,
    flag: AtomicBool,
}
```

Core semantics:

- `new()` – create with `flag = false`.
- `as_bool() -> bool` – observe the interrupt flag with `Acquire` ordering.
- `reset()` – clear any pending interrupt (`Release` ordering).
- `invoke()` – set the flag true and `notify_all()` on the condition variable.
- `sleep_for(rel_time: StdDuration) -> bool`:
  - Returns `false` if an interrupt happens **before** the full timeout elapses.
  - Returns `true` if the full `rel_time` passes without interruption.

Implementation detail: `sleep_for` loops, computing the remaining time until a pre‑computed `deadline` and calling `Condvar::wait_for`. After each wakeup it re‑checks the interrupt flag and the clock.

This primitive is useful for threads that must be **cooperatively** cancellable while still respecting a bounded maximum sleep.

### 7. Low‑level waiting: `wait_until`

```rust
pub fn wait_until<T: ?Sized, P>(
    cv:       &parking_lot::Condvar,
    guard:    &mut parking_lot::MutexGuard<'_, T>,
    deadline: std::time::Instant,
    mut predicate: P,
) -> bool
where
    P: FnMut() -> bool,
{ /* ... */ }
```

This helper repeatedly waits on `cv` until either:

- `predicate()` becomes `true`, in which case it returns `true`, or
- the `deadline` is reached and a final predicate check still fails, in which case it returns that final predicate result.

The pattern corresponds to the standard condition‑variable idiom:

\[
  \text{while } \neg P \text{ and } t < T_{\text{deadline}}: \text{ wait}.
\]

Mathematically, this implements a partial function `f: (P, T_deadline) -> bool` where the result is the final valuation of `P` at or after the deadline, ensuring **spurious wakeups** do not violate the semantics.

### 8. Scoped raw mutex: `ScopedRawMutex` and `ScopedRawMutexGuard`

For scenarios requiring direct access to a raw `parking_lot::RawMutex` while still benefiting from RAII unlocking:

```rust
pub struct ScopedRawMutex(RawMutex);

pub struct ScopedRawMutexGuard<'a> {
    lock: &'a ScopedRawMutex,
}
```

- `ScopedRawMutex::default()` – constructs with `RawMutex::INIT`.
- `ScopedRawMutex::lock(&self) -> ScopedRawMutexGuard<'_>` – locks the underlying raw mutex, returning a guard.
- On `Drop` of `ScopedRawMutexGuard`, `RawMutexTrait::unlock` is invoked.

This is intentionally minimal; it exists primarily as a bridge towards the C++‑style code generation and binding layers.

### 9. Debug lock‑order analysis (`DEBUG_LOCKORDER`)

When built with `DEBUG_LOCKORDER`, the crate enables an internal module that maintains a global lock‑graph to detect:

- Double‑locking of non‑recursive mutexes on the same thread.
- Lock order inversions across threads (potential deadlocks).

Key concepts:

- **LockLocation** – describes where a lock was acquired: mutex name, source file, line, thread name, and whether the acquisition was a try‑lock.
- **LockStack** – per‑thread stack of `(mutex_ptr, LockLocation)` pairs.
- **LockOrders** – a map from ordered pairs `(A, B)` to the stack snapshot when `B` was acquired after `A`.
- **InvLockOrders** – set of the reversed pairs `(B, A)` for fast detection of inversions.

Algorithmically, each time a lock is pushed, the module:

1. Checks for **double lock**: if the same pointer already appears in the stack, it reports a double lock.
2. For each previously held lock `L` in the current stack, considers pair `(L, current)`:
   - If `(current, L)` is already in `lockorders`, a cycle exists → **potential deadlock**.
   - Otherwise inserts `(L, current)` into `lockorders` and `(current, L)` into `invlockorders`.

In both double‑lock and potential‑deadlock cases, a detailed textual dump of the involved lock stacks is produced, and `abort_or_panic` is called. A global `AtomicBool` `G_DEBUG_LOCKORDER_ABORT` determines whether to abort the process or panic.

In non‑debug builds (`cfg(not(DEBUG_LOCKORDER))`), the functions become no‑ops, so there is no runtime overhead beyond the macro calls themselves.

---

## Example usage

### Basic mutex usage with RAII

```rust
use bitcoin_sync::{Mutex, UniqueLock};

static GLOBAL: Mutex = Mutex::default();

fn critical_section() {
    // Create a guard that locks immediately.
    let mut guard = UniqueLock::new(&GLOBAL, "GLOBAL", file!(), line!(), None);

    // do work while lock is held

    // Optionally release early
    guard.unlock();
}
``

### Using the helper macros

```rust
use bitcoin_sync::{Mutex};
use bitcoin_sync::{lock, with_lock};

static COUNTER_LOCK: Mutex = Mutex::default();
static mut COUNTER: i32 = 0;

fn increment() {
    with_lock!(COUNTER_LOCK, unsafe {
        COUNTER += 1;
    });
}

fn guarded_section() {
    lock!(COUNTER_LOCK);
    // RAII guard lives until end of scope
}
```

### Semaphore and grants

```rust
use std::sync::Arc;
use bitcoin_sync::{Semaphore, SemaphoreGrant};

fn bounded_concurrency() {
    let sem = Arc::new(Semaphore::new(3));

    let mut grant = SemaphoreGrant::new(sem.clone(), None);
    // we now hold one permit; scope exit will release automatically

    // do limited‑concurrency work here

    // explicit release is also possible
    grant.release();
}
```

### Interruptible worker thread

```rust
use std::sync::Arc;
use std::time::Duration;
use bitcoin_sync::{ThreadInterrupt, launch_traced_thread};

fn main() {
    let ti = Arc::new(ThreadInterrupt::new());
    let ti_worker = ti.clone();

    let handle = launch_traced_thread!("worker", move || {
        while ti_worker.sleep_for(Duration::from_secs(5)) {
            // periodic task; returns false if interrupted
        }
        // cleanup
    });

    // later
    ti.invoke();    // request shutdown
    handle.join().unwrap();
}
```

---

## Crate metadata

- **Crate**: `bitcoin-sync`
- **Version**: `0.1.19`
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **License**: MIT
- **Edition**: Rust 2021

This crate is designed as a building block within a larger Bitcoin reimplementation effort; its public API focuses on concurrency primitives, not high‑level Bitcoin logic.

---

## When to use this crate

Use `bitcoin-sync` if you:

- Are porting or closely emulating Bitcoin Core behaviour in Rust and need compatible synchronization primitives.
- Need early deadlock detection and explicit lock‑order tracking similar to Core’s `DEBUG_LOCKORDER` tooling.
- Prefer explicit, RAII‑based concurrency constructs over more abstract frameworks.

If you only need generic high‑level Rust concurrency, standard library types (`Mutex`, `RwLock`, `Condvar`) or `tokio`/`async` primitives may be more appropriate. `bitcoin-sync` is primarily for **systems programmers** who require tight control over lock semantics and diagnostics.
