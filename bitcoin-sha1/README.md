# bitcoin-sha1

A minimal, low-level SHA‑1 implementation tuned for the needs of `bitcoin-rs`.

This crate exposes a pointer-based, allocation-free SHA‑1 compression function and a thin stateful hasher suitable for environments where you want:

- **Tight control over memory layout and lifetimes** (raw pointers, no `std::io` traits, no heap allocations in the fast path).
- **Deterministic, standalone SHA‑1** without a large dependency stack.
- **Instrumentation-friendly internals** (round-level tracing in debug builds).

It is primarily intended as an internal building block within the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) project, but is usable as a general SHA‑1 primitive for other applications that need similar control.

---

## Cryptographic note (security)

SHA‑1 is *cryptographically broken* for collision resistance and must **not** be used for new security-sensitive designs (e.g., digital signatures, long-term content integrity, authentication tags).

Within Bitcoin, SHA‑1 may appear only in legacy or non‑security‑critical contexts. This implementation should therefore be regarded as a **compatibility / research / tooling** component, not as a modern cryptographic primitive.

If you need a secure hash function, use a SHA‑2 or SHA‑3 family implementation, or a modern construction such as BLAKE3.

---

## Features and design

- **Explicit state struct**: `Sha1` exposes its internal state array, buffer, and byte counter (via getters) for inspection and debugging.
- **Raw-pointer oriented API**: `write` accepts `*const u8` and a `len: usize` for zero-copy integration with existing memory management.
- **Pure Rust core**: Implements the SHA‑1 compression function (80‑round transform) directly in Rust, with explicit endianness handling.
- **Streaming interface**: Feed arbitrarily sized inputs incrementally, then finalize to yield a 20‑byte digest.
- **Deterministic padding and length encoding**: SHA‑1 padding logic is implemented explicitly; finalization writes the bit length as big‑endian `u64`.

The implementation closely follows the standard SHA‑1 structure:

- State vector \(H_0, \dots, H_4\) in `s: [u32; 5]`.
- 512‑bit blocks processed in `sha1_transform`.
- Message schedule `W_t` computed from the first 16 32‑bit words using the classic recurrence with rotate‑left by 1 (`sha1_left`).
- Boolean functions `f1`, `f2`, `f3` and phase‑dependent constants `K1..K4`.

---

## Minimum supported Rust version

- Rust **1.56+** is expected due to the 2021 edition, but this crate is developed with a modern toolchain. Refer to the `bitcoin-rs` repository if you require an explicit MSRV guarantee.

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-sha1 = "0.1.19"
```

Then import it as usual:

```rust
use bitcoin_sha1::Sha1;
```

---

## High-level usage

### Hashing a byte slice (via raw pointers)

The `Sha1` API is deliberately low‑level: `write` operates on a raw pointer and a length. For safe Rust slices, you typically wrap the call in an `unsafe` block:

```rust
use bitcoin_sha1::Sha1;

const SHA1_OUTPUT_SIZE: usize = 20; // from the crate

fn sha1_digest_bytes(input: &[u8]) -> [u8; SHA1_OUTPUT_SIZE] {
    let mut hasher = Sha1::new();

    unsafe {
        hasher.write(input.as_ptr(), input.len());
    }

    let mut out = [0u8; SHA1_OUTPUT_SIZE];
    hasher.finalize(&mut out);
    out
}

fn main() {
    let msg = b"hello world";
    let digest = sha1_digest_bytes(msg);
    println!("SHA1(\"hello world\") = {:02x?}", digest);
}
```

### Reusing a hasher

The `reset` method re-initializes the internal state using the same IV as `new`, without reallocating the struct:

```rust
use bitcoin_sha1::Sha1;

