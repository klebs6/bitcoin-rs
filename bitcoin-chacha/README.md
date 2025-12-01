# bitcoin-chacha

A low-level, Bitcoin-flavoured implementation of the ChaCha20 stream cipher and the ChaCha20‑Poly1305@bitcoin AEAD construction, suitable for interoperating with Bitcoin's P2P transport encryption.

---

## Overview

`bitcoin-chacha` provides:

- A **constant‑time, low‑level ChaCha20** implementation (Bernstein, 2008).
- A **ChaCha20‑Poly1305 AEAD** that follows the *Bitcoin Core* P2P transport construction (`chacha20-poly1305@bitcoin`), not the generic RFC 7539 layout.
- A **quarterround macro** and supporting primitives mirroring the original public‑domain C implementation, translated into Rust.
- A **Poly1305 tag computation wrapper** around `bitcoin_poly1305::poly1305_auth` with constant‑time tag comparison.

The crate focuses on **bit‑exact compatibility** with the Bitcoin reference implementation and on predictable performance properties (constant‑time tag checks, explicit keystream use, explicit memory cleansing of key material).

This is **not** a high‑level AEAD abstraction; instead, it gives you precise control over sequence numbers, nonces, counters, and buffer management, which is required for correct implementation of the Bitcoin P2P handshake and transport.

---

## Cryptographic Background

### ChaCha20

ChaCha20 is a 256‑bit stream cipher built from a 4×4 matrix of 32‑bit words:

\[
\text{state} =
\begin{bmatrix}
  c_0 & c_1 & c_2 & c_3 \\
  k_0 & k_1 & k_2 & k_3 \\
  k_4 & k_5 & k_6 & k_7 \\
  ctr & n_0 & n_1 & n_2
\end{bmatrix}
\]

Each **round** applies the `quarterround` operation four times on the columns, then on the diagonals. After 20 rounds (10 double‑rounds), the result is added to the original state and serialized little‑endian into 64 bytes of keystream.

This crate exposes that via:

- `ChaCha20::set_key` — initialize from 128‑ or 256‑bit key (16 or 32 bytes) using the standard `TAU`/`SIGMA` constants.
- `ChaCha20::setiv` — set a 64‑bit nonce (Bitcoin uses a 64‑bit sequence number as nonce).
- `ChaCha20::seek` — set a 64‑bit block counter (word 12/13 of the state).
- `ChaCha20::keystream` — fill a buffer with raw ChaCha20 keystream.
- `ChaCha20::crypt` — XOR the keystream with a message (encryption and decryption are symmetric).

The `quarterround!` macro operates directly on four mutable `u32` registers and is inlined by the compiler.

### Poly1305 and AEAD

Poly1305 is a one‑time message authentication code over a prime field modulo \(2^{130} − 5\). Given a 256‑bit key, it produces a 128‑bit tag. Correct usage requires a **fresh Poly1305 key per message**.

`bitcoin-chacha` derives the Poly1305 key from the first 256 bits of ChaCha20 keystream under `K₂` for each packet, following the Bitcoin transport design. It then authenticates **both the encrypted 3‑byte length field and the ciphertext payload**.

Tag computation/verification is done via:

- `compute_poly1305_tag(key, msg)` – wraps `bitcoin_poly1305::poly1305_auth` and returns a `GenericArray<u8, U16>`.
- `timingsafe_bcmp` – constant‑time equality check used for tag verification.

---

## Bitcoin-specific AEAD Construction

Bitcoin’s `chacha20-poly1305@bitcoin` differs from RFC 7539 AEAD in:

- **Two independent ChaCha20 instances**:
  - `K₁`: encrypts the 3‑byte length field ("AAD" in this crate’s nomenclature).
  - `K₂`: drives both payload encryption and Poly1305 key derivation.
- **Encrypted length field**: the first 3 bytes of each packet are encrypted using a cached keystream derived from `K₁`, sequence numbers, and a position in the 64‑byte keystream block.
- **Per‑packet Poly1305 key** derived from `K₂` using:
  - Nonce: packet sequence number as a 64‑bit little‑endian value.
  - Counter: 0 for Poly1305 key derivation, `1` for payload encryption.
- **MAC over (encrypted length || ciphertext payload)**.

This crate models that as:

```rust
#[derive(MutGetters, Debug, Getters)]
pub struct ChaCha20Poly1305AEAD {
    chacha_header:        ChaCha20,                 // K₁ instance
    chacha_main:          ChaCha20,                 // K₂ instance
    aad_keystream_buffer: [u8; CHACHA20_ROUND_OUTPUT],
    cached_aad_seqnr:     u64,
}
```

