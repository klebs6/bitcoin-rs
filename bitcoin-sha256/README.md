# bitcoin-sha256

An aggressively optimized, Bitcoin-Core–compatible SHA‑256 implementation in Rust, with explicit control over compression rounds, FFI‑oriented entry points, and CPU‑feature–aware backend selection.

---

## Overview

`bitcoin-sha256` provides a low‑level, **bit‑for‑bit compatible** SHA‑256 implementation tuned for Bitcoin workloads:

- Streaming hasher `Sha256` implementing `std::io::Write`.
- A `ComputeSha256` trait for computing SHA‑256 on `u256` values.
- Explicit, FIPS‑180‑4–accurate bitwise primitives for Σ/σ, Ch, Maj, and a canonical `sha256_round` implementation.
- Single‑block and batched compression entry points that mirror Bitcoin Core’s C++ layout.
- Auto‑detected CPU backends (scalar, SHANI, AVX2) for double‑SHA‑256 on 64‑byte blocks.
- FFI‑compatible pointer APIs for integration with C/C++ or other runtime systems.

The design goal is **mechanical fidelity** to Bitcoin Core’s SHA‑256 logic while leveraging Rust’s type system and tooling (e.g., `tracing`) for observability and safety where it does not interfere with performance.

---

## Features at a Glance

- **Canonical SHA‑256 compression**
  - `sha256_transform_block` implements the full 64‑round compression function on a single 64‑byte block, with explicit round scheduling and feed‑forward.
  - `sha256_round` gives a pedagogical, round‑by‑round view of the inner operation, matching the original C++ macro semantics.

- **Streaming hasher**
  - `Sha256` struct represents a stateful hasher:
    - `s: [u32; 8]` – internal state words.
    - `buf: [u8; 64]` – 64‑byte block buffer.
    - `bytes: u64` – total bytes consumed.
  - Implements `Default`, `Write`, and convenient helpers like `new`, `reset`, `finalize`, and `finalize_wipe`.

- **Bitcoin‑style double‑SHA‑256**
  - Scalar reference path: `transform_d64_scalar(out32, in64)` computes `SHA256(SHA256(message))` for a single 64‑byte message.
  - Backend abstraction (`Sha256Backend`) supports optimized 2‑way, 4‑way, and 8‑way double‑SHA‑256 transforms.

- **Backend auto‑detection**
  - `sha256auto_detect()` inspects CPU features (e.g., `sha`, `avx2`) and configures the best implementation:
    - Standard scalar backend.
    - SHANI‑accelerated backend (`enable-shani` feature).
    - AVX2 8‑way double‑SHA‑256 (`enable-avx2` feature).
  - Returns a human‑readable description closely mirroring Bitcoin Core’s strings, e.g.:
    - `"standard"`
    - `"shani(1way,2way)"`
    - `"avx2(8way)"`
    - `"shani(1way,2way),avx2(8way)"`

- **FFI‑friendly surface**
  - Pointer‑based APIs such as `sha256_write`, `sha256_finalize`, `sha256_transform`, `transform_d64_scalar`, and the `TransformType` / `TransformD64Type` aliases allow direct integration with C ABI call sites.

- **Internal validation**
  - `self_test()` exhaustively validates compression states and double‑SHA‑256 helpers against canonical fixtures, including misaligned input cases used in Bitcoin Core.

---

## Cryptographic Background

The implementation is a direct encoding of FIPS 180‑4 (SHA‑256) semantics:

- **State**: eight 32‑bit words \( (a,b,c,d,e,f,g,h) \).
- **Message schedule** \(w_0,\dots,w_{63}\):
  - First 16 words are big‑endian decoded input.
  - Remaining 48 words derive via:
    \[
    w_t = w_{t-16} + \sigma_0(w_{t-15}) + w_{t-7} + \sigma_1(w_{t-2}) \pmod{2^{32}}
    \]
- **Round function** for round \(t\):
  - `Ch` (choice): \( \operatorname{Ch}(x,y,z) = z \oplus (x \land (y \oplus z)) \)
  - `Maj` (majority): \( \operatorname{Maj}(x,y,z) = (x \land y) \lor (z \land (x \lor y)) \)
  - Uppercase sigmas:
    - \(\Sigma_0(x) = x\rotateRight{2} \oplus x\rotateRight{13} \oplus x\rotateRight{22}\)
    - \(\Sigma_1(x) = x\rotateRight{6} \oplus x\rotateRight{11} \oplus x\rotateRight{25}\)
  - Lowercase sigmas for schedule:
    - \(\sigma_0(x) = x\rotateRight{7} \oplus x\rotateRight{18} \oplus (x \gg 3)\)
    - \(\sigma_1(x) = x\rotateRight{17} \oplus x\rotateRight{19} \oplus (x \gg 10)\)

The crate explicitly exposes these functions (`big_sigma0`, `big_sigma1`, `sha256_sigma0`, `sha256_sigma1`, `sha256_ch`, `sha256_maj`, `sha256_round`) to let advanced users audit, instrument, or swap specific components while preserving algebraic correctness.

