# bitcoin-u160

A minimal, Bitcoin-flavored 160‑bit opaque blob type for Rust.

`bitcoin-u160` provides a small, allocation‑free wrapper around a fixed 160‑bit buffer. It is designed for identifiers like Bitcoin address hashes and similar cryptographic digests that are conventionally 160 bits wide but are not manipulated as integers.

---

## Design goals

- **Opaque, non‑numeric**: The type is intentionally *not* an integer. No arithmetic is provided, avoiding semantic confusion between hashes and numbers.
- **Deterministic, fixed size**: Always 160 bits (20 bytes). Layout is stable and transparent via slices.
- **Bitcoin‑style semantics**: Mirrors the conventions of 160‑bit blobs in the Bitcoin reference implementation.
- **No heap allocations**: Everything is stack‑allocated and `Copy`‑free, with efficient slice access via `AsRef<[u8]>` / `AsMut<[u8]>`.
- **Formatted as hex**: Human‑readable representation is lower‑case hex, suitable for logs and APIs.

---

## Core type

```rust
#[derive(Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct u160 {
    blob: BaseBlob160,
}
```

`u160` is a thin newtype around an internal `BaseBlob160` that owns exactly 20 bytes. The `u160` name is historical: the type does **not** model an integer and does not provide numeric operations.

### Trait implementations

- `Clone`, `Default`, `PartialOrd`, `Ord`, `PartialEq`, `Eq`, `Hash`
- `core::fmt::Debug`: prints as `u160(<hex>)` to make the type visually obvious in logs and tests.
- `core::fmt::Display`: prints the canonical hex string (delegates to `to_string`).
- `Deref<Target = BaseBlob160>` / `DerefMut`: enables seamless reuse of `BaseBlob160` methods.
- `AsRef<[u8]>` / `AsMut<[u8]>`: borrow the underlying 20‑byte slice with no copying.

### Constructors and utilities

```rust
impl u160 {
    /// Constant size in bytes for a 160‑bit blob.
    pub fn byte_len(&self) -> usize { 20 }

    /// Construct from a 20‑byte array at compile time.
    pub fn from_bytes_20(arr: [u8; 20]) -> Self { /* ... */ }

    /// Hex‐encode as a String.
    pub fn to_string(&self) -> String { /* ... */ }

    /// Construct from a little‑endian 20‑byte array.
    ///
    /// Reverses the byte order into the canonical big‑endian
    /// representation, mirroring `u256` behavior.
    pub fn from_le_bytes(mut bytes: [u8; 20]) -> Self { /* ... */ }
}
```

#### From hex strings

```rust
impl From<&str> for u160 {
    fn from(x: &str) -> Self { /* parse hex into internal blob */ }
}

impl From<&String> for u160 {
    fn from(x: &String) -> Self { /* parse hex */ }
}
```

These constructors interpret the input as a hex string and populate the 160‑bit blob accordingly. Invalid hex will typically surface via panics or internal validation errors depending on `BaseBlob160`.

#### From raw bytes (vec)

```rust
impl From<&Vec<u8>> for u160 {
    fn from(vch: &Vec<u8>) -> Self {
        let required_len = 160 / 8; // 20 bytes
        if vch.len() != required_len {
            panic!(
                "u160::from(&Vec<u8>): input must be {} bytes, got={}",
                required_len,
                vch.len(),
            );
        }
        // copy into internal blob
    }
}
```

This is a strict constructor: it **panics** if the slice length is not exactly 20 bytes. Use it when you have a protocol‑validated buffer and prefer fail‑fast behavior.

#### Endianness

`u160::from_le_bytes` is explicitly little‑endian: it expects bytes ordered least‑significant first and reverses them into the blob's canonical big‑endian ordering.

This matches the pattern of primitive integer constructors in the standard library (e.g., `u32::from_le_bytes`) but is applied to a 160‑bit opaque identifier. In Bitcoin‑style protocols, this is useful when serializing or deserializing network formats that store hashes in reversed byte order.

### Slice access

```rust
impl AsRef<[u8]> for u160 {
    fn as_ref(&self) -> &[u8] { /* &blob[..] */ }
}

impl AsMut<[u8]> for u160 {
    fn as_mut(&mut self) -> &mut [u8] { /* &mut blob[..] */ }
}
```

This makes `u160` easy to feed into cryptographic APIs, network encoders, and storage layers without additional copying or boxing.

