# bitcoin-service-flags

A small, focused Rust crate that models and evaluates Bitcoin P2P `nServices` flags, closely mirroring Bitcoin Core's C++ semantics while exposing a modern, type-safe Rust API.

This crate is intended for node implementations, indexers, routing daemons, and P2P tooling that need to:

- Represent the Bitcoin wire-level `services` bitfield in a principled way.
- Decide whether a peer is *interesting* enough to occupy an outbound slot.
- Decide whether a peer is likely to host a useful `addrman`-style address database.
- Translate service-flag bitmasks to human-readable strings (for tracing, metrics, or operator UIs).

---

## Design goals

- **Semantic fidelity to Bitcoin Core**: The logic follows the original C++ behaviour, including interpretation of `NODE_NETWORK`, `NODE_NETWORK_LIMITED`, and the global Initial Block Download (IBD) completion state.
- **Type safety**: Service flags are represented as a strongly typed `bitflags` `ServiceFlags` enum instead of raw `u64` bitfields, while still allowing unknown bits to round-trip untouched.
- **Observability**: Functions emit rich `tracing` spans and fields to assist in debugging and production monitoring.
- **Predictability**: Public functions are pure (except for the global IBD flag read) and document exactly how desirability is computed.

---

## Features

### `ServiceFlags`

```rust
use bitcoin_service_flags::ServiceFlags;

// Known service bits (mirrors Bitcoin Core):
//   NODE_NONE
//   NODE_NETWORK
//   NODE_BLOOM
//   NODE_WITNESS
//   NODE_COMPACT_FILTERS
//   NODE_NETWORK_LIMITED
```

This bitflag type is backed by `u64` and implements `Serialize`, `Deserialize` (`serde`), `Default`, and `From<u64>`.

Properties:

- `Default` is `ServiceFlags::NODE_NONE`.
- `From<u64>` preserves **all** bits, including unknown/experimental ones, by delegating to `from_bits_unchecked`. This is essential for protocol robustness: the peer's advertised services are treated as an *opaque bitmap* while still allowing explicit reasoning about known bits.

### Global IBD state

```rust
use bitcoin_service_flags::INITIAL_BLOCK_DOWNLOAD_COMPLETED;
use std::sync::atomic::Ordering;

// Mark that local node has left Initial Block Download
INITIAL_BLOCK_DOWNLOAD_COMPLETED.store(true, Ordering::Relaxed);
```

This `AtomicBool` encodes whether the **local** node has completed Initial Block Download.

The desirability of a peer's `NODE_NETWORK_LIMITED` flag is state-dependent:

- **During IBD**: Full `NODE_NETWORK` capability is preferred.
- **After IBD**: `NODE_NETWORK_LIMITED` peers become acceptable if they also satisfy other desirable bits.

This follows Bitcoin Core's logic for managing outbound connections and peers.

### Computing desirable service flags

```rust
use bitcoin_service_flags::{get_desirable_service_flags, ServiceFlags};

let peer_services = ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_WITNESS;
let desirable     = get_desirable_service_flags(peer_services);

assert!(desirable.contains(ServiceFlags::NODE_WITNESS));
```

`get_desirable_service_flags(services: ServiceFlags) -> ServiceFlags` returns the **current** set of service flags that a peer *should* advertise to be considered "interesting" for outbound slots or prioritised retention.

Behaviour:

- If `services` includes `NODE_NETWORK_LIMITED` **and** `INITIAL_BLOCK_DOWNLOAD_COMPLETED == true`:
  - Desirable flags are `NODE_NETWORK_LIMITED | NODE_WITNESS`.
- Otherwise:
  - Desirable flags are `NODE_NETWORK | NODE_WITNESS`.

This allows the caller to express both:

- Policy: which services we care about given our current sync state.
- Diagnostics: which bits we expected from a particular peer.

### Checking whether a peer already satisfies all desirable flags

```rust
use bitcoin_service_flags::{
    has_all_desirable_service_flags,
    ServiceFlags,
    INITIAL_BLOCK_DOWNLOAD_COMPLETED,
};
use std::sync::atomic::Ordering;

// Example: assume IBD is done
INITIAL_BLOCK_DOWNLOAD_COMPLETED.store(true, Ordering::Relaxed);

let peer = ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS;

if has_all_desirable_service_flags(peer) {
    // safe to keep/use this peer as a full-featured outbound
}
```

`has_all_desirable_service_flags(services: ServiceFlags) -> bool` is effectively:

```text
(services & desirable) == desirable
```

where `desirable = get_desirable_service_flags(services)`.

This is the canonical check for *"is this peer fully compliant with what we currently want?"* given our local IBD state and the peer's advertised capabilities.

### Assessing whether a peer may host a useful address database

```rust
use bitcoin_service_flags::{may_have_useful_addressdb, ServiceFlags};

let peer = ServiceFlags::NODE_NETWORK_LIMITED | ServiceFlags::NODE_WITNESS;
assert!(may_have_useful_addressdb(peer));
```