For Bitcoin applications, **double‑SHA‑256** on fixed 64‑byte inputs is a first‑class workload (block headers, transaction IDs). This crate’s `transform_d64_scalar` and the `TransformD64Type` family are designed specifically for that regime, enabling tight vectorization and multi‑lane execution.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-sha256 = "0.1.20"
```

Optional CPU‑specific features:

```toml
[dependencies.bitcoin-sha256]
version = "0.1.20"
features = ["enable-shani", "enable-avx2"]
```

> Note: exact feature names and combinations should be verified against the published crate metadata.

---

## Core Types and Traits

### `Sha256`

Streaming hasher for SHA‑256. Internally equivalent to the FIPS 180‑4 reference with Bitcoin Core–style block handling.

Key capabilities:

- `Sha256::new() -> Sha256` – construct a freshly initialized hasher.
- `Sha256::reset(&mut self)` – reset to IV, zeroing buffer and counters.
- `Sha256::finalize(&mut self, &mut [u8; 32])` – finalize without wiping internal state words.
- `Sha256::finalize_wipe(&mut self, &mut [u8; 32])` – finalize and zero state words for better key‑material hygiene.
- `Sha256::initialize(&mut self)` – (re)initialize internal state from IV.
- `Sha256::write_ptr(&mut self, data: *const u8, len: usize)` – low‑level pointer writer.
- `Sha256::write_from_iterator(&mut self, iter: Box<dyn Iterator<Item = u8>>, len: usize)` – drive from a byte iterator of known length.

`Sha256` also implements `std::io::Write`, making it straightforward to integrate into existing I/O paths.

#### Example: Hashing arbitrary data

```rust
use bitcoin_sha256::Sha256; // path may differ depending on crate layout

fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    use std::io::Write;

    hasher.write_all(data).expect("write to Sha256");

    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    out
}
```

### `ComputeSha256`

Trait for computing SHA‑256 on types that can be treated as a 32‑byte value.

```rust
pub trait ComputeSha256 {
    fn sha256(&self) -> u256;
}
```

The crate provides an implementation for `u256`:

```rust
impl ComputeSha256 for u256 {
    fn sha256(&self) -> u256 {
        let mut result = u256::zero();
        let mut sha = Sha256::new();
        sha.write(self.as_ref());
        sha.finalize(result.as_mut_slice_exact());
        result
    }
}
```

This is particularly convenient for Bitcoin transaction IDs and block hashes.

#### Example: Hashing a `u256`

```rust
use bitcoin_sha256::ComputeSha256; // and a suitable u256 type from your stack

fn double_sha256_u256(x: &u256) -> u256 {
    x.sha256()
}
```

> The exact `u256` type originates from the surrounding `bitcoin-rs` ecosystem; ensure you import the correct one.

---

## Low‑Level Compression API

The crate exposes the complete compression machinery for expert users.

### IV initialization

```rust
pub fn sha256_initialize(s: *mut u32)
```

Writes the standard FIPS 180‑4 IV into an 8‑word state array pointed to by `s`.

### Single‑block compression

```rust
pub fn sha256_transform_block(s: *mut u32, chunk: *const u8)
```

- `s`: pointer to 8 writable `u32` words.
- `chunk`: pointer to exactly 64 bytes.
- Performs one SHA‑256 compression on the block and feed‑forwards the result into `*s`.

### Batched compression

```rust
pub fn sha256_transform(s: *mut u32, chunk: *const u8, blocks: usize)
```

Processes `blocks` consecutive 64‑byte chunks starting at `chunk`.

### Big‑endian word API

```rust
pub fn sha256_transform_one_block_be_words(s: *mut u32, chunk: *const u32)
```

Processes a 64‑byte chunk represented as 16 big‑endian `u32` words.

### Bitwise components

```rust
pub fn big_sigma0(x: u32) -> u32
pub fn big_sigma1(x: u32) -> u32
pub fn sha256_sigma0(x: u32) -> u32
pub fn sha256_sigma1(x: u32) -> u32
pub fn sha256_ch(x: u32, y: u32, z: u32) -> u32
pub fn sha256_maj(x: u32, y: u32, z: u32) -> u32

pub fn sha256_round(
    a: u32, b: u32, c: u32, d: &mut u32,
    e: u32, f: u32, g: u32, h: &mut u32,
    k: u32, w: u32,
)
```

These are useful for:

- Writing alternative or instrumented compression loops.
- Formal verification or symbolic execution environments.
- Fine‑grained performance and microarchitectural analysis.

---

## Double‑SHA‑256 Helpers and Backends

### Scalar reference path

```rust
pub fn transform_d64_scalar(out32: *mut u8, in64: *const u8)
```

Computes `SHA256(SHA256(m))` for a single 64‑byte message `m`:

1. Hashes the 64‑byte message with standard SHA‑256.
2. Serializes the first digest to big‑endian bytes.
3. Hashes that 32‑byte digest with SHA‑256 again.
4. Writes the final 32‑byte result to `out32`.

This implementation is architecturally neutral and acts as the reference behavior against which vectorized backends are validated.

### Backend abstraction

```rust
pub type TransformType    = unsafe fn(*mut u32, *const u8, usize);
pub type TransformD64Type = unsafe fn(*mut u8,  *const u8);