The key points:

- **Length cipher (`K₁`)**
  - Nonce: AAD sequence number (`seqnr_aad`).
  - Counter: 0.
  - Keystream block (64 bytes) is cached per `seqnr_aad` to amortize multiple 3‑byte length encryptions.
  - Reuse of `{key, nonce, position}` is forbidden beyond the allowed bounds (2^70 bytes/nonce).

- **Payload + MAC cipher (`K₂`)**
  - Nonce: payload sequence number (`seqnr_payload`).
  - Counter 0: first 32 keystream bytes form the Poly1305 key.
  - Counter 1+: used for payload encryption.

---

## API Surface

### quarterround!

```rust
quarterround!(a, b, c, d);
```

Expands to a ChaCha quarter‑round on four `u32` variables `a, b, c, d`, using `wrapping_add`, XOR, and `rotl32` rotations by {16, 12, 8, 7} bits. This macro is used internally by `ChaCha20::keystream` but is exported for use in specialized low‑level code (e.g., constant‑time gadget experimentation, verification against test vectors).

### ChaCha20

#### Construction and configuration

```rust
use bitcoin_chacha::ChaCha20;

// From a 256‑bit key
let key: [u8; 32] = [0x00; 32];
let mut chacha = ChaCha20::new(key.as_ptr(), key.len());

// Or create then set key later
let mut chacha2 = ChaCha20::default();
chacha2.set_key(key.as_ptr(), key.len());

// Set 64‑bit nonce and counter
chacha.setiv(42u64);       // nonce / IV
chacha.seek(0);            // block counter
```

`set_key` expects either:

- 16‑byte key (128‑bit; uses `TAU` constants), or
- 32‑byte key (256‑bit; uses `SIGMA` constants).

Any other length panics via `assert!` to prevent silent misconfiguration.

#### Stream generation

```rust
// Generate 64 bytes of raw keystream
let mut ks = [0u8; 64];
chacha.keystream(ks.as_mut_ptr(), ks.len());
```

`keystream` uses the current `(key, nonce, counter)` state and updates the internal 64‑bit counter words (12 and 13) as it consumes blocks. Partial‑block requests are handled safely by writing to a temporary 64‑byte buffer and copying only the required prefix.

#### Encryption / decryption

```rust
let plaintext = b"hello world";
let mut ciphertext = vec![0u8; plaintext.len()];

chacha.crypt(
    plaintext.as_ptr(),
    ciphertext.as_mut_ptr(),
    plaintext.len(),
);

// Decrypt: reset counter/nonce and XOR again
chacha.setiv(42u64);
chacha.seek(0);
let mut recovered = vec![0u8; plaintext.len()];
chacha.crypt(
    ciphertext.as_ptr(),
    recovered.as_mut_ptr(),
    ciphertext.len(),
);
assert_eq!(&recovered[..], &plaintext[..]);
```

`crypt` is a convenience wrapper around `keystream` + XOR. It allocates a temporary `Vec<u8>` of size `bytes` for the keystream, which is acceptable in typical Bitcoin P2P packet sizes; if you need zero‑allocation, use `keystream` and XOR manually.

### ChaCha20Poly1305AEAD

#### Construction

```rust
use bitcoin_chacha::ChaCha20Poly1305AEAD;

let k1 = [0u8; CHACHA20_POLY1305_AEAD_KEY_LEN];
let k2 = [1u8; CHACHA20_POLY1305_AEAD_KEY_LEN];

let mut aead = ChaCha20Poly1305AEAD::new(
    k1.as_ptr(), k1.len(),
    k2.as_ptr(), k2.len(),
);
```

Both `k_1_len` and `k_2_len` must equal `CHACHA20_POLY1305_AEAD_KEY_LEN` (256‑bit keys). The constructor asserts on mismatch.

The expected key mapping for a full duplex connection:

- Initiator:
  - Send:   `K₁_A, K₂_A`
  - Receive:`K₁_B, K₂_B`
- Responder:
  - Send:   `K₁_B, K₂_B`
  - Receive:`K₁_A, K₂_A`

Exactly as in Bitcoin’s transport specification.

#### Packet encryption/decryption

