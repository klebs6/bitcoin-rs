# bitcoin-hmac-sha512

A minimal, FFI‑friendly HMAC‑SHA‑512 implementation tailored for Bitcoin-style cryptographic workflows.

This crate exposes a low-level `HmacSha512` type that:

- Implements **HMAC** (Keyed-Hash Message Authentication Code) using **SHA‑512** as specified in [RFC 2104] and [FIPS 198-1].
- Uses raw pointers (`*const u8`) and explicit lengths for data and keys.
- Avoids heap allocations and operates entirely on fixed-size stack buffers.
- Targets use in performance‑sensitive or foreign‑language (C/C++) bindings scenarios.

[RFC 2104]: https://datatracker.ietf.org/doc/html/rfc2104
[FIPS 198-1]: https://csrc.nist.gov/publications/detail/fips/198/1/final

---

## Core API

```rust
pub struct HmacSha512  {
    outer: Sha512,
    inner: Sha512,
}

impl HmacSha512 {
    pub fn new(key: *const u8, keylen: usize) -> Self;

    pub fn write(&mut self, data: *const u8, len: usize) -> &mut HmacSha512;

    pub fn finalize(&mut self, hash: &mut [u8; HMAC_SHA512_OUTPUT_SIZE]);

    pub fn finalize_to_array(self) -> [u8; 64];
}
```

`HMAC_SHA512_OUTPUT_SIZE` is expected to be `64` (the SHA‑512 digest size in bytes).

### Construction

```rust
use bitcoin_hmac_sha512::HmacSha512;

// Example key and message
let key: [u8; 32] = [0x11; 32];
let msg: [u8; 16] = [0x22; 16];

let mut hmac = unsafe { HmacSha512::new(key.as_ptr(), key.len()) };
```

Semantics of `new`:

- If `keylen <= 128` (one SHA‑512 block):
  - The key is copied into a 128‑byte buffer and zero‑padded.
- If `keylen > 128`:
  - The key is first hashed with SHA‑512 (producing 64 bytes), then that digest is zero‑padded to 128 bytes.
- From this **block-sized key** `rkey`, the inner/outer pads are created:
  - `outer_key = rkey ^ 0x5c`
  - `inner_key = rkey ^ 0x36` (via `rkey ^ (0x5c ^ 0x36)` in the code)
- Two internal `Sha512` states are initialized by absorbing these pads.

This is the canonical HMAC key-derivation procedure.

### Incremental input (`write`)

```rust
unsafe {
    hmac.write(msg.as_ptr(), msg.len());
}

// Additional chunks may follow; the state is streaming.
```

Properties:

- `write` feeds bytes into the **inner** SHA‑512 state.
- Multiple calls append data logically: `write(a); write(b)` ≡ `write(a || b)`.
- Returns `&mut HmacSha512` to allow chaining.

Example:

```rust
unsafe {
    HmacSha512::new(key.as_ptr(), key.len())
        .write(b"hello".as_ptr(), 5)
        .write(b" world".as_ptr(), 6);
}
```

### Finalization

There are two ways to obtain the final 64‑byte MAC.

#### 1. `finalize(&mut self, hash: &mut [u8; 64])`

```rust
let mut hmac = unsafe { HmacSha512::new(key.as_ptr(), key.len()) };
unsafe { hmac.write(msg.as_ptr(), msg.len()); }

let mut out = [0u8; 64];
hmac.finalize(&mut out);

// `out` now contains the HMAC-SHA-512 tag.
```

Mechanics:

1. The inner `Sha512` is finalized into a 64‑byte temporary buffer.
2. That inner digest is then fed into the outer `Sha512` state.
3. The outer state is finalized into the user‑provided buffer `hash`.

This matches the formal HMAC definition:

\[
\operatorname{HMAC}_k(m) = H\bigl((k'\oplus opad) \; || \; H((k'\oplus ipad) || m)\bigr)
\]

where `H = SHA‑512` and `k'` is the block‑sized key derived from `k`.

#### 2. `finalize_to_array(self) -> [u8; 64]`

```rust
let tag: [u8; 64] = unsafe {
    HmacSha512::new(key.as_ptr(), key.len())
        .write(msg.as_ptr(), msg.len())
        .finalize_to_array()
};
```

