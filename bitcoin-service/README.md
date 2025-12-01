# bitcoin-service

A low-level, `no_std`-friendly (when built without the `std`-bound dependencies) Rust port of Bitcoin Core's `CService` type and associated hashing utilities. It models a network *service* as the combination of a network address and a TCP port, together with canonical hashing and ordering semantics compatible with the Bitcoin reference implementation.

---

## Overview

`bitcoin-service` provides a thin but precise abstraction around network endpoints in the Bitcoin stack:

- `Service`: a combination of a `NetAddr` (IP/Tor/I2P/Internal) and a TCP port.
- `ServiceHash`: a `BuildHasher` for use in `HashMap`/`HashSet` keyed by `Service`, using salted SipHash.
- `GetServiceRef` / `GetServiceMut`: small traits for types that logically contain or expose a `Service` value.

The APIs closely follow the semantics of Bitcoin Core's C++ `CService` implementation, including byte-level encoding of addresses and ports, comparison ordering, and keyed hashing.

This crate lives in the broader `bitcoin-rs` repository, which aims to be a faithful, modular port of Bitcoin Core into safe(ish) Rust.

---

## Crate goals

- **Fidelity to Bitcoin Core**: Behavior and byte layouts should match `CService` so that higher-level P2P and consensus logic can be ported without semantic drift.
- **Deterministic ordering and hashing**: Implement `Eq`, `Ord`, `Hash`, and `BuildHasher` in a way that is compatible with network data structures and peer management algorithms.
- **Interoperability with libc sockets**: Provide conversions to and from `libc::sockaddr_in` / `sockaddr_in6`, for integration with low-level networking stacks.
- **Ergonomic Rust API**: Simple constructors, conversion helpers, and trait abstractions for use across the `bitcoin-rs` workspace.

---

## Data model

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Service {
    base: NetAddr,
    port: u16,
}
```

A `Service` is conceptually:

- `base: NetAddr`: the underlying network address (IPv4, IPv6, Tor, I2P, internal, etc.).
- `port: u16`: the TCP port in host byte order.

From a Bitcoin P2P perspective, a `Service` corresponds to a reachable network endpoint: e.g., a full node's advertised address, an outbound connection target, or an address book entry.

The crate also defines:

```rust
pub struct ServiceHash {
    salt_k0: u64,
    salt_k1: u64,
}
```

This type implements `BuildHasher` to construct SipHash instances with per-instance randomness. This is important when using `Service` as a key in `HashMap`/`HashSet`, to reduce hash-collision attacks and accidental worst-case behavior.

---

## Core traits

### `GetServiceRef` / `GetServiceMut`

```rust
pub trait GetServiceRef {
    fn service(&self) -> &Service;
}

pub trait GetServiceMut {
    fn service_mut(&mut self) -> &mut Service;
}
```

These traits are minimal accessor traits for types that *contain* or *are logically identified by* a `Service`. They are useful for generic algorithms that operate over peers, address entries, or routing table items.

**Example:**

```rust
use bitcoin_service::{GetServiceRef, GetServiceMut, Service};

struct Peer {
    addr: Service,
    // other fields
}

impl GetServiceRef for Peer {
    fn service(&self) -> &Service {
        &self.addr
    }
}

impl GetServiceMut for Peer {
    fn service_mut(&mut self) -> &mut Service {
        &mut self.addr
    }
}
```

This allows you to write generic utilities that accept any `T: GetServiceRef` and work uniformly across peer representations.

---

## Hashing and ordering semantics

### `Hash` implementation

```rust
impl Hash for Service {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.net.hash(state);
        self.port.hash(state);
        self.base.addr.hash(state);
    }
}
```

The hash function incorporates:

- `base.net`: typically the network/variant of the address (e.g., IPv4, IPv6, Tor, etc.).
- `port`: the TCP port.
- `base.addr`: the underlying address bytes.

This ensures that two services collide in hash space if and only if they represent the same `(network, address, port)` triple, following Bitcoin Core's internal assumptions.

### `ServiceHash` as a `BuildHasher`

```rust
impl BuildHasher for ServiceHash {
    type Hasher = SipHasher;

