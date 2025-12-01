# bitcoin-bufferedfile

High‑performance, C‑compatible, rewindable buffered reader for `FILE*` streams, modeled on Bitcoin Core's `BufferedFile`. Provides deterministic RAII semantics, fixed‑capacity ring buffering, and a trait‑based deserialization interface suitable for consensus‑critical binary formats.

---

## Overview

`bitcoin-bufferedfile` exposes a low‑level primitive, [`BufferedFile`], that wraps a raw `*mut libc::FILE` and presents a logically linear byte stream backed by a circular buffer with a configurable rewind window.

This design allows you to:

- Perform streaming reads from large files without unbounded memory growth.
- Rewind the stream by a bounded amount without re‑seeking the OS file descriptor.
- Precisely control error semantics: most failures are treated as logic errors and result in panics with clear diagnostics, matching the expectations of consensus validation code.
- Integrate with higher‑level, type‑driven deserialization logic via the [`BufferedFileReadable`] trait and the `>>` operator (via `Shr`).

The crate is deliberately stringent: it prefers to panic rather than silently continue on I/O failure, end‑of‑file, or logical mis‑use (e.g., reading past an explicit limit). This mirrors C++ Bitcoin Core semantics where such violations indicate bugs in the higher‑level protocol code rather than routine recoverable errors.

---

## Core Concepts

### Circular buffer and rewind window

`BufferedFile` maintains:

- A fixed‑size byte buffer `vch_buf`.
- A **source position** `n_src_pos`: total bytes pulled from the underlying `FILE*` into the buffer.
- A **read position** `n_read_pos`: logical cursor indicating how many bytes have been consumed.
- A **rewind budget** `n_rewind`: maximum number of bytes by which reads are permitted to move backwards, implicitly enforced by the buffer size.

The buffer is indexed by `n_src_pos % buf_len` and `n_read_pos % buf_len`, turning it into a ring (circular) buffer. The invariant `n_src_pos - n_read_pos <= buf_len - n_rewind` ensures that the last `n_rewind` bytes are always available for rewinding without being overwritten by new data.

This is materially different from `BufReader`: the buffer is not merely an optimization but also encodes *how much history you may rely on*. Attempting to "rewind" beyond what the buffer can retain will clamp to the earliest feasible position.

### RAII and FILE* ownership

`BufferedFile` owns a `*mut libc::FILE` and guarantees:

- Automatic `fclose` on `Drop` if the pointer is still non‑null.
- A safe manual close via [`fclose`](#explicitly-closing-the-file), which nulls out the pointer to preclude double close.

This makes it straightforward to interoperate with C APIs (including those from Bitcoin Core) that hand out raw `FILE*` handles, while still providing Rust‑style lifetime semantics.

### Type‑driven deserialization

The trait:

```rust
pub trait BufferedFileReadable {
    fn read_from_buffer(stream: &mut BufferedFile, out: &mut Self);
}
```

abstracts over *how* a type `T` should be read from a `BufferedFile`. Implementors consume bytes from the stream and populate `out`, leaving `stream` positioned *just after* the reading. The trait is intentionally minimal to allow:

- Blanket derives in other crates for simple data structures.
- Hand‑written implementations for consensus‑critical or bit‑level encodings.

The crate also implements the right‑shift operator `>>` for ergonomic chaining:

```rust
impl<T: BufferedFileReadable> Shr<&mut T> for BufferedFile {
    type Output = Self;

    fn shr(mut self, rhs: &mut T) -> Self::Output {
        T::read_from_buffer(&mut self, rhs);
        self
    }
}
```

This lets you write C++‑style extraction code:

```rust
// pseudocode-style usage
buffered_file >> &mut header >> &mut payload >> &mut trailer;
```

without exposing the internal reading machinery in user code.

---

## Usage

### Constructing a `BufferedFile`

You typically start from some external `FILE*` (for example, produced in FFI code or via `libc::fopen`):

```rust
use bitcoin_bufferedfile::BufferedFile;
use std::ffi::CString;

fn open_buffered(path: &str) -> BufferedFile {
    let c_path = CString::new(path).expect("no interior NULs");

    let mode   = CString::new("rb").unwrap();
    let file = unsafe { libc::fopen(c_path.as_ptr(), mode.as_ptr()) };
    if file.is_null() {
        panic!("unable to open file: {path}");
    }

    // buffer size and rewind budget in bytes
    let buf_size   = 1 << 16;   // 64 KiB
    let rewind     = 1 << 14;   // 16 KiB of rewind
    let n_type     = 0;         // application-specific
    let n_version  = 0;         // application-specific

    BufferedFile::new(file, buf_size, rewind, n_type, n_version)
}
```

Requirements:

- `n_rewind_in < n_buf_size`. Violating this will panic on construction.
- `file_in` must be a valid, open `FILE*`. `BufferedFile` does not perform ownership analysis; it assumes exclusive ownership for the purposes of `fclose`.

### Reading raw bytes

To pull raw bytes into a caller‑provided buffer:

```rust
let mut bf   = open_buffered("blocks.dat");
let mut buf  = [0u8; 80]; // e.g., read a Bitcoin block header

// Panics if EOF is reached before 80 bytes or if limit is exceeded
bf.read(buf.as_mut_ptr(), buf.len());

// Position has advanced by 80 bytes
let pos = bf.get_pos();
println!("current position: {pos}");
```

Semantics:

- `read` will keep refilling the internal buffer until either `n_size` bytes have been copied or the underlying file cannot supply more (EOF or error).
- If the current read position + requested size exceeds `n_read_limit`, the call panics.
- On encountering EOF before satisfying `n_size`, the call panics with a diagnostic.

### Rewinding and positioning

`BufferedFile` does not expose a direct "rewind by N" function but allows you to set the absolute logical position, subject to the constraints of the ring buffer.

```rust
let mut bf = open_buffered("blocks.dat");

// ... perform some reads ...
let checkpoint = bf.get_pos();

// read some speculative data
let mut tmp = [0u8; 16];
bf.read(tmp.as_mut_ptr(), tmp.len());

// Try to return to the checkpoint
let ok = bf.set_pos(checkpoint);
if !ok {
    // the requested position was outside the maintainable window
    // the stream has been clamped to the earliest feasible position
    eprintln!("checkpoint outside rewind window; position clamped");
}
```

`set_pos` behavior:

- If `n_pos + bufsize < n_src_pos`, the position is *too far back* to be fully represented by the current buffer contents; it clamps to `n_src_pos - bufsize` and returns `false`.
- If `n_pos > n_src_pos`, the position is *too far forward*; it clamps to `n_src_pos` and returns `false`.
- Otherwise, it sets `n_read_pos = n_pos` and returns `true`.

This gives you a robust mechanism to implement limited backtracking parsers where the maximum rollback distance is bounded and known a priori.

### Limiting the readable region

You can impose a hard upper bound on the logical read position:

```rust
let mut bf = open_buffered("blocks.dat");

// allow reading only the first 1 MiB
let ok = bf.set_limit(Some(1 << 20));
assert!(ok);

// remove any explicit limit
let _ = bf.set_limit(None);
```

Properties:

- A limit smaller than the current read position is rejected: `set_limit` returns `false` and leaves the limit unchanged. This protects against nonsensical configurations such as retroactively forbidding already‑performed reads.
- `read` will panic if a requested read would cross the limit.

This is particularly useful in protocol code where a message length is read from the wire and then used to bound the subsequent parsing logic.

### Searching for a byte

`find_byte` scans forward until a target byte is encountered:

```rust
let mut bf = open_buffered("log.bin");

// search for newline
bf.find_byte(b'\n');
let newline_pos = bf.get_pos();
println!("found newline at {newline_pos}");
```

Details:

- The read pointer ends **on** the matching byte (i.e., `get_pos()` returns the index of `ch`).
- `fill` is called as needed; failures propagate as panics.
- If EOF is reached without finding the target byte, the function panics.

This provides a primitive for delimiter‑based framing without needing to materialize the entire preceding data.

### EOF detection

You can query whether the stream is at EOF:

```rust
if bf.eof() {
    // both the buffer is exhausted and the underlying FILE* reports EOF
}
```

`eof()` returns `true` *only* if:

- `n_read_pos == n_src_pos` (no buffered unread bytes), and
- `feof(src) != 0` (the underlying C stream is at EOF).

---

## Integrating `BufferedFileReadable`

To leverage the ergonomic `>>` operator and type‑driven parsing, implement `BufferedFileReadable` for your types.

### Example: reading a fixed‑layout struct

Suppose you have a simple header format:

```rust
#[derive(Debug, Default)]
struct Header {
    magic: u32,
    version: u32,
}
```

You can implement `BufferedFileReadable` manually:

```rust
use bitcoin_bufferedfile::BufferedFile;
use bitcoin_bufferedfile::BufferedFileReadable;
use std::mem::size_of;

impl BufferedFileReadable for Header {
    fn read_from_buffer(stream: &mut BufferedFile, out: &mut Self) {
        unsafe {
            // read magic (little-endian assumed as example)
            let mut buf = [0u8; size_of::<u32>()];
            stream.read(buf.as_mut_ptr(), buf.len());
            out.magic = u32::from_le_bytes(buf);

            // read version
            let mut buf = [0u8; size_of::<u32>()];
            stream.read(buf.as_mut_ptr(), buf.len());
            out.version = u32::from_le_bytes(buf);
        }
    }
}
```

Now you can consume a `Header` from a `BufferedFile` via the shift operator:

```rust
use std::ops::Shr;

let mut bf      = open_buffered("header.bin");
let mut header  = Header::default();

// bf >> &mut header consumes bytes and leaves bf advanced
bf = bf.shr(&mut header);
println!("parsed header: {header:?}");
```

Contract for `BufferedFileReadable::read_from_buffer`:

- **Must** advance the stream to just after the consumed object.
- **Must not** attempt to read beyond `n_read_limit`; if it does, a panic will occur inside `read`.
- On encountering malformed data, it is acceptable to panic or to encode an error state into `out`, depending on higher‑level design. The crate itself is agnostic.

### Relationship to Bitcoin serialization

In the original Bitcoin Core codebase, `CBufferedFile` is used in conjunction with `Unserialize(Stream&, T&)` free functions. This crate exposes the minimal machinery required to port that pattern into Rust:

- `BufferedFile` as the low‑level `Stream`.
- `BufferedFileReadable::read_from_buffer` as the trait‑based analog of `Unserialize`.
- `Shr` implementation to preserve the extraction syntax idiom.

This makes the crate particularly suitable for projects that:

- Reuse Bitcoin Core's on‑disk or on‑wire formats.
- Need behaviorally faithful reimplementations for consensus‑sensitive code.

---

## Error Model

The crate intentionally uses panics to signal conditions that, in the context of consensus code, are programmer errors or fatal I/O failures:

- **Construction**: `BufferedFile::new` panics if `n_rewind_in >= n_buf_size`.
- **fill**: panics if `fread` returns 0 and either EOF is reached or the call fails.
- **read**: panics if a read attempts to cross `n_read_limit` or if EOF occurs before all requested bytes are read.
- **find_byte**: panics if the target byte is not found before EOF.

All such panics are accompanied by `tracing` diagnostics (`error`, `warn`, `trace`, `debug`), enabling you to capture structured logs in production deployments.

If you require a fully fallible API (returning `Result` instead of panicking), you can build a thin wrapper around `BufferedFile` that:

- Catches panics at the FFI boundary or at top‑level worker threads.
- Encodes error causes using your project's error type.

However, for environments mirroring Bitcoin Core's expectations—where data corruption or I/O failures are terminal—panicking is an adequate and simpler model.

---

## Logging and Observability

The crate uses the `tracing` ecosystem for diagnostics. Log statements include structured fields such as positions, limits, and buffer capacities:

- `trace!` for high‑frequency events like buffer refills and reads.
- `debug!` for state transitions like `set_pos` and `fclose`.
- `warn!` for non‑fatal misconfigurations (e.g., invalid `set_limit` attempts).
- `error!` for conditions that immediately precede panics.

To take advantage of this, initialize a subscriber in your application, for example using `tracing-subscriber` in `main`:

```rust
fn main() {
    use tracing_subscriber::FmtSubscriber;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting tracing default failed");

    // now use bitcoin-bufferedfile APIs as usual
}
```

---

## Safety Considerations

`BufferedFile` is explicitly low‑level and interacts with raw pointers and C APIs. The crate confines `unsafe` to its implementation, but consumers must still respect its contracts:

- The `FILE*` passed to `BufferedFile::new` must not be used elsewhere while the `BufferedFile` exists, to avoid data races and double closes.
- The lifetime of the underlying file must extend across all operations on the `BufferedFile` (this is enforced in practice by RAII, given correct ownership transfer).
- When using `read` directly, you are responsible for ensuring that `pch` points to a valid, writable region of at least `n_size` bytes.

If you stay at the level of `BufferedFileReadable` implementations and slice‑based reading, the primary risk vectors are logical (e.g., misinterpreting on‑disk formats) rather than memory safety violations.

---

## Relationship to the `bitcoin-rs` Workspace

This crate lives in the [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs) repository and is designed to be used as an internal building block for a larger ecosystem of Bitcoin‑related crates. It focuses narrowly on faithfully replicating the semantics of Bitcoin Core's buffered file abstraction, rather than trying to provide a general‑purpose Rust I/O layer.

Upstream crates in the workspace typically provide:

- Higher‑level consensus and P2P message parsing.
- Derives and utilities that implement `BufferedFileReadable` (and related traits) for Bitcoin primitives.
- Ergonomic, strongly typed frontends that avoid direct exposure of `FILE*` and unsafe APIs.

You can, however, use `bitcoin-bufferedfile` independently in other projects that require C‑compatible, rewind‑capable buffered reading of binary data.

---

## License

This crate is distributed under the MIT license, as part of the `bitcoin-rs` repository.

See the repository for full license text and additional context:

- Repository: <https://github.com/klebs6/bitcoin-rs>
