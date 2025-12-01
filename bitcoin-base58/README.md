# bitcoin-base58

A minimal, Bitcoin-compatible Base58 and Base58Check codec implemented in Rust. This crate focuses on strict, consensus-faithful behavior suitable for wallet software, node implementations, and security‑sensitive tooling.

---

## Features

- **Bitcoin-compatible Base58 alphabet** (`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`).
- **Leading‑zero preservation**: leading `0x00` bytes encode to leading `1` characters and round‑trip precisely.
- **Base58Check support**: double‑SHA256 checksum on the payload, validated on decode.
- **Strict decoder**:
  - Rejects whitespace in `decode_base58`.
  - Rejects characters outside the Base58 alphabet.
  - Enforces caller‑supplied maximum output length to prevent unbounded allocation.
- **Raw C‑pointer variants** (`*_raw`) for FFI and low‑level interop with C APIs or in‑place buffers.

This crate is intentionally narrow: it does *only* Base58/Base58Check, and does so in a way that mirrors the reference Bitcoin implementation.

---

## Base58 and Base58Check: Brief Technical Background

Bitcoin’s Base58 encoding is a positional numeral system in radix 58 over a custom alphabet. It is designed to:

- Avoid visually ambiguous characters (`0`, `O`, `I`, `l`).
- Avoid punctuation that is hard to select or double‑click.
- Encode arbitrary byte strings into a human‑copyable representation.

Formally, if a byte sequence is interpreted as a big‑endian integer

\[ n = \sum_{i=0}^{k-1} b_i 256^{k-1-i}, \quad b_i \in [0, 255], \]

then the Base58 digits are the coefficients in the base‑58 expansion of `n`.

The algorithm implemented here does not construct the big integer explicitly. Instead, it uses repeated division/modulo by 58 (or 256) on an array representation, which is asymptotically and practically efficient while avoiding big‑int libraries.

**Base58Check** augments Base58 with a 4‑byte checksum:

1. Compute `d = SHA256(SHA256(payload))`.
2. Take the first 4 bytes `d[0..4]`.
3. Append these 4 bytes to `payload`.
4. Base58‑encode the result.

Decoding performs the inverse and verifies that the checksum bytes match `checksum4_sha256d(payload)`. This is the format used for addresses, WIF keys, and other human‑facing Bitcoin identifiers.

---

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-base58 = "0.1.20"
```

This crate targets Rust **2021** edition and is licensed under **MIT**.

---

## High‑Level API

### Encoding: raw Base58

```rust
use bitcoin_base58::encode_base58;

let data = b"hello";
let encoded = encode_base58(data);
println!("{}", encoded);
```

#### Function

```rust
pub fn encode_base58(input: &[u8]) -> String
```

- **Input**: arbitrary byte slice.
- **Output**: Base58 string using the canonical Bitcoin alphabet.
- **Leading zero handling**: each leading `0x00` becomes a leading `'1'`.

This function is allocation‑conscious: it precomputes an upper bound on the number of Base58 digits using

\[ \lceil \log_{58}(256) \rceil \approx 1.38, \]

and reserves `input.len() * 138 / 100 + 1` bytes to avoid repeated reallocations.

---

### Encoding: Base58Check (with checksum)

```rust
use bitcoin_base58::encode_base_58check;

let payload = b"versioned payload";
let addr_like = encode_base_58check(payload);
println!("{}", addr_like);
```

#### Function

```rust
pub fn encode_base_58check(input: &[u8]) -> String
```

- Appends `checksum4_sha256d(input)` to `input`.
- Encodes the concatenation using `encode_base58`.
- Suitable for Bitcoin‑style addresses and WIF keys once you have prepended the appropriate version byte(s).

The checksum is **big‑endian bytes in digest order**; there is no integer reinterpretation or endianness flipping, which aligns with Bitcoin Core semantics.

---

### Decoding: raw Base58

```rust
use bitcoin_base58::decode_base58;

let s = "StV1DL6CwTryKyV"; // Base58 for "hello" under this alphabet
let mut out = Vec::new();
let ok = decode_base58(s, &mut out, 64);

assert!(ok);
println!("decoded: {:?}", out);
```

#### Function

```rust
pub fn decode_base58(input: &str, vch_ret: &mut Vec<u8>, max_ret_len: usize) -> bool
```

- **`input`**: the Base58 string; leading and trailing Unicode whitespace is trimmed.
- **`vch_ret`**: output buffer that will be **cleared** and then filled with the decoded bytes on success.
- **`max_ret_len`**: hard cap on the decoded length; if the decoded data would exceed this, the function returns `false`.

Behavioral notes:

- Leading `'1'` characters are interpreted as `0x00` bytes and counted explicitly.
- Any internal whitespace (after trimming) is rejected.
- Any character outside the Base58 alphabet is rejected.
- On **failure**, `vch_ret` is left in a cleared or explicitly empty state.

Use `max_ret_len` defensively when decoding user‑supplied data to avoid memory abuse.

---

### Decoding: Base58Check (with checksum)

```rust
use bitcoin_base58::decode_base_58check;

