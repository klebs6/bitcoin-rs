# bitcoin-locked-pool

High‑assurance, page‑locked memory pool for secret material, extracted from Bitcoin Core's secure allocator design and implemented in Rust.

---

## Overview

`bitcoin-locked-pool` provides an arena‑based allocator for **locked (non‑swappable) memory**, intended for cryptographic secrets such as private keys, seeds, and authentication credentials.

It builds on a pluggable `LockedPageAllocator` implementation (e.g. `mlock`/`VirtualLock` via the companion `bitcoin_locked_page_allocator` crate) to:

- Reserve contiguous regions of page‑locked memory (`LockedPageArena`).
- Allocate and free variable‑sized chunks from these arenas (`LockedPool`).
- Expose a process‑wide singleton manager (`LockedPoolManager`) suitable for use in custom allocators and higher‑level abstractions.
- Provide rigorous destruction ordering so that locked pages are always freed correctly and without use‑after‑free.

The design mirrors the original C++ Bitcoin Core secure memory allocator: metadata is kept **out of** locked memory so that the limited locked quota is reserved for actual secrets.

This crate does **not** itself implement a `GlobalAlloc` or STL‑compatible allocator, but is intended as the building block for such facilities.

---

## Core Concepts

### Locked vs. pageable memory

On most operating systems, virtual memory pages can be swapped to disk under memory pressure. This is unacceptable for long‑lived secrets, because it leaves residues on persistent storage.

A locked page allocator requests that specific ranges of memory be **pinned** in RAM (e.g. via `mlock(2)` on POSIX or `VirtualLock` on Windows). Systems usually impose a strict limit on the total lockable memory per process or per user.

`bitcoin-locked-pool` is built around this constraint:

- The first arena is capped by the process limit (if non‑zero) to ensure it can be fully locked.
- Subsequent arenas are allocated as needed, but still go through the underlying `LockedPageAllocator`, which may lock all or none of the pages.
- A callback lets the application **fail hard** or **continue with a warning** when locking fails.

### Arena‑based design

`LockedPool` manages a `Vec<LockedPageArena>`. Each `LockedPageArena` owns:

- A raw base pointer (`*mut c_void`).
- A byte size (`usize`).
- A pointer back to the underlying `LockedPageAllocator` used to free the memory.
- An internal `Arena` that does the sub‑allocation bookkeeping.

Allocation strategy:

1. Short‑circuit if the requested `size == 0` or exceeds a maximum arena size constant.
2. Try to allocate from existing arenas.
3. If all are full, ask the allocator for a **new locked arena**, then retry allocation from it.

Freeing locates the owning arena by address range and delegates to its `Arena` instance.

---

## Crate Structure

### Types

#### `type LockingFailed_Callback = fn() -> bool;`

A callback invoked when an arena allocation succeeds but **locking that memory fails**.

- Return `true` to proceed with the allocation (non‑locked, but at least resident for now).
- Return `false` to abort: the pool will immediately free the just‑allocated pages and propagate failure to the caller.

This allows you to decide at runtime whether to:

- Strictly require page‑locking for all secret allocations; or
- Degrade gracefully in constrained environments, while emitting logs/metrics.

---

#### `struct LockedPool`

A pool for locked memory chunks. Fields (readable via the generated getters):

- `allocator: Box<dyn LockedPageAllocator + Send + Sync>` – Strategy for allocating and freeing locked pages.
- `arenas: Vec<LockedPageArena>` – Current set of arenas.
- `lf_cb: Option<LockingFailed_Callback>` – Optional callback for locking failures.
- `cumulative_bytes_locked: usize` – Aggregate bytes successfully locked across all arenas.
- `mutex: Mutex<()>` – Internal synchronization for statistics and other multi‑arena operations.

##### Construction