This convenience method consumes `self`, internally calls `finalize`, and returns the 64‑byte array by value.

---

## Safety and Usage Constraints

This crate is intentionally low‑level and uses raw pointers for maximum interop and minimal abstraction.

### `unsafe` requirements

All of the following must hold whenever you call functions that take pointers:

- `key` and `data` pointers must be **valid** for reads of `keylen`/`len` bytes.
- The memory they point to must be **properly aligned** for `u8` (trivially true for standard slices/arrays).
- The buffers must be **non‑overlapping** with internal state in ways assumed by `ptr::copy_nonoverlapping`.
- They must remain valid for the full duration of the call.

Therefore, you will usually write:

```rust
let key: Vec<u8> = obtain_key();
let msg: Vec<u8> = obtain_msg();

let mut hmac = unsafe { HmacSha512::new(key.as_ptr(), key.len()) };
unsafe { hmac.write(msg.as_ptr(), msg.len()); }

let mut out = [0u8; 64];
hmac.finalize(&mut out);
```

### Thread safety

`HmacSha512` instances are not intrinsically synchronized. Treat them as single‑threaded objects: one instance per logical computation, no concurrent mutation across threads without external synchronization.

### Zeroization and key erasure

The snippet does not explicitly zeroize:

- The internal key buffer `rkey` after initialization.
- Intermediate state in `Sha512` after use.

If you need strict key erasure semantics (e.g., side‑channel‑aware zeroization), verify the implementation of `Sha512` and consider wrapping usage with a secure‑memory abstraction.

---

## Intended Use Cases

- **Bitcoin / cryptocurrency libraries** that already depend on SHA‑512 internally and want a compact HMAC adaptor.
- **FFI bindings** where the host language (C/C++/Go/others) holds raw buffers and lengths and calls into Rust.
- **Deterministic key derivation** or message authentication where SHA‑512 is mandated by protocol.

Notably, HMAC-SHA-512 is ubiquitous in:

- HD wallet key derivation procedures (e.g., BIP32 uses HMAC-SHA-512 under a fixed key for chain code generation).
- MAC layers in bespoke protocols that rely on 512‑bit security margins.

---

## Mathematical Background

HMAC is defined as:

\[
\operatorname{HMAC}_k(m) = H\bigl((k'\oplus opad) \; || \; H((k'\oplus ipad) || m)\bigr)
\]

- `H` is a cryptographic hash function (here: SHA‑512).
- `k` is the secret key; `k'` is `k` either truncated or hashed and then padded to the block size `B=128` bytes.
- `ipad` is the byte `0x36` repeated `B` times.
- `opad` is the byte `0x5c` repeated `B` times.

This construction provides:

- **PRF** (pseudorandom function) behavior assuming the underlying hash behaves as a PRF on its compression function.
- **Message authentication** and **integrity** when the key is secret.

The implementation in this crate follows the standard two‑pass formulation using two `Sha512` instances and XOR padding.

---

## Example: Constant‑time tag comparison

When verifying an HMAC tag, always compare in constant time to avoid timing side channels.

```rust
fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut acc = 0u8;
    for (&x, &y) in a.iter().zip(b.iter()) {
        acc |= x ^ y;
    }
    acc == 0
}

fn verify_tag(key: &[u8], msg: &[u8], expected: &[u8; 64]) -> bool {
    let mut out = [0u8; 64];
    let mut hmac = unsafe { HmacSha512::new(key.as_ptr(), key.len()) };
    unsafe { hmac.write(msg.as_ptr(), msg.len()); }
    hmac.finalize(&mut out);

    ct_eq(&out, expected)
}
```

You may wish to replace `ct_eq` with a formally audited constant‑time comparison from another crate when integrating into production systems.

---

## Integration Guidelines

- **No‑std**: From the given interface, the crate itself appears independent of heap allocation and most of `std`, but verify the manifest and any additional modules before assuming `#![no_std]` compatibility.
- **Versioning**: This documentation aligns with crate version `0.1.1` as indicated by the user. If using another version, confirm that the API matches.
- **License**: MIT, suitable for both open‑source and proprietary integration.

---