fn reuse_example() {
    let mut h = Sha1::new();

    unsafe { h.write(b"first".as_ptr(), 5); }
    let mut out1 = [0u8; bitcoin_sha1::SHA1_OUTPUT_SIZE];
    h.finalize(&mut out1);

    h.reset();

    unsafe { h.write(b"second".as_ptr(), 6); }
    let mut out2 = [0u8; bitcoin_sha1::SHA1_OUTPUT_SIZE];
    h.finalize(&mut out2);

    // out1 and out2 are independent digests
}
```

---

## Low-level primitives

The crate also exposes the internal building blocks of SHA‑1. These are primarily useful for:

- Implementing custom SHA‑1 variants.
- Analysing or instrumenting the compression function.
- Educational purposes.

### Boolean functions

These correspond to the typical SHA‑1 round functions:

```rust
use bitcoin_sha1::{sha1_f1, sha1_f2, sha1_f3};

let (b, c, d) = (0x1234_5678, 0x89ab_cdef, 0x0fed_cba9);
let f1 = sha1_f1(b, c, d); // used in rounds 0..=19
let f2 = sha1_f2(b, c, d); // used in rounds 20..=39 and 60..=79
let f3 = sha1_f3(b, c, d); // used in rounds 40..=59
```

Mathematically, these are:

- \(f_1(b,c,d) = d \oplus (b \wedge (c \oplus d))\)
- \(f_2(b,c,d) = b \oplus c \oplus d\)
- \(f_3(b,c,d) = (b \wedge c) \lor (d \wedge (b \lor c))\)

where `^` is XOR, `&` is AND, and `|` is OR.

### Single-round helper

`sha1_round` computes one round given the working variables and a schedule word:

```rust
use bitcoin_sha1::sha1_round;

let mut b = 0x0123_4567u32;
let mut e = 0x89ab_cdefu32;
let a = 0xfedc_ba98;
let c = 0x7654_3210;
let d = 0xf0e1_d2c3;
let f = 0x0bad_f00d;
let k = 0x5a82_7999;
let w = 0x1234_5678;

sha1_round(a, &mut b, c, d, &mut e, f, k, w);
// b and e have now been updated as per a SHA-1 round
```

This can be used to experiment with reduced-round SHA‑1 or alternative message schedules.

### Core compression function: `sha1_transform`

The central function is:

```rust
pub fn sha1_transform(state: *mut u32, chunk: *const u8)
```

- `state` points to an array of 5 big‑endian 32‑bit words (the running hash state).
- `chunk` points to a 64‑byte block of message data.

`sha1_transform` performs the full 80‑round compression on one 512‑bit block, mutating the `state` in place.

Usage pattern:

```rust
use bitcoin_sha1::{sha1_initialize, sha1_transform, SHA1_OUTPUT_SIZE};

fn manual_transform_example(input: &[u8; 64]) -> [u8; SHA1_OUTPUT_SIZE] {
    let mut s = [0u32; 5];

    unsafe {
        sha1_initialize(s.as_mut_ptr());
        sha1_transform(s.as_mut_ptr(), input.as_ptr());
    }

    let mut out = [0u8; SHA1_OUTPUT_SIZE];
    for (i, chunk) in out.chunks_exact_mut(4).enumerate() {
        chunk.copy_from_slice(&s[i].to_be_bytes());
    }
    out
}
```

### Endianness helpers

The crate provides small utilities to convert between raw pointers and big‑endian integers:

- `read_be32(p: *const u8) -> u32`
- `write_be32(p: *mut u8, v: u32)`
- `write_be64(p: *mut u8, v: u64)`

They are used internally by the compression and padding logic but are also exposed for callers that already work directly with raw buffers.

---

## The `Sha1` state machine

The `Sha1` struct encapsulates:

- `s: [u32; 5]` — the internal state words \(H_0..H_4\).
- `buf: [u8; 64]` — a partial block buffer.
- `bytes: u64` — the total number of **bytes** processed so far.

### Initialization

```rust
use bitcoin_sha1::Sha1;

