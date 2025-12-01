# bitcoin-network

A low-level, Bitcoin Core–compatible network address library for Rust. It focuses on precise modelling of Bitcoin's internal network addressing, serialization, reachability, and bucketing logic, including full support for ADDRv1/ADDRv2 (BIP155) and overlay networks (Tor v3, I2P, CJDNS).

## Overview

`bitcoin-network` provides a faithful Rust translation of Bitcoin Core's `CNetAddr` and associated logic. The crate is designed for systems that need to:

- Parse, normalise, and classify network endpoints used in the Bitcoin P2P layer.
- Support both legacy ADDRv1 and modern ADDRv2/BIP155 address encodings.
- Reason about reachability across IPv4, IPv6, Tor v3, I2P, CJDNS, and internal pseudo‑addresses.
- Compute peer grouping and AS‑based bucketing identifiers compatible with Core's AddrMan.
- Interoperate with existing Bitcoin ecosystem crates (`bitcoin-bitstream`, `bitcoin-hash`, `bitcoin-asmap`, `bitcoin-string`, etc.).

The crate gives you the same semantics as Core for address validity, routability, reachability scoring, group identifiers, and overlay-encoding. This is essential if you want a Rust implementation to behave identically to Core peers on the public network.

## Core Types

### `Network`

```rust
#[repr(u8)]
#[derive(Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum Network {
    NET_UNROUTABLE,
    NET_IPV4,
    NET_IPV6,
    NET_ONION,
    NET_I2P,
    NET_CJDNS,
    NET_INTERNAL,
    NET_MAX,
}
```

`Network` classifies addresses into high-level network classes. These are used for:

- Reachability decisions (e.g. whether an address is considered publicly reachable).
- BIP155 network id mapping.
- AddrMan bucketing and group formation.

Semantics mirror Bitcoin Core exactly:

- `NET_UNROUTABLE` – invalid / unroutable / placeholder.
- `NET_IPV4`, `NET_IPV6` – global IP networks.
- `NET_ONION` – Tor v3 `.onion` services.
- `NET_I2P` – I2P `.b32.i2p` destinations.
- `NET_CJDNS` – CJDNS addresses (must start with `0xfc`).
- `NET_INTERNAL` – pseudo‑addresses used by AddrMan for DNS seeds, internal bookkeeping, etc.
- `NET_MAX` – sentinel (not a real network, used for sizing / iteration bounds in Core).

### `BIP155Network`

```rust
#[repr(u8)]
pub enum BIP155Network {
    IPV4,
    IPV6,
    TORV2,
    TORV3,
    I2P,
    CJDNS,
}
```

Internal representation of BIP155 network ids. TORv2 is kept only to validate/ignore legacy payloads; the crate enforces TORv3 addresses for new encodings.

### `NetAddr`

```rust
#[derive(Builder, Setters, Getters, MutGetters, Debug, Serialize, Deserialize, Clone, Hash)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
#[builder(setter(into))]
pub struct NetAddr {
    addr:     PreVector<u8, ADDR_IPV6_SIZE>,
    net:      Network,
    scope_id: u32,
}
```

`NetAddr` represents a single network address in the sense of Bitcoin Core:

- `addr` holds the raw address bytes. Its interpretation depends on `net`.
- `net` is the high-level `Network` classification.
- `scope_id` is used for IPv6 scoped addresses (e.g. link‑local with an interface index).

The crate provides a large number of methods that match the C++ behaviour, including:

- Construction and conversion from `InAddr` (IPv4) and `In6Addr` (IPv6).
- Parsing of Tor v3 `.onion` strings and I2P `.b32.i2p` strings.
- Mapping and unmapping of IPv4⟷IPv6 embeddings (RFC6052, RFC6145, 6to4, Teredo).
- BIP155 (ADDRv2) serialization / deserialization.
- Legacy ADDRv1 serialization / deserialization.
- Validity, routability, and classification according to a wide range of IANA/IETF registry ranges (RFC1918, RFC3927, RFC4193, RFC5737, RFC3849, RFC4843, RFC7343, etc.).
- AS‑based mapping with an `asmap` bitvector for AddrMan.

## Features by Domain

### 1. Address Formatting Helpers

The crate contains helpers to render addresses to canonical string representations with the same rules as Core:

```rust
pub fn ipv4_to_string(a: &[u8]) -> String;

pub fn ipv6_to_string(a: &[u8], scope_id: u32) -> String;

pub fn onion_to_string(addr: &[u8]) -> String;
```

