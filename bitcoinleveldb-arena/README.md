# bitcoinleveldb-arena

A low-level memory arena allocator and test harness used in the Bitcoin-inspired LevelDB port (`bitcoinleveldb`) for Rust. This crate provides a small, allocation-centric building block that closely mirrors the original C++ `Arena` used in LevelDB, while embracing Rust's safety guarantees where feasible and preserving raw-pointer semantics where required for performance parity.

---

## Overview

`bitcoinleveldb-arena` implements a monotonic bump allocator backed by a set of heap-allocated blocks. It is designed for workloads where many small allocations are performed and freed en masse by dropping the arena, rather than via fine-grained deallocation of individual objects.

The crate intentionally exposes a low-level interface using raw pointers, matching the behavior and layout of the original C++ LevelDB `Arena` to ease validation, benchmarking, and cross-language comparison.

Core components:

- **`Arena`** – a bump-allocating arena:
  - Carves fixed-size backing blocks into small allocations.
  - Provides both unaligned (`allocate`) and aligned (`allocate_aligned`) allocation entry points.
  - Tracks total memory used for debugging and profiling via `memory_usage()`.
  - Frees all blocks in `Drop`.
- **`ArenaTest`** – a trivial placeholder struct used to mirror the C++ `ArenaTest` harness for tests/benchmarks.
- **`LcgRandom`** – a minimal linear congruential generator used for randomized stress tests.

The allocator is *monotonic*: allocation is O(1), and memory is reclaimed only when the arena is dropped. There is no deallocation API for individual allocations.

This crate is expected to be used as an internal dependency of higher-level database/LSM components rather than as a general-purpose allocator. Nevertheless, the interface is straightforward to integrate into other high-throughput, allocation-heavy systems.

---

## Design

### Memory model

The `Arena` maintains:

- `alloc_ptr: *mut u8` – pointer to the current bump position inside the active block.
- `alloc_bytes_remaining: usize` – remaining capacity in bytes in the current block.
- `blocks: Vec<*mut u8>` – owned raw pointers to all blocks allocated so far.
- `memory_usage: AtomicUsize` – a relaxed-ordered counter tracking the total heap footprint of all blocks.

Allocation proceeds as follows:

1. **Fast path** – If `bytes <= alloc_bytes_remaining`, the arena returns `alloc_ptr` and advances the bump pointer and remaining count.
2. **Fallback path** – Otherwise, it calls an internal `allocate_fallback(bytes)` (not shown in the snippet but expected to:
   - Allocate a new backing block (typically at least `bytes` and at least some configured block size).
   - Push the block's base pointer into `blocks`.
   - Update `memory_usage`.
   - Serve the requested `bytes` from the newly allocated block.

`Drop` reconstructs `Box<[u8]>` (or equivalent) from each pointer in `blocks` and drops them, reclaiming all underlying memory.

### Alignment

`allocate_aligned(bytes)` ensures the returned pointer has at least `max(align_of::<*const c_void>(), 8)` alignment. The method computes the current pointer modulus with respect to the alignment and either:

- Serves the allocation from the current block by introducing a small "slop" offset, or
- Falls back to `allocate_fallback(bytes)` and asserts that the resulting pointer meets the alignment constraint.

This suffices for typical pointer- and 64-bit aligned data structures used in database internals.

### Randomized testing with `LcgRandom`

The included `LcgRandom` is a classic linear congruential generator (LCG):

- State transition: `seed := seed * 6364136223846793005 + 1 (mod 2^64)`.
- `uniform(range)` returns `(seed >> 32) % range` in `[0, range)`, which is adequate for fuzz-style tests.
- `one_in(n)` returns `true` with probability ≈ `1/n` by sampling a uniform integer in `[0, n)` and checking equality to 0.

It is not cryptographically secure and is only intended for performance and correctness tests of the arena.

---

## Logging and observability

The crate uses the `log` facade (`info!`, `debug!`, `trace!`) to:

- Announce creation of `Arena` and `ArenaTest` instances.
- Record allocation sizes and decisions about falling back to new blocks.
- Expose memory usage via `Arena::memory_usage()`.
- Trace pseudo-random sequences in `LcgRandom`.

To observe these logs, depend on a `log`-compatible logger (e.g. `env_logger`, `tracing-log`, `fern`) and initialize it in your executable before using this crate.

---

## Usage

### Adding the dependency

```toml
[dependencies]
bitcoinleveldb-arena = "0.1.19"
log = "0.4"     # to see internal logging
``

### Basic arena allocation

```rust
use bitcoinleveldb_arena::Arena; // assuming the crate exports Arena at the root

fn main() {
    // Initialize a logger of your choice here to see logs.

    let mut arena = Arena::default();

    // Allocate 128 bytes (unaligned beyond the arena's natural bump position)
    let ptr = arena.allocate(128);
    unsafe {
        // For example, initialize the memory
        std::ptr::write_bytes(ptr, 0, 128);
    }

    // Request an aligned allocation (pointer alignment or 8 bytes, whichever is larger)
    let aligned = arena.allocate_aligned(64);
    unsafe {
        // You may cast the aligned pointer to any type whose alignment is <= arena alignment
        let slice = std::slice::from_raw_parts_mut(aligned, 64);
        slice[0] = 42;
    }

    // Retrieve an approximate total memory footprint
    let usage = arena.memory_usage();
    println!("arena is using ~{} bytes", usage);

    // When `arena` is dropped here, all internal blocks are freed en masse.
}
```

### Randomized workload generation

```rust
use bitcoinleveldb_arena::{Arena, LcgRandom};

fn randomized_allocations() {
    let mut arena = Arena::default();
    let mut rng = LcgRandom::new(0x1234_5678_9abc_def0);

    for _ in 0..10_000 {
        let size = 1 + rng.uniform(256) as usize;
        let ptr = if rng.one_in(4) {
            arena.allocate_aligned(size)
        } else {
            arena.allocate(size)
        };

        unsafe { std::ptr::write_bytes(ptr, 0xAA, size); }
    }

    println!("total usage = {} bytes", arena.memory_usage());
}
```

---

## Safety considerations

This crate purposefully exposes raw pointers and performs manual memory management, so correct usage requires care:

- **Lifetime discipline**: Pointers returned by `Arena::allocate` and `Arena::allocate_aligned` remain valid until the arena is dropped. Do not access them after the arena is out of scope.
- **No individual free**: You must not attempt to deallocate or `Box::from_raw` these pointers yourself. The arena owns them and will free them when dropped.
- **Type aliasing and alignment**:
  - Only cast returned `*mut u8` pointers to types whose alignment is less than or equal to the arena's alignment guarantees (`allocate_aligned` for stricter alignment).
  - Ensure that your casts do not violate Rust's aliasing rules. Consider using `std::ptr::write`, `ptr::read`, and carefully designed lifetimes.
- **Threading**: `Arena` uses `AtomicUsize` for `memory_usage` but, given the shared mutable state in `alloc_ptr` and `alloc_bytes_remaining`, treat an individual `Arena` as **not** `Sync`. Access it from a single thread or protect access with synchronization primitives.

Because this crate is designed for integration inside performance-critical systems, it chooses explicit `unsafe` usage rather than high-level abstractions in some places. Review any unsafe blocks before adapting the allocator to new use cases.

---

## When to use this crate

This crate is appropriate when you:

- Need a LevelDB-compatible memory arena for porting or interop tests.
- Want a high-throughput, monotonic allocator for ephemeral data structures (e.g., table building, compaction buffers, query scratchpads).
- Prefer deterministic, simple allocation behavior over general-purpose, fragmentation-resistant allocators.

It is **not** a general-purpose allocator replacement and should not be used for arbitrary application-wide memory management without careful profiling and analysis.

---

## Repository and contribution

The code lives in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Issues and pull requests should be filed against that repository. When modifying this crate, preserve behavioral parity with the original C++ LevelDB arena where relevant, especially in terms of pointer alignment, block sizing policies, and performance characteristics.

---

## License

Licensed under the MIT License. See the repository for full license text.
