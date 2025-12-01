# bitcoin-siphash

Bitcoin-Core-compatible SipHash-2-4 primitives and utilities, factored out as a standalone Rust crate.

This crate exposes:

- Low-level macros mirroring Bitcoin Core's C++ macros:
  - `rotl!` – 64-bit rotate-left.
  - `sipround!` – a single SipHash compression round (`SIPROUND`).
- A streaming SipHash-2-4 state machine:
  - `BitcoinSipHasher` – incremental, constant-time, branch-free where it matters.
- Two optimized helpers for hashing 256-bit values (Bitcoin-style `uint256`):
  - `sip_hash_uint256` – SipHash-2-4 over a 256-bit value.
  - `sip_hash_uint_256extra` – SipHash-2-4 over a 256-bit value plus an additional 32-bit tag.
- A `rotl64` helper as a `const fn` for compile-time evaluation.

All algorithms are direct, structurally faithful translations of the Bitcoin Core reference implementation and are intended for consensus-critical and index-key use in Bitcoin-related code.

---

## Background: SipHash-2-4 in Bitcoin

SipHash is a family of keyed pseudorandom functions over byte strings, designed by Aumasson and Bernstein. `SipHash-2-4` refers to the parameterization with 2 compression rounds per message block and 4 finalization rounds. It is used in Bitcoin Core for structured hashing tasks such as hash table keys, short transaction IDs, and other non-cryptographic but DoS-sensitive indexing scenarios.

Bitcoin treats SipHash as a keyed PRF, not as a collision-resistant hash for consensus content. Security properties that matter here:

- **Keyed**: Protects against adversarially chosen inputs targeting hash table collisions.
- **Constant-time and branch-free** in the hot path: reduces side channels and improves predictability.
- **Deterministic bit-exact behavior**: necessary for consensus-adjacent indexing and compatibility with existing nodes.

This crate aims to reproduce that behavior exactly, but in idiomatic Rust, enabling interoperability with Bitcoin Core's C++ implementation.

---

## Crate goals and scope

- Provide **bit-for-bit compatibility** with Bitcoin Core's SipHash usage.
- Expose **low-level building blocks** for crates that need to implement Bitcoin's hash-indexing logic.
- Preserve **macro-level semantics** (`ROTL`, `SIPROUND`) while restoring Rust macro hygiene.
- Avoid unnecessary abstraction layers so that **generated assembly remains close to the reference**.

This crate does *not* attempt to be a general-purpose cryptographic library; it focuses on Bitcoin-style usages.

---

## Features at a glance

- `rotl!` – macro for `rotate_left` with wrap-around semantics.
- `sipround!` – a single SipHash compression round; logs state transitions via `tracing` when enabled.
- `rotl64(x, b)` – `const fn` rotate-left useful in const contexts.
- `BitcoinSipHasher` – streaming SipHash-2-4 state:
  - `new(k0, k1)` – initialize state with a 128-bit key `(k0, k1)`.
  - `write(&mut self, data: &[u8])` – append arbitrary bytes.
  - `write_u64(&mut self, data: u64)` – append exactly 8 bytes in little-endian order, with alignment checking.
  - `finalize(&self) -> u64` – non-destructive finalization (does not mutate `self`).
- `sip_hash_uint256(k0, k1, &u256)` – optimized SipHash-2-4 for a 256-bit value.
- `sip_hash_uint_256extra(k0, k1, &u256, extra: u32)` – optimized SipHash-2-4 for a 256-bit value plus a 32-bit tag.

---

## Dependency surface

- **Rust edition:** 2021
- **License:** MIT
- **Core dependencies:**
  - `tracing` for optional debug/trace instrumentation.
  - Derive macros `CopyGetters`, `MutGetters`, `Getters` (from `derive-getters` or equivalent), used to generate ergonomic accessors for `BitcoinSipHasher`'s internal state.
  - A `u256` type from a companion crate (for example, the `bitcoin-rs` project). This crate assumes a `u256` with a method `get_uint64(i: usize) -> u64` providing little-endian 64-bit limbs.

You are expected to supply a `u256` implementation that matches this interface if you directly use `sip_hash_uint256` or `sip_hash_uint_256extra`.

