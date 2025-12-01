# bitcoin-locked-page-allocator

OS-dependent allocation and deallocation of **locked/pinned memory pages** for security-sensitive applications (e.g., key material, wallet state).

This crate provides a thin, testable abstraction over platform primitives (`VirtualAlloc`/`VirtualLock` on Windows; `mmap`/`mlock` on POSIX) and integrates basic hardening such as page-aligned sizing, best-effort de-paging, and memory cleansing before release.

## Rationale

Secrets such as private keys, seeds, and authentication tokens should not:

1. Be transparently paged out to disk.
2. Linger in RAM after logical deallocation.
3. Depend on ad hoc, application-level discipline for correct handling.

Operating systems expose facilities to "pin" memory (mark it non-pageable) and to influence crash-dump behavior. This crate provides a focused implementation of these primitives in Rust, shaped by the security requirements of Bitcoin-style key management.

### Security Properties

Within the constraints of the underlying OS, allocations via this crate aim to:

- **Stay resident in RAM**: use `VirtualLock` (Windows) or `mlock` (POSIX) to request non-pageable pages.
- **Avoid crash dumps where possible**: use `madvise` with `MADV_DONTDUMP` (Linux) or `MADV_NOCORE` (FreeBSD) to reduce accidental leak in core dumps.
- **Zero memory on free**: call `memory_cleanse` before unlocking and deallocating pages.
- **Use page-aligned sizes**: allocations are rounded *up* to a power-of-two-compatible system page size to maintain alignment invariants (relevant to address- and side-channel hygiene).

No software abstraction can cryptographically guarantee non-paging; instead, this crate implements a **best-effort defense-in-depth** strategy consistent with contemporary secure memory handling.

## Platform Coverage

The crate exposes two concrete allocators behind a common trait surface:

- `Win32LockedPageAllocator` (Windows)
- `PosixLockedPageAllocator` (Unix-like OSes)

Both determine the system page size at runtime and enforce a power-of-two invariant to remain compatible with internal alignment logic.

### Windows

On Windows, `Win32LockedPageAllocator` uses:

- `VirtualAlloc` for page-aligned, committed allocations
- `VirtualLock` to request non-pageable pages
- `VirtualUnlock` and `VirtualFree` for cleanup

It treats the per-process lockable memory limit as effectively unbounded (`usize::MAX`), because Windows does not expose an mlock-style soft limit in the same manner as POSIX `RLIMIT_MEMLOCK`.

### POSIX

On POSIX systems, `PosixLockedPageAllocator` uses:

- `mmap` with `PROT_READ | PROT_WRITE` and `MAP_PRIVATE | MAP_ANONYMOUS`
- `mlock` to pin the pages
- `madvise` with `MADV_DONTDUMP` (Linux) or `MADV_NOCORE` (FreeBSD) when available
- `munlock` and `munmap` for cleanup

`get_limit` returns the process `RLIMIT_MEMLOCK` soft limit (via `getrlimit`), or `usize::MAX` on failure/`RLIM_INFINITY`.

The default constructor runs a **robust page size discovery** routine via `sysconf(_SC_PAGESIZE)`, falling back to 4 KiB on error and rounding up to the next power of two if the reported page size is not a power of two.

## Core Traits

The public interface is intentionally minimal and composable:

```rust
pub trait LockedPageAllocator {}

pub trait AllocateLocked {
    fn allocate_locked(&mut self, len: usize, locking_success: *mut bool) -> *mut c_void;
}

pub trait FreeLocked {
    fn free_locked(&mut self, addr: *mut c_void, len: usize);
}

pub trait GetLimit {
    fn get_limit(&mut self) -> usize;
}
``

- `LockedPageAllocator` is a marker trait to tag allocator implementations with the locked-memory semantics.
- `AllocateLocked` / `FreeLocked` express the two primary life-cycle operations.
- `GetLimit` exposes the OS-imposed limit on locked memory, when meaningful.

This separation of concerns allows you to:

- Write generic code over the traits (`dyn AllocateLocked + FreeLocked + GetLimit`).
- Wrap the allocator in higher-level pools/arenas without exposing platform details.
- Swap allocators in tests or in specialized builds.

## Concrete Types

### `Win32LockedPageAllocator`

```rust
#[cfg(windows)]
#[derive(Getters, Builder)]
#[getset(get = "pub")]
pub struct Win32LockedPageAllocator {
    page_size: usize,
}
```

Construction:

```rust
impl Win32LockedPageAllocator {
    /// Construct a new allocator, determining the system page size at runtime.
    pub fn new() -> Self { /* ... */ }
}

impl Default for Win32LockedPageAllocator {
    fn default() -> Self { Self::new() }
}
```

Implements:

- `LockedPageAllocator`
- `AllocateLocked`
- `FreeLocked`
- `GetLimit`

### `PosixLockedPageAllocator`

```rust
#[cfg(unix)]
#[derive(Getters, Builder)]
#[getset(get = "pub")]
pub struct PosixLockedPageAllocator {
    page_size: usize,
}
```

Construction via `Default` performs conservative page-size discovery and validation.

Also implements:

- `LockedPageAllocator`
- `AllocateLocked`
- `FreeLocked`
- `GetLimit`

## Usage

### Basic Allocation and Free

```rust
use core::ffi::c_void;
#[cfg(unix)]
use bitcoin_locked_page_allocator::{
    PosixLockedPageAllocator, AllocateLocked, FreeLocked, GetLimit, LockedPageAllocator,
};

