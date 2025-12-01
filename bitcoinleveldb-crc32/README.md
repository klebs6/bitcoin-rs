# bitcoinleveldb-crc32

Low-level, allocation-free CRC32C primitives extracted from the Bitcoin LevelDB port, with explicit control over masking, alignment, and hardware acceleration probing.

This crate is intended for consumers who need tight control over binary layout and performance characteristics, and who are comfortable working with raw pointers and unsafe Rust when necessary.

---

## Overview

`bitcoinleveldb-crc32` provides a small, focused API around the CRC32C (Castagnoli) checksum, mirroring the behavior and conventions of the original LevelDB implementation used in Bitcoin.

The design goals are:

- **Bit‑for‑bit compatibility** with LevelDB's CRC32C implementation
- **No hidden allocations**; operates directly on `*const u8` buffers
- **Masking support** suitable for on-disk or on-wire embedding of CRCs
- **Alignment utilities** for efficient SIMD or hardware-accelerated backends
- **Hardware capability probing** to determine if CRC32C acceleration is available

This crate is particularly useful in contexts such as:

- Storage engines and LSM trees (LevelDB, RocksDB-style systems)
- Filesystem or block-level integrity checks
- Network protocols that embed CRC32C checksums in framing
- Reimplementations of Bitcoin Core's LevelDB-based storage in Rust

