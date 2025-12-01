# bitcoin-bech32m

A minimal, self-contained implementation of Bitcoin's Bech32 and Bech32m checksum algorithm (BIP-173, BIP-350) in Rust.

---

## Overview

`bitcoin-bech32m` provides low-level primitives for encoding and decoding Bech32 and Bech32m strings as used in modern Bitcoin address and data encodings:

- **Bech32** (BIP-173) — used by legacy SegWit addresses (e.g. `bc1qw508d6...`).
- **Bech32m** (BIP-350) — used by Taproot and future SegWit versions (e.g. `bc1p...`).

This crate focuses on:

- Exact adherence to the reference checksum construction.
- Transparent access to the underlying finite-field polynomial arithmetic (`poly_mod`).
- Explicit control over encoding type (`Encoding::BECH32` vs `Encoding::BECH32M`).
- Deterministic, allocation-conscious implementation with explicit data paths.

It is designed for consumers who want deterministic, inspectable primitives rather than a high-level address abstraction.

---

## Core Concepts

Bech32/Bech32m encodings are built from the following components:

- **HRP (Human-Readable Part)**: network or application discriminator, ASCII, lower-case enforced by this crate (e.g. `"bc"`, `"tb"`, `"lnbc"`).
- **Data part**: base-32 sequence over a restricted 32-character alphabet; typically derived from underlying binary data by regrouping bits into 5-bit symbols.
- **Checksum**: 6 symbols that guarantee high-probability detection of input corruption.

Mathematically, the checksum is constructed as a **BCH code** over GF(32):

- Data symbols are treated as coefficients of a polynomial \( v(x) \) over GF(32).
- The checksum is the remainder of \( v(x) \) times a fixed factor modulo a generator polynomial \( g(x) \) of degree 6:

  \[
  g(x) = x^6 + {29}x^5 + {22}x^4 + {20}x^3 + {21}x^2 + {29}x + {18}
  \]

- The implementation uses an efficient bit-packed state machine (`poly_mod`) to evolve the remainder as new symbols are appended.
- **Bech32 vs Bech32m** differ only by a final XOR with an encoding-specific constant (1 for Bech32, `0x2bc830a3` for Bech32m), which shifts the target remainder from `1` to `0x2bc830a3` and yields different error-locality invariants.

This crate exposes these mechanics directly so you can reason about correctness, compose your own higher-level encodings, or integrate with existing protocol logic where full control is required.

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
bitcoin-bech32m = "0.1.20"
```

---

## Data Model

### `Encoding`

```rust
#[derive(Debug, PartialEq, Eq)]
pub enum Encoding {
    INVALID,
    BECH32,
    BECH32M,
}
```

Represents the logical checksum variant:

- `Encoding::BECH32`  — BIP-173 checksum constant.
- `Encoding::BECH32M` — BIP-350 checksum constant.
- `Encoding::INVALID` — used to signal decoding / verification failure.

### `DecodeResult`

```rust
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct DecodeResult {
    encoding: Encoding,
    hrp:      String,
    data:     Vec<u8>,
}

impl Default for DecodeResult { /* ... */ }
impl DecodeResult {
    pub fn new(enc: Encoding, h: String, d: Vec<u8>) -> Self { /* ... */ }
}
```

`DecodeResult` captures the outcome of decoding a Bech32/Bech32m string:

- `encoding()` → `Encoding`
- `hrp()`      → `&String` (normalized to lower-case)
- `data()`     → `&Vec<u8>` (payload without the 6-symbol checksum)

On any validation failure, `decode` returns `DecodeResult::default()`, which has:

- `encoding == Encoding::INVALID`
- `hrp == ""`
- `data == []`

Callers should treat `Encoding::INVALID` as the definitive failure signal.

---

## High-Level Usage

### Encoding a Bech32 / Bech32m String

```rust
use bitcoin_bech32m::{Encoding, encode};

