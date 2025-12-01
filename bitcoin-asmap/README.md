# bitcoin-asmap

High‑fidelity Rust implementation of Bitcoin Core's ASMAP interpreter, decoder, and structural validator.

This crate provides a bit‑exact reimplementation of the Autonomous System (AS) mapping logic used by Bitcoin Core to probabilistically infer the AS number (ASN) associated with a peer's IP address. It is designed for:

- Node operators and infrastructure providers who need deterministic ASMAP behavior in Rust services.
- Researchers performing network topology or AS‑level analysis of the Bitcoin network.
- Library authors integrating Bitcoin‑style AS‑based peer selection or routing policies into Rust applications.

---

## Conceptual overview

### ASMAP and ASN inference

An **ASMAP** file encodes a decision tree over IP address bits that maps each address to an **Autonomous System Number (ASN)**. In Bitcoin Core, this mapping is used to diversify peer selection by AS to reduce the risk that many peers are controlled by the same routing domain.

Key ideas:

- The ASMAP is a compact, prefix‑based program over bits of an IP address.
- An IP address is represented as a boolean slice `[bool; 128]` (for IPv6) or appropriately padded for IPv4‑in‑IPv6.
- The ASMAP is represented as a boolean program `asmap: &[bool]` which is **interpreted** to yield a final ASN.

This crate mirrors the C++ reference implementation used by Bitcoin Core, including its variable‑length integer coding, instruction set, and comprehensive structural sanity checks.

---

## Core data model

### Instruction set

```rust
#[repr(u32)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    RETURN,
    JUMP,
    MATCH,
    DEFAULT,
}
``

The instruction stream is encoded as bits and decoded via `decode_type`, which internally uses `decode_bits` and a compact opcode encoding.

- **RETURN**: Decode and return an ASN; terminates the program if structurally valid.
- **JUMP**: Conditionally jump forward in the instruction stream based on the next IP bit.
- **MATCH**: Match a short bit pattern against the next IP bits; either continue or fall back to the current default ASN.
- **DEFAULT**: Update the current default ASN used when a `MATCH` fails.

These instructions, combined with a stack of jump targets, implement a structured decision tree with compact representation.

---

## Public API

### Reading and validating an ASMAP file

```rust
use bitcoin_asmap::decode_asmap;

let asmap_bits = decode_asmap("/path/to/asmap.dat");
if asmap_bits.is_empty() {
    // Failed to read or validate ASMAP
}
```

`decode_asmap<P: AsRef<Path>>(path: P) -> Vec<bool>`:

- Reads the raw ASMAP file from disk (little‑endian, LSB‑first, identical to C++).
- Converts bytes into a `Vec<bool>` bit stream (bit 0 is the LSB of each byte).
- Runs a **full structural sanity check** via `sanity_check_as_map`.
- Returns the validated bit vector, or an **empty** vector on failure.

This is the entry point you usually want when consuming an ASMAP file shipped with Bitcoin Core or derived tools.

### Interpreting an ASMAP for a given IP

```rust
use bitcoin_asmap::{decode_asmap, interpret};

let asmap = decode_asmap("/path/to/asmap.dat");
if asmap.is_empty() {
    panic!("Invalid ASMAP");
}

// Example: represent an IPv4 address as 128 bits (IPv4‑mapped)
fn ipv4_to_bits(addr: [u8; 4]) -> Vec<bool> {
    // Very naive 32‑bit big‑endian; real usage should follow Bitcoin Core's
    // IPv4‑in‑IPv6 representation and bit ordering.
    let mut bits = Vec::with_capacity(32);
    for byte in addr {
        for offset in (0..8).rev() { // big‑endian per byte
            bits.push(((byte >> offset) & 1) != 0);
        }
    }
    bits
}

let ip_bits = ipv4_to_bits([203, 0, 113, 1]);
let asn = interpret(&asmap, &ip_bits);

if asn == 0 {
    // 0 is not a valid ASN; indicates structural failure during interpret()
}
```

`interpret(asmap: &[bool], ip: &[bool]) -> u32`:

- Executes the ASMAP program over `ip`'s bits.
- Returns the mapped ASN on success.
- Returns `0` on structural failure (e.g., invalid jumps, truncated encodings), even if `sanity_check_as_map` should normally prevent such cases in production.

> **Important**: This crate expects the same IP bit ordering and normalization that Bitcoin Core uses. If you want behavior identical to Core, ensure you pass in `ip` bits constructed according to its reference logic (e.g., IPv4 mapped into IPv6).

### Structural sanity checking

```rust
use bitcoin_asmap::{decode_asmap, sanity_check_as_map};

let asmap = decode_asmap("/path/to/asmap.dat");
if asmap.is_empty() {
    // Already failed, but you can also explicitly check:
}