The exposed functions are instrumented with [`tracing`](https://docs.rs/tracing) to allow fine-grained observability in high-performance applications.

---

## CRC32C background

CRC32C is a 32-bit cyclic redundancy check using the Castagnoli polynomial
\[ p(x) = x^{32} + x^{28} + x^{27} + x^{26} + x^{25} + x^{23} + x^{22} + x^{20} + x^{19} + x^{18} + x^{14} + x^{13} + x^{11} + x^{10} + x^9 + x^8 + x^6 + 1 \]
which is widely used due to its strong error-detection properties and efficient hardware support (e.g., SSE4.2 `crc32` instructions on x86, corresponding intrinsics on other architectures).

The library maintains compatibility with LevelDB's treatment of:

- Initial CRC seed
- Bit ordering and reflection
- Final XOR and masking conventions

so that checksums computed here will match those stored by Bitcoin's C++ LevelDB bindings, given identical inputs.

---

## Features at a glance

- `crc32c_value` — Compute CRC32C of a contiguous buffer.
- `crc32c_extend` — Incrementally update a running CRC32C over streaming data.
- `crc32c_mask` / `crc32c_unmask` — Apply and remove LevelDB-compatible CRC masking for storing CRCs inside data.
- `crc32c_read_uint32le` — Read a little-endian `u32` from a 32-bit aligned pointer (no allocation, no copy beyond a 4-byte stack buffer).
- `crc32c_round_up` — Align a pointer to a power-of-two boundary for SIMD or hardware-friendly access.
- `crc32c_can_accelerate` — Probe at runtime whether a hardware-accelerated backend is active and correct.

All functions are `#[inline]` (where appropriate) and instrumented with `tracing::instrument` annotations.

---

## Safety model

Most functions accept raw pointers (`*const u8`). The caller is responsible for:

- Ensuring the pointer is non-null and points to at least `n` valid bytes where required
- Respecting any implied alignment when using `crc32c_read_uint32le` and `crc32c_round_up`
- Avoiding data races when used in concurrent contexts

The library itself does **not** perform bounds checking; it is designed for use in performance-critical engines where the caller typically controls the full lifecycle of the buffers.

---

## Installation

Add to `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-crc32 = "0.1.19"
tracing = "0.1" # for instrumentation visibility (optional but recommended)
```

This crate targets Rust edition 2021 and is licensed under MIT.

---

## API reference and usage

The public interface consists of the following functions:

### `crc32c_value`

```rust
use bitcoinleveldb_crc32::crc32c_value;

fn crc_of_slice(data: &[u8]) -> u32 {
    unsafe { crc32c_value(data.as_ptr(), data.len()) }
}
```

Computes the CRC32C value of `data[0..n)`. Internally it simply seeds the CRC with `0` and invokes `crc32c_extend`.

**Semantics:**

- Equivalent to `crc32c_extend(0, data, n)`
- Designed for one-shot checksums over contiguous memory

### `crc32c_extend`

```rust
use bitcoinleveldb_crc32::crc32c_extend;

fn crc_stream(chunks: &[&[u8]]) -> u32 {
    let mut crc = 0u32;
    for chunk in chunks {
        unsafe {
            crc = crc32c_extend(crc, chunk.as_ptr(), chunk.len());
        }
    }
    crc
}
```

Extends an existing CRC32C value `crc` with additional bytes `data[0..n)`, as if concatenating the underlying byte streams.

This is suitable for:

- Streaming data ingestion
- Log-structured storage
- Incrementally computing CRCs over large files without storing them in memory at once

Implementation notes:

- Currently calls a `crc32c_extend_portable` backend, but is architected to allow a hardware-accelerated path.

### `crc32c_mask` and `crc32c_unmask`

```rust
use bitcoinleveldb_crc32::{crc32c_mask, crc32c_unmask};

let crc: u32 = 0x1234_5678;
let masked = crc32c_mask(crc);
assert_eq!(crc, crc32c_unmask(masked));
```

Masking is necessary when embedding a CRC inside the data that is itself being checksummed. Without masking, the CRC field can cause pathological fixed points where the checksum of the structure always equals the embedded CRC, even if other bits change in certain patterns.

**LevelDB-style masking algorithm:**

1. Rotate the CRC right by 15 bits (or equivalently left by 17 bits for the remaining bits),
2. Add a fixed constant `MASK_DELTA` using wrapping arithmetic.

This mapping is invertible; `crc32c_unmask` reverses the operation by:

1. Subtracting `MASK_DELTA` with wrapping semantics,
2. Rotating left by 15 bits (right by 17 for the remaining bits).

Use these when you store CRCs in:

- SSTable files
- WAL (write-ahead log) records
- Protocol frames that might themselves be protected by a higher-level CRC

### `crc32c_read_uint32le`

```rust
use bitcoinleveldb_crc32::crc32c_read_uint32le;

fn read_u32_le(buf: &[u8]) -> u32 {
    assert!(buf.len() >= 4);
    unsafe { crc32c_read_uint32le(buf.as_ptr()) }
}
```

Reads a 32-bit little-endian integer from a 32-bit-aligned buffer.

**Constraints and behavior:**

- `buffer` must be valid for at least 4 bytes of read access.
- Assumes platform is little-endian or handles conversion via `u32::from_le_bytes`.
- Internally, copies 4 bytes into a stack buffer and interprets them as `u32`.

Typical usage is to efficiently parse on-disk or on-wire headers that carry lengths, CRCs, or version fields.

### `crc32c_round_up`

```rust
use bitcoinleveldb_crc32::crc32c_round_up;

fn align_to_64(ptr: *const u8) -> *mut u8 {
    // 64 must be a power of two; enforced by debug_assert in the function.
    crc32c_round_up::<64>(ptr)
}
```

Rounds a pointer up to the smallest address `>= pointer` that is aligned to `N` bytes, where `N` is a power of two.

This is a standard alignment operation:

\[
\text{rounded} = (\text{addr} + N - 1) \& \neg (N - 1)
\]

Applications:

- Aligning buffers for vectorized CRC implementations
- Ensuring cacheline or page alignment for IO-bound data structures

### `crc32c_can_accelerate`

```rust
use bitcoinleveldb_crc32::crc32c_can_accelerate;

if crc32c_can_accelerate() {
    // Hardware-accelerated CRC32C backend is active and passes self-test.
} else {
    // Fallback to portable implementation.
}
```

Performs a self-test using the original LevelDB logic:

- Compute `AcceleratedCRC32C(0, b"TestCRCBuffer", 14)`.
- Compare against the constant `0xdcbc59fa`.

If and only if the computed value matches, the accelerated backend is considered correct and available.

This function is useful for:

- Runtime feature detection
- Deciding whether to favor a hardware-accelerated path vs a portable scalar fallback

---

## Tracing and observability

All public functions are annotated with `#[instrument(level = "trace", skip_all)]` from the `tracing` crate, and emit `trace!` events with relevant metadata:

- `crc32c_value`: length and resulting CRC
- `crc32c_mask` / `crc32c_unmask`: input and transformed values
- `crc32c_read_uint32le`: pointer and decoded value
- `crc32c_round_up`: original and aligned addresses, alignment
- `crc32c_can_accelerate`: computed and expected CRCs, data length, decision flag
- `crc32c_extend`: initial CRC, length, final CRC

To enable these logs:

```rust
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default failed");

    // call functions from bitcoinleveldb-crc32 here
}
```

In production, you may choose to record only higher levels (e.g., `INFO` or `WARN`) and enable `TRACE` selectively during diagnostics or benchmarking.

---

## Safety and correctness checklist

When integrating this crate into a storage engine or protocol implementation, verify the following:

1. **Pointer validity**: All pointers passed into the functions must be derived from valid Rust slices, vectors, or other properly allocated memory regions.
2. **Bounds**: Ensure that `n` never exceeds the actual buffer length.
3. **Alignment**: Only rely on `crc32c_read_uint32le` for buffers that are at least naturally aligned or non-problematic on your target architecture.
4. **Masking usage**: If you store CRCs inside the data they protect, always mask before writing and unmask after reading, consistently.
5. **Interoperability**: When targeting existing LevelDB data or Bitcoin Core’s on-disk formats, verify one or two known-checksum fixtures to confirm end-to-end compatibility.

---

## Repository, license, and maintenance

- **Repository**: <https://github.com/klebs6/bitcoin-rs>
- **Crate**: `bitcoinleveldb-crc32`
- **License**: MIT
- **Rust edition**: 2021
- **Authors**: `klebs <none>`

This crate is intended as a modular component within the broader `bitcoin-rs` project. For higher-level abstractions (e.g., full LevelDB bindings or Bitcoin node functionality), consult that repository.
