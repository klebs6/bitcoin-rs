# bitcoin-sha256-sse41

Highly specialized SHA-256 double-hash (sha256d) transform for Bitcoin workloads, vectorized with SSE4.1 and designed to process four 64-byte message blocks in parallel.

---

## Overview

This crate exposes a low-level, `unsafe`-oriented implementation of the Bitcoin-style double SHA-256 compression function, tuned for x86-64 CPUs with SSE4.1.

Conceptually, the implementation follows the standard SHA-256 compression function as defined in FIPS 180-4, but organizes the state into 128-bit SIMD registers (`__m128i`) so that four independent message streams are processed in lockstep ("4-way" SIMD). This matches highly parallel Bitcoin mining or block verification workloads where the same midstate is reused across many candidate nonces.

The core entry point is:

```rust
pub fn sha256d64_sse41_transform_4way(out: *mut u8, in_: *const u8)
```

This performs:

1. A first SHA-256 compression on 4 × 64-byte inputs.
2. A second SHA-256 compression over the resulting 4 × 32-byte digests plus padding (the standard Bitcoin `sha256d` construction).

It is intentionally close to a direct port of the corresponding C++ SSE4.1 routine used in high-performance Bitcoin implementations.

**Important properties:**

- **Throughput over ergonomics:** The API is pointer-based and expects preformatted input. There is no allocation, buffering, or incremental hasher abstraction.
- **SIMD parallelism:** Uses 128-bit SIMD lanes (`__m128i`) to process 4 parallel message blocks.
- **Deterministic arithmetic:** Operations are limited to bitwise logic and 32-bit integer arithmetic, which is deterministic across compliant x86-64 implementations.
- **SSE4.1 dependency:** The implementation relies on SSE4.1 intrinsics (`_mm_set1_epi32`, `_mm_shuffle_epi8`, etc.). Running this on hardware without SSE4.1 is undefined.

This crate targets expert consumers building custom Bitcoin mining kernels, high-throughput block/transaction verification engines, or cryptographic research prototypes where full control over layout and scheduling is desired.

---

## When should you use this crate?

Use this crate if you:

- Need **maximum throughput** for Bitcoin-style double SHA-256 on x86-64.
- Can orchestrate work in batches of 4 parallel inputs.
- Are comfortable managing `unsafe` pointers and CPU feature detection yourself.

You probably do *not* want this crate if you only need a conventional, ergonomic SHA-256 hash API. For that, see crates like `sha2` or `bitcoin_hashes`.

---

## Safety and prerequisites

This crate is fundamentally low-level. You must uphold the following invariants:

- The CPU must support **SSE4.1**.
- Pointers passed to the API must be valid for the required number of bytes and correctly aligned for `u32` loads/stores (the code is written as if 32-bit loads/stores are safe).
- You must ensure that buffers do not alias in ways that break the algorithm's assumptions.

A typical program should:

1. Detect SSE4.1 support (e.g., via `is_x86_feature_detected!("sse4.1")`).
2. Dispatch to `sha256d64_sse41_transform_4way` only when it is available.
3. Provide correctly sized and aligned input/output buffers.

---

## Cryptographic background

### SHA-256 compression

SHA-256 operates on 512-bit (64-byte) message blocks. For each block, it maintains eight 32-bit state words \(a, b, c, d, e, f, g, h\). Each compression consists of 64 rounds structured as

\[
\begin{aligned}
T_1 &= h + \Sigma_1(e) + \operatorname{Ch}(e,f,g) + K_t + W_t,\\
T_2 &= \Sigma_0(a) + \operatorname{Maj}(a,b,c),
\end{aligned}
\]

followed by a state rotation. Here:

- `Ch(x,y,z) = (x \land y) \oplus (\neg x \land z)` (choose)
- `Maj(x,y,z) = (x \land y) \oplus (x \land z) \oplus (y \land z)` (majority)
- \(\Sigma_0, \Sigma_1\) and the lower-case \(\sigma_0, \sigma_1\) are bitwise rotate/shift-based functions.
- `K_t` is a fixed round constant, and `W_t` is the message schedule.

### Bitcoin `sha256d`

Bitcoin uses **double SHA-256** (`sha256d`):

1. Compute \(h_1 = \text{SHA256}(m)\).
2. Compute \(h_2 = \text{SHA256}(h_1)\).