fn main() {
    #[cfg(unix)]
    {
        let mut alloc = PosixLockedPageAllocator::default();

        // Determine available locked-memory budget
        let limit_bytes = alloc.get_limit();
        eprintln!("Process RLIMIT_MEMLOCK: {} bytes", limit_bytes);

        let mut locking_success = false;
        let len = 64; // bytes of secret material

        let ptr = unsafe {
            alloc.allocate_locked(len, &mut locking_success as *mut bool as *mut bool)
        };

        if ptr.is_null() {
            panic!("locked allocation failed");
        }

        if !locking_success {
            // Decide whether to treat this as fatal depending on your threat model.
            eprintln!("warning: mlock/VirtualLock failed; memory might be pageable");
        }

        // Use the memory via raw pointer or wrap into a custom type.

        unsafe {
            // Zero and free (the allocator also performs its own cleanse)
            alloc.free_locked(ptr, len);
        }
    }
}
```

> Note: `allocate_locked` and `free_locked` are `unsafe`-style operations in spirit, because they return and operate on raw pointers. The trait signatures reflect the expectation that higher-level wrappers will enforce type safety and lifetime discipline.

### Integrating Into a Secret Key Type

For ergonomic and safe usage, you typically wrap the allocator in a custom buffer type that:

- Encapsulates the raw pointer and length
- Implements `Drop` to call `free_locked`
- Exposes safe methods for reading/writing the underlying bytes

Sketch (simplified):

```rust
use core::ffi::c_void;
use core::ptr;

#[cfg(unix)]
use bitcoin_locked_page_allocator::{PosixLockedPageAllocator, AllocateLocked, FreeLocked};

pub struct LockedBuf<A> {
    alloc: A,
    ptr: *mut u8,
    len: usize,
}

impl<A> LockedBuf<A>
where
    A: AllocateLocked + FreeLocked,
{
    pub fn new(mut alloc: A, len: usize) -> Option<Self> {
        let mut locking_success = false;
        let ptr = unsafe {
            alloc.allocate_locked(len, &mut locking_success as *mut bool)
        } as *mut u8;

        if ptr.is_null() {
            return None;
        }

        // Application may choose to enforce `locking_success == true`.

        Some(Self { alloc, ptr, len })
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        assert!(!self.ptr.is_null());
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl<A> Drop for LockedBuf<A>
where
    A: AllocateLocked + FreeLocked,
{
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.len != 0 {
            unsafe {
                self.alloc.free_locked(self.ptr as *mut c_void, self.len);
            }
        }
    }
}
```

This pattern enables **RAII-style** secure memory management: allocation in `new`, automatic cleansing and deallocation in `Drop`.

## Error Handling and Diagnostics

Logging is used to trace and diagnose behavior:

- `trace!` for allocation/free paths and resolved page sizes
- `warn!` when locking fails or page-size anomalies occur
- `error!` when OS-level allocation calls fail

In security-critical environments, you should configure your logging backend to collect these diagnostics and feed them into your observability stack. This allows you to detect conditions such as unexpectedly low `RLIMIT_MEMLOCK` or repeated `mlock` failures.

## Interaction with OS Limits

### Windows

- There is no direct per-process mlock analog; this crate assumes an effectively unbounded process-level capacity for pinned pages, but physical memory pressure still applies.

### POSIX

- `RLIMIT_MEMLOCK` bounds the total amount of memory that can be locked.
- Exceeding it causes `mlock` to fail; the allocation will still succeed, but pages will not be pinned. Your application must decide whether to treat this as a hard error.

This crate intentionally separates *allocation success* from *locking success* using an out-parameter `*mut bool`. This enables calling code to implement nuanced policies, e.g.:

- Hard-fail if `locking_success == false` for private keys.
- Soft-warn in less critical contexts.

## Safety Considerations

1. The interface operates on `*mut c_void` and expects the caller to maintain memory safety.
2. The crate guarantees that, once `free_locked` returns, the memory has been cleansed via `memory_cleanse` and returned to the OS.
3. You must not use the pointer after `free_locked`.
4. Mixing allocators or freeing a pointer with the wrong allocator is undefined behavior.

## Mathematical / Systems Considerations

The allocator enforces a power-of-two page size invariant to maintain the correctness of `align_up`, which typically implements an operation such as:

\[
\operatorname{align\_up}(x, a) = \frac{(x + a - 1)}{a} \cdot a
\]

Correctness of this formula relies on properties of integer division and alignment; using a non-power-of-two `a` could violate implicit invariants assumed by code that operates on bit-level properties of addresses or lengths. By ensuring `page_size` is a power of two, it becomes straightforward to reason about cache line alignment, page indexing, and address masking.

## Versioning and Stability

- Crate: `bitcoin-locked-page-allocator`
- Example version: `0.1.19`

The crate is focused and low-level; API surface area is intentionally small to ease formal reasoning and external auditing. Expect conservative evolution: breaking changes should be rare and well-justified.

## License

Licensed under the MIT License.

## Authors

- klebs6 <tpk3.mx@gmail.com>
