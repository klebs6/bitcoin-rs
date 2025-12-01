# bitcoin-poly1305

A constant-time, trace-instrumented Poly1305 one-time authenticator used by Bitcoin Core, implemented as a small, focused Rust crate. The code is a close structural translation of the original `poly1305-donna-unrolled.c` routine, with the carry, clamping, and reduction behavior preserved to maintain bit-for-bit compatibility.

---

## Overview

`bitcoin-poly1305` exposes a low-level, allocation-free Poly1305 MAC implementation tuned for correctness and auditability in cryptographic contexts (notably Bitcoin-related tooling). It:

- Implements Poly1305 over the prime field \( p = 2^{130} - 5 \) using 5 radix-\(2^{26}\) limbs.
- Preserves Donna's exact reduction, clamping, and carry propagation semantics to avoid subtle divergences from well-established test vectors.
- Keeps the critical arithmetic constant-time with respect to secret data (no data-dependent branching on limb values).
- Provides optional, structured tracing via the `tracing` ecosystem, allowing full inspection of intermediate states during testing or formal verification.

This crate is intentionally low-level. It exposes the raw arithmetic and a single top-level `poly1305_auth` function; it does not manage keys, nonces, or higher-level AEAD constructs.

---

## Cryptographic background

Poly1305 computes a 16-byte authenticator (tag) for an arbitrary-length message `m` under a 32-byte one-time key `(r ‖ s)`:

1. Interpret `r` as an element of \( \mathbb{Z} / (2^{130} - 5) \), after *clamping* certain bits to enforce bounds crucial to security and correctness.
2. Split the message `m` into 16-byte blocks; each block is interpreted as a 130-bit integer in little-endian form, with an implicit high bit.
3. Maintain an accumulator `h` in radix-\(2^{26}\):
   - Add each block into `h` (with the implicit high bit).
   - Multiply by `r` and reduce modulo \( 2^{130} - 5 \).
4. After processing all blocks, perform a final carry propagation and conditional subtraction of `p`.
5. Add the 128-bit pad `s`, then output the least-significant 128 bits as the tag.

Internally, this crate uses the standard 5-limb radix-\(2^{26}\) representation

\[
  h = h_0 + h_1 2^{26} + h_2 2^{52} + h_3 2^{78} + h_4 2^{104}, \quad 0 \le h_i < 2^{26}.
\]

The reduction leverages the congruence \( 2^{130} \equiv 5 \pmod p \) so that overflow from the top limb can be folded back into lower limbs via multiplication by 5.

All field operations are designed to avoid branches on secret values, using bit masks and fixed sequences of operations instead.

---

## Features and design goals

- **Constant-time arithmetic** for secret-dependent values (accumulators, limbs, keys).
- **Exact Donna semantics**: replicate the reference implementation's order of operations and truncations, including non-obvious details like when not to mask `h[1]`.
- **Fine-grained tracing hooks** via `tracing` and custom macros, permitting black-box debugging and side-channel analysis.
- **No allocations / no_std-ready core**: only relies on the Rust core primitives; you can integrate it into constrained environments (subject to your own `tracing` setup).
- **Bitcoin-aligned behavior**: suitable for downstream code that must match Bitcoin Core's Poly1305 behavior exactly.

---

## Crate layout and public API

The crate primarily exposes:

- **Types**
  - `pub type LimbArr5 = [u32; 5];` – five 26-bit limbs (radix-\(2^{26}\))
  - `pub type LimbArr4 = [u32; 4];` – multipliers \(r_i \cdot 5\) for \( i = 1..4 \)

- **Macros**
  - `mul32x32_64!(a, b)` – 32×32→64-bit multiplication as `u64`, mirroring C's `((uint64_t)a * (b))` with `wrapping_mul`.
  - `trace_step!(step, { fields... })` – helper for emitting structured `TRACE` events with a fixed `step` label.

- **Core functions**
  - `pub fn poly1305_auth(out: &mut [u8; POLY1305_TAGLEN], msg: &[u8], key: &[u8; POLY1305_KEYLEN])`
    - Top-level, constant-time Poly1305 authenticator.
    - Uses: `expand_key`, `accumulate_block`, `multiply_and_reduce`, `final_carry_and_sub_p`, and `add_pad_serialize`.

- **Key expansion**
  - `pub fn expand_key(key: &[u8; POLY1305_KEYLEN]) -> (LimbArr5, LimbArr4)`
    - Parses and clamps the 32-byte key into five 26-bit limbs `r` and their associated \(5\times\) multipliers `s`.