This crate implements a fused, batch-optimized variant of this composition, specialized to 4 parallel 64-byte input blocks and their 4 resulting 32-byte double-hashes.

---

## API

The public surface is deliberately minimal and low-level. It consists of two layers:

1. **Top-level transform**: one function that performs the complete double SHA-256 on four 64-byte inputs.
2. **SIMD intrinsics wrappers**: a set of helper functions modeling the arithmetic and bitwise operations in terms of `__m128i`. These are useful if you want to extend or inspect the internal pipeline.

### Core transform

```rust
pub fn sha256d64_sse41_transform_4way(out: *mut u8, in_: *const u8)
```

#### Semantics

- Interprets `in_` as a contiguous buffer of **4 × 64** bytes.
- Computes Bitcoin `sha256d` for each of the four 64-byte blocks in parallel.
- Writes the resulting 4 × 32-byte digests into `out`, laid out contiguously.

A natural layout is:

- Input: `[block0[0..64], block1[0..64], block2[0..64], block3[0..64]]` (256 bytes total).
- Output: `[hash0[0..32], hash1[0..32], hash2[0..32], hash3[0..32]]` (128 bytes total).

The actual endianness and layout should be verified against tests and the internal `read4`/`write4` routines, which use `_mm_shuffle_epi8` to normalize to big-endian word order as defined by SHA-256.

#### Safety

This function is `unsafe`-equivalent even if not marked as such, because it accepts raw pointers. You must guarantee:

- `in_` points to at least 4 × 64 = 256 readable bytes.
- `out` points to at least 4 × 32 = 128 writable bytes.
- Both buffers are valid for the duration of the call and correctly aligned for 32-bit access.
- The pointers do not alias in a way that violates Rust's aliasing rules for simultaneous read/write.

In idiomatic Rust, you would typically expose a safe wrapper in your own crate that internally calls this function via `unsafe`.

---

### SIMD helper functions

All helper functions operate on `__m128i`, where each lane represents a 32-bit word from one of the 4 parallel message streams.

- `k(x: u32) -> __m128i`: Broadcast a 32-bit constant into all 4 lanes.
- `add(...) -> __m128i`: 2-, 3-, 4-, and 5-operand vector addition with 32-bit lanewise wraparound.
- `inc(x: &mut __m128i, ...) -> __m128i`: In-place increment of `x` by one or more other vectors; returns the updated value.
- `xor`, `or`, `and`: Standard bitwise operations on 4 lanes in parallel.
- `shr`, `shl`: Logical right/left shifts of each 32-bit lane.
- `ch(x, y, z) -> __m128i`: Implements the SHA-256 choose function in SIMD.
- `maj(x, y, z) -> __m128i`: Implements the SHA-256 majority function in SIMD.

There are two groups of sigma functions:

- *Uppercase* `sigma0`, `sigma1` (often denoted \(\Sigma_0, \Sigma_1\) in the specification) defined via rotations at positions `(2, 13, 22)` and `(6, 11, 25)` respectively.
- *Lowercase* `sigma0`, `sigma1` (the message-schedule sigmas) defined via rotations and shifts at `(7,18)`/`>>3` and `(17,19)`/`>>10` respectively.

The transform uses these for:

- Evolving the message schedule `W_t` via `Inc` calls.
- Computing `T1` and `T2` in each `round` step.

The `round` function implements one iteration of the SHA-256 main loop with eight state words distributed across SIMD lanes:

```rust
pub fn round(
    a: __m128i,
    b: __m128i,
    c: __m128i,
    d: &mut __m128i,
    e: __m128i,
    f: __m128i,
    g: __m128i,
    h: &mut __m128i,
    k: __m128i,
)
```

Here each parameter represents 4 parallel values of the corresponding scalar variable.

---

### Input/output packing: `read4` and `write4`

```rust
pub fn read4(chunk: *const u8, offset: i32) -> __m128i
```

- Loads 4 × 32-bit words from 4 different 64-byte blocks at the same byte offset.
- Uses `_mm_set_epi32` with `ReadLE32` on four locations, then `_mm_shuffle_epi8` to normalize endianness.

Conceptually:

- Lane 0: word from block 0 at `offset`.
- Lane 1: word from block 1 at `64 + offset`.
- Lane 2: word from block 2 at `128 + offset`.
- Lane 3: word from block 3 at `192 + offset`.