- `ipv4_to_string` renders a 4‑octet IPv4 address as `d.d.d.d`.
- `ipv6_to_string` uses `std::net::Ipv6Addr` and yields RFC 5952–compliant compressed text forms, optionally with `%scope_id` suffix.
- `onion_to_string` computes the Tor v3 checksum and version, base32 encodes the 32‑byte public key and metadata, and appends `.onion`.

### 2. BIP155 / ADDRv2 Support

`NetAddr` can serialize and unserialize itself in both legacy ADDRv1 and modern ADDRv2 forms:

```rust
impl NetAddr {
    pub fn serialize<Stream>(&self, s: &mut Stream) where
        Stream: bitcoin_bitstream::GetVersion,
        for<'s> &'s mut Stream: core::ops::Shl<u8, Output = &'s mut Stream>
            + core::ops::Shl<u64, Output = &'s mut Stream>,
        for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

    pub fn unserialize<Stream>(&mut self, s: &mut Stream) where
        Stream: bitcoin_bitstream::GetVersion + bitcoin_bitstream::Backend,
        for<'s, 'a> &'s mut Stream:
            core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u8, Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>;

    pub fn serialize_v1array(&self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]);
    pub fn serialize_v1stream<Stream>(&self, s: &mut Stream)
        where for<'s, 'a> &'s mut Stream: core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

    pub fn unserialize_v1array(&mut self, arr: &mut [u8; NET_ADDR_V1_SERIALIZATION_SIZE]);
    pub fn unserialize_v1stream<Stream>(&mut self, s: &mut Stream)
        where for<'s, 'a> &'s mut Stream: core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>;

    pub fn serialize_v2stream<Stream>(&self, s: &mut Stream) where
        for<'s> &'s mut Stream:
            core::ops::Shl<u8, Output = &'s mut Stream>
            + core::ops::Shl<u64, Output = &'s mut Stream>,
        for<'s, 'a> &'s mut Stream:
            core::ops::Shl<&'a [u8], Output = &'s mut Stream>;

    pub fn unserialize_v2stream<Stream>(&mut self, s: &mut Stream) where
        Stream: bitcoin_bitstream::Backend,
        for<'s, 'a> &'s mut Stream:
            core::ops::Shr<&'a mut u8, Output = &'s mut Stream>
            + core::ops::Shr<&'a mut u64, Output = &'s mut Stream>
            + core::ops::Shr<&'a mut [u8], Output = &'s mut Stream>;
}
```

`serialize` and `unserialize` automatically choose between v1 and v2 based on the stream's version flag (`ADDRV2_FORMAT` bit), exactly matching Core's feature gating. This makes the crate safe to use in mixed-version networks and when persisting AddrMan on disk.

The mapping between `Network` and `BIP155Network` is handled by:

```rust
impl NetAddr {
    pub fn get_bip155network(&self) -> BIP155Network;
    pub fn set_net_from_bip155network(&mut self, id: u8, address_size: usize) -> bool;
}
```

Unknown future ids are ignored (payload skipped), while mismatched founding ids trigger panics, mimicking Core's behaviour.

### 3. Validity, Routability, and Reachability

The crate encodes all of Core's validity and routing heuristics:

```rust
impl NetAddr {
    pub fn is_valid(&self) -> bool;
    pub fn is_routable(&self) -> bool;
    pub fn is_local(&self) -> bool;
    pub fn is_bind_any(&self) -> bool; // 0.0.0.0 / ::

    pub fn is_ipv4(&self) -> bool;
    pub fn is_ipv6(&self) -> bool;
    pub fn is_tor(&self) -> bool;
    pub fn isi2p(&self) -> bool;
    pub fn iscjdns(&self) -> bool;
    pub fn is_internal(&self) -> bool;

    pub fn isrfc1918(&self) -> bool;
    pub fn isrfc2544(&self) -> bool;
    pub fn isrfc3927(&self) -> bool;
    pub fn isrfc6598(&self) -> bool;
    pub fn isrfc5737(&self) -> bool;
    pub fn isrfc3849(&self) -> bool;
    pub fn isrfc3964(&self) -> bool;
    pub fn isrfc6052(&self) -> bool;
    pub fn isrfc4380(&self) -> bool;
    pub fn isrfc4862(&self) -> bool;
    pub fn isrfc4193(&self) -> bool;
    pub fn isrfc6145(&self) -> bool;
    pub fn isrfc4843(&self) -> bool;
    pub fn isrfc7343(&self) -> bool;
}
```

This allows you to reason at the same level of granularity as Core about whether an address should be gossiped, connected to, or deprioritised.

#### Reachability Metrics

`NetAddr` exposes the same integer reachability scale that Core uses in outbound peer selection and connection policy:

```rust
impl NetAddr {
    pub fn get_reachability_from(&self, paddr_partner: *const NetAddr) -> i32;
}
```