```rust
use bitcoin_locked_page_allocator::{
    PosixLockedPageAllocator,
    // or Win32LockedPageAllocator on Windows
};
use bitcoin_locked_pool::LockedPool;

fn locking_failed() -> bool {
    // Log, increment a metric, etc. Return false to enforce hard failure.
    eprintln!("WARNING: unable to lock newly allocated arena; proceeding unlocked");
    true
}

let allocator: Box<dyn bitcoin_locked_page_allocator::LockedPageAllocator + Send + Sync> =
    Box::new(PosixLockedPageAllocator::default());

let mut pool = LockedPool::new(allocator, Some(locking_failed));
```

##### Allocation & free

```rust
use core::ffi::c_void;

let p: *mut c_void = pool.alloc(64); // allocate 64 bytes
if p.is_null() {
    // Allocation failed (either full, oversize, or system error)
} else {
    unsafe {
        // Use `p` as raw memory. Wrap it in your own abstraction that zeroizes on drop.
    }

    // Free when done; `null` is a no‑op.
    pool.free(p);
}
```

Guarantees and behavior:

- `alloc(size == 0)` returns `null`.
- `alloc(size > LOCKED_POOL_ARENA_SIZE)` returns `null`.
- On failure to create or lock a new arena (with a `false` callback result), `alloc` returns `null`.
- `free(null)` is a no‑op.
- Freeing a pointer not belonging to any arena causes a `panic!`, surfacing allocator misuse early.

##### Statistics

```rust
use bitcoin_locked_pool::LockedPoolStats;

let stats: LockedPoolStats = pool.stats();
println!(
    "used={} free={} total={} locked={} chunks_used={} chunks_free={}",
    stats.used(),
    stats.free(),
    stats.total(),
    stats.locked(),
    stats.chunks_used(),
    stats.chunks_free(),
);
```

The `stats` call holds a mutex, aggregates per‑arena statistics, and returns an owned `LockedPoolStats` value.

---

#### `struct LockedPoolManager`

A lazily‑initialized, process‑wide singleton around a `LockedPool`, intended for use as the backing storage for global secure allocators.

Key properties:

- `Send + Sync` – safe to share and access from multiple threads.
- Uses `once_cell::sync::OnceCell` for one‑time initialization.
- On Unix, uses `PosixLockedPageAllocator::default()`.
- On Windows, uses `Win32LockedPageAllocator::default()`.

##### Accessing the global pool

```rust
use bitcoin_locked_pool::LockedPoolManager;
use core::ffi::c_void;

let manager: &LockedPoolManager = LockedPoolManager::instance();

// `LockedPoolManager` implements `Deref<Target = LockedPool>`
let p: *mut c_void = manager.alloc(32);
```

For mutating operations that need unique access to the manager itself, you can obtain `&'static mut LockedPoolManager` only via interior mutability or custom patterns; in most cases, you should treat `instance()` as giving you a shared reference and rely on any internal synchronization designed into higher‑level abstractions.

---

#### `struct LockedPageArena`

Represents a single contiguous region of memory obtained from `LockedPageAllocator`.

Construction is `unsafe` because the caller must guarantee that `base_in .. base_in + size_in` is valid for the arena's lifetime:

```rust
use core::ffi::c_void;
use bitcoin_locked_pool::LockedPageArena;

unsafe {
    let raw_ptr: *mut c_void = /* from some low-level allocator */;
    let allocator: *mut dyn bitcoin_locked_page_allocator::LockedPageAllocator = /* ... */;

    let arena = LockedPageArena::new(allocator, raw_ptr, 4096, 8);
    let p = arena.alloc(128);
    arena.free(p);
}
```

On `Drop`, the arena calls `free_locked(base, size)` on its stored allocator pointer, ensuring that pages are both unlocked and freed.

High‑level users typically do **not** construct arenas directly; `LockedPool::new_arena` encapsulates this logic alongside page locking and accounting.

---

#### `struct LockedPoolStats`

Immutable snapshot of pool‑wide memory accounting:

- `used: usize` – total bytes currently allocated to clients across all arenas.
- `free: usize` – total free bytes still available for allocation.
- `total: usize` – `used + free`, the overall capacity of all arenas combined.
- `locked: usize` – cumulative bytes successfully locked via the underlying allocator.
- `chunks_used: usize` – number of active allocation chunks.
- `chunks_free: usize` – number of free chunks in the arenas' internal data structures.

