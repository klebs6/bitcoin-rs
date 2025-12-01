# bitcoinleveldb-util

Utility components used by the `bitcoin-rs` / `bitcoinleveldb` codebase, focused on precise control over object lifetime and destructor behavior.

This crate is intentionally small and low-level. It exports primitives that help emulate C++-style *"never call the destructor"* patterns in Rust, primarily for function-local statics and other long-lived objects that must not run cleanup code on shutdown.

---

## Overview

The key types provided are:

- `DoNotDestruct` – A test/utility struct whose destructor **must never run**. If its `Drop` implementation is invoked, the process will immediately abort.
- `NoDestructor<T>` – A wrapper that stores a fully-initialized instance of `T` inside `MaybeUninit<T>` and **never drops it**. This is particularly useful for singletons, function-local statics, or global state that must remain valid for the lifetime of the process and does not require orderly teardown.
- `NoDestructorTest` – A trivial grouping struct, mainly used for tests.

The semantics are deliberately strict and closer to systems-programming idioms, trading graceful shutdown for predictable performance and the avoidance of complex destructor interactions at process exit.

---

## `DoNotDestruct`

```rust
use bitcoinleveldb_util::DoNotDestruct;

// Panics? No. If this is dropped, the process aborts.
let obj = DoNotDestruct::new(0xdead_beefu32, 0x0123_4567_89abu64);

// Read and write fields via generated getters/setters (from `getset`):
let a = *obj.a();
let b = *obj.b();

obj.set_a(1);
obj.set_b(2);
```

### Behavior

```rust
impl Drop for DoNotDestruct {
    fn drop(&mut self) {
        error!("DoNotDestruct destructor called! Aborting...");
        std::process::abort();
    }
}
```

- Any attempt to drop a `DoNotDestruct` instance (including unwinding across stack frames) will unconditionally call `process::abort()`.
- The type is primarily intended as a *guardrail* in tests or as documentation of intent: this object is meant to be effectively immortal.

### Construction

```rust
impl DoNotDestruct {
    pub fn new(a: u32, b: u64) -> Self {
        info!("Constructing DoNotDestruct with a=0x{:x}, b=0x{:x}", a, b);
        Self { a, b }
    }
}
```

Logging macros (`info!`, `error!`) are used from the standard Rust logging ecosystem (e.g., `log` crate + chosen logger backend). To see log output, configure a logger in your binary (e.g. `env_logger`, `tracing_subscriber`, etc.).

---

## `NoDestructor<T>`

`NoDestructor<T>` stores an instance of `T` in `MaybeUninit<T>`, ensuring that Rust never runs its destructor. This is the Rust analogue of C++ code that uses aligned storage plus placement-new and deliberately omits destructor calls.

### Type definition

```rust
use core::mem::MaybeUninit;

#[derive(Debug)]
pub struct NoDestructor<InstanceType> {
    instance_storage: MaybeUninit<InstanceType>,
}
```

### Constructor

```rust
impl<InstanceType> NoDestructor<InstanceType> {
    /// Fully constructs the instance and stores it in `MaybeUninit`,
    /// then **never** runs the destructor.
    pub fn new(instance: InstanceType) -> Self {
        info!("NoDestructor::new invoked");
        let storage = MaybeUninit::new(instance);
        Self { instance_storage: storage }
    }
}
```

Usage:

```rust
use bitcoinleveldb_util::NoDestructor;

struct Cache { /* fields omitted */ }

fn global_cache() -> &'static mut Cache {
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicBool, Ordering};

    static mut STORAGE: MaybeUninit<NoDestructor<Cache>> = MaybeUninit::uninit();
    static INIT: AtomicBool = AtomicBool::new(false);

    // Very low-level: caller must ensure this is called in a single-threaded
    // initialization phase or otherwise synchronize access.
    if !INIT.load(Ordering::Acquire) {
        let cache = Cache { /* ... */ };
        unsafe {
            STORAGE.write(NoDestructor::new(cache));
        }
        INIT.store(true, Ordering::Release);
    }

    unsafe {
        // `get()` yields *mut Cache; we convert to &'static mut Cache.
        &mut *STORAGE.assume_init_ref().get()
    }
}
```

