# bitcoin-u256

A minimal, Bitcoin-core–faithful implementation of 256‑bit integers and blobs, extracted from the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) project. It provides:

* `u256`: an **opaque 256‑bit blob** used for block / tx hashes and IDs
* `ArithU256`: a **256‑bit unsigned big integer** with full arithmetic and bit‑ops
* Conversions between the two, matching Bitcoin Core’s limb layout and semantics
* Bitcoin’s **compact difficulty** encoding/decoding (`nBits`) on top of `ArithU256`
* Serde serialization for `u256`

---

## Design overview

This crate mirrors Bitcoin Core’s split between:

* `uint256` (here: `u256`): logically a 256‑bit **blob**. No arithmetic; just ordering, equality, hashing, byte access, and hex conversions. This corresponds to how block and transaction hashes are usually modelled: as 32‑byte identifiers rather than numeric values.
* `arith_uint256` (here: `ArithU256`): a 256‑bit **unsigned integer** represented internally as 8 32‑bit limbs with full arithmetic and bitwise operations. This is used for difficulty computations, work calculations, etc.

The separation keeps identifier semantics distinct from arithmetic semantics, which is important in Bitcoin where endianness and representation details frequently cause bugs.

Both types are thin wrappers around internal `BaseBlob256` / `BaseUInt256` primitives exposed by the parent project. The public API is intentionally close to Bitcoin Core’s C++ interfaces so that porting logic from upstream is nearly mechanical.


## Endianness model

The key concept to internal correctness here is **limb endianness**.

* `ArithU256` stores its value as 8 32‑bit limbs: `pn[0] .. pn[7]`.
* The conversion functions `arith_to_uint256` and `uint_to_arith256` write/read each 32‑bit limb in **little‑endian** order into/from the 32‑byte `u256` buffer.

Mathematically, a 256‑bit unsigned integer

\[
N = \sum_{i=0}^{7} \text{limb}_i \cdot 2^{32 i}
\]

is represented internally by `BaseUInt256` as `limb_i = pn[i]`. The mapping to the 32‑byte `u256` is then:

\[
\text{bytes}[4i..4i+4] = \text{LE32}(\text{limb}_i).
\]

This exactly matches Bitcoin Core’s encoding and is critical for correctness when computing difficulty or work from consensus data.

Additionally, `u256::from_le_bytes` performs a full 32‑byte reversal before delegating to `from_bytes_32`, because `BaseBlob` uses a big‑endian internal representation of the hex string, while Bitcoin often publishes hashes as little‑endian numbers. This inversion matches upstream’s behaviour.


## Core types

### `u256`: 256‑bit opaque blob

```rust
#[derive(Getters, Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[getset(get = "pub")]
#[allow(non_camel_case_types)]
pub struct u256 {
    blob: BaseBlob256,
}
``

**Characteristics:**

* Exactly 32 bytes
* Totally ordered (`Ord`), hashable (`Hash`), clonable (`Clone`), send + sync
* Implements `Default` (all zero), `Debug`, `Display` (hex), `Serialize`, and `Deserialize`
* Exposes multiple byte views:
  * `as_slice(&self) -> &[u8]`
  * `as_slice_mut(&mut self) -> &mut [u8]`
  * `as_slice_exact(&self) -> &[u8; 32]`
  * `as_mut_slice_exact(&mut self) -> &mut [u8; 32]`

**Helpers / constructors:**

```rust
impl u256 {
    pub fn zero() -> Self;           // all 0s
    pub fn one() -> Self;            // least-significant byte = 1
    pub fn byte_len(&self) -> usize; // always 32

    pub fn is_null(&self) -> bool;
    pub fn set_null(&mut self);

    pub fn to_string(&self) -> String;        // hex (big-endian style)
    pub fn from_bytes_32(arr: [u8; 32]) -> Self;
    pub fn from_le_bytes(le: [u8; 32]) -> Self; // reverses before storing

    pub fn get_uint64(&self, index: usize) -> u64; // 0 <= index < 4, little-endian limbs
    pub fn low64(&self) -> u64;                   // lower 64 bits, little-endian
}