let mut h = Sha1::new();
```

`Sha1::new`:

1. Constructs a zeroed `Sha1` via `Default`.
2. Calls `sha1_initialize` to set the standard SHA‑1 IV.

The IV corresponds to the classical SHA‑1 initial constants, stored as big‑endian words. (Always refer to the source code to check for any project-specific constants.)

### Incremental updates: `write`

```rust
impl Sha1 {
    pub fn write(&mut self, data: *const u8, len: usize) -> &mut Self;
}
```

`write` implements a classic streaming design:

1. Fill any partially filled buffer to 64 bytes; once full, call `sha1_transform`.
2. Process any further full 64‑byte chunks directly from `data` with no intermediate copying.
3. Copy any trailing bytes (< 64) into `buf` for the next call.

The internal `bytes` counter tracks the total number of bytes absorbed and is later used by `finalize` to encode the message length in bits.

### Finalization: `finalize`

```rust
impl Sha1 {
    pub fn finalize(&mut self, hash_out: &mut [u8; SHA1_OUTPUT_SIZE]);
}
```

`finalize` performs:

1. **Padding**: Writes a `0x80` byte, then enough zero bytes so that the total length (including the 8‑byte length field) is congruent to 0 modulo 64.
2. **Length encoding**: Appends an 8‑byte big‑endian representation of the bit length (`self.bytes << 3`).
3. **Final compression**: Uses `write` to push the padding and length into the usual streaming pipeline.
4. **Output**: Writes the big‑endian 32‑bit state words into `hash_out`.

You can call `finalize` multiple times on the same `Sha1` instance without resetting; it will recompute the digest from the current internal state and byte count. To start a fresh hash, call `reset`.

### Reset

```rust
impl Sha1 {
    pub fn reset(&mut self) -> &mut Self;
}
```

`reset` sets `bytes` back to 0 and re‑initializes the state with `sha1_initialize`, but retains the allocated buffers.

---

## Logging and debug behavior

The implementation uses logging macros like `trace!`, `debug!`, and `info!`:

- In debug builds, `sha1_transform` can log round boundaries every 20 rounds.
- `Sha1::new` and `Sha1::reset` log initialization events.

This requires a compatible logging facade to be set up in your application (e.g., `log` plus a specific logger implementation). If the macros resolve to no‑ops, the hash behavior is unaffected.

---

## Safety considerations

- The API exposes multiple `unsafe` functions and raw pointer parameters.
- Callers are responsible for ensuring that pointers are valid, correctly aligned (for the operations performed), and live for the duration of the call.
- Endianness conversions are implemented via bytewise operations and are portable across architectures.

When wrapping this crate in higher-level, safe abstractions, consider:

- Providing `fn update(&mut self, data: &[u8])` and `fn digest(&self) -> [u8; 20]` style APIs.
- Avoiding exposing raw pointers directly to end users.

---

## Example: safe wrapper

If you want a safe façade for general application code, you can define:

```rust
use bitcoin_sha1::{Sha1, SHA1_OUTPUT_SIZE};

pub struct SafeSha1(Sha1);

impl SafeSha1 {
    pub fn new() -> Self {
        SafeSha1(Sha1::new())
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            self.0.write(data.as_ptr(), data.len());
        }
    }

    pub fn finalize(mut self) -> [u8; SHA1_OUTPUT_SIZE] {
        let mut out = [0u8; SHA1_OUTPUT_SIZE];
        self.0.finalize(&mut out);
        out
    }
}
```

---

## Repository and contribution

This crate resides in the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) repository. Issues and pull requests should be opened there.

Please:

- Keep changes minimal and well‑reviewed; SHA‑1 is easy to implement incorrectly.
- Add tests that compare against known SHA‑1 test vectors for any behavior change.

---

## License

This crate is licensed under the **MIT** license. Refer to the `LICENSE` file in the repository for full terms.

> **Note:** This README was generated by GPT5-1.