let addr = "1BoatSLRHtKNngkdXEeobR76b53LETtpyT"; // example Bitcoin address-like string
let mut payload = Vec::new();

if decode_base_58check(addr, &mut payload, 128) {
    // payload now contains: [version_byte, data_bytes...]
    println!("payload len = {}", payload.len());
} else {
    eprintln!("invalid Base58Check string or checksum mismatch");
}
```

#### Function

```rust
pub fn decode_base_58check(
    input: &str,
    vch_ret: &mut Vec<u8>,
    max_ret_len: usize,
) -> bool
```

Steps:

1. Guard `max_ret_len + 4` against `usize` overflow.
2. Call `decode_base58(input, vch_ret, max_ret_len + 4)`.
3. Require that at least 4 bytes are present (checksum).
4. Split `vch_ret` into `payload` and `checksum`.
5. Verify `checksum == checksum4_sha256d(payload)`.
6. On success, truncate `vch_ret` to `payload_len` and return `true`.

On any failure (invalid characters, excessive length, insufficient length, checksum mismatch), `vch_ret` is cleared and `false` is returned.

Use this for all contexts where you expect a Bitcoin Base58Check value and want deterministic, strict verification.

---

## Low‑Level C‑Pointer APIs

The crate also exposes raw pointer variants that more closely mirror Bitcoin Core’s C++ implementation and are suitable for FFI or in‑place decoding scenarios.

### `decode_base58_raw`

```rust
pub fn decode_base58_raw(psz: *const u8, vch: &mut Vec<u8>, max_ret_len: i32) -> bool
```

- `psz` is a NUL‑terminated C string (`*const u8`), not a Rust slice.
- Leading spaces are skipped.
- Leading `'1'` characters are converted into leading zero bytes; this count is checked against `max_ret_len`.
- Uses a pre‑sized big‑endian base256 work buffer with length `strlen(psz) * 733 / 1000 + 1`, reflecting

  \[ \lceil \log_{256}(58) \rceil \approx 0.733. \]

- Stops parsing at the first whitespace after the content and requires that the remainder to NUL be whitespace only; any extra nonspace characters cause failure.
- Enforces `length + zeroes <= max_ret_len`.

`vch` is cleared and filled with the decoded bytes on success; leading zeros are explicitly inserted.

### `decode_base_58check_raw`

```rust
pub fn decode_base_58check_raw(
    psz: *const u8,
    vch_ret: &mut Vec<u8>,
    max_ret_len: i32,
) -> bool
```

- Computes an internal budget of `max_ret_len + 4` (clamped to `i32::MAX`) to account for the checksum.
- Calls `decode_base58_raw` with that budget.
- Requires at least 4 trailing checksum bytes.
- Verifies `checksum4_sha256d(payload)` and truncates `vch_ret` to the payload on success.

These functions are ideal when interfacing with existing C libraries or when decoding directly from C‑style buffers.

---

## Checksum Utility

```rust
pub fn checksum4_sha256d(payload: &[u8]) -> [u8; 4]
```

- Computes `SHA256(SHA256(payload))` using the `bitcoin-hash` crate’s `Hash256` type.
- Returns the **first 4 bytes** of the resulting digest, in digest order.

This function is the canonical checksum used by Bitcoin’s Base58Check. You can compose it with your own encoding logic or use it standalone for manual verification.

Example:

```rust
use bitcoin_base58::checksum4_sha256d;

let payload = b"example";
let chk = checksum4_sha256d(payload);
println!("checksum: {}", hex::encode(chk));
```

---

## Error Handling and Security Considerations

- All decode functions return `bool` instead of panicking for malformed input.
- Internal arithmetic (`size` estimates, buffer operations) uses checked or conservative calculations to avoid overflow.
- `max_ret_len` / `max_ret_len + 4` guards are explicitly enforced to avoid unbounded growth from attacker‑controlled strings.
- Whitespace rules are strict to avoid accepting visually modified or accidentally extended input.

For network‑facing or wallet software, strongly prefer the `*_check` variants to get checksum verification and boundary checks in one call.

---

## Logging

Some functions (e.g., `encode_base_58check`, `decode_base_58check`, `decode_base_58check_raw`, `checksum4_sha256d`) emit structured logs using macros such as `trace!`, `debug!`, `info!`, `warn!`, and `error!`.

If you enable a compatible logging backend (such as `tracing` with a subscriber), you will see detailed diagnostics about payload lengths, computed checksums, and error reasons (e.g., checksum mismatch). In high‑assurance systems, this can aid observability and incident analysis.

---

## Relationship to `bitcoin-rs` Repository

This crate lives inside the repository:

- <https://github.com/klebs6/bitcoin-rs>

It is designed as a modular component that can be reused independently in other projects needing Bitcoin‑style Base58 semantics without pulling in the full node or protocol stack.

---

## License

This crate is distributed under the **MIT license**.

See the repository for full licensing details.
