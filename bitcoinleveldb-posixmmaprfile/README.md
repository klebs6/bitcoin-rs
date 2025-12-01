# bitcoinleveldb-posixmmaprfile

High‑performance, POSIX `mmap(2)`‑backed implementation of LevelDB's `RandomAccessFile` interface for read‑only files, with deterministic resource management and integrated mmap limiting.

---

## Overview

`bitcoinleveldb-posixmmaprfile` provides a single core type:

```rust
pub struct PosixMmapReadableFile { /* fields elided */ }
```

It implements:

- `RandomAccessFileRead` – a low‑level random read API accepting raw pointers.
- `RandomAccessFile` – marker trait indicating conformance to the LevelDB random access contract.
- `Named` – exposing a cheap, borrowed `Cow<'_, str>` filename.

The implementation uses POSIX `mmap()` to map an entire file into memory, then serves random reads by bounding‑checked pointer arithmetic and `Slice` construction. It is designed to be:

- **Thread‑safe for concurrent reads** – state is immutable after construction; reads are race‑free.
- **Zero‑copy for reads** – no per‑read allocation or copying, only slice creation over an existing mapping.
- **Robust against overflow** – uses `u128` arithmetic to validate `(offset + n) <= length` before casting.
- **Resource‑disciplined** – integrates with a `Limiter` object that caps the number of simultaneous mmaps.

This crate is primarily a building block within the Bitcoin‑adapted LevelDB stack but can be used in any context where you already have an mmap, a limiter, and want a LevelDB‑style `RandomAccessFile` implementation.

---

## Core Type: `PosixMmapReadableFile`

```rust
#[derive(Getters, Builder, Debug)]
#[getset(get = "pub")]
pub struct PosixMmapReadableFile  {
    mmap_base:    *const u8,
    length:       usize,
    mmap_limiter: *const Limiter,
    filename:     String,
}
```

### Construction

```rust
impl PosixMmapReadableFile {
    /// `mmap_base` must come from a successful `mmap()` and this instance
    /// takes ownership of the region.
    ///
    /// `mmap_limiter` must outlive this instance; one mmap token is released
    /// when the instance is dropped.
    pub fn new(
        filename:     String,
        mmap_base:    *mut u8,
        length:       usize,
        mmap_limiter: *mut Limiter,
    ) -> Self { /* ... */ }
}
```

Invariants and expectations:

- `mmap_base` / `length` describe the full mapped region `[0, length)`.
- The region **must** be read‑only or at least safe for concurrent reads; the type does not perform synchronization.
- `mmap_limiter` encapsulates the global policy for how many mmaps are allowed; the caller is responsible for acquiring a slot before constructing the object.
- A `(null, 0)` mapping is allowed for degenerate/test cases; production code should only allow `mmap_base == null` if `length == 0`.

The `new` function logs with `tracing` (`trace!`/`warn!`) and uses the auto‑derived builder under the hood for field initialization.

### Reading: `RandomAccessFileRead`

```rust
impl RandomAccessFileRead for PosixMmapReadableFile {
    fn read(
        &self,
        offset:   u64,
        n:        usize,
        result:   *mut Slice,
        _scratch: *mut u8,
    ) -> Status { /* ... */ }
}
```

Semantics:

- Computes `offset + n` in `u128` and requires `offset + n <= length`.
- On **out‑of‑range**:
  - Writes `Slice::default()` into `*result`.
  - Returns an IO error `Status` (akin to `PosixError(fname, EINVAL)` in LevelDB's C++ implementation).
- On **success**:
  - Computes `let ptr = self.mmap_base().add(offset_usize);`.
  - Sets `*result = Slice::from_ptr_len(ptr, n);`.
  - Returns `Status::ok()`.

This interface is intentionally low‑level for compatibility with the surrounding LevelDB FFI/domain model:

- `Slice` is a lightweight view into `&[u8]`‑like data without transferring ownership.
- `Status` represents a result status (OK / IO error / etc.) without using Rust's `Result` in the external interface.

Because `PosixMmapReadableFile` does not mutate internal state during `read`, multiple threads can call `read` concurrently on a shared instance without coordination.

### Resource Management & Drop

```rust
impl Drop for PosixMmapReadableFile {
    fn drop(&mut self) {
        use libc::{munmap, c_void};
        // 1. munmap the region if base != null and length > 0
        // 2. release one token back into the Limiter
    }
}
```

Drop behavior:

1. If `mmap_base != null` and `length > 0`, calls
   `munmap(mmap_base as *mut c_void, length)`.
   - On failure, logs a warning with the OS error.
2. If `mmap_limiter != null`, it unsafely dereferences it as `&Limiter` and calls `release()`.

This ensures that:

- The OS mapping is torn down exactly once.
- The global `Limiter` is updated even in the presence of panics or early returns elsewhere.

### Naming

```rust
impl Named for PosixMmapReadableFile {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.filename)
    }
}
```

`Named` enables introspection and logging; downstream components can treat any `Named` implementor uniformly for diagnostics and metrics.

---

## Safety Considerations

This crate uses `unsafe` for:

- Holding raw pointers to the mmap region (`*const u8`).
- Calling `libc::munmap`.
- Constructing `Slice` from raw pointers.
- Interacting with `Limiter` via raw pointer.

Key safety assumptions:

1. **Lifetime of mapping**: The memory pointed to by `mmap_base` remains valid and mapped for the entire lifetime of `PosixMmapReadableFile`.
2. **Lifetime of limiter**: `mmap_limiter` outlives the `PosixMmapReadableFile` instance, and exactly one acquire/release pair is associated with it.
3. **Immutability for concurrent reads**: The mapped file is not mutated in a way that would violate Rust's aliasing rules as observed through `Slice` views.

Downstream code should ensure these invariants hold. The crate is designed to encapsulate the unsafe operations while exposing a predictable, LevelDB‑compatible random access abstraction.

---

## Example Usage

Below is a sketch of how you might integrate `PosixMmapReadableFile` into a higher‑level system. Types like `Limiter`, `Slice`, `Status`, `RandomAccessFileRead`, and `RandomAccessFile` are presumed to come from the broader bitcoinleveldb ecosystem.

```rust
use std::{fs::File, os::fd::AsRawFd, ptr};
use libc::{mmap, munmap, PROT_READ, MAP_SHARED, MAP_FAILED};

use bitcoinleveldb_posixmmaprfile::PosixMmapReadableFile;
use bitcoinleveldb_core::{Limiter, Slice, Status, RandomAccessFileRead};

fn open_mmaped_random_access(
    path: &str,
    limiter: &Limiter,
) -> Result<PosixMmapReadableFile, std::io::Error> {
    // Acquire a token in the limiter (API depends on your Limiter type).
    limiter.acquire()?;

    let file = File::open(path)?;
    let len = file.metadata()?.len() as usize;

    let fd = file.as_raw_fd();
    let base = unsafe {
        mmap(
            ptr::null_mut(),
            len,
            PROT_READ,
            MAP_SHARED,
            fd,
            0,
        )
    };

    if base == MAP_FAILED {
        // Release token if mmap failed
        limiter.release();
        return Err(std::io::Error::last_os_error());
    }

    // Safety: `base` is from a successful mmap, `limiter` will outlive
    // the PosixMmapReadableFile, and we have acquired exactly one token.
    let file_ra = PosixMmapReadableFile::new(
        path.to_owned(),
        base as *mut u8,
        len,
        limiter as *const Limiter as *mut Limiter,
    );

    Ok(file_ra)
}

fn read_slice_example(f: &PosixMmapReadableFile, offset: u64, n: usize) -> Result<&[u8], String> {
    let mut slice = Slice::default();
    let status: Status = f.read(offset, n, &mut slice as *mut Slice, std::ptr::null_mut());

    if !status.is_ok() {
        return Err(status.to_string());
    }

    Ok(slice.as_bytes()) // assuming Slice::as_bytes() -> &[u8]
}
```

In realistic usage, you would not usually call `mmap` or `munmap` yourself; other components establish the mappings and pass them to `PosixMmapReadableFile`.

---

## Logging & Observability

The implementation uses the [`tracing`](https://crates.io/crates/tracing) ecosystem macros:

- `trace!` on construction, entry into reads, and drop.
- `debug!` on successful reads and error cases with full context (filename, offsets, lengths, status string).
- `warn!` when inconsistencies or OS errors are encountered (e.g., non‑zero length with null base, `munmap` failure).

This yields high‑fidelity observability for:

- Diagnosing unexpected IO patterns.
- Verifying limiter behavior at runtime.
- Correlating OS‑level failures with higher‑level operations.

---

## Performance Characteristics

Given a correctly mapped file and a well‑dimensioned `Limiter`:

- Per‑read overhead is essentially:
  - Bounds checking via `u128` arithmetic.
  - Pointer addition and `Slice` construction.
  - Logging at the configured level.
- There are **no heap allocations** in the steady‑state `read` path.
- The operating system's page cache and mmap machinery provide the effective IO bandwidth.

This makes the type suitable for read‑intensive workloads such as LevelDB table lookups, block index scans, or any workload dominated by many small random reads.

---

## When Not to Use This Crate

You may prefer a different strategy when:

- The platform does not provide POSIX `mmap` semantics.
- You are constrained by address space (e.g., 32‑bit environments with very large files).
- You require write access with transactional guarantees instead of read‑only access.

In such settings, a pread‑based implementation or explicit buffered IO layer might be more appropriate.

---

## License

This crate is distributed under the MIT license.

---

## Status

This README is auto‑generated and may diverge slightly from the actual API surface or dependencies as the crate evolves. Consult the source for the authoritative definition of `Limiter`, `Slice`, `Status`, and the traits implemented here.
