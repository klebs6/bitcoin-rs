# bitcoin-crc32c

High-performance CRC32C (Castagnoli polynomial) for Rust, with runtime-dispatched SIMD backends (x86_64 SSE4.2 and AArch64 CRC+PMULL) and a portable, bit-correct fallback. Designed for workloads that continuously validate or authenticate byte streams, such as blockchains, storage engines, and network protocols.

---

## Features

- **CRC32C (Castagnoli) polynomial**: \(0x1EDC6F41\) with standard initialization/finalization (XOR with `0xFFFF_FFFF`).
- **Runtime CPU feature dispatch**:
  - x86_64: SSE4.2-accelerated implementation using `_mm_crc32_*` intrinsics.
  - AArch64: CRC32 + PMULL-accelerated implementation using `__crc32c*` and polynomial merging in GF(2).
  - All other targets: optimized portable scalar implementation.
- **Zero-copy pointer-based API** for maximal control and minimal overhead.
- **Safe convenience wrappers** for `&str` and for extending an existing CRC accumulator.
- **Cache-aware prefetching** and alignment helpers for high throughput on large buffers.
- **Deterministic, bit-exact output** across platforms.

This crate is particularly suitable for systems code (e.g., Bitcoin node implementations, storage/index structures) where CRC32C is used as a fast integrity check over large, frequently accessed byte ranges.

---

## Core concepts

CRC32C computes a 32‑bit checksum over a byte sequence using arithmetic over the finite field \(\mathbb{F}_2\). The implementation leverages the following principles:

- **Polynomial representation**: A byte stream is viewed as coefficients of a polynomial over \(\mathbb{F}_2\). Division by a fixed generator polynomial yields the CRC remainder.
- **Extension property**: Given CRC of prefix \(P\), one can efficiently compute the CRC of \(P || S\) without reprocessing \(P\). This crate exposes that extension primitive (`crc32c_extend`).
- **Vectorization and parallel segments**:
  - SSE4.2/AArch64-CRC instructions directly update the CRC state per 8/4/2/1 bytes.
  - Large buffers are partitioned into blocks that are processed in parallel (e.g., `crc0`…`crc3`) and then merged using precomputed GF(2) multiplication tables and skip tables.
- **Alignment and prefetching**: The algorithms align to 8- or 4-byte boundaries and prefetch future data to hide memory latency.

These techniques converge to a design well-suited for sustained multi-GB/s CRC computation on modern CPUs.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-crc32c = "0.1.19"
```

This crate targets Rust **2021** edition and is licensed under **MIT**.

---

## High-level API

For most use cases, you want the safe, dispatching helpers.

### CRC of a raw buffer (unsafe)

```rust
use bitcoin_crc32c::crc32c_value;

fn crc_of_slice(bytes: &[u8]) -> u32 {
    unsafe { crc32c_value(bytes.as_ptr(), bytes.len()) }
}
```

- Uses the best available backend (SSE4.2, AArch64-CRC, or portable) chosen at runtime.
- `data` must be valid for `count` bytes; otherwise behavior is undefined.

### CRC of a string (safe)

```rust
use bitcoin_crc32c::crc32c_with_str;

let checksum = crc32c_with_str("hello world");
```

- Computes CRC32C over the UTF‑8 bytes of the string.
- Internally dispatches to the optimal backend.

### Short-hand pointer API (unsafe)

```rust
use bitcoin_crc32c::crc32c;

let buf: &[u8] = b"example";
let crc = unsafe { crc32c(buf.as_ptr(), buf.len()) };
```

`crc32c` is equivalent to `crc32c_value` and exists primarily for parity with C-style interfaces.

### Extending an existing CRC (streaming)

The extension API allows you to stream data in multiple chunks without re-hashing previous data.

```rust
use bitcoin_crc32c::crc32c_extend;

fn crc_streaming(chunks: &[&[u8]]) -> u32 {
    let mut crc = 0u32; // initial accumulator
    for chunk in chunks {
        unsafe {
            crc = crc32c_extend(crc, chunk.as_ptr(), chunk.len());
        }
    }
    crc
}
```

- Passing `crc = 0` and one single chunk is equivalent to `crc32c_value` of the concatenation.
- The function internally chooses the optimal backend based on CPU capabilities and compiled-in features.

---

## Backend selection and CPU feature detection

The crate uses a dispatcher that selects at runtime among the following backends:

```rust
use bitcoin_crc32c::{can_use_sse42, can_use_arm64_crc32};

if can_use_sse42() {
    // SSE4.2 will be used by `crc32c_extend` on x86_64
}

if can_use_arm64_crc32() {
    // AArch64 CRC+PMULL will be used by `crc32c_extend` on aarch64
}
```

### Dispatch logic

```rust
use bitcoin_crc32c::crc32c_extend;