### Pointer access

```rust
impl<InstanceType> NoDestructor<InstanceType> {
    /// Returns a mutable raw pointer to the stored instance.
    /// Lifetime and aliasing are **not** tracked at the type level.
    pub fn get(&self) -> *mut InstanceType {
        trace!("NoDestructor::get returning pointer to the stored instance");
        self.instance_storage.as_ptr() as *mut InstanceType
    }
}
```

Important properties:

- The destructor of `InstanceType` is **never called**.
- `get()` returns a `*mut InstanceType` even though `&self` is shared; this mirrors some C++ patterns but deviates from idiomatic Rust. The burden of ensuring sound aliasing rules and exclusive mutation lies with the caller.
- The underlying `InstanceType` **must be fully initialized** by the time you call `get()` (which is guaranteed for instances constructed via `NoDestructor::new`).

### Safety considerations

While `NoDestructor<T>` itself is safe to construct and use at the type level, it allows patterns where you can trivially violate Rust's higher-level aliasing disciplines if misused. You must:

- Ensure that at most one mutable reference to the underlying `T` exists at any time.
- Ensure no mutable reference is used concurrently with any shared references where `T`'s invariants would not permit it.
- Accept that resources owned by `T` (file descriptors, sockets, memory buffers, lock guards, etc.) will never be released by RAII; they must either be leak-tolerant or managed manually.

This design is appropriate for process-long singletons where resource reclamation is unnecessary or undesirable.

---

## `NoDestructorTest`

```rust
#[derive(Debug, Getters, Setters, Builder)]
pub struct NoDestructorTest {}
```

A minimal struct serving as a convenient grouping for tests. Its presence indicates that the crate is used to validate and regression-test the above lifetime primitives.

---

## Design rationale

The patterns implemented in this crate arise from a specific class of problems in systems and database code:

- Some singletons or subsystems are more expensive, brittle, or risky to tear down than to leak.
- During shutdown, destructors may depend on global order-of-destruction guarantees that are fragile or impossible to enforce (especially across FFI boundaries or multiple static initializers).
- In such contexts, an explicit decision is made to construct once and **never destruct**, trading finite resource leaks at process exit for determinism during the lifetime of the program.

### Relation to C++ patterns

C++ often uses patterns like:

- `static SomeType instance;` with non-trivial destructors, whose order-of-destruction across translation units is notoriously complex.
- `std::aligned_storage` + placement-new, followed by intentionally omitting `~SomeType()` calls.

`NoDestructor<T>` mirrors the second approach using `MaybeUninit<T>` in Rust. `DoNotDestruct` serves as an explicit sentinel type that aborts if any destructor tries to execute, making tests detect any unintentional destruction.

---

## Logging

The snippets use logging macros such as `info!`, `error!`, and `trace!`. At integration time, you will typically:

```toml
[dependencies]
log = "0.4"
# and a backend of your choice, e.g.
env_logger = "0.11"
```

Initialize logging in your binary:

```rust
fn main() {
    env_logger::init();
    // use bitcoinleveldb-util types here
}
```

This allows you to observe constructor calls and any unexpected destructor activity during testing.

---

## When *not* to use this crate

Avoid these primitives if:

- You want idiomatic Rust RAII where destructors reliably run and free resources.
- You require structured teardown, e.g., for graceful shutdown of network servers.
- You can express your requirements using `lazy_static`, `once_cell`, or `std::sync::OnceLock` without destructors that cause problems.

Use this crate when you:

- Need full control over when (or whether) destructors execute.
- Are porting C++ code that already relies on immortal singletons or non-dropping patterns.
- Operate in a low-level environment (such as an embedded database, block storage engine, or consensus node) where teardown cost and complexity outweigh the benefits.

---

## Repository and license

- Repository: <https://github.com/klebs6/bitcoin-rs>
- Crate: `bitcoinleveldb-util`
- License: MIT

This crate is a component in the broader `bitcoin-rs` ecosystem. Consult that repository for examples of how these primitives are used in practice inside a leveldb-like storage engine and related Bitcoin infrastructure.
