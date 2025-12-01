# bitcoin-hmac-sha256

A minimal, allocation-free implementation of HMAC-SHA-256 and RFC 6979 deterministic nonces tailored for Bitcoin-style cryptography and FFI use.

---

## Overview

This crate exposes two tightly scoped primitives:

- `HmacSha256` – a streaming HMAC-SHA-256 construction over an internal `Sha256` implementation.
- `Rfc6979HmacSha256` – a deterministic pseudo-random generator implementing RFC 6979 §3.2 using HMAC-SHA-256.

The primary design goals are:

- **Exact behavioral compatibility** with common C/C++-style Bitcoin cryptography code.
- **FFI-friendly** pointer-based interfaces that mirror C function signatures.
- **No heap allocation**, predictable stack usage, and fixed-size state.
- **Deterministic ECDSA nonce generation** for signing, via RFC 6979.

This is a low‑level crate intended for cryptographic libraries and bindings, not for general application-level hashing.

---

## Cryptographic background

### HMAC-SHA-256

HMAC (Hash-based Message Authentication Code) is defined as

\[
\mathrm{HMAC}_k(m) = H\Big((k' \oplus opad) \,\Vert\, H((k' \oplus ipad) \,\Vert\, m)\Big),
\]

where

- `H` is SHA-256,
- `k'` is the key normalized to the block size of SHA-256 (64 bytes), either by zero-padding or hashing and then zero-padding,
- `ipad = 0x36` repeated 64 times,
- `opad = 0x5c` repeated 64 times.

`HmacSha256` implements exactly this construction with an inner and outer `Sha256` state.

### RFC 6979 deterministic nonces

RFC 6979 describes how to derive deterministic ECDSA nonces from a secret key and message hash using an HMAC-based DRBG. This crate:

- Implements the **HMAC-DRBG** procedure from RFC 6979 §3.2 using HMAC-SHA-256.
- Maintains internal state `(K, V, retry)` in `Rfc6979HmacSha256`.
- Provides `initialize`, `generate`, and `finalize` procedures via C-friendly functions.

This is useful in contexts where you must avoid reliance on ambient randomness and instead derive nonces purely from secret key material and the hashed message (e.g., Bitcoin ECDSA signatures).

---

## Features and non-goals

### Features

- HMAC-SHA-256 with a 32‑byte output.
- Construct from raw key bytes (`*const u8`, length) or safe slices.
- Streaming API: multiple `write` / `write_ref` calls before `finalize_into`.
- RFC 6979 HMAC-based DRBG with explicit state.
- C-compatible FFI functions for both primitives.
- No allocations, fixed-size internal buffers.

### Non-goals

- No high-level key management or secret zeroization beyond what is explicitly implemented.
- No asynchronous or multithread-safe abstractions by default.
- No generic digest abstraction; this crate is specifically about HMAC-SHA-256.

---

## Safety and FFI model

The crate provides two layers of API:

1. **Safe Rust methods** on the Rust types (`HmacSha256`, `Rfc6979HmacSha256`).
2. **`unsafe` FFI-style functions** that operate on raw pointers, mirroring the original C implementation.

The FFI functions are `unsafe` from the caller’s perspective because they assume:

- All passed pointers are valid for reads/writes of the corresponding length.
- Lifetimes of the pointed-to data exceed the call duration.
- The output buffers are at least 32 bytes where required.

If you can, prefer the safe methods; use the FFI-style functions only when integrating with C or C++.

---

## Types and functions

### `HmacSha256`

```rust
#[derive(Getters, MutGetters, Setters)]
pub struct HmacSha256 {
    outer: Sha256,
    inner: Sha256,
}
```

#### Construction

```rust
impl HmacSha256 {
    /// Construct from a raw pointer + length (FFI-style, unsafe boundary).
    pub fn new(key: *const u8, keylen: usize) -> Self { ... }

    /// Construct from a Rust byte slice (preferred).
    #[inline]
    pub fn new_with_key(key: &[u8]) -> Self { ... }

    /// Construct directly from a slice.
    pub fn from_slice(key: &[u8]) -> Self { ... }
}
```

`from_slice` implements the standard HMAC key normalization:

- If `key.len() <= 64`, it copies the key into a 64-byte array, zero-padding on the right.
- If `key.len() > 64`, it hashes the key with `Sha256`, places the 32-byte digest into the beginning of the 64-byte array, and zero-pads the rest.
- It then derives the `ipad` and `opad` states and initializes the inner and outer `Sha256` contexts.

#### Writing and finalizing

```rust
impl HmacSha256 {
    /// Write data from a raw pointer + length (unsafe boundary).
    pub fn write(&mut self, data: *const u8, len: usize) -> &mut HmacSha256 { ... }

    /// Write data from a byte slice (safe, preferred).
    #[inline]
    pub fn write_ref(&mut self, data: &[u8]) -> &mut HmacSha256 { ... }

    /// Finalize into the provided 32-byte output buffer.
    pub fn finalize_into(&mut self, out: &mut [u8; HMAC_SHA256_OUTPUT_SIZE]) { ... }

    /// Legacy finalize signature, takes `hash` by value.
    pub fn finalize(&mut self, mut hash: [u8; HMAC_SHA256_OUTPUT_SIZE]) { ... }
}
```

