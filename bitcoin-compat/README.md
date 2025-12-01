# bitcoin-compat

A small, low-level compatibility shim used by `bitcoin-rs` to emulate the C/C++ platform surface that upstream Bitcoin Core expects: endian helpers, byte-swapping, CPUID, errno, C-style string helpers, and portable terminal/STDIN handling.

---

## Goals

* Provide a thin, well-instrumented Rust façade over platform-specific primitives that Bitcoin Core historically relies on.
* Preserve the C/C++ invariants (sizes, endianness, IEEE-754 behavior) required by validation, cryptography, and consensus logic.
* Offer a small, stable API surface that other crates can reuse when they need identical compatibility guarantees.

This crate is explicitly oriented around *portability constraints* (endianness, integer widths, `size_t` assumptions, `errno` semantics, terminal behavior) that matter for a consensus system like Bitcoin.

---

## Features Overview

### Compile-time platform invariants

The crate asserts a series of assumptions at compile time using `const_assert!`:

* **IEEE 754 floats**: `f32` and `f64` are validated to be IEC 559 (IEEE 754) compliant (`f32_is_iec559`, `f64_is_iec559`).
* **Eight bits per byte**: `u8::BITS == 8`.
* **Fixed integer widths**:
  * `size_of::<u16>() == 2` (16-bit short assumed)
  * `size_of::<i32>() == 4` (32-bit int assumed)
  * `size_of::<u32>() == 4`
* **`size_t` constraints**:
  * `size_of::<usize>() == 4 || size_of::<usize>() == 8`
* **Pointer size equivalence**:
  * `size_of::<usize>() == size_of::<*mut c_void>()`

These mirror the implicit assumptions present in C++ Bitcoin Core code and make violations immediately visible at compile time on exotic targets.

### C-style string helper

```rust
pub fn strnlen(start: *const u8, max_len: usize) -> usize
```

* Computes the length of a C string starting at `start`, inspecting at most `max_len` bytes.
* Returns `max_len` if no NUL byte is encountered.
* If `start` is null, it logs an error and returns `0` instead of reading arbitrary memory.

This is a direct semantic translation of the classic `strnlen(3)` from libc, with additional structured logging.

### GLIBC++ sanity checks

These functions reproduce the behavior of upstream GLIBC++ sanity checks used by Bitcoin Core’s test suite to validate ABI and library behavior.

```rust
pub fn sanity_test_widen(testchar: u8) -> bool
pub fn sanity_test_list(size: u32) -> bool
pub fn sanity_test_range_fmt() -> bool
pub fn glibcxx_sanity_test() -> bool
```

* **`sanity_test_widen`**: Round-trips a `u8` through Rust `char` and back to `u8`, verifying byte preservation (analogous to C++ `char`/`wchar_t` widening and narrowing).
* **`sanity_test_list`**: Exercises `LinkedList` append/pop behavior, checking list size and element order to mimic container splice/hook invariants.
* **`sanity_test_range_fmt`**: Intentionally triggers an out-of-range access on an empty `String` and verifies the panic is caught.
* **`glibcxx_sanity_test`**: Runs all of the above and returns `true` iff they all succeed.

Use these to confirm that the standard library and runtime behave as expected on your target before executing consensus-critical logic.

### CPUID wrapper

```rust
pub fn getcpuid(
    leaf:    u32,
    subleaf: u32,
    a:       &mut u32,
    b:       &mut u32,
    c:       &mut u32,
    d:       &mut u32,
)
```

* On `x86`/`x86_64`, executes the `CPUID` instruction via `core::arch::{x86,x86_64}::__cpuid_count`.
* On other architectures, logs a warning and zeroes all outputs.
* Exposes the same logical contract as the original C++ helper, but the Rust API is safe: internal unsafe usage is fully encapsulated.

Example:

```rust
let mut eax = 0;
let mut ebx = 0;
let mut ecx = 0;
let mut edx = 0;

bitcoin_compat::getcpuid(1, 0, &mut eax, &mut ebx, &mut ecx, &mut edx);

// Inspect feature bits in ecx/edx, vendor info, etc.
```