    fn build_hasher(&self) -> Self::Hasher {
        SipHasher::new_with_keys(self.salt_k0, self.salt_k1)
    }
}

impl Default for ServiceHash {
    fn default() -> Self {
        Self {
            salt_k0: Self::get_rand_salt(),
            salt_k1: Self::get_rand_salt(),
        }
    }
}
```

Here, `ServiceHash` acts as a keyed SipHash factory:

- `salt_k0` and `salt_k1` are 64-bit random salts.
- `get_rand_salt()` uses `get_rand(u64::MAX)` (from the broader `bitcoin-rs` utility layer) to produce salts.

**Example usage in a `HashMap`:**

```rust
use std::collections::HashMap;
use bitcoin_service::{Service, ServiceHash};

// A hashmap keyed by Service with randomized SipHash-based hashing
let mut map: HashMap<Service, usize, ServiceHash> =
    HashMap::with_hasher(ServiceHash::default());

// insert, remove, and query as usual
```

### Equality and ordering

`Service` implements `Eq`, `PartialEq`, `Ord`, and `PartialOrd`. The semantics mirror Bitcoin Core:

- `Eq`: two `Service` values are equal if their `NetAddr` components are equal and their `port` values match.
- `Ord`: ordering first compares the `NetAddr`; if equal, compares the port.

This makes `Service` appropriate as a key in ordered maps (`BTreeMap`, `BTreeSet`) and provides deterministic, topology-related ordering consistent with the reference client.

---

## Conversions and socket interoperability

### From `libc::sockaddr_in` and `sockaddr_in6`

```rust
impl From<&libc::sockaddr_in> for Service { /* ... */ }
impl From<&libc::sockaddr_in6> for Service { /* ... */ }
```

These `From` implementations allow the crate to integrate tightly with low-level socket APIs. Conceptually:

- IPv4: constructs a `NetAddr` from `sin_addr` and sets `port` from `ntohs(sin_port)`, after asserting `AF_INET`.
- IPv6: constructs a `NetAddr` from `sin6_addr` and `sin6_scope_id` and sets `port` from `ntohs(sin6_port)`, after asserting `AF_INET6`.

### `set_sock_addr`

```rust
pub fn set_sock_addr(&mut self, paddr: *const SocketAddr) -> bool { /* ... */ }
```

`set_sock_addr` mutates the `Service` based on a `SocketAddr` (backed by `sockaddr_in` or `sockaddr_in6`), returning `true` on success and `false` for unsupported families. This aligns with the C++ logic of dispatching on `sa_family`.

### `get_sock_addr`

```rust
pub fn get_sock_addr(
    &self,
    paddr: *mut SocketAddr,
    addrlen: *mut libc::socklen_t,
) -> bool { /* ... */ }
```

`get_sock_addr` produces a concrete `sockaddr_in` or `sockaddr_in6` structure representing this service:

- For IPv4, fills a `sockaddr_in` with `AF_INET`, the underlying IPv4 bytes, and `htons(port)`.
- For IPv6, fills a `sockaddr_in6` with `AF_INET6`, the IPv6 bytes, scope id, and `htons(port)`.
- Returns `false` if the requested buffer is too small, or if the underlying `NetAddr` cannot be represented as IPv4 or IPv6.

This function is safe to call from network code that expects stable POSIX address structures, enabling integration with epoll/kqueue or custom networking stacks.

---

## Construction and helper methods

### Constructors

```rust
impl Service {
    pub fn new_from_net_addr(cip: &NetAddr, port_in: u16) -> Self { /* ... */ }

    pub fn new_from_ip4(ipv4_addr: &InAddr, port_in: u16) -> Self { /* ... */ }

    pub fn new_from_ip6(ipv6_addr: &In6Addr, port_in: u16) -> Self { /* ... */ }
}
```

The provided constructors create a `Service` from higher-level address representations:

- `new_from_net_addr`: wrap an existing `NetAddr` with a port.
- `new_from_ip4` / `new_from_ip6`: construct the underlying `NetAddr` from `InAddr` / `In6Addr` and combine with the port.

**Example:**

```rust
use bitcoin_netaddr::{NetAddr, InAddr};
use bitcoin_service::Service;