fn main() {
    let hrp = String::from("bc");         // human-readable part, must be lowercase
    let data: Vec<u8> = vec![0, 1, 2, 3];  // 5-bit groups (0..31), not raw bytes

    // Bech32m for modern SegWit (e.g. Taproot)
    let addr = encode(Encoding::BECH32M, &hrp, &data);
    println!("{}", addr);
}
```

**Important**: `values` passed into `encode` are *already base32 symbols* (5-bit groups). The crate intentionally does not perform 8-bit ↔ 5-bit regrouping for you. This separation keeps the primitive pure and lets higher-level libraries decide how to map application data into 5-bit symbols.

### Decoding a Bech32 / Bech32m String

```rust
use bitcoin_bech32m::{decode, Encoding};

fn main() {
    let input = "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kg3g4ty";

    let res = decode(input);
    if res.encoding() == &Encoding::INVALID {
        eprintln!("Invalid Bech32/Bech32m string");
        return;
    }

    println!("encoding = {:?}", res.encoding());
    println!("hrp       = {}",  res.hrp());
    println!("data ({} symbols) = {:?}", res.data().len(), res.data());
}
```

The decoder enforces the following invariants:

- All characters must be printable ASCII in `[0x21..0x7e]`.
- No mixed case: either entirely lower-case or entirely upper-case inputs (the HRP is normalized to lower-case in the result).
- Presence of a separator `'1'` that is not the first character.
- Total length ≤ 90 characters.
- At least 6 checksum symbols (`sep + 7 ≤ input.len()`).
- All data characters must live in the Bech32 alphabet.
- Checksum must match Bech32 or Bech32m.

If any of these checks fail, you receive `DecodeResult::default()` with `Encoding::INVALID`.

---

## Lower-Level Primitives

These functions are useful when you need deeper control or are designing custom protocols on top of Bech32/Bech32m.

### `encoding_constant`

```rust
pub fn encoding_constant(encoding: Encoding) -> u32
```

Returns the final XOR constant used in the checksum definition:

- For `Encoding::BECH32`  → `1`
- For `Encoding::BECH32M` → `0x2bc830a3`

The function asserts that `encoding` is either `BECH32` or `BECH32M`.

### `expand_hrp`

```rust
pub fn expand_hrp(hrp: &String) -> Vec<u8>
```

Expands the human-readable part into a sequence of 5-bit-like values used as the prefix to the checksum computation. This follows BIP-173 exactly:

- High 5 bits of each character.
- A zero separator.
- Low 5 bits of each character.

This is part of the domain separation between HRP and data.

### `poly_mod`

```rust
pub fn poly_mod(v: &Vec<u8>) -> u32
```

Computes the polynomial remainder used by the Bech32/Bech32m checksum algorithm. The input vector is interpreted as coefficients in GF(32) as specified in the BIP and associated finite-field analysis:

- Remainder is evolved iteratively as symbols are appended.
- Internally, state `c` is a 30-bit packed representation of the coefficients modulo the generator polynomial.
- Conditional XOR with precomputed constants implements multiplication by `{1, 2, 4, 8, 16}` in GF(32) times the `k(x) = x^6 mod g(x)` term.

This is an excellent hook if you need to:

- Validate your own implementation by cross-checking remainders.
- Explore alternative BCH generator polynomials or code parameters.
- Perform incremental checksum updates as data streams in.

### `create_checksum`

```rust
pub fn create_checksum(encoding: Encoding, hrp: &String, values: &Vec<u8>) -> Vec<u8>
```

Builds the 6-symbol checksum for a given HRP and data vector:

1. Concatenates `expand_hrp(hrp)` and `values`.
2. Appends six zero symbols.
3. Runs `poly_mod` and XORs the result with `encoding_constant(encoding)`.
4. Decomposes the resulting 30-bit value into six 5-bit symbols.

The returned vector can be appended to the original data to produce a fully checksummed sequence.

### `verify_checksum`

```rust
pub fn verify_checksum(hrp: &String, values: &Vec<u8>) -> Encoding
```

Determines whether the provided HRP and data+checksum vector is consistent with either Bech32 or Bech32m:

- Returns `Encoding::BECH32` if the remainder equals `encoding_constant(BECH32)`.
- Returns `Encoding::BECH32M` if the remainder equals `encoding_constant(BECH32M)`.
- Returns `Encoding::INVALID` otherwise.

This lets you distinguish which variant a decoded string uses, which is essential for correct downstream semantics (e.g. SegWit version validation).

### `encode`

```rust
pub fn encode(encoding: Encoding, hrp: &String, values: &Vec<u8>) -> String
```

Constructs a full Bech32/Bech32m string from:

- A lower-case HRP (asserts that no characters in `hrp` are upper-case).
- Data symbols `values` in `0..=31`.
- A 6-symbol checksum computed via `create_checksum`.

The final string is:

```text
<lowercase-hrp> "1" <data+checksum encoded using the Bech32 charset>
```

If you supply a HRP containing any uppercase letters, the function will panic via assertion failure. This is deliberate: using a mixed-case HRP breaks the canonical encoding properties required by BIP-173/350.

### `decode`

```rust
pub fn decode(input: &str) -> DecodeResult
```

Comprehensive decoder that:

1. Validates character range and case uniformity.
2. Locates the separator `'1'` and checks length constraints.
3. Translates the post-separator section into 5-bit values using an internal reverse-charset table.
4. Normalizes the HRP to lower-case.
5. Runs `verify_checksum` to identify encoding type or reject the string.

This is the entry point you will generally use for reading Bech32/Bech32m strings.

### Utility: `cat`

```rust
pub fn cat<T: Copy>(v1: Vec<T>, v2: &Vec<T>) -> Vec<T>
```

Efficient vector concatenation that takes ownership of the first vector and appends the second by copying. Used internally in checksum construction, but also usable for general concatenation in Bech32-related pipelines.

### Utility: `lower_case`

```rust
pub fn lower_case(c: u8) -> u8
```

Maps a single ASCII byte to its lowercase form, panicking if the conversion fails. Primarily useful where you need to enforce canonical casing on raw bytes rather than `char`s.

---

## Error Handling and Invariants

This crate prefers **deterministic failure modes** with simple semantics:

- **Decoding** never panics on malformed input — instead it returns `DecodeResult::default()`.
- **Encoding** may panic if used incorrectly, specifically:
  - HRP contains uppercase characters.
  - An invalid `Encoding` variant is passed to `encoding_constant` (the function asserts that the variant is one of the two valid constants).

These invariants are intended to keep protocol-facing code predictable while surfacing programmer errors early in development.

---

## Integration Patterns

### Building Bitcoin Address Support

Typical Bitcoin address stacks perform the following steps:

1. Derive a witness program (version + program bytes).
2. Convert uint8 data into 5-bit base32 symbols.
3. Use `encode(Encoding::BECH32 or Encoding::BECH32M, &hrp, &symbols)` to produce the final address.
4. On decoding, use `decode`, check `Encoding`, HRP, and then convert the `data()` 5-bit symbols back into 8-bit bytes.

`bitcoin-bech32m` intentionally focuses on steps (3) and (4). You are free to choose any bit-grouping strategy (or integrate an existing one) without being constrained by this crate.

### Custom Protocol Tags

Because the HRP is an arbitrary ASCII string, you can define protocol-specific prefixes (e.g. `"cust1"`, `"sig1"`) and still benefit from Bech32m's strong error-detection guarantees. Use `Encoding::BECH32M` for new constructions unless you have a compatibility requirement for the original Bech32.

---

## Performance Considerations

- Uses pre-allocation (`reserve`) in key paths to reduce allocations.
- The checksum state machine (`poly_mod`) is branch-predictable with a small set of conditional XOR operations.
- Suitable for high-throughput address validation or encoding in services that handle large volumes of Bitcoin-related traffic.

The algorithmic complexity is linear in input length, with very small constant factors.

---

## License

This crate is licensed under the **MIT** license.

See the repository for details:

- Repository: <https://github.com/klebs6/bitcoin-rs>

---
