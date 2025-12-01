# bitcoin-autofile

Non-refcounted RAII wrapper around a C `FILE*` used by the `bitcoin-rs` project to reproduce Bitcoin Core's bit-exact serialization semantics while remaining idiomatic Rust.

---

## Overview

`bitcoin-autofile` exposes a single core type:

```rust
pub struct AutoFile {
    n_type:    i32,
    n_version: i32,
    file:      *mut libc::FILE,
}
```

`AutoFile`:

- Owns a raw `*mut libc::FILE` and closes it automatically on `Drop` using `libc::fclose`.
- Can be explicitly closed early via [`fclose`](#method-fclose) (idempotent).
- Can relinquish ownership of the underlying `FILE*` via [`release`](#method-release).
- Implements `std::io::Read` and `std::io::Write` using the C stdio API (`fread`, `fwrite`, `fflush`).
- Provides low-level, panic-on-error `read_ptr`, `write_ptr`, and `ignore` methods modeled on the historical Bitcoin Core C++ implementation.
- Integrates with the project's custom Bitcoin (de)serialization traits via `<<` / `>>` operator overloading using Rust's `Shl` and `Shr` traits.

This crate exists to provide a thin, controlled interop layer between Rust code and the C `FILE*` abstractions used in legacy Bitcoin Core code, while matching binary serialization behavior exactly.

---

## When should you use this crate?

You likely want `bitcoin-autofile` if:

- You are porting or interfacing with the Bitcoin Core C++ codebase and must preserve its wire format and file layout byte-for-byte.
- You need to use the project's `BtcSerialize` / `BtcUnserialize` traits over `FILE*` in a way that mirrors C++ `CAutoFile` semantics.
- You already have a `FILE*` coming from C, from `libc::fopen`, or indirectly via `std::fs::File::into_raw_fd()` and `fdopen`, and you want RAII-based cleanup in Rust.

If you do not care about `FILE*` or C stdio compatibility, using `std::fs::File` with `Read` / `Write` is more idiomatic and safer.

---

## Safety model

`AutoFile` is intentionally low-level and `unsafe`-adjacent. Key properties:

- Construction is **safe** in Rust's type system but documented with an `# Safety` contract. You must only pass valid, uniquely-owned `FILE*` handles.
- All methods treat a null `file` pointer as an error; some panic, some return an `io::Error`.
- `Drop` will `fclose` the `FILE*` if and only if it is non-null.

Because `AutoFile` operates on a raw pointer and calls into `libc`, misuse can lead to undefined behavior at the C level. In particular:

- Do not share the same `FILE*` concurrently with other code that can write to it.
- Do not call `fclose` manually on the underlying `FILE*` while it is still owned by an `AutoFile`.
- Do not double-wrap the same `FILE*` in two `AutoFile` instances.

The crate is designed so that, in the intended usage, correctness is enforced by disciplined ownership rather than Rust's borrow checker alone.

---

## Core API

### Construction

```rust
use bitcoin_autofile::AutoFile;
use libc;

// SAFETY: You must ensure `file` is a valid, uniquely-owned FILE*.
let file: *mut libc::FILE = unsafe {
    libc::fopen("blocks.dat\0".as_ptr() as *const i8, "rb+\0".as_ptr() as *const i8)
};

if file.is_null() {
    panic!("failed to open file via libc::fopen");
}

let autofile = AutoFile::new(file, n_type, n_version);
```

The `n_type` and `n_version` parameters mirror Bitcoin Core's `CDataStream` / `CAutoFile` metadata and are passed on to `BtcSerialize` / `BtcUnserialize` implementations.

**Safety contract (from `AutoFile::new`):**

- `file` must come from `libc::fopen` or from an equivalent raw descriptor conversion (e.g. `File::into_raw_fd()` followed by `fdopen`).
- No other live owner may close or mutably alias this `FILE*`.

### Ownership and lifetime

```rust
impl AutoFile {
    pub fn fclose(&mut self);
    pub fn release(&mut self) -> *mut libc::FILE;
    pub fn get(&self) -> *mut libc::FILE;
    pub fn is_null(&self) -> bool;
}
```

- `fclose`: Closes the file early, logging via `trace!`, and sets `file` to `nullptr`. It is safe to call multiple times.
- `release`: Transfers ownership of the `FILE*` to the caller and nulls out the internal pointer. `Drop` will no longer close the file.
- `get`: Returns the raw pointer **without** transferring ownership; it must not be closed externally.
- `is_null`: Lightweight check for whether the `AutoFile` currently owns a `FILE*`.

After `release` or `fclose`, the value becomes inert: all I/O operations detect the null pointer and either panic (low-level methods) or return `io::Error` (trait-based I/O).

### Low-level, panic-on-error primitives

These methods provide tightly constrained, Bitcoin-Core-style I/O behavior, panicking on error rather than returning `Result`.

```rust
impl AutoFile {
    /// Read exactly `n_size` bytes into `pch` or panic.
    pub fn read_ptr(&mut self, pch: *mut u8, n_size: usize);

    /// Skip exactly `n_size` bytes by reading into a temporary buffer or panic.
    pub fn ignore(&mut self, n_size: usize);

    /// Write exactly `n_size` bytes from `pch` or panic.
    pub fn write_ptr(&mut self, pch: *const u8, n_size: usize);
}
```

All three methods:

- Require `self.file` to be non-null; otherwise they log and panic.
- Use `libc::fread` / `libc::fwrite` directly.
- Examine `feof` to differentiate EOF from other errors.
- Panic on short read/write or any failure to match `n_size`.

This is deliberate: the surrounding Bitcoin serialization logic often assumes infallible I/O and treats I/O failure as fatal.

### Standard I/O traits

`AutoFile` implements `std::io::Read` and `std::io::Write` so it can be used with the broader Rust ecosystem where appropriate.

#### `Read`

```rust
impl std::io::Read for AutoFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
```

- Returns an `io::Error` if `file` is null.
- Uses `fread`; returns `Ok(0)` at EOF (`got == 0 && feof != 0`).
- Otherwise returns the byte count read.

#### `Write`

```rust
impl std::io::Write for AutoFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>;
}
```

- Returns an `io::Error` if `file` is null.
- Uses `fwrite` for `write`, returning an error if a short write occurs.
- Uses `fflush` for `flush`, returning `last_os_error` on failure.

These trait impls are idiomatic and integrate with any library that expects a `Read + Write` object, while still preserving the underlying C `FILE*` semantics.

### Drop semantics

```rust
impl Drop for AutoFile {
    fn drop(&mut self) {
        if !self.file.is_null() {
            unsafe { libc::fclose(self.file) };
            self.file = std::ptr::null_mut();
        }
    }
}
```

- `Drop` closes the `FILE*` exactly once, assuming it has not been nulled by `release` or `fclose`.
- Logging via `trace!` can be enabled to audit lifecycle behavior.

This models classic RAII: deterministic resource cleanup when the value goes out of scope.

---

## Integration with Bitcoin (de)serialization

`AutoFile` is designed to work with the project's custom serialization traits:

```rust
trait BtcSerialize<S> {
    fn serialize(&self, s: &mut S);
}

trait BtcUnserialize<S> {
    fn unserialize(&mut self, s: &mut S);
}
```

To provide a C++-like stream operator interface, `AutoFile` implements `Shl` and `Shr`:

```rust
impl<'a, T> std::ops::Shl<&'a T> for AutoFile
where
    T: BtcSerialize<AutoFile>,
{
    type Output = AutoFile;

    fn shl(mut self, rhs: &'a T) -> Self::Output {
        BtcSerialize::<AutoFile>::serialize(rhs, &mut self);
        self
    }
}

impl<'a, T> std::ops::Shr<&'a mut T> for AutoFile
where
    T: BtcUnserialize<AutoFile>,
{
    type Output = AutoFile;

    fn shr(mut self, rhs: &'a mut T) -> Self::Output {
        BtcUnserialize::<AutoFile>::unserialize(rhs, &mut self);
        self
    }
}
```

This allows code mathematically analogous to C++'s `CAutoFile` usage:

```rust
use bitcoin_autofile::AutoFile;
use bitcoin_autofile::{BtcSerialize, BtcUnserialize};

let autofile = AutoFile::new(file, n_type, n_version);

let header: BlockHeader = /* ... */;
let tx: Transaction = /* ... */;

// Serialize:
let autofile = autofile << &header << &tx;

// Deserialize:
let mut header2 = BlockHeader::default();
let mut tx2 = Transaction::default();
let autofile = autofile >> &mut header2 >> &mut tx2;
```

This style is primarily beneficial for codebases being ported directly from the C++ reference implementation where the `<<` and `>>` idioms are already pervasive.

---

## Error handling philosophy

The crate intentionally mixes two modes of error handling:

- **Panic-on-error** for low-level primitives (`read_ptr`, `write_ptr`, `ignore`, `Shl`/`Shr`-driven serialization). This matches Bitcoin Core’s behavior where I/O failure is treated as a fatal, unrecoverable condition.
- **`Result`-based** for trait-based `Read`/`Write`, allowing integration with Rust infrastructure that expects recoverable I/O errors.

In performance-critical or consensus-critical Bitcoin code, panicking on disk or file corruption can be preferable to attempting partial recovery that might compromise invariants. For application-level code, you may prefer using the `Read`/`Write` APIs and handling `io::Result` explicitly.

---

## Example: reading a fixed-size record

```rust
use bitcoin_autofile::AutoFile;
use libc;
use std::io::{Read, Write};

fn open_autofile(path: &str) -> AutoFile {
    use std::ffi::CString;

    let c_path = CString::new(path).expect("no interior NUL");
    let c_mode = CString::new("rb+").unwrap();

    let file = unsafe { libc::fopen(c_path.as_ptr(), c_mode.as_ptr()) };
    if file.is_null() {
        panic!("failed to open {} via fopen", path);
    }

    // n_type and n_version chosen to match Bitcoin Core semantics
    AutoFile::new(file, /* n_type = */ 1, /* n_version = */ 70015)
}

fn read_magic_prefix(mut af: AutoFile) -> [u8; 4] {
    let mut buf = [0u8; 4];
    // Using standard Read trait:
    af.read_exact(&mut buf).expect("short read");
    buf
}

fn write_then_flush(mut af: AutoFile, data: &[u8]) {
    af.write_all(data).expect("write failed");
    af.flush().expect("flush failed");
}
```

---

## Logging

`AutoFile` uses the standard Rust logging macros (`trace!`, `debug!`, `error!`). To see lifecycle and error logs, enable a compatible logger (e.g. `env_logger`, `tracing-subscriber`) and set appropriate log levels, particularly for the `autofile` target.

---

## Repository, license, and maintenance

- **Crate name:** `bitcoin-autofile`
- **Version:** 0.1.20
- **Repository:** <https://github.com/klebs6/bitcoin-rs>
- **License:** MIT
- **Edition:** Rust 2021
- **Author:** `klebs <none>`

This crate is part of the broader `bitcoin-rs` codebase; it is not a general-purpose I/O abstraction but a focused compatibility layer intended primarily for that project and for developers who explicitly need bit-exact Bitcoin Core behavior.