Generated via `LockedPool::stats()` using `LockedPoolStatsBuilder`.

---

## Drop Ordering & Safety Considerations

Two key invariants are enforced:

1. **Arenas must be destroyed before the allocator**.
   - `LockedPool` implements a custom `Drop` that first `take`s the `arenas` vec, drops it, and only then allows the `allocator` field to be dropped.
   - This prevents arenas from calling `free_locked` on a freed allocator instance, which would otherwise be undefined behavior.

2. **Arena deallocation must use the original allocator**.
   - Each `LockedPageArena` stores a raw `*mut dyn LockedPageAllocator` pointer.
   - This pointer is assumed to remain valid for the arena's entire lifetime; the `LockedPool` drop sequence guarantees this.

When building higher‑level abstractions (e.g., `ZeroizingBox`, `SecureVec<T>`), you should:

- Ensure that logical lifetimes of clients are bound by the lifetime of the underlying pool/manager.
- Aggressively zero memory before freeing, if you store secrets in these chunks.
- Treat `*mut c_void` pointers as unsafe; encapsulate them in strongly‑typed wrappers.

---

## Integration Patterns

### Custom secret container

A typical use‑case is to wrap the raw pool allocations in a safe secret type:

```rust
use bitcoin_locked_pool::LockedPoolManager;
use core::{ffi::c_void, ptr};

pub struct SecretBox {
    ptr: *mut u8,
    len: usize,
}

impl SecretBox {
    pub fn new(len: usize) -> Option<Self> {
        let pool = LockedPoolManager::instance();
        let raw = pool.alloc(len) as *mut u8;
        if raw.is_null() {
            return None;
        }
        unsafe { ptr::write_bytes(raw, 0, len); }
        Some(Self { ptr: raw, len })
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        assert!(!self.ptr.is_null());
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for SecretBox {
    fn drop(&mut self) {
        if self.ptr.is_null() { return; }
        unsafe {
            core::ptr::write_bytes(self.ptr, 0, self.len);
        }
        let pool = LockedPoolManager::instance();
        pool.free(self.ptr as *mut c_void);
    }
}
```

This pattern localizes all `unsafe` usage, ensures zeroization, and leverages the global locked pool.

---

## Platform Behavior

`LockedPoolManager::instance()` selects a platform‑appropriate allocator:

- **Unix**: `PosixLockedPageAllocator`
  - Typically uses `mlock`/`munlock` and may also apply `mlockall` or `MADV_DONTDUMP` semantics.
- **Windows**: `Win32LockedPageAllocator`
  - Typically uses `VirtualLock`/`VirtualUnlock`.

Observe OS‐specific limits (e.g., `RLIMIT_MEMLOCK` on Unix). If the first arena cannot be fully locked due to a strict limit, the `LockingFailed_Callback` is invoked; you can decide whether to:

- Refuse to operate without locked memory.
- Continue with degraded security while tracking the condition.

---

## Error Handling Philosophy

- **Hard logic errors** (e.g., freeing a pointer that does not belong to any arena) result in a `panic!`. This is a programming bug and should be detected during testing.
- **Resource exhaustion** (e.g., pool full, OS refusing a new locked region) is signaled via `null` from `alloc`.
- **Locking failure after successful allocation** is under your control via `LockingFailed_Callback`.

This approach keeps the core logic simple while allowing applications to layer richer error handling and telemetry as needed.

---

## Safety & `unsafe` Code

This crate uses low‑level, `unsafe` constructs:

- Raw pointers to arenas and allocators.
- Assumptions about allocator lifetime and memory validity.

The public API attempts to:

- Confine `unsafe` to boundary construction (e.g., `LockedPageArena::new`).
- Provide safe high‑level operations (`LockedPool::alloc/free`, `LockedPool::stats`) once the invariants are established.

When extending the crate or integrating deeply:

- Treat all raw pointers as **unsafe contracts**.
- Ensure that any types built on top of this pool handle zeroization and destruction in a principled manner.

---

## License

This crate is licensed under the **MIT** license.

See the `LICENSE` file for details.