// Pseudocode behavior:
// - if x86_64 and SSE4.2 available -> `crc32c_extend_sse42`
// - else if aarch64 and CRC available -> `crc32c_extend_arm64`
// - else -> `crc32c_extend_portable`
```

The detection functions internally use `std::arch::is_x86_feature_detected!("sse4.2")` and `std::arch::is_aarch64_feature_detected!("crc")`, cached via `OnceLock` to avoid repeated CPUID queries.

---

## Low-level building blocks

Fully general users rarely need these, but they are useful for systems-level integration, benchmarking, or experimentation.

### Portable backend wrapper for testing

```rust
use bitcoin_crc32c::{PortableTestTraits, crc32c_extend_portable};

let buf = b"test data";
let crc = unsafe { crc32c_extend_portable(0, buf.as_ptr(), buf.len()) };

// Equivalent via traits wrapper (useful for generic tests):
let crc2 = unsafe { PortableTestTraits::extend(0, buf.as_ptr(), buf.len()) };
assert_eq!(crc, crc2);
```

`PortableTestTraits` provides a stable handle to the pure-Rust implementation, ensuring you can compare accelerated vs. non-accelerated paths in tests.

### Alignment helpers

```rust
use bitcoin_crc32c::round_up_with_uintptr;

let ptr_value: usize = 0x1003;
let aligned = round_up_with_uintptr::<8>(ptr_value);
assert_eq!(aligned, 0x1008);
```

```rust
use bitcoin_crc32c::round_up;

let buf = [0u8; 32];
let p = buf.as_ptr();
let aligned_ptr = round_up::<8>(p);
```

These helpers are used internally to align data to 4/8-byte boundaries before entering tight loops.

### Unaligned little-endian loads

```rust
use bitcoin_crc32c::{read_uint32le, read_uint64le};

let buf = [1u8, 0, 0, 0, 2, 0, 0, 0];
let x = read_uint32le(buf.as_ptr());      // 1
let y = read_uint64le(buf.as_ptr());      // 2^32 + 1 on little-endian
```

They correctly handle both little- and big-endian hosts while assuming that `buffer` is valid for 4/8 bytes respectively.

### Cache prefetching

```rust
use bitcoin_crc32c::request_prefetch;

let data = [0u8; 4096];
unsafe {
    request_prefetch(data.as_ptr().add(1024));
}
```

- On x86_64 with SSE: uses `_mm_prefetch` with `_MM_HINT_NTA` (non-temporal), hinting that data will be read soon but not necessarily reused.
- On AArch64 with NEON: uses `prefetch_read_data` into L1.
- On other targets: becomes a no-op.

---

## Safety notes

Several functions expose raw-pointer APIs for maximal performance and zero copies. The caller must uphold the documented contracts:

- Pointers must be valid for at least `count` bytes.
- Pointers must either be non-null or only used in the `count == 0` case.
- There is no aliasing requirement for CRC computation, but the lifetime must cover the call duration.

Violating these preconditions leads to undefined behavior at the Rust level.

When in doubt, wrap your data in slices and call the unsafe functions in small, well-audited FFI or boundary modules.

---

## Example: incremental CRC over a file

```rust
use std::fs::File;
use std::io::{self, Read};
use bitcoin_crc32c::crc32c_extend;

fn crc32c_of_file(path: &str) -> io::Result<u32> {
    let mut file = File::open(path)?;
    let mut buf = [0u8; 64 * 1024];
    let mut crc = 0u32;

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 { break; }
        let chunk = &buf[..n];
        unsafe {
            crc = crc32c_extend(crc, chunk.as_ptr(), chunk.len());
        }
    }

    Ok(crc)
}
```

This pattern is ideal for very large inputs, as it avoids allocations and leverages streaming extension.

---

## Integration in Bitcoin / blockchain contexts

The crate resides in the `bitcoin-rs` repository and is tailored for high-throughput checksum computation in systems that:

- Validate serialized blocks, transactions, or auxiliary indices.
- Maintain on-disk structures (e.g., UTXO sets, mempool journals) where cheap integrity checks are beneficial.
- Need a predictable, fast, low-level primitive instead of hashing with heavier cryptographic algorithms.

While CRC32C is **not** a cryptographic hash and must not be used for adversarial collision resistance, it is extremely effective for catching random corruption, storage bit flips, and transmission errors.

---

## Performance considerations

- Large inputs benefit most from the SSE4.2 and ARM CRC+PMULL backends.
- The algorithms aggressively use unrolled loops, 8-byte loads, and GF(2) skip tables to amortize overhead.
- Prefetch distance (`PREFETCH_HORIZON`) and block sizes (`BLOCK_0SIZE`, `BLOCK_1SIZE`, `BLOCK_2SIZE`) are chosen to balance instruction-level parallelism and cache behavior.

For maximum throughput in your application, prefer:

- Large, contiguous buffers.
- Fewer function calls (batch data into larger chunks when possible).
- Reuse of the extension API (`crc32c_extend`) rather than recomputing from zero for concatenated streams.

---

## License and provenance

- **License**: MIT
- **Edition**: Rust 2021
- **Repository**: <https://github.com/klebs6/bitcoin-rs>

This crate is a component within a broader Bitcoin-related codebase, exposing a well-factored, standalone CRC32C implementation suitable for reuse in other high-performance Rust projects.