```rust
let seqnr_payload: u64 = 0; // packet seq for payload/MAC
let seqnr_aad: u64     = 0; // packet seq for length/AAD
let aad_pos: i32       = 0; // position within cached 64‑byte keystream block

// Layout: [ 3‑byte AAD length | payload bytes (N) ]
let mut src = Vec::new();
let payload = b"example payload";
let len = payload.len() as u32; // must fit in 24 bits
src.extend_from_slice(&[ (len & 0xFF) as u8,
                         ((len >> 8) & 0xFF) as u8,
                         ((len >> 16) & 0xFF) as u8 ]);
src.extend_from_slice(payload);

// Destination must hold AAD + payload + MAC (when encrypting)
let mut dest = vec![0u8; src.len() + POLY1305_TAGLEN];

let ok = aead.crypt(
    seqnr_payload,
    seqnr_aad,
    aad_pos,
    dest.as_mut_ptr(),
    dest.len(),
    src.as_ptr(),
    src.len(),
    true, // encrypt
);
assert!(ok);

// To decrypt, reverse: dest becomes input, a new buffer for output
let mut decrypted = vec![0u8; dest.len() - POLY1305_TAGLEN];
let ok = aead.crypt(
    seqnr_payload,
    seqnr_aad,
    aad_pos,
    decrypted.as_mut_ptr(),
    decrypted.len(),
    dest.as_ptr(),
    dest.len(),
    false, // decrypt (also verifies MAC)
);
assert!(ok);
```

**Contract of `crypt`**:

- When `is_encrypt == true`:
  - `src` must hold `[AAD || payload]`.
  - `dest` must have length ≥ `src_len + POLY1305_TAGLEN`.
  - Output layout: `[AAD' || payload' || tag]`.

- When `is_encrypt == false`:
  - `src` must hold `[AAD' || payload' || tag]`.
  - `dest` must have length ≥ `src_len - POLY1305_TAGLEN`.
  - If the MAC is invalid, returns `false` and does not yield plaintext.

Bounds checks are done up front; the function returns `false` on any size violation.

Internally, `crypt` ensures:

1. Derive the Poly1305 key from `K₂` with nonce `seqnr_payload`, counter = 0.
2. If decrypting, compute a fresh tag on `[AAD' || payload']` and verify it in constant time using `timingsafe_bcmp`. Reject on failure.
3. Ensure the cached AAD keystream buffer matches `seqnr_aad`; if not, recompute 64 bytes of `K₁` keystream for that sequence number.
4. XOR the 3 AAD bytes against the cached keystream starting at `aad_pos`.
5. Set `K₂` counter to 1, then encrypt/decrypt the payload with ChaCha20.
6. If encrypting, compute and append the Poly1305 tag over `[encrypted AAD || ciphertext payload]`.
7. Zeroize Poly1305 key material and actively cleanse memory (`memory_cleanse`).

This closely matches the design constraints stated in the Bitcoin documentation: avoiding oracles via independent keying for lengths, preventing `{key, nonce}` reuse, and enforcing MAC verification *before* payload usage.

#### Decrypting only the length (get_length)

`get_length` is optimized for the critical path where you must know the packet length **before** reading the entire payload.

```rust
let mut len24: u32 = 0;
let ok = aead.get_length(
    &mut len24 as *mut u32,
    seqnr_aad,
    aad_pos,
    encrypted_len_ptr, // *const u8, first 3 bytes of packet
);
assert!(ok);

let packet_len = len24 & 0x00FF_FFFF; // 24‑bit length field
```

- `aad_pos` must satisfy:

  ```text
  0 <= aad_pos < CHACHA20_ROUND_OUTPUT - CHACHA20_POLY1305_AEAD_AAD_LEN
  ```

  This is asserted at runtime.

- Reuses the same AAD keystream cache machinery as `crypt`, keyed by `seqnr_aad`.

---

## Poly1305 Utilities

### compute_poly1305_tag

```rust
use bitcoin_chacha::compute_poly1305_tag;

let key: [u8; POLY1305_KEYLEN] = [0u8; POLY1305_KEYLEN];
let msg = b"data";
let tag = compute_poly1305_tag(&key, msg);
assert_eq!(tag.len(), POLY1305_TAGLEN);
```

- Delegates to `bitcoin_poly1305::poly1305_auth`.
- Produces a 16‑byte tag as `GenericArray<u8, U16>`.
- Tracing hooks record message length.

### timingsafe_bcmp

```rust
use bitcoin_chacha::timingsafe_bcmp;

let a = [0u8; 16];
let b = [0u8; 16];
let res = timingsafe_bcmp(a.as_ptr(), b.as_ptr(), a.len());
assert_eq!(res, 0); // 0 means equal
```