impl From<&[u8; 32]> for u256 { /* via AsRef */ }
impl From<&Vec<u8>> for u256;    // panics if len != 32
impl From<u8> for u256;
impl From<*const u8> for u256;   // hex-encoded C string
impl From<&String> for u256;     // hex string
impl From<&str> for u256;        // hex string
```

### `ArithU256`: 256‑bit unsigned big integer

```rust
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArithU256 {
    base: BaseUInt256,
}
```

`ArithU256` models a 256‑bit integer with full arithmetic and logic operations. It supports:

* Addition, subtraction, multiplication, division:
  * `Add`, `AddAssign` with `&ArithU256` and `u64`
  * `Sub`, `SubAssign` with `&ArithU256` and `u64`
  * `Mul`, `MulAssign` with `u32`, `i64`, `u64`, `&ArithU256`
  * `Div`, `DivAssign` with `u32`, `i64`, `&ArithU256`
* Bitwise operations:
  * `BitAnd`, `BitAndAssign` with `ArithU256`, `&ArithU256`, `u64`
  * `BitOr`, `BitOrAssign` with `ArithU256`, `&ArithU256`, `u64`
  * `BitXor`, `BitXorAssign` with `ArithU256`, `&ArithU256`, `u64`
  * `Not` (bitwise complement)
* Shift operations:
  * `Shl<u32>`, `ShlAssign<u32>`
  * `Shr<u32>`, `ShrAssign<u32>`
* Negation:
  * `Neg` implemented as modular negation modulo 2²⁵⁶ via two’s complement (`~x + 1`).

Additional helpers:

```rust
impl ArithU256 {
    pub fn low64(&self) -> u64;
    pub fn get_limb(&self, index: usize) -> u32;       // 0 <= index < limb_count
    pub fn get_hex(&self) -> String;                   // big-endian hex
    pub fn size_in_bytes(&self) -> usize;              // usually 32
    pub fn getdouble(&self) -> f64;                    // lossy > ~53 bits

    pub fn set_compact(
        &mut self,
        n_compact: u32,
        pf_negative: *mut bool,
        pf_overflow: *mut bool,
    ) -> &mut ArithU256;

    pub fn get_compact(&self, negative: Option<bool>) -> u32;
}

impl From<&BaseUInt256> for ArithU256;
impl From<u64> for ArithU256;
impl From<&str> for ArithU256;   // hex string
```

#### Compact difficulty format

`set_compact` and `get_compact` encode/decode Bitcoin’s **compact target representation** as used in `nBits`:

* `n_compact` is a 32‑bit unsigned integer with the layout:
  * `[31:24]` – exponent (number of bytes of the target)
  * `[23:0]`  – mantissa
  * bit 23 (0x0080_0000) – sign bit

The effective target `N` is:

\[
N = (-1)^{\text{sign}} \cdot \text{mantissa} \cdot 256^{\text{exponent} - 3}.
\]

This matches the C++ implementation in Bitcoin Core, including overflow rules and negative flag handling. The implementation avoids going through OpenSSL / MPI and instead manipulates limbs directly, which is both simpler and faster in pure Rust.


## Conversions between `u256` and `ArithU256`

Two conversion functions provide a precise bridge between the blob and arithmetic worlds:

```rust
pub fn arith_to_uint256(a: &ArithU256) -> u256;
pub fn uint_to_arith256(a: &u256) -> ArithU256;
```

* `arith_to_uint256`:
  * Takes each 32‑bit limb from `ArithU256::base` via `get_limb(i)`
  * Converts it to 4 little‑endian bytes and writes them into the `u256` slice at offset `4*i`
* `uint_to_arith256`:
  * Reads 4‑byte chunks from the `u256` slice, interprets them as little‑endian `u32`, and stores them as limbs via `set_limb(i, val)`

In pseudocode, for `i = 0..7`:

```text
u256[4*i..4*i+4] = LE32(ArithU256.limb[i])
```

This is exactly the logic used in upstream Bitcoin Core. When porting any consensus or difficulty‑related code, you can assume this mapping.


## CheckpointData

```rust
pub type MapCheckpoints = HashMap<i32, u256>;

#[derive(Default)]
pub struct CheckpointData {
    map_checkpoints: MapCheckpoints,
}