The metric differentiates between:

- Direct IPv4↔IPv4 reachability.
- Native IPv6 vs. tunneled IPv6 (6to4, Teredo, etc.).
- Overlay networks (Tor, I2P) with special cases for private communication.

The helper trait `CheckIsReachable` provides a higher‑level boolean classification:

```rust
pub trait CheckIsReachable {
    fn is_reachable(&self) -> bool;
}

impl CheckIsReachable for Network { /* TODO in this crate; wired for Core semantics */ }
impl CheckIsReachable for NetAddr {
    fn is_reachable(&self) -> bool {
        self.get_network().is_reachable()
    }
}
```

Currently `Network::is_reachable()` is a placeholder mirroring Core's `vfLimited` gating.

### 4. Overlay Parsing: Tor and I2P

To correctly support overlay networks, `NetAddr` can parse and validate textual Tor/I2P destinations:

```rust
impl NetAddr {
    pub fn set_tor(&mut self, addr: &String) -> bool;
    pub fn seti2p(&mut self, addr: &String) -> bool;
    pub fn set_special(&mut self, addr: &String) -> bool; // Tor or I2P

    pub fn to_stringip(&self) -> String;
    pub fn to_string(&self) -> String; // alias for to_stringip
}
```

Tor v3 handling matches spec:

- Parses `*.onion` hostnames.
- Base32‑decodes payload without padding.
- Splits into `PUBKEY | CHECKSUM | VERSION`.
- Verifies version byte.
- Recomputes checksum as
  `SHA3_256(b".onion checksum" || PUBKEY || VERSION)[0..2]` and compares.

I2P handling similarly enforces the `52`-char base32 + `.b32.i2p` suffix, then decodes via the shared `bitcoin_string` utilities.

### 5. IPv4/IPv6 Embeddings and Legacy Encodings

`NetAddr` provides precise handling of IPv4 mapped and translated addresses as used in legacy encodings and in IPv6 transition mechanisms:

```rust
impl NetAddr {
    pub fn has_linked_ipv4(&self) -> bool;
    pub fn get_linked_ipv4(&self) -> u32;

    pub fn set_legacy_ipv6(&mut self, ipv6: &[u8]);
    pub fn is_addr_v1compatible(&self) -> bool;
}
```

- `set_legacy_ipv6` decodes embedded IPv4, INTERNAL, and deprecated TORv2 prefixes into the appropriate `Network` variant and canonical `addr` bytes.
- `has_linked_ipv4` and `get_linked_ipv4` reconstruct the equivalent IPv4 address from IPv6 forms (6to4, SIIT translated, Teredo, etc.), always in network byte order.
- `is_addr_v1compatible` tests whether the address can be encoded in ADDRv1 (`NET_IPV4`, `NET_IPV6`, `NET_INTERNAL`).

### 6. AS‑based Bucketing and Grouping

For peer selection and sybil resistance, Core uses an AS map (`asmap`) and group identifiers. This crate reproduces that behaviour exactly:

```rust
impl NetAddr {
    pub fn get_mappedas(&self, asmap: &Vec<bool>) -> u32;
    pub fn get_group(&self, asmap: &Vec<bool>) -> Vec<u8>;
}
```

- `get_mappedas` interprets the IP as a 128‑bit bitstring, then uses `bitcoin_asmap::interpret` to map into an autonomous system number. If there is no mapping (or `asmap` is empty, or the address is non‑IPv4/IPv6), it returns `0`, which is reserved by RFC7607.
- `get_group` uses either the AS mapping or network‑specific prefix rules to return a canonical byte vector representing the address group. No two outbound connections are attempted to addresses with the same group, mirroring Core's design:
  - IPv4: /16 prefix buckets.
  - IPv6: /32 or /36 for HE.net, depending on address.
  - Tor/I2P/CJDNS: /4 prefix in their overlay space.
  - Internal: full 10‑byte prefix.
  - Local & unroutable: single shared group by network class.

### 7. Hashing and Byte Representation

For indexing and deduplication, `NetAddr` provides deterministic, Core‑compatible hashing and raw byte extraction:

```rust
impl NetAddr {
    pub fn get_addr_bytes(&self) -> Vec<u8>;
    pub fn get_hash(&self) -> u64;
}
```

- `get_addr_bytes` returns either the legacy ADDRv1 16‑byte representation or the raw `addr` payload, depending on `is_addr_v1compatible`.
- `get_hash` computes a 256‑bit hash (`bitcoin_hash::hash1`, double‑SHA256‑style) over the raw address bytes and returns the first 64 bits in little‑endian order as a `u64`, following Core's conventions.