`may_have_useful_addressdb(services: ServiceFlags) -> bool` returns `true` if the peer is likely to support a **robust** on-disk address database (a la Bitcoin Core's `addrman`), i.e.:

- `services` intersects `NODE_NETWORK | NODE_NETWORK_LIMITED`.

This can be used to bias address seeding logic, peer selection for address gossip, or out-of-band bootstrap strategies.

### Human-readable rendering of service flags

Render a **single bit** index (`0..=63`) to a string:

```rust
use bitcoin_service_flags::service_flag_to_str;

assert_eq!(service_flag_to_str(0), "NETWORK");
assert_eq!(service_flag_to_str(3), "WITNESS");

// Unknown bit
assert_eq!(service_flag_to_str(42), "UNKNOWN[2^42]");
```

`service_flag_to_str(bit: usize) -> String`:

- Recognises known bits and yields stable labels:
  - `NODE_NETWORK`           → `"NETWORK"`
  - `NODE_BLOOM`            → `"BLOOM"`
  - `NODE_WITNESS`          → `"WITNESS"`
  - `NODE_COMPACT_FILTERS`  → `"COMPACT_FILTERS"`
  - `NODE_NETWORK_LIMITED`  → `"NETWORK_LIMITED"`
- For unknown bits, returns `"UNKNOWN[2^{n}]"` exactly in the C++ style.

Render a **full mask** to a list of strings:

```rust
use bitcoin_service_flags::{service_flags_to_str, ServiceFlags};

let flags = (ServiceFlags::NODE_NETWORK | ServiceFlags::NODE_WITNESS).bits();
let labels = service_flags_to_str(flags);

assert_eq!(labels, vec!["NETWORK", "WITNESS"]);
```

`service_flags_to_str(flags: u64) -> Vec<String>`:

- Iterates from LSB to MSB over all 64 bits.
- For each bit set in `flags`, calls `service_flag_to_str` and collects the result.
- The output order is deterministic and bit-ordered, matching Bitcoin Core's loop.

---

## Mathematical / logical structure

While the crate operates on simple bitfields, thinking in algebraic terms clarifies the API:

- The set of service flags forms a **Boolean algebra** over a 64-dimensional binary vector space:
  - Each basis vector corresponds to a distinct service bit.
  - The usual operations (`|`, `&`, `!`) correspond to vector-space operations over `GF(2)` if restricted to linear combinations, but for interpretive semantics we treat them as set union, intersection, and complement over a finite set of bits.

Key derived constructs:

- **Desirability mask** `D(services, ibd_done)`:

  - If `ibd_done == false` or `!services.contains(NODE_NETWORK_LIMITED)`:

    \[
    D = \{NODE\_NETWORK, NODE\_WITNESS\}
    \]

  - Else (post-IBD and peer supports limited networking):

    \[
    D = \{NODE\_NETWORK\_LIMITED, NODE\_WITNESS\}
    \]

- **Satisfaction predicate** `S(services)`:

  \[
  S(services) := (services \land D(services, ibd\_done)) = D(services, ibd\_done)
  \]

- **Address DB capability predicate** `A(services)`:

  \[
  A(services) := (services \land (NODE\_NETWORK \lor NODE\_NETWORK\_LIMITED)) \neq 0
  \]

This formalisation is useful if you integrate the crate into more sophisticated peer-scoring or optimisation logic.

---

## Usage examples

### Basic integration with a P2P handshake

```rust
use bitcoin_service_flags::{
    ServiceFlags,
    get_desirable_service_flags,
    has_all_desirable_service_flags,
    may_have_useful_addressdb,
};

fn on_version_message(services_raw: u64) {
    let services = ServiceFlags::from(services_raw);

    // Decide if we want to keep this peer long-term.
    let desirable = get_desirable_service_flags(services);

    if has_all_desirable_service_flags(services) {
        // high-value peer: reserve outbound slot, prioritise keep-alive
    } else {
        // optional: degrade to lower priority or schedule disconnect
    }

    if may_have_useful_addressdb(services) {
        // peer is a good candidate for addrman and address gossip
    }

    // Log human-readable view
    let labels = bitcoin_service_flags::service_flags_to_str(services.bits());
    tracing::info!(?services_raw, ?labels, "Peer services evaluated");
}
```

### Logging and metrics

You can feed the label list to metrics systems:

```rust
use bitcoin_service_flags::{service_flags_to_str, ServiceFlags};

fn describe_services_for_metrics(raw: u64) -> String {
    let flags = ServiceFlags::from(raw);
    service_flags_to_str(flags.bits()).join(",")
}
```

---

## Crate metadata

- **Name**: `bitcoin-service-flags`
- **Version**: `0.1.19`
- **Edition**: `2021`
- **License**: `MIT`
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Authors**: `klebs <none>`

This crate is part of the `bitcoin-rs` repository and is designed to be small, composable, and focused.

---

## Feature flags

This README does not enumerate additional Cargo feature flags; consult `Cargo.toml` in the repository if features are added later.

Dependencies of note (at time of writing):

- [`bitflags`](https://crates.io/crates/bitflags) for `ServiceFlags`.
- [`serde`](https://crates.io/crates/serde) for serialisation.
- [`tracing`](https://crates.io/crates/tracing) for structured logging.
- [`lazy_static`](https://crates.io/crates/lazy_static) for the global IBD flag.

---

## Safety and correctness notes

- `From<u64> for ServiceFlags` intentionally uses `unsafe { from_bits_unchecked(raw) }` to preserve all bits, including unknown/experimental ones. This is safe because the domain is the full `u64` space and the type is a transparent wrapper; there are no invariants violated by arbitrary bit patterns.
- Callers must not assume that `ServiceFlags` only ever contains known bits; code should be written to be robust against future service flags.
- All global state is confined to `INITIAL_BLOCK_DOWNLOAD_COMPLETED`. Concurrency is handled via `AtomicBool` with relaxed semantics, appropriate for a latch-like state that is monotonic from `false` to `true`.

---

## License

MIT License. See the repository for the full license text.