---

## Usage examples

### Basic construction and formatting

```rust
use bitcoin_u160::u160;

fn main() {
    // From hex string
    let id: u160 = "0123456789abcdef0123456789abcdef01234567".into();

    // Debug and Display
    assert!(format!("{:?}", id).starts_with("u160("));
    let hex = id.to_string();
    println!("u160 as hex: {}", hex);

    // Length
    assert_eq!(id.byte_len(), 20);
}
```

### From raw bytes

```rust
use bitcoin_u160::u160;

fn from_bytes_example() {
    // Little‑endian constructor for interoperability with protocols
    let le_bytes: [u8; 20] = [0u8; 20];
    let id = u160::from_le_bytes(le_bytes);

    // Direct 20‑byte constructor (canonical order)
    let canonical: [u8; 20] = [1u8; 20];
    let id2 = u160::from_bytes_20(canonical);

    // AsRef<[u8]> for crypto / IO APIs
    let slice: &[u8] = id.as_ref();
    assert_eq!(slice.len(), 20);
}
```

### Integrating with cryptographic or protocol code

`u160` is particularly useful for:

- 160‑bit hash digests (e.g., RIPEMD‑160, HASH160) used in Bitcoin and similar systems.
- Compact identifiers or keys where the size and exact representation must be controlled.
- Structures that need deterministic ordering and hashing (e.g., BTreeMap / HashMap keys).

Example: receiving a network message with a 20‑byte identifier in little‑endian order:

```rust
use bitcoin_u160::u160;

fn parse_id(buf: &[u8]) -> u160 {
    assert_eq!(buf.len(), 20);
    let mut arr = [0u8; 20];
    arr.copy_from_slice(buf);
    u160::from_le_bytes(arr)
}
```

---

## Endianness and representation details

In cryptographic and blockchain protocols, byte order frequently differs between human‑readable hex and on‑wire or on‑disk formats. `u160` implements the following conventions:

- **Canonical representation**: big‑endian, consistent with most hex string encodings. `Display`, `Debug`, and `to_string` use this ordering.
- **Little‑endian constructor**: `from_le_bytes` expects least‑significant byte first, then internally reverses to canonical big‑endian. This is analogous to mapping between internal integer representation and various protocol encodings, but applied to a 160‑bit opaque value.

From a mathematical perspective, if one interprets the 20‑byte array as an element of the finite ring `ℤ / 2^160ℤ`, `from_le_bytes` simply applies the involutive permutation that reverses byte indices, preserving the underlying abstract element while changing its coordinate representation.

---

## Error handling and safety

- `From<&Vec<u8>> for u160` **panics** if the input length is not exactly 20 bytes. This is deliberate for low‑level code where incorrect lengths indicate a programmer error or corrupted protocol state.
- Hex parsing via `From<&str>` / `From<&String>` assumes that the string contains a valid representation. The exact failure mode depends on `BaseBlob160`'s implementation; consult the crate source if you require strictly fallible construction APIs.

When building higher‑level systems exposed to untrusted inputs, you may want to wrap these constructors with your own checked parsing layer that returns `Result<u160, E>` rather than panicking.

---

## Logging and observability

`u160::from_le_bytes` emits a `tracing::trace!` with the provided byte array:

```rust
tracing::trace!("u160::from_le_bytes ⇒ {:02X?}", bytes);
```

This makes it easy to instrument binary protocol handling; simply enable a `trace` subscriber for the relevant target and you will see the raw bytes involved in each construction.

---

## Crate metadata

- **Crate name**: `bitcoin-u160`
- **Version**: `0.1.19`
- **Edition**: Rust 2021
- **License**: MIT
- **Repository**: <https://github.com/klebs6/bitcoin-rs>

This crate is intended to compose with the broader `bitcoin-rs` ecosystem and can also be used as a standalone primitive in other cryptographic or protocol‑oriented projects.

---

## When to use this crate

Use `bitcoin-u160` when:

- You need a compact, deterministic 160‑bit identifier in Rust.
- You care about explicit control over byte order and textual representation.
- You want a type that integrates naturally with Rust's ordering, hashing, and formatting traits without imposing arithmetic semantics.

If your use case genuinely requires arithmetic over a 160‑bit integer (e.g., modular arithmetic, multi‑precision operations), consider a big‑integer crate instead; `u160` is deliberately non‑numeric.