let ok = sanity_check_as_map(&asmap, 128);
assert!(ok);
```

`sanity_check_as_map(asmap: &[bool], bits: i32) -> bool`:

- Performs **exhaustive structural validation** of the ASMAP program.
- Mirrors Bitcoin Core's rules:
  - No jumps into the middle of another instruction.
  - No intersecting jumps.
  - No consuming IP bits past the declared input width.
  - No consecutive `DEFAULT`s.
  - No `RETURN` immediately after `DEFAULT`.
  - No unmatched trailing code after `RETURN`.
  - Correct use of padding and zero padding bits.
  - Constraints on sequences of short `MATCH` instructions.
- Returns `true` iff the structure is valid.

This is useful if you:

- Load ASMAPs from untrusted sources.
- Mutate or generate ASMAPs programmatically.
- Want defensive validation independent of `decode_asmap`.

### Bit‑level primitives

These are low‑level helpers that directly mirror the C++ implementation and are usually not needed by typical users but are valuable for experimentation or custom tooling.

#### `decodeasn`

```rust
pub fn decodeasn(asmap: &[bool], pos: &mut usize) -> u32
```

Decodes an ASN value from `asmap` starting at `*pos` using a variable‑length integer scheme defined by `ASN_BIT_SIZES`.

- `pos` is updated in‑place.
- Returns the ASN, or an `INVALID` sentinel value (internal constant) on failure.

#### `decode_jump`

```rust
pub fn decode_jump(asmap: &[bool], pos: &mut usize) -> u32
```

Decodes a jump offset using `JUMP_BIT_SIZES`. The returned value is a **forward** offset from the current `pos`.

#### `decode_match`

```rust
pub fn decode_match(asmap: &[bool], pos: &mut usize) -> u32
```

Decodes a match pattern plus a sentinel bit using `MATCH_BIT_SIZES`. The number of data bits in the pattern is `count_bits(m) - 1`, where `m` is the decoded value.

#### `decode_bits`

```rust
pub fn decode_bits(
    asmap:     &[bool],
    pos:       &mut usize,
    minval:    u8,
    bit_sizes: &[u8],
) -> u32
```

This is the **core variable‑length integer decoder**, parameterized by:

- `minval`: base value added to the final decoded integer.
- `bit_sizes`: an increasing sequence of mantissa widths.

The decoding logic is exponential‑Golomb‑like:

1. For each `bitsize` except the last, it reads an **exponent bit**.
2. If the exponent bit is `1`, it increments `val` by `1 << bitsize` and continues to the next `bitsize`.
3. If the exponent bit is `0`, it consumes `bitsize` mantissa bits into `val` and returns.
4. For the last `bitsize`, no exponent bit is read; instead, the path falls through if all previous exponent bits were `1`.

On early EOF, returns `INVALID` and leaves an error trace.

#### `decode_type`

```rust
pub fn decode_type(asmap: &[bool], pos: &mut usize) -> Instruction
```

Decodes an instruction opcode using `TYPE_BIT_SIZES` and returns the corresponding `Instruction` variant.

#### `count_bits`

```rust
#[inline]
pub fn count_bits(x: u32) -> u32 {
    x.count_ones()
}
```

Helper that wraps `u32::count_ones` for internal clarity.

---

## Logging and diagnostics

The implementation uses structured logging macros such as `trace!`, `debug!`, `info!`, and `error!` (typically from the `tracing` crate). These log at every decision point:

- Early returns during decoding.
- Invalid opcodes or truncated integers.
- Sanity check violations including exact reason and offset.

To benefit from these diagnostics, configure a compatible subscriber in your application, for example:

```rust
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // use bitcoin_asmap here
}
```

---

## Error handling and invariants

- `decode_asmap` returns an empty `Vec<bool>` on any I/O error or failed sanity check. Empty means **"do not use"**.
- `interpret` returns `0` on structural failure; `0` is not a valid ASN by design.
- Internally, decoding uses an `INVALID` sentinel (not exposed) to signal truncated encodings.
- `sanity_check_as_map` should be run on any externally provided ASMAP before using `interpret` in production.

If you require stronger typing (e.g., non‑zero ASN newtypes) or richer error types, you can wrap this crate in a thin adapter layer.

---

## Performance considerations

- Bit representation uses `Vec<bool>`, which is compact but may have non‑trivial overhead for heavy random access. This mirrors the reference C++ behavior but not necessarily the most optimal Rust representation.
- The interpreter is single‑pass and operates in `O(|asmap| + |ip|)` time.
- Sanity checking is more expensive (`O(|asmap|)` with additional structural invariants) and should be performed once per ASMAP file, not per lookup.

For workloads performing a large number of `interpret` calls against a static ASMAP, the typical pattern is:

1. Load and validate the ASMAP once via `decode_asmap` (or manually reading the file and using `sanity_check_as_map`).
2. Reuse the resulting `Vec<bool>` for all subsequent IP lookups.

---

## Intended use cases

- **Bitcoin node infrastructure**: Implement AS‑aware peer selection or connection policies in Rust node software.
- **Research and analysis**: Offline analysis of Bitcoin network AS distribution using the same mapping logic as Bitcoin Core.
- **Monitoring and policy engines**: Integrate ASMAP‑based classification into monitoring agents or traffic policy components written in Rust.

---

## Repository, license, and edition

- **Crate name**: `bitcoin-asmap`
- **Version**: `0.1.19`
- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **License**: MIT
- **Rust edition**: 2021

---

## Caveats and compatibility

- Behavior is designed to be bit‑for‑bit compatible with the original Bitcoin Core C++ implementation. Nevertheless, you should validate against known test vectors when integrating into a critical system.
- Ensure that the IP bit ordering and normalization (IPv4 vs IPv6) match Bitcoin Core if you seek identical outcomes.
- This crate does not fetch or update ASMAP data itself; you must supply a valid ASMAP file.