Usage pattern:

```rust
use bitcoin_hmac_sha256::HmacSha256;

const HMAC_SHA256_OUTPUT_SIZE: usize = 32;

let key = b"my hmac key";
let msg1 = b"hello";
let msg2 = b" world";

let mut hmac = HmacSha256::new_with_key(key);
hmac.write_ref(msg1)
    .write_ref(msg2);

let mut tag = [0u8; HMAC_SHA256_OUTPUT_SIZE];
hmac.finalize_into(&mut tag);

// `tag` now holds HMAC-SHA-256(key, msg1 || msg2)
```

### `Rfc6979HmacSha256`

```rust
#[derive(Builder, Getters, Setters, MutGetters)]
pub struct Rfc6979HmacSha256 {
    v:     [u8; 32],
    k:     [u8; 32],
    retry: i32,
}
```

This struct implements the internal state of the RFC 6979 HMAC-DRBG with SHA-256.

#### Initialization

```rust
pub fn rfc6979_hmac_sha256_initialize(
    rng: *mut Rfc6979HmacSha256,
    key: *const u8,
    keylen: usize,
) { ... }
```

This function performs RFC 6979 §3.2 steps (b)–(f):

- Sets `V` to 0x01 repeated.
- Sets `K` to 0x00 repeated.
- Mixes in the `key` via a sequence of HMAC invocations (`K`, `V`, and the input key), using the `HmacSha256` primitive.
- Sets `retry` to `0`.

The `key` argument should be the RFC 6979 “key material” – usually the concatenation of the private key and hashed message, but that combination is handled by higher-level code.

#### Generation

```rust
pub fn rfc6979_hmac_sha256_generate(
    rng: *mut Rfc6979HmacSha256,
    mut out: *mut u8,
    mut outlen: usize,
) { ... }
```

This function implements RFC 6979 §3.2(h):

- Optionally performs an additional update step when `retry != 0`.
- Iteratively HMACs the internal `V` value to produce pseudo-random output.
- Writes exactly `outlen` bytes into `out`, possibly across multiple 32-byte blocks.
- Sets `retry = 1` to mark that subsequent calls should do the "extra" update.

Typical use in ECDSA:

1. Call `rfc6979_hmac_sha256_initialize` with the concatenated key material.
2. Call `rfc6979_hmac_sha256_generate` to obtain enough bytes for a candidate scalar `k`.
3. Reduce modulo group order, check validity; if rejected, call `generate` again.

#### Finalization

```rust
pub fn rfc6979_hmac_sha256_finalize(rng: *mut Rfc6979HmacSha256) { ... }
```

This function wipes `K` and `V` with zeros and resets `retry` to `0`. You should call this once you are done with the nonce generator to reduce key material lifetime in memory.

---

## FFI-style HMAC functions

The crate also provides C-like functions that operate on `HmacSha256` via raw pointers. These are useful when exposing the crate to C or C++ or when preserving existing foreign interfaces.

```rust
pub fn hmac_sha256_initialize(
    hash: *mut HmacSha256,
    key: *const u8,
    keylen: usize,
) { ... }

pub fn hmac_sha256_write(
    hash: *mut HmacSha256,
    data: *const u8,
    size: usize,
) { ... }

pub fn hmac_sha256_finalize(
    hash: *mut HmacSha256,
    out32: *mut u8,
) { ... }
```

### Example (FFI-like, unsafe)

```rust
use bitcoin_hmac_sha256::{
    HmacSha256,
    hmac_sha256_initialize,
    hmac_sha256_write,
    hmac_sha256_finalize,
};

unsafe {
    let key = b"secret";
    let data = b"message";

    // Memory owned on Rust side; passed to FFI-style API
    let mut ctx: HmacSha256 = core::mem::zeroed();
    let mut out = [0u8; 32];

    hmac_sha256_initialize(&mut ctx, key.as_ptr(), key.len());
    hmac_sha256_write(&mut ctx, data.as_ptr(), data.len());
    hmac_sha256_finalize(&mut ctx, out.as_mut_ptr());

    // `out` now contains the HMAC tag
}
```

---

## Usage with Cargo

Add the dependency:

```toml
[dependencies]
bitcoin-hmac-sha256 = "0.1.2"
```

The crate is licensed under MIT and targets the Rust **2024** edition.

---

## Security considerations

- This crate provides **low-level primitives** only. It is not a full cryptographic subsystem.
- It does not attempt to provide constant-time behavior across all operations beyond what direct, branch-minimal code implies. Review for your threat model.
- Always treat keys and RFC 6979 state as sensitive. Use `rfc6979_hmac_sha256_finalize` to wipe DRBG state once done.
- Do not reuse RFC 6979 key material across different algorithms or curves without a formal derivation.

For production systems, you should layer these primitives beneath a rigorously audited signing and key-handling framework.

---

## Testing and interoperability

For practical use, you should:

- Verify HMAC-SHA-256 outputs against known-good test vectors (e.g., RFC 4231).
- Verify RFC 6979 nonce streams against reference implementations for the same curve and key material.
- Confirm that behavior matches the original C or C++ implementation you are migrating from, bit-for-bit.

---

## License

This crate is distributed under the **MIT** license. See your `Cargo.toml` or accompanying license file for details.