- **Arithmetic primitives** (primarily used internally, but public where appropriate for verification / reuse):
  - `pub fn propagate_26bit_carries_once(h: &mut [u32; 5])`
  - `pub fn accumulate_block(h: &mut LimbArr5, block: &[u8; 16], add_high_bit: bool)`
  - `pub fn multiply_and_reduce(h: &mut LimbArr5, r: &LimbArr5, s: &LimbArr4)`
  - `pub fn ct_select_limbs(h: &mut LimbArr5, g: &LimbArr5, select_mask: u32)`

Lower-level helpers like `read_le32`, `write_le32`, `final_carry_and_sub_p`, `add_pad_serialize`, and `compute_g_plus5_minus_p` are central to the reduction flow but may be crate-internal depending on the version and visibility choices.

---

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-poly1305 = "0.1.19"
tracing = "0.1"       # optional but recommended for observability
```

Compute a Poly1305 tag for a message:

```rust
use bitcoin_poly1305::poly1305_auth;

const KEY_LEN: usize = 32;  // POLY1305_KEYLEN
const TAG_LEN: usize = 16;  // POLY1305_TAGLEN

fn compute_tag(msg: &[u8], key: &[u8; KEY_LEN]) -> [u8; TAG_LEN] {
    let mut tag = [0u8; TAG_LEN];
    poly1305_auth(&mut tag, msg, key);
    tag
}

fn main() {
    let key: [u8; KEY_LEN] = [0u8; KEY_LEN]; // never reuse in production
    let message = b"example message";

    let tag = compute_tag(message.as_ref(), &key);
    println!("tag: {:02x?}", tag);
}
```

### Tracing and debugging

The crate uses `tracing` to emit structured events at various levels:

- `trace_step!` for fine-grained arithmetic inspection.
- `tracing::trace!` and `tracing::debug!` from internal routines.
- `tracing::info!` around `poly1305_auth` entry/exit.

Example setup:

```rust
use bitcoin_poly1305::poly1305_auth;
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter("bitcoin_poly1305=trace")
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let key = [0u8; 32];
    let msg = b"trace me";
    let mut tag = [0u8; 16];

    poly1305_auth(&mut tag, msg, &key);
}
```

Then run with e.g. `RUST_LOG=trace` to obtain detailed logs of limb states, carries, and the final tag.

---

## Constant-time considerations

The arithmetic is structured to avoid secret-dependent branches:

- Carry propagation uses fixed loops and bit operations.
- Conditional subtraction of `p` in `final_carry_and_sub_p` is implemented with `ct_select_limbs`, which performs branchless selection using a 32-bit mask.
- Multiplication uses `mul32x32_64!` with `wrapping_mul`, matching C's semantics and avoiding undefined behavior.

Nonetheless, constant-time behavior is highly environment-dependent. When integrating into a security-critical system:

- Compile with optimizations enabled and avoid LTO configurations that may reintroduce secret-dependent behavior.
- Disable debug assertions in production builds, as they may introduce branches tied to internal invariants.
- Audit your `tracing` configuration for potential side-channel leakage if enabled in non-test environments.

---

## Key handling and security model

- The key `(r ‖ s)` **must be unique per message**. Reusing a Poly1305 key with different messages catastrophically compromises security.
- This crate **does not**:
  - manage randomness, key derivation, or nonces;
  - provide AEAD constructions (e.g., ChaCha20-Poly1305);
  - zeroize keys or tags automatically.

It is intended to be composed inside a higher-level protocol or cryptographic library that enforces correct key usage, nonces, and lifecycle management.

---

## Integration in Bitcoin-related tools

The repository `https://github.com/klebs6/bitcoin-rs` groups Bitcoin-focused components. `bitcoin-poly1305` can be used wherever you need a Poly1305 implementation matching Bitcoin Core semantics:

- custom or experimental transaction relay/authentication schemes,
- protocol experimentation where deterministic, cross-language equivalence with Bitcoin Core is mandatory,
- test harnesses that validate or differential-test against reference C implementations.

---

## Testing

To validate behavior against existing vectors (if included in the repository):

```bash
cargo test -p bitcoin-poly1305
```

You can also enable detailed tracing during tests:

```bash
RUST_LOG=trace cargo test -p bitcoin-poly1305 -- --nocapture
```

This is particularly useful when investigating mismatches against RFC 7539 or Bitcoin Core vectors, or when formally reasoning about the reduction procedure.

---

## License

This crate is licensed under the **MIT** license. See the repository for full license text.

---

## Caveats

- This is a low-level, performance-oriented cryptographic primitive. Misuse (e.g., key reuse, incorrect protocol composition) will render it insecure.
- The README is AI-generated; when in doubt, defer to the actual Rust source and accompanying tests.