## Example Usage

### Creating and Formatting Addresses

```rust
use bitcoin_network::{NetAddr, Network};

fn basic_examples() {
    // IPv4: 1.2.3.4
    let ipv4_bytes = [1u8, 2, 3, 4];
    let mut ipv4 = NetAddr::default();
    *ipv4.net_mut()  = Network::NET_IPV4;
    *ipv4.addr_mut() = PreVector::from(&ipv4_bytes[..]);

    assert!(ipv4.is_ipv4());
    assert!(ipv4.is_valid());
    assert_eq!(ipv4.to_string(), "1.2.3.4");

    // A Tor v3 address
    let mut tor = NetAddr::default();
    let ok = tor.set_tor(&"pg6mmjiyjmcrsslvykfwnntlaru7p5svn6y2ymmju6nubxndf4pscryd.onion".to_string());
    assert!(ok);
    assert!(tor.is_tor());
    println!("Tor address: {}", tor.to_string());
}
```

### Serializing with ADDRv2 / BIP155

```rust
use bitcoin_network::NetAddr;
use bitcoin_bitstream::{MemoryStream, GetVersion};

const ADDRV2_FORMAT: u32 = 1 << 31; // Example feature bit; use the same as in your stack.

fn roundtrip_bip155(addr: &NetAddr) {
    let mut stream = MemoryStream::new();
    stream.set_version(ADDRV2_FORMAT);

    // Serialize in ADDRv2 format
    addr.serialize(&mut stream);

    // Reset read cursor and unserialize
    let mut decoded = NetAddr::default();
    decoded.unserialize(&mut stream);

    assert_eq!(addr.get_net_class(), decoded.get_net_class());
    assert_eq!(addr.get_addr_bytes(), decoded.get_addr_bytes());
}
```

### Using AS Maps for Grouping

```rust
use bitcoin_network::NetAddr;

fn bucket_key(addr: &NetAddr, asmap: &Vec<bool>) -> Vec<u8> {
    // Returns a stable identifier representing the AddrMan bucket/group.
    addr.get_group(asmap)
}
```

## Algorithmic and Standards Background

The crate embeds a substantial amount of network‑theoretic and protocol knowledge:

- **BIP155 / ADDRv2**: variable‑length address payloads tagged with network ids, enabling extensible support for non‑IP overlays without abusing IPv6 space.
- **IPv6 Text Representation**: RFC 5952 mandates canonical zero compression and lowercase hex; using `std::net::Ipv6Addr` ensures consistent formatting.
- **Transition Mechanisms**: 6to4 (RFC3964), Teredo (RFC4380), IPv4‑embedded formats (RFC6052, RFC6145), autoconfiguration (RFC4862) are used in reachability classification.
- **Private and Documentation Ranges**: RFC1918, RFC3927, RFC5737, RFC3849, RFC4193, RFC4843, RFC7343 are explicitly vetted in `is_valid` and `is_routable`.
- **Autonomous System Mapping**: `asmap` is a compressed DFA over IP bits. `get_mappedas` builds a 128‑bit boolean vector representation and feeds it into `bitcoin_asmap::interpret`, which yields a stable AS number used for bucket diversification.

This crate effectively exports the same policy surface that Core uses, allowing you to replicate its peer selection and address management logic precisely in Rust.

## Safety and Panics

Many methods are designed under the assumption of internal invariants, just as in Core:

- Several functions `assert!` on address length (e.g., IPv4 must be 4 bytes, IPv6 must be 16 bytes, Tor v3 must be 32 bytes, etc.).
- Certain misconfigurations (e.g. founding BIP155 ids with incorrect payload sizes, `NET_UNROUTABLE`/`NET_MAX` used as actual network types) panic.
- Pointer‑based APIs (`get_in_addr`, `get_in_6addr`, `get_reachability_from`) use `unsafe` and assume the caller provides valid pointers.

This is intentional: the crate is engineered as a near drop‑in for Core internals, and invalid data is expected to be filtered or asserted away earlier in the call chain.

## Integration Notes

- **Rust Edition**: 2021.
- **License**: MIT.
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Intended Consumers**: implementers of Bitcoin P2P stacks, alternative full node implementations, network simulators, asmap tooling, and research systems requiring bit‑exact compatibility with Bitcoin Core's address handling.

To use the crate in your project:

```toml
[dependencies]
bitcoin-network = "0.1.19"
```

Then import the relevant items:

```rust
use bitcoin_network::{NetAddr, Network, BIP155Network, CheckIsReachable};
```

If you integrate with other `bitcoin-rs` components, the dependency set and versions should be aligned with the `bitcoin-rs` workspace for maximum compatibility.