- Returns `0` if equal, `1` otherwise.
- Time‑independent with respect to the first differing byte.
- Intended for MAC/tag comparison.

---

## Safety and Correctness Considerations

- The crate makes **heavy use of raw pointers** (`*const u8`, `*mut u8`) to match the structure and performance properties of the C reference implementation. All unsafe blocks are narrowly scoped and rely on documented preconditions:
  - Pointers must be valid for the specified length.
  - Buffers must be non‑overlapping where required (e.g., `copy_nonoverlapping`).
- The AEAD logic **zeroizes** per‑packet Poly1305 keys and calls `memory_cleanse` to clear sensitive material.
- Tag verification uses `timingsafe_bcmp`, not `memcmp`, to avoid timing side channels.
- **Key/nonce reuse bounds** are enforced only by documentation and invariants in higher‑level protocol code; the library asserts lengths, but does not attempt to track global nonce usage.
- The design is intended to be **bit‑compatible** with Bitcoin Core. When integrating, you should validate against known‑good test vectors or run cross‑implementation tests between this crate and reference nodes.

---

## Example: Bitcoin‑style Transport Packet

The following is a schematic example of end‑to‑end send/receive for a single packet:

```rust
fn send_packet(
    aead: &mut ChaCha20Poly1305AEAD,
    seqnr_payload: u64,
    seqnr_aad: u64,
    aad_pos: i32,
    payload: &[u8],
) -> Vec<u8> {
    assert!(payload.len() < (1 << 24));

    let len24 = payload.len() as u32;
    let mut src = Vec::with_capacity(3 + payload.len());
    src.push((len24 & 0xFF) as u8);
    src.push(((len24 >> 8) & 0xFF) as u8);
    src.push(((len24 >> 16) & 0xFF) as u8);
    src.extend_from_slice(payload);

    let mut dest = vec![0u8; src.len() + POLY1305_TAGLEN];
    let ok = aead.crypt(
        seqnr_payload,
        seqnr_aad,
        aad_pos,
        dest.as_mut_ptr(),
        dest.len(),
        src.as_ptr(),
        src.len(),
        true,
    );
    assert!(ok);

    dest
}

fn recv_packet(
    aead: &mut ChaCha20Poly1305AEAD,
    seqnr_payload: u64,
    seqnr_aad: u64,
    aad_pos: i32,
    packet: &[u8],
) -> Option<Vec<u8>> {
    use core::ptr;

    // 1. Decrypt length only
    let mut len24: u32 = 0;
    let ok = aead.get_length(
        &mut len24 as *mut u32,
        seqnr_aad,
        aad_pos,
        packet.as_ptr(),
    );
    if !ok { return None; }

    let len = (len24 & 0x00FF_FFFF) as usize;
    if packet.len() < 3 + len + POLY1305_TAGLEN { return None; }

    // 2. Decrypt + authenticate full packet
    let mut out = vec![0u8; 3 + len];
    let ok = aead.crypt(
        seqnr_payload,
        seqnr_aad,
        aad_pos,
        out.as_mut_ptr(),
        out.len(),
        packet.as_ptr(),
        3 + len + POLY1305_TAGLEN,
        false,
    );
    if !ok { return None; }

    Some(out[3..].to_vec())
}
```

This pattern illustrates the split between early length decryption and full AEAD processing consistent with the Bitcoin specification.

---

## Repository and License

- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Crate**: `bitcoin-chacha`
- **License**: MIT
- **Edition**: Rust 2021

The implementation is based on the public‑domain ChaCha20 reference by D. J. Bernstein and adapted for Bitcoin’s P2P transport.

---

## When (and When Not) to Use This Crate

Use `bitcoin-chacha` when:

- You need **bit‑exact compatibility with Bitcoin Core’s ChaCha20‑Poly1305@bitcoin transport**.
- You need **fine control** over sequence numbers, counters, and nonce usage for Bitcoin‑style protocols.
- You accept **pointer‑level, low‑level APIs** in exchange for tight control and minimal overhead.

You may prefer higher‑level AEAD crates (e.g., `chacha20poly1305`) when you want:

- Generic RFC 8439‑style AEAD semantics.
- Safer, slice‑based APIs with no explicit raw pointers.
- Non‑Bitcoin‑specific data layout and nonce semantics.

---

*Generated programmatically by an AI model; review before relying on it for security‑critical documentation.*