---

## Usage

### 1. Basic keyed SipHash over arbitrary bytes

```rust
use bitcoin_siphash::BitcoinSipHasher;

fn keyed_hash(key0: u64, key1: u64, msg: &[u8]) -> u64 {
    let mut hasher = BitcoinSipHasher::new(key0, key1);
    hasher.write(msg);
    hasher.finalize()
}
```

This uses the streaming interface with the exact SipHash-2-4 constant/round structure used in Bitcoin Core.

### 2. Block-aligned `u64` streaming

For callers that naturally process 64-bit words, using `write_u64` is more direct:

```rust
use bitcoin_siphash::BitcoinSipHasher;

fn hash_u64_blocks(key0: u64, key1: u64, blocks: &[u64]) -> u64 {
    let mut h = BitcoinSipHasher::new(key0, key1);

    for &w in blocks {
        // Precondition: current count is a multiple of 8 bytes.
        h.write_u64(w);
    }

    h.finalize()
}
```

Internally, `write_u64` asserts that the byte count is aligned to 8 bytes. This mirrors Bitcoin Core's assumption that certain usages are limb-aligned.

### 3. Hashing a 256-bit value (Bitcoin-style `uint256`)

The helper `sip_hash_uint256` is equivalent to constructing a `BitcoinSipHasher`, writing the four 64-bit limbs, and finalizing:

```rust
use bitcoin_siphash::sip_hash_uint256;
use your_uint256_crate::u256; // you supply this

fn short_id(key0: u64, key1: u64, txid: &u256) -> u64 {
    sip_hash_uint256(key0, key1, txid)
}
```

Conceptually, this is:

```rust
// Pseudocode equivalence
let h = BitcoinSipHasher::new(k0, k1)
    .write_u64(val.get_uint64(0))
    .write_u64(val.get_uint64(1))
    .write_u64(val.get_uint64(2))
    .write_u64(val.get_uint64(3));
let out = h.finalize();
```

But implemented as a single optimized routine with fewer temporaries and branches.

### 4. Hashing a 256-bit value plus a 32-bit tag

`SipHashUint256Extra` from Bitcoin Core is translated as `sip_hash_uint_256extra` here. It treats the data as 32 bytes (`u256`) plus a 4-byte tag, for a total logical length of 36 bytes:

```rust
use bitcoin_siphash::sip_hash_uint_256extra;
use your_uint256_crate::u256;

fn hash_txid_with_tag(k0: u64, k1: u64, id: &u256, extra: u32) -> u64 {
    sip_hash_uint_256extra(k0, k1, id, extra)
}
```

Internally, the length encoding and finalization steps are kept exactly as in the Bitcoin Core C++ implementation (including the `(36u64 << 56)` length tag and the standard `v2 ^= 0xFF` finalization pattern).

---

## Low-level macros: `rotl!` and `sipround!`

### `rotl!`

```rust
use bitcoin_siphash::rotl;

let x: u64 = 0x0123_4567_89ab_cdef;
let r: u32 = 13;
let y: u64 = rotl!(x, r);
assert_eq!(y, x.rotate_left(r));
```

`rotl!` is kept close in spirit to Bitcoin Core's `ROTL` macro, but leverages Rust's intrinsic `rotate_left` on integer types.

### `sipround!`

`sipround!` is the core compression round of SipHash. It operates on four mutable `u64` variables, which are passed by identifier. This design matches the original C++ macro but is macro-hygienic:

```rust
use bitcoin_siphash::sipround;

let (mut v0, mut v1, mut v2, mut v3) = (1u64, 2, 3, 4);

sipround!(v0, v1, v2, v3);
// v0..v3 now contain the transformed state.
```

When the `tracing` subscriber is enabled at trace level, entry and exit states of every `sipround!` invocation are logged. This is helpful for implementation verification and bit-level debugging against Bitcoin Core's reference output.

---

## `rotl64` helper

```rust
use bitcoin_siphash::rotl64;

const X: u64 = 0x0123_4567_89ab_cdef;
const R: u32 = 32;
const Y: u64 = rotl64(X, R);

// Use Y in other const contexts.
```