let ipv4 = InAddr::from([127, 0, 0, 1]);
let svc = Service::new_from_ip4(&ipv4, 8333);

assert_eq!(svc.get_port(), 8333);
```

### Port and key extraction

```rust
impl Service {
    pub fn get_port(&self) -> u16 { /* ... */ }

    pub fn get_key(&self) -> Vec<u8> { /* ... */ }
}
```

- `get_port`: returns the TCP port.
- `get_key`: returns a byte vector representing the service's address and port, where the port is appended in network-consistent big-endian format: `MSB`, then `LSB`. In pseudocode:

  1. `key = GetAddrBytes()` (address bytes from `NetAddr`).
  2. `key.push_back(port / 0x100)`.
  3. `key.push_back(port & 0x0FF)`.

This key is typically used for address table lookups, caches, or deduplication.

### String representations

```rust
impl Service {
    pub fn to_string_port(&self) -> String { /* ... */ }
    pub fn to_string_ip_port(&self) -> String { /* ... */ }
    pub fn to_string(&self) -> String { /* ... */ }
}
```

- `to_string_port`: returns a decimal representation of the port, e.g. `"8333"`.
- `to_string_ip_port`: formats the IP and port in a URI-appropriate way:
  - For IPv4, Tor, I2P, or internal: `ip:port`.
  - For IPv6: `[ip]:port`.
- `to_string`: alias for `to_string_ip_port`.

These formatting rules matter when interoperating with Bitcoin Core logs, user interfaces, or configuration formats, where exact string equality can be significant.

---

## Example: Using `Service` as a key in peer tracking

```rust
use std::collections::{HashMap, HashSet};
use bitcoin_service::{Service, ServiceHash};

struct PeerInfo {
    // ... details such as services, last_seen, ban score, etc.
}

// A deduplicated set of known peers
let mut known_peers: HashSet<Service, ServiceHash> =
    HashSet::with_hasher(ServiceHash::default());

// A map from peer service to its metadata
let mut peer_map: HashMap<Service, PeerInfo, ServiceHash> =
    HashMap::with_hasher(ServiceHash::default());

// For each discovered or inbound connection, derive a Service and store it
fn on_new_connection(addr: Service, peer_info: PeerInfo,
                     known_peers: &mut HashSet<Service, ServiceHash>,
                     peer_map: &mut HashMap<Service, PeerInfo, ServiceHash>) {
    if known_peers.insert(addr.clone()) {
        peer_map.insert(addr, peer_info);
    }
}
```

This pattern is typical in P2P systems: the `Service` acts as the canonical identifier for a remote endpoint.

---

## Serialization

`Service` derives `Serialize` and `Deserialize` (via Serde), enabling it to be encoded into structured formats (JSON, CBOR, bincode, etc.) or nested inside higher-level protocol data structures.

Internally, the crate intends to align serialization with the Bitcoin Core wire format semantics, but for user-facing Serde usage, you can treat `Service` as a conventional Rust struct.

**Example (JSON):**

```rust
use bitcoin_service::Service;
use serde_json;

// Assume you have some existing Service value
let svc: Service = /* ... */;

let json = serde_json::to_string(&svc)?;
let decoded: Service = serde_json::from_str(&json)?;
assert_eq!(svc, decoded);
```

---

## Integration and repository

This crate is part of the `bitcoin-rs` monorepo:

- Repository: <https://github.com/klebs6/bitcoin-rs>

Within that repository, `bitcoin-service` is one module among several, including network addresses, protocol messages, and consensus logic. It is intended to be imported directly if you only need low-level service/address abstractions, or indirectly via higher-level crates in the workspace.

---

## License

`bitcoin-service` is distributed under the MIT License, consistent with the rest of the `bitcoin-rs` repository.

---

## Status and caveats

- Several methods in the current code are present as skeletons (`todo!()`), reflecting functions ported from C++ with semantics described in comments but not fully implemented yet.
- The README describes the *intended* behavior, matching Bitcoin Core; consult the actual source and test suite in `bitcoin-rs` for the current implementation status.
- The public API may evolve as the broader `bitcoin-rs` project refines its module boundaries and trait structure.
