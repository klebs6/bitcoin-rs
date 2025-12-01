# bitcoin-ripemd

A standalone, low-level RIPEMD‑160 implementation tuned for Bitcoin-style workloads.

This crate provides a small, `no_std`‑friendly core for computing RIPEMD‑160 message digests, with a focus on:

- Exact replication of the canonical RIPEMD‑160 bit‑level behavior
- Efficient streaming over arbitrarily long byte sequences
- Transparent, inspectable implementation suitable for audit and reuse inside Bitcoin libraries

---

## Features at a glance

- Pure Rust RIPEMD‑160 compression function and padding logic
- Stateful hasher type `Ripemd160` with `new`, `update`, `finalize`, and `reset`
- Carefully inlined bitwise round functions (`ripemd160_f1`–`f5`) and per‑round helpers
- Deterministic, Bitcoin‑compatible outputs (e.g., for HASH160: `RIPEMD160(SHA256(x))`)
- Fine‑grained tracing via the `tracing` crate to inspect internal state evolution (optional at runtime)

This crate is extracted from, and maintained in sync with, the `bitcoin-rs` repository, and is intended to be usable independently when a minimal RIPEMD‑160 implementation is desired.

---

## Use cases

Typical applications include:

- Bitcoin address construction (P2PKH, P2SH) via HASH160
- Re‑implementing or auditing Bitcoin consensus code
- Cryptographic tooling that requires RIPEMD‑160 hashes for interoperability
- Verification harnesses and fuzzing against alternative RIPEMD‑160 implementations

Note that RIPEMD‑160, while still widely used in Bitcoin, is not recommended for new generic cryptographic designs. Use this crate when *interoperability* with existing protocols explicitly demands RIPEMD‑160.

---

## Quick start

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-ripemd = "0.1.19"
```

Hash a byte slice into a 20‑byte RIPEMD‑160 digest:

```rust
use bitcoin_ripemd::Ripemd160;

fn ripemd160_hex(data: &[u8]) -> String {
    let mut hasher = Ripemd160::new();
    hasher.update(data);

    let mut out = [0u8; 20]; // RIPEMD160_OUTPUT_SIZE
    hasher.finalize(&mut out);

    // encode as lowercase hex (example, using `hex` crate)
    hex::encode(out)
}

fn main() {
    let digest_hex = ripemd160_hex(b"hello world");
    println!("RIPEMD-160(\"hello world\") = {}", digest_hex);
}
```

Streaming large data is identical: call `update` repeatedly and `finalize` once at the end.

```rust
use bitcoin_ripemd::Ripemd160;

fn hash_chunks(chunks: &[&[u8]]) -> [u8; 20] {
    let mut h = Ripemd160::new();

    for &chunk in chunks {
        h.update(chunk);
    }

    let mut out = [0u8; 20];
    h.finalize(&mut out);
    out
}
```

If you want to reuse the same hasher instance for multiple messages:

```rust
let mut h = Ripemd160::new();

h.update(b"message one");
let mut out1 = [0u8; 20];
h.finalize(&mut out1);