`rotl64` is a `const fn` rotate-left implementation, deliberately written with explicit shifts:

```rust
pub fn rotl64(x: u64, b: u32) -> u64 {
    (x << b) | (x >> (64 - b))
}
```

This allows the compiler to evaluate rotations at compile time without depending on macro expansion.

---

## Design notes and invariants

### Streaming state (`BitcoinSipHasher`)

`BitcoinSipHasher` keeps the standard SipHash internal state:

- `v: [u64; 4]` – the four SipHash state words, keyed during `new(k0, k1)`.
- `tmp: u64` – buffer for partial 64-bit blocks constructed byte-by-byte.
- `count: u8` – total number of bytes absorbed so far modulo 256.

Key invariants:

- After any call to `write_u64`, `count % 8 == 0` (enforced by `assert_eq!`).
- `write` packs bytes into `tmp` little-endian, consuming a full block when `count & 7 == 0`.
- `finalize` computes the final 64-bit result by:
  1. Encoding the total byte count into the high 8 bits of the last partial word: `tmp | ((count as u64) << 56)`.
  2. Injecting that word into the state (`v3 ^= t; ... v0 ^= t`).
  3. Applying the standard `v2 ^= 0xFF` finalization constant and 4 SipRounds.
  4. XOR-folding `v0 ^ v1 ^ v2 ^ v3`.

`finalize(&self)` is **non-destructive**: you may call it multiple times on the same state to recompute the digest without mutating the underlying state. This is different from some streaming interfaces that finalize by consuming.

### Constant-time and branch behavior

The core block processing (`sipround!` and its use in `sip_hash_uint256` and `sip_hash_uint_256extra`) is branch-free with respect to secret data; loops in `write` may depend on input length but not on input value. This is consistent with the expectations for SipHash in protocol-adjacent contexts.

---

## Integration with `u256`

The crate assumes a `u256` with the following minimal interface:

```rust
trait U256Like {
    fn get_uint64(&self, i: usize) -> u64; // little-endian limb access: 0..=3
}
```

The `bitcoin-rs` repository (https://github.com/klebs6/bitcoin-rs) likely provides such a type. If you use a custom `u256`, you can write a small adapter or implement the same method signatures.

The order of limbs and the endianness are consensus-relevant if you are replicating Bitcoin Core behavior; ensure your `u256` matches Core's layout.

---

## Logging and diagnostics

The crate uses `tracing` macros (`trace!`, `debug!`) to instrument:

- Hasher initialization (`BitcoinSipHasher::new`).
- `write_u64` input words.
- Entry/exit states of `sipround!` for both streaming use and compact helpers.
- Final output values of the high-level SipHash helpers.

To use this effectively, configure a subscriber with at least `Level::TRACE` in your application or tests. For performance-critical production builds where logs are not needed, compile with logging disabled or at a higher severity threshold.

---

## Example: verifying against Bitcoin Core

A robust way to validate compatibility is to compare outputs with the C++ reference implementation. One pattern is:

1. Choose fixed keys `(k0, k1)` and random `uint256` values.
2. Compute `SipHashUint256` / `SipHashUint256Extra` in C++.
3. Compute `sip_hash_uint256` / `sip_hash_uint_256extra` in Rust.
4. Assert equality for many test vectors.

With tracing enabled, you can also log intermediate states and compare per-round values.

---

## Safety, `no_std`, and environment assumptions

- This crate is currently written assuming `std` is available (for logging infrastructure and potentially for dependencies). With minor adjustments (not documented here), it could be made `no_std` + `alloc`-friendly if the dependency stack permits.
- All arithmetic uses wrapping semantics (`wrapping_add`, shifts, and rotates) to match the original C++ code paths and to avoid undefined behavior.

---

## Repository and maintenance

- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **Crate:** `bitcoin-siphash` (this component of the broader `bitcoin-rs` ecosystem)
- **License:** MIT

If you rely on this crate for consensus-adjacent code or production infrastructure, you should:

- Vendor and pin exact versions (e.g., via `Cargo.lock` and reproducible builds).
- Maintain your own test vectors and CI that cross-check against Bitcoin Core.

---