pub(crate) struct Sha256Backend {
    transform: TransformType,
    d64:       TransformD64Type,
    d64_2:     Option<TransformD64Type>,
    d64_4:     Option<TransformD64Type>,
    d64_8:     Option<TransformD64Type>,
    desc:      &'static str,
}
```

Dispatch helpers:

```rust
pub fn dispatch_transform(state: *mut u32, chunk: *const u8, blocks: usize)
pub fn dispatch_d64(out: *mut u8, inp: *const u8)
```

They route through the globally selected backend configured by `sha256auto_detect()`.

#### Auto‑detection

```rust
pub fn sha256auto_detect() -> String
```

- Inspects `x86_64` CPU features using `std::arch::is_x86_feature_detected!`.
- Conditionally enables SHANI and AVX2 double‑SHA‑256 implementations.
- Publishes chosen function pointers into global `TRANSFORM*` slots.
- Runs `self_test()` to guarantee that the chosen backend is bit‑exact.

You should call `sha256auto_detect()` early in your process if you intend to use the global function pointers or want deterministic logging of which backend was selected.

#### Example: Explicit backend selection and use

```rust
fn init_and_use_backend(header: &[u8; 64]) -> [u8; 32] {
    let desc = bitcoin_sha256::sha256auto_detect();
    eprintln!("Selected SHA-256 backend: {desc}");

    let mut out = [0u8; 32];
    unsafe {
        bitcoin_sha256::dispatch_d64(out.as_mut_ptr(), header.as_ptr());
    }
    out
}
```

> All pointer‑based functions are `unsafe` for a reason: you must uphold the documented size and aliasing constraints.

---

## Tagged Hash Initialization

Bitcoin uses **tagged hashes** of the form

\[
H(\text{tag} \parallel \text{tag} \parallel m)
\]

where \(H\) is SHA‑256 and \(\text{tag}\) is a context string (e.g., `"TapLeaf"`).

The helper:

```rust
pub fn sha256_initialize_tagged(hash: *mut Sha256, tag: *const u8, taglen: usize)
```

performs:

1. Initialize a `Sha256` context.
2. Compute `SHA256(tag)` into a temporary 32‑byte buffer.
3. Reset the context.
4. Write `SHA256(tag) || SHA256(tag)` into the context.

After calling this, `hash` is ready to receive the message `m` such that `finalize` yields `SHA256(SHA256(tag) || SHA256(tag) || m)`.

---

## FFI Surface and Safety Contracts

A subset of functions is specifically shaped for FFI use:

- `sha256_write(hash: *mut Sha256, data: *const u8, len: usize)`
- `sha256_finalize(hash: *mut Sha256, out32: *mut u8)`
- `sha256_transform(s: *mut u32, chunk: *const u8, blocks: usize)`
- `transform_d64_scalar(out32: *mut u8, in64: *const u8)`

Typical safety pre‑conditions:

- Pointers must be valid for reads/writes of the documented length.
- Pointers must obey Rust’s aliasing rules (i.e., do not create mutable aliases that overlap with other live references).
- Regions must not overlap when the API says so.

Use these from C or C++ by exposing C‑ABI wrappers in your Rust crate that depend on `bitcoin-sha256`.

---

## Self‑Testing

```rust
pub fn self_test() -> bool
```

Runs an internal suite that checks:

1. Multiple‑block compression states against precomputed fixtures.
2. Double‑SHA‑256 helpers (scalar and available vectorized variants) against canonical outputs.

If any mismatch is detected, diagnostic information is printed to `stderr`. `sha256auto_detect()` calls `self_test()` and panics on failure to avoid silently running with an incorrect backend.

You can also integrate `self_test()` into your own startup checks if you manage backends manually.

---

## Logging and Instrumentation

The crate uses the `tracing` ecosystem for internal observability under the `"sha256"` target. Functions such as `sha256_sigma0`, `sha256_sigma1`, `sha256_round`, `sha256_transform`, `Sha256::write_ptr`, and `self_test` emit trace‑level events.

To use this effectively, include a `tracing-subscriber` setup in your binary and enable the `sha256` target at the desired level (e.g., TRACE in dev builds, OFF or ERROR in production).

---

## Performance Considerations

- Core hot paths are annotated with `#[inline(always)]` where it materially affects performance, matching Bitcoin Core’s expectations.
- The internal buffering strategy in `Sha256::write_ptr` minimizes copying by:
  - Topping up a partially filled internal 64‑byte buffer.
  - Processing as many full blocks as possible directly from the caller’s memory.
  - Buffering any trailing bytes < 64.
- Double‑SHA‑256 backends are designed to maximize throughput on x86‑64 with SHANI and AVX2 by processing multiple 64‑byte lanes in parallel.

You should benchmark your specific workload with and without the SHANI/AVX2 features enabled to measure the impact on your deployment targets.

---

## License

This crate is distributed under the **MIT** license.

See the repository for full license text:

- Repository: <https://github.com/klebs6/bitcoin-rs>

---