h.reset();
h.update(b"message two");
let mut out2 = [0u8; 20];
h.finalize(&mut out2);
```

---

## API overview

### `struct Ripemd160`

```rust
pub struct Ripemd160  {
    s:     [u32; 5],   // internal state words (5 × 32 bits = 160 bits)
    buf:   [u8; 64],   // partial chunk buffer
    bytes: u64,        // total bytes ingested
}
```

Key methods:

- `Ripemd160::new() -> Self`
  - Constructs a new hasher with the standard RIPEMD‑160 IV.
- `Ripemd160::default() -> Self`
  - Alias for `new`, also initializes the state via `ripemd160_initialize`.
- `fn update(&mut self, data: &[u8]) -> &mut Self`
  - High‑level, safe ingestion of arbitrary byte slices.
  - Internally forwards to `write` and handles chunking into 64‑byte blocks.
- `fn finalize(&mut self, hash: &mut [u8; RIPEMD160_OUTPUT_SIZE])`
  - Completes the hash computation, writes the 20‑byte digest into `hash`.
  - Performs RIPEMD‑160 padding: appends `0x80`, then zeros, then the 64‑bit little‑endian bit length.
  - Encodes the final 5 × 32‑bit state words in little‑endian order into the caller buffer.
- `fn reset(&mut self) -> &mut Self`
  - Zeroes the byte counter and reinitializes the internal state to the RIPEMD‑160 IV, allowing reuse.

There is also a low‑level pointer interface:

- `fn write(&mut self, data: *const u8, len: usize) -> &mut Self`
  - Unsafe, but tightly aligned with the internal buffering and transform logic.
  - Intended for performance‑sensitive code that already manipulates raw pointers.

Generated accessors (via `getset`):

- `s()`, `s_mut()`: internal state array
- `buf()`, `buf_mut()`: internal buffer
- `bytes()`, `bytes_mut()`: total byte counter

These are mainly useful for diagnostics, testing or integrating with other low‑level primitives.

---

## Cryptographic core

RIPEMD‑160 is a Merkle–Damgård hash function operating on 512‑bit message blocks and maintaining a 160‑bit internal state. Each block is processed through five rounds of non‑linear mixing in *two parallel lanes*, followed by a feed‑forward with the previous state.

This crate exposes the internal round primitives explicitly, which is invaluable for verification and optimization work:

### Boolean round functions

```rust
pub fn ripemd160_f1(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
pub fn ripemd160_f2(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!x & z) }
pub fn ripemd160_f3(x: u32, y: u32, z: u32) -> u32 { (x | !y) ^ z }
pub fn ripemd160_f4(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !z) }
pub fn ripemd160_f5(x: u32, y: u32, z: u32) -> u32 { x ^ (y | !z) }
```

These are the five canonical RIPEMD‑160 boolean functions. Each round uses one of them together with:

- a message word `x`
- a round constant `k`
- a rotation amount `r`

### One-round update

```rust
pub fn ripemd160_round(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    f: u32,
    x: u32,
    k: u32,
    r: i32,
)
```

Mathematically, this performs (in 32‑bit arithmetic):

\[
\begin{aligned}
A' &= \mathrm{ROL}_{r}(A + f + x + k) + E \\
C' &= \mathrm{ROL}_{10}(C)
\end{aligned}
\]

where `ROL` is a left rotation modulo 2³², implemented by `ripemd160_rol`.

### Per‑round wrappers

The functions `ripemd160_rXY` (e.g., `ripemd160_r11`, `ripemd160_r22`, `ripemd160_r51`, etc.) bind in the appropriate boolean function, round constant, and rotation schedule for each phase. Their signatures are homogeneous:

```rust
pub fn ripemd160_r11(
    a: &mut u32,
    b: u32,
    c: &mut u32,
    d: u32,
    e: u32,
    x: u32,
    r: i32,
)
```

Each wrapper computes the boolean function `f` for that round and delegates to `ripemd160_round` with its fixed constant `k`. This design keeps the top‑level compression loop (`ripemd160_transform`) legible while still being amenable to inlining and constant propagation.

### Compression function

```rust
pub fn ripemd160_transform(s: *mut u32, chunk: *const u8)
```

- Expects five 32‑bit state words at `s` and a 64‑byte block at `chunk`.
- Loads sixteen little‑endian message words `w0…w15`.
- Initializes two parallel lanes `(a1…e1)` and `(a2…e2)` from the current state.
- Executes all 80 rounds in the canonical RIPEMD‑160 schedule using the `Rxy` helpers.
- Performs the final feed‑forward, mixing both lanes back into `s[0..5]`.

This function is the exact Merkle–Damgård compression primitive; it is exposed for specialized applications that need tighter control or independent verification.

### State initialization

```rust
pub fn ripemd160_initialize(s: *mut u32)
```

Writes the standard RIPEMD‑160 IV into `s[0..5]`:

```text
H0 = 0x67452301
H1 = 0xEFCDAB89
H2 = 0x98BADCFE
H3 = 0x10325476
H4 = 0xC3D2E1F0
```

These constants mirror those of SHA‑1 up to endianness nuances and are part of the RIPEMD‑160 specification.

---

## Tracing and observability

The implementation is instrumented with `tracing` calls at various granularities:

- `trace!` in boolean functions, rotation, and per‑round helpers
- `debug!` in `ripemd160_transform`
- `info!` in initialization, default construction, reset, and finalize paths

To exploit this, configure a `tracing` subscriber in your binary or test harness. For example:

```rust
use tracing_subscriber::EnvFilter;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // now use Ripemd160; internal state transitions will be logged
}
```

You can then enable detailed logs for a specific module, such as:

```bash
RUST_LOG=debug,ripemd160=trace cargo test
```

This makes the crate useful not only as a hashing library but also as an educational and diagnostic tool when studying RIPEMD‑160.

---

## Safety and low-level behavior

- The `update` API is fully safe and should be preferred for regular application code.
- The pointer‑based `write`, `ripemd160_transform`, and `ripemd160_initialize` APIs are `unsafe` and assume that the pointers are valid and aligned for the required sizes.
- All internal arithmetic is performed modulo 2³² using `wrapping_add` and bit rotations, as required for deterministic RIPEMD‑160 semantics.

If you are integrating this crate into constant‑time critical code, note that RIPEMD‑160 is largely constructed from data‑independent operations (bitwise logic, rotations, and modular additions). However, the usual caveats apply: host/compiler behavior, logging side‑channels, and surrounding protocol choices still matter.

---

## Interoperability notes

- Output size is always 20 bytes (`RIPEMD160_OUTPUT_SIZE`), encoded **little‑endian** word by word.
- Bitcoin's `HASH160` (RIPEMD‑160(SHA‑256(x))) is supported by composing this crate with a SHA‑256 implementation; this crate intentionally focuses only on RIPEMD‑160.
- The implementation is designed to be bit‑for‑bit compatible with the widely deployed RIPEMD‑160 reference algorithm.

---

## Repository and maintenance

This crate is developed and maintained within the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) repository and is versioned as an independent crate (`bitcoin-ripemd`). Issues and pull requests should be filed against that repository.

---

## License

This crate is licensed under the MIT license, matching the rest of `bitcoin-rs`.

You are free to use it in proprietary or open‑source software, subject to the terms of the MIT license.