```rust
pub fn write4(out: *mut u8, offset: i32, v: __m128i)
```

- Inverse operation of `read4`: reorders bytes via `_mm_shuffle_epi8` and serializes each 32-bit lane to the appropriate output location using `WriteLE32`.

The effect is that `sha256d64_sse41_transform_4way` internally works in a layout optimized for 4-way SIMD while still presenting a straightforward contiguous layout at the API boundary.

---

## Example usage

A minimal, *unsafe* example assuming you have four 64-byte pre-padded blocks (for illustration; real Bitcoin headers are not pre-padded like this):

```rust
use core::arch::x86_64::__m128i;

extern "Rust" {
    fn sha256d64_sse41_transform_4way(out: *mut u8, in_: *const u8);
}

fn main() {
    // 4 blocks × 64 bytes
    let mut input = [0u8; 4 * 64];
    // Fill input[0..64], input[64..128], input[128..192], input[192..256]

    let mut output = [0u8; 4 * 32];

    unsafe {
        // In production, check is_x86_feature_detected!("sse4.1") first
        sha256d64_sse41_transform_4way(output.as_mut_ptr(), input.as_ptr());
    }

    // output now contains 4 × 32-byte sha256d digests
}
```

In a real application you will typically:

- Reuse the input and output buffers over many invocations to amortize allocation cost.
- Use a separate, scalar path for odd counts of inputs or for CPUs without SSE4.1.
- Integrate this into a higher-level Bitcoin hashing abstraction that deals with padding, endianness, and header construction.

---

## Performance considerations

### Vectorization model

By mapping 4 independent SHA-256 streams into SIMD lanes, the implementation:

- Avoids branch divergence: all four streams execute the same instruction sequence.
- Maximizes arithmetic intensity: multiple independent additions, rotates, and logical operations are carried out in single instructions.
- Exploits the fact that Bitcoin `sha256d` has very regular structure.

The cost is that you are constrained to work in multiples of 4 inputs per call, or accept that some SIMD lanes may be wasted if you pad with dummy data.

### Microarchitectural notes

- The implementation is intended for modern x86-64 cores with efficient SSE4.1 execution units.
- Register pressure is non-trivial. The staged organization into three transforms (first hash, midstate, second hash) reduces live state.
- The shuffles in `read4`/`write4` and the rotate emulation via shifts and ORs are designed to minimize latency while keeping dependencies manageable.

For precise performance characteristics, benchmark on your target CPUs with realistic workloads (e.g., candidate header sets for mining, or block/transaction verification pipelines).

---

## Integration guidance

### CPU feature dispatch

You should gate use of this crate behind runtime feature detection, and optionally compile-time configuration:

```rust
#[cfg(target_arch = "x86_64")]
fn sha256d64_4way(out: &mut [u8; 4 * 32], input: &[u8; 4 * 64]) {
    assert!(is_x86_feature_detected!("sse4.1"));
    unsafe {
        bitcoin_sha256_sse41::sha256d64_sse41_transform_4way(
            out.as_mut_ptr(),
            input.as_ptr(),
        );
    }
}
```

You can then fall back to a portable implementation when SSE4.1 is not available.

### Endianness and Bitcoin specifics

- Bitcoin serializes block headers as little-endian fields, but SHA-256 operates on big-endian 32-bit words.
- This crate mirrors the standard Bitcoin C++ SSE4.1 path where `read4` and `write4` perform the necessary byte reordering.

Validate correctness by comparing against a reference implementation on well-known test vectors (e.g., genesis block header hash, random headers, and adversarial patterns).

---

## Build, edition, license

- **Edition:** Rust 2024
- **Minimum Rust:** The intent is to work with stable Rust for x86-64 with `core::arch::x86_64`, though actual MSRV may depend on your toolchain; validate in CI.
- **License:** MIT

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-sha256-sse41 = "0.1.1"
```

Then import and call the functions from your own higher-level abstractions.

---

## Status

At the time of writing, parts of the code are annotated with `todo!()` and the README is AI-generated. Treat the crate as **experimental**:

- Audit the implementation carefully before deployment.
- Add comprehensive tests comparing against known-good scalar implementations for a broad corpus of inputs.
- Benchmark across your target CPUs and workloads.

Contributions that refine correctness proofs, extend tests, and provide measured performance data are particularly valuable.