impl CheckpointData {
    /// Return the highest (final) checkpoint height.
    pub fn get_height(&self) -> i32 { /* ... */ }
}
```

This struct models a simple mapping from block height to checkpoint hashes (`u256`). It supports:

* `map_checkpoints`: `HashMap<i32, u256>` containing height → block hash
* `get_height()`: returns the maximum height present (0 if empty), mirroring C++ `mapCheckpoints.rbegin()->first`.

This is primarily useful when constructing chain parameters or validating that a given chain tip is beyond a known checkpoint.


## Serde integration

`u256` implements `serde::Serialize` and `serde::Deserialize`:

* Serialized as raw bytes (`serialize_bytes`) using the internal layout (32 bytes).
* Deserialized from a byte slice which must be exactly 32 bytes.

This is appropriate for binary protocols or persistence layers where you want the canonical fixed‑width representation. If you need hex string encoding, wrap `u256` in your own newtype with a custom serializer, or use `to_string()` / `From<&str>` at your API boundary.


## Trait implementations for `u256`

`u256` is wired for ergonomic use as a low‑level blob:

* `Default` – all zeros
* `Debug` – `u256(<hex>)`
* `Display` – hex string
* `Clone`, `Copy` (by manual clone), `Eq`, `Ord`, `Hash`
* `AsRef<[u8]>`, `AsMut<[u8]>` – borrow as `[u8]`
* `AsRef<[u8; 32]>`, `AsMut<[u8; 32]>` – exact length view
* `Send`, `Sync` – safe to move/share between threads

This makes `u256` suitable as a key in maps, inclusion in network messages, and use in any crypto or protocol‑level context that expects a 32‑byte identifier.


## Usage examples

Add to `Cargo.toml`:

```toml
[dependencies]
bitcoin-u256 = "0.1.19"
```

### Constructing and inspecting `u256`

```rust
use bitcoin_u256::u256;

// From a hex string (big-endian representation)
let h = u256::from("0000000000000000000b4d0d2c5b1f...");

// From literal bytes
let raw: [u8; 32] = [0u8; 32];
let id = u256::from_bytes_32(raw);

// Check zero / one helpers
assert!(u256::zero().is_null());
assert_eq!(u256::one().low64(), 1);

// Hex display
println!("block hash = {}", h);

// Access as bytes
let bytes: &[u8] = h.as_ref();
assert_eq!(bytes.len(), 32);
```

### Using `ArithU256` for difficulty math

```rust
use bitcoin_u256::{ArithU256, arith_to_uint256, uint_to_arith256};

// From a hex target
let mut target = ArithU256::from("00000000FFFF0000000000000000000000000000000000000000000000000000");

// Multiply by 2
target <<= 1;

// Convert to blob for storage / wire format
let target_blob = arith_to_uint256(&target);

// Later, restore arithmetic form
let target_again = uint_to_arith256(&target_blob);
assert_eq!(target, target_again);
```

### Working with compact difficulty (`nBits`)

```rust
use bitcoin_u256::ArithU256;

// Example compact representation from a block header
let n_bits: u32 = 0x1d00ffff; // Bitcoin mainnet genesis

let mut target = ArithU256::default();
let mut negative = false;
let mut overflow = false;

// SAFETY: we pass valid pointers to local flags
let target_ref = unsafe {
    target.set_compact(
        n_bits,
        &mut negative as *mut bool,
        &mut overflow as *mut bool,
    )
};

assert!(!negative);
assert!(!overflow);

// Convert back to compact for round-trip check
let n_bits_roundtrip = target_ref.get_compact(Some(false));
assert_eq!(n_bits_roundtrip, n_bits);
```

### Checkpoints

```rust
use std::collections::HashMap;
use bitcoin_u256::{u256, CheckpointData, MapCheckpoints};

let mut cps: MapCheckpoints = HashMap::new();

cps.insert(11111, u256::from("0000000069e244f73c..."));
cps.insert(33333, u256::from("00000000000291ce28..."));

let data = CheckpointData { map_checkpoints: cps };

assert_eq!(data.get_height(), 33333);
```


## Safety and panics

This crate is relatively low‑level and makes several assumptions:

* `u256::from(&Vec<u8>)` panics if the input length is not exactly 32 bytes.
* Several operations use `assert!` to enforce index bounds (e.g., `get_uint64(index)` requires `index < 4`).
* Some conversions (e.g. `MulAssign<i64>`, `DivAssign<i64>`) assume non‑negative arguments and will panic if negative values are passed.
* `from_le_bytes` assumes the array is exactly 32 bytes (enforced by type system).

In performance‑sensitive or consensus‑critical code, you should validate inputs at API boundaries before invoking these operations.


## When to use this crate

Use `bitcoin-u256` when you need:

* A **Bitcoin‑compatible** 256‑bit integer layer for:
  * Difficulty / target computation
  * Work calculations
  * Compact target (`nBits`) encoding/decoding
* A faithful **translation of Bitcoin Core’s `uint256` / `arith_uint256` semantics** in Rust
* Zero‑cost access to internal bytes with consistent endianness mechanics

If you only need a generic 256‑bit integer without Bitcoin’s nuanced encoding/endianness rules, more general big‑integer crates might be more appropriate. This crate is tailored specifically to Bitcoin‑style 256‑bit operations.


## License

This crate is distributed under the MIT license, as declared in `Cargo.toml`.

Source repository: <https://github.com/klebs6/bitcoin-rs>
