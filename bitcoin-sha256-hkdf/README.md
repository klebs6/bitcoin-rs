# bitcoin-sha256-hkdf

A compact, single‑purpose HKDF (RFC 5869) implementation specialized for:

- HMAC-SHA256 as the underlying PRF
- A fixed output length of 32 bytes (L = 32)
- Compatibility with Bitcoin-style codebases and key-derivation workflows

This crate is intentionally narrow in scope: it exists to derive *exactly one* 32‑byte key from an input keying material (IKM), with a salt and an info string, using the expand step of HKDF with a single block.

---

## Motivating use case

Many Bitcoin and cryptocurrency systems need a simple, auditable, deterministic key-derivation mechanism built from SHA‑256 primitives. Typical scenarios include:

- Deriving per‑channel keys or per‑session secrets from a long‑term KDF root key
- Generating context‑scoped 32‑byte secrets (e.g., for protocol instances, channels, or peers)
- Avoiding ad‑hoc derivation constructions while still staying very close to Bitcoin’s low‑level SHA‑256/HMAC style

This crate encapsulates that pattern behind a small, predictable API, without depending on large cryptographic frameworks.

---

## Cryptographic background (HKDF + HMAC-SHA256)

HKDF, specified in [RFC 5869](https://datatracker.ietf.org/doc/html/rfc5869), is a key derivation function built on top of a pseudorandom function (PRF), typically HMAC. It has two logical phases:

1. **Extract**: `PRK = HMAC(salt, IKM)`
2. **Expand**: `OKM = T(1) || T(2) || ... || T(n)`, where each block is derived inductively.

Where:
- `IKM` — input keying material (possibly high‑entropy secret)
- `salt` — non‑secret randomizer (can be public; recommended to be at least hash length)
- `info` — optional context string to bind derived keys to a domain or usage

For HMAC-SHA256, the hash output length is 32 bytes. When you derive only a single 32‑byte key, `n = 1`, and the expand step reduces to:

\[
\text{OKM} = \text{HMAC-SHA256}(\text{PRK}, \text{info} \Vert 0x01)
\]

This crate implements exactly this pattern:

- `Extract`: performed once in `CHKDF_HMAC_SHA256_L32::new` to compute `prk`
- `Expand`: performed once in `expand32`, for a single 32‑byte block

The design is intentionally minimal to align with deterministic protocols, auditability, and constant 32‑byte secret sizes (Bitcoin keys, scalar fields, etc.).

---

## Crate goals and non‑goals

**Goals**

- Provide a small, easily inspectable HKDF-SHA256 implementation with fixed 32‑byte output
- Mirror Bitcoin‑style usage patterns, where 32‑byte secrets are the canonical primitive
- Avoid configuration overhead: no dynamic output lengths, no algorithm switching

**Non‑goals**

- General HKDF implementation for arbitrary lengths or multiple blocks
- Pluggable PRFs or hash functions
- High‑level protocol logic; this crate just performs a single HKDF extract+expand

---

## Security notes

- HKDF is a standard, well‑analyzed KDF construction. This crate adheres to the canonical HMAC-SHA256 variant with L=32.
- The **salt** and **info** parameters are caller‑supplied. Proper choice of these parameters (e.g., domain separation via `info`) is your responsibility.
- `info` length is typically bounded (the C++ reference code asserts `info.size() <= 128`). Enforcing such a bound helps match existing deployments and avoid misuse.
- All cryptographic assumptions reduce to the usual properties of HMAC-SHA256: pseudorandomness and collision resistance of SHA‑256 and the unforgeability of HMAC.

This crate does **not** attempt to be side‑channel hardened beyond what the underlying HMAC-SHA256 implementation provides. For highly adversarial environments, review the implementation with a security engineer.

---

## API overview

Public interface (simplified):

```rust
/// RFC 5869 HKDF using HMAC-SHA256 with fixed 32-byte output (L = 32).
pub struct CHKDF_HMAC_SHA256_L32 {
    prk: [u8; 32],
}

impl CHKDF_HMAC_SHA256_L32 {
    /// Perform HKDF-Extract: PRK = HMAC_SHA256(salt, ikm).
    pub fn new(ikm: *const u8, ikmlen: usize, salt: &String) -> Self;

    /// Perform HKDF-Expand for a single 32-byte block.
    /// Writes exactly 32 bytes to `hash`.
    pub fn expand32(&mut self, info: &String, hash: [u8; CHKDF_HMAC_SHA256_L32_OUTPUT_SIZE]);
}
```

The naming follows the originating C++ implementation. Internally, `new` computes the pseudorandom key (`prk`), and `expand32` uses it to derive a single 32‑byte output.

> **Note:** The pointer‑based `ikm` interface (`*const u8`, `ikmlen`) mirrors C/C++ FFI style. In higher‑level Rust code, you will typically want your own safe wrapper that operates on slices.

---

## Usage

### Adding the dependency

```toml
[dependencies]
bitcoin-sha256-hkdf = "0.1.1"
```

### Basic example (unsafe pointer interface)

```rust
use bitcoin_sha256_hkdf::CHKDF_HMAC_SHA256_L32;

fn derive_key_example() {
    // Input keying material (IKM): could be an ECDH shared secret, master key, etc.
    let ikm: [u8; 32] = [0x11; 32];

    // Salt: non-secret randomizer; can be zero or protocol-specific
    let salt = String::from("bitcoin-hkdf-salt");

    let mut hkdf = unsafe {
        CHKDF_HMAC_SHA256_L32::new(ikm.as_ptr(), ikm.len(), &salt)
    };

    // Info: domain separation tag / context string
    let info = String::from("channel-key-derivation:v1");

    const CHKDF_HMAC_SHA256_L32_OUTPUT_SIZE: usize = 32;
    let mut out = [0u8; CHKDF_HMAC_SHA256_L32_OUTPUT_SIZE];

    hkdf.expand32(&info, out);

    // `out` now holds a derived 32-byte key
    // Use `out` as a symmetric key, scalar seed, or similar.
}
```

In production code you would typically encapsulate this pattern in a safe wrapper that:

- Accepts `&[u8]` for `ikm`
- Accepts `&[u8]` or `&str` for salt and info
- Returns `[u8; 32]` instead of handling raw pointers

This crate keeps the core primitive faithful to its origin; you can build ergonomic layers on top as needed.

---

## Design alignment with Bitcoin ecosystems

Bitcoin and related systems often:

- Use 32‑byte secrets ubiquitously (private keys, chain codes, seeds)
- Favor SHA‑256 and HMAC-SHA256 as cryptographic primitives
- Prefer minimal, reviewable cryptographic code without runtime configurability

`CHKDF_HMAC_SHA256_L32` fits directly into that model:

- Output is always `[u8; 32]`
- Hash function is always SHA‑256
- Key derivation is standardized by RFC 5869 and easy to audit

This makes it a natural building block for Bitcoin‑centric Rust libraries that want HKDF’s standard guarantees while maintaining familiarity with existing C++ / C codebases.

---

## Error handling and constraints

The underlying C++ reference includes an assertion like:

```c++
assert(info.size() <= 128);
```

If the Rust implementation keeps the same constraint, `expand32` will panic when `info` exceeds the allowed length. This is not a cryptographic requirement of HKDF, but rather a protocol policy to:

- Keep info fields small and predictable
- Match historical deployment constraints

When designing protocols on top of this crate, treat `info` as a short, structured label rather than an arbitrary blob.

---

## FFI and interoperability

Because `new` takes `ikm` as `*const u8` with a separate length, and both `salt` and `info` are `String`, the type signature is suitable for binding to C/C++ style APIs or for embedding in larger, FFI‑heavy systems.

If you are integrating with an existing C++ codebase that already uses `CHMAC_SHA256` and `CHKDF_HMAC_SHA256_L32`, this crate gives you a Rust side that is conceptually isomorphic:

- `ikm` and `ikmlen` match typical C `uint8_t*` + length pairs
- `salt` and `info` mirror `std::string` payloads
- The internal `prk` and derived 32‑byte `hash` are binary‑compatible sequences of bytes

---

## License

This crate is distributed under the **MIT** license.

---

## Caveats

- This README is generated by an AI model. Verify the exact function signatures and behavioral details against the actual source code.
- Do not treat this documentation as a security review. If you are using this crate in a high‑value system, commission an expert cryptographic review of the implementation and its usage patterns.