### Endian conversion helpers

Big- and little-endian conversions are made explicit and aggressively traced:

```rust
pub fn htobe16(x: u16) -> u16
pub fn htole16(x: u16) -> u16
pub fn be_16toh(x: u16) -> u16
pub fn le_16toh(x: u16) -> u16

pub fn htobe32(x: u32) -> u32
pub fn htole32(x: u32) -> u32
pub fn be_32toh(x: u32) -> u32
pub fn le_32toh(x: u32) -> u32

pub fn htobe64(x: u64) -> u64
pub fn htole64(x: u64) -> u64
pub fn be_64toh(x: u64) -> u64
pub fn le_64toh(x: u64) -> u64
```

They wrap the intrinsic `to_be`, `to_le`, `from_be`, `from_le` methods, and log hex-formatted input and output. This is particularly important for cross-checking Bitcoin wire-format serialization, which is defined in terms of specific endianness (e.g., block headers in little-endian but some hashes printed in big-endian order).

### Byte-swapping helpers

```rust
pub fn bswap_16(x: u16) -> u16
pub fn bswap_32(x: u32) -> u32
pub fn bswap_64(x: u64) -> u64
```

These wrap `swap_bytes()` for explicit byte permutations:

* `bswap_16`: `0xAABB → 0xBBAA`
* `bswap_32`: `0xAABB_CCDD → 0xDDCC_BBAA`
* `bswap_64`: `0x1122_3344_5566_7788 → 0x8877_6655_4433_2211`

They are useful for implementing protocol-level formats that specify byte order directly, for low-level network code, and for verification of compatibility with historical C/C++ implementations.

### Cross-platform errno

```rust
pub fn last_errno() -> i32
```

Retrieves the thread-local `errno` value across platforms:

* Linux/Android: `libc::__errno_location()`
* Darwin/BSDs: `libc::__error()`
* Windows: `_get_errno()`

This small helper decouples upper layers from OS-specific errno access patterns while still exposing the raw numeric value for comparison against C APIs.

### STDIN / terminal handling

The crate provides both primitive functions and an RAII guard for manipulating terminal echo and detectability of STDIN state.

#### Core functions

```rust
pub fn set_stdin_echo(enable: bool)
pub fn stdin_terminal() -> bool
pub fn stdin_ready() -> bool
```

* **`set_stdin_echo`**:
  * On Unix, toggles the `ECHO` flag via `termios` (`tcgetattr`/`tcsetattr`).
  * On Windows, toggles the `ENABLE_ECHO_INPUT` console mode.
* **`stdin_terminal`**: `true` iff `STDIN` is attached to a TTY.
  * Unix: `isatty(STDIN_FILENO)`
  * Windows: `_isatty(_fileno(STDIN_FILE))`
* **`stdin_ready`**:
  * If `STDIN` is *not* a TTY, assumes it is ready (typical for pipes or redirected input) and returns `true`.
  * On Unix TTYs, uses `poll` with zero timeout to check for pending data.
  * On Windows, always returns `false` because the console subsystem does not offer a universally reliable polling interface for redirected consoles.

#### RAII no-echo guard

```rust
pub struct NoechoInst;

impl NoechoInst {
    pub fn new() -> Self;
}
```

`NoechoInst` disables terminal echo on construction and re-enables it on drop. This is an ergonomic tool for reading secrets (e.g., wallet passphrases) from the console without accidentally leaving the terminal in a broken state.

Macros:

```rust
#[macro_export]
macro_rules! no_stdin_echo { {
    () => {
        let _no_echo = $crate::stdin::NoechoInst::new();
    };
} }
```

This macro creates an unnamed guard bound to `_no_echo` for the current lexical scope.

Usage:

```rust
use bitcoin_compat::no_stdin_echo;

fn read_secret() -> String {
    use std::io::{self, Read};

    let mut buf = String::new();

    {
        // Echo disabled for this scope
        no_stdin_echo!();
        io::stdin().read_line(&mut buf).expect("read failed");
    } // echo automatically restored here

    buf.trim_end().to_owned()
}
```

This pattern guarantees proper cleanup even if a panic occurs inside the guarded block.

### Windows socket and ssize_t compatibility (C-interop)

The crate defines some platform-conditional type aliases and macros to emulate C-style signatures and error codes, particularly for Windows builds of networking code:

* `type Socket = u32` on non-Windows (mirroring Unix `int` file descriptors).
* `type ssize_t` mapped to the appropriate signed integer width on MSVC/Windows (`i64` on `_WIN64`, `i32` otherwise).
* `wsaeagain!` macro that maps the `EAGAIN`/`WSAEWOULDBLOCK` mismatch.

These are primarily for internal compatibility with code that expects the C/POSIX/WinSock API surface.

---

## Usage

Add to `Cargo.toml` (version pinned for illustration):

```toml
[dependencies]
bitcoin-compat = "0.1.19"
```

Example: combine endian helpers and byte-swaps when implementing a Bitcoin-style serialization routine:

```rust
use bitcoin_compat::{htole32, bswap_32};

fn encode_block_height(height: u32) -> [u8; 4] {
    // Bitcoin typically uses little-endian encoding on the wire.
    let le = htole32(height);
    le.to_le_bytes()
}

fn reverse_hash_bytes(hash: [u8; 32]) -> [u8; 32] {
    // Often one wants to interpret a little-endian internal value
    // as a big-endian printable hex. A canonical approach reverses
    // the byte order:
    let mut out = hash;
    out.reverse();
    out
}
```

Or, run the GLIBC++-style sanity tests at process startup:

```rust
fn main() {
    if !bitcoin_compat::glibcxx_sanity_test() {
        eprintln!("FATAL: glibcxx sanity tests failed; incompatible runtime");
        std::process::exit(1);
    }

    // continue with the rest of the node or wallet startup
}
```

---

## Relationship to mathematics and computer architecture

The crate encodes several low-level architectural invariants that are key to reproducible behavior in a distributed consensus system:

* **IEEE 754 floating-point**: Many analytic and estimation routines in Bitcoin Core assume specific behavior for NaNs, infinities, rounding, and division by zero. Deviations could, in principle, cause divergent behavior between nodes on different targets. By asserting IEC 559 conformance, the crate explicitly acknowledges and enforces these expectations.
* **Fixed integer widths and `size_t`**: Serialization sizes, checksum computations, and memory-usage accounting rely on the relationship between integer widths and pointer sizes. The compile-time `const_assert!` checks are effectively small model-checking constraints over the target architecture.
* **Endianness and byte ordering**: Bitcoin’s on-wire protocol defines exact byte orders for headers, hashes, and integer fields. The distinction between host order, little-endian representation, and big-endian presentation (e.g., hash hex) is codified into explicit helper functions. From a mathematical perspective, these are trivial bijections on finite bitstrings, but in practice, misapplication leads to silent, non-local defects.
* **CPUID feature discovery**: For cryptographic primitives, feature detection (e.g., SIMD or special instruction sets) can influence both performance and, in misconfigured systems, correctness when assumptions are violated. Making the CPUID surface explicit and easily testable reduces the probability of mis-detecting CPU capabilities.

By aggregating these concerns into a coherent crate, `bitcoin-compat` reduces entropy and increases the chance that independent implementations converge on identical behavior.

---

## Repository, license, and maintenance

* **Repository**: <https://github.com/klebs6/bitcoin-rs>
* **Crate**: `bitcoin-compat`
* **Version**: `0.1.19`
* **Edition**: Rust 2021
* **License**: MIT

The crate is primarily intended as an internal building block of `bitcoin-rs`, but it is general enough to be reused by other systems that need strict C/C++ compatibility with Bitcoin Core’s expectations.
