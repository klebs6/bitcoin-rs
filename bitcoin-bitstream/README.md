# bitcoin-bitstream

Low-level, allocation-aware bit and byte stream utilities extracted from the Bitcoin Core serialization layer, implemented in Rust.

This crate focuses on:

- **Bit-level I/O** over `Read` / `Write` streams (`BitStreamReader`, `BitStreamWriter`)
- **Byte-stream buffering** with an internal, security-oriented allocator (`DataStream`)
- **Overrideable stream views** for type/version–aware serialization front-ends (`OverrideStream`)
- **Constant-endianness integer accessors** (`read*` / `write*` helpers)
- **Logging-first design** using `tracing` for observability of all core operations

It is primarily intended for Bitcoin-style protocol implementors who require strict control over:

- on-the-wire layout (endian, bit packing),
- buffer lifetime and zeroing behavior, and
- incremental parsing / framing with explicit rewind and compaction semantics.

---

## Features at a Glance

- **Bit-oriented streaming**
  - Read and write **up to 64 bits at a time** from/to an arbitrary I/O stream.
  - No dependency on higher-level serialization frameworks.
  - Deterministic, shift-based implementation derived from Bitcoin Core.

- **DataStream: a Bitcoin-like serialization buffer**
  - Backed by `SerializeData = Vec<u8, ZeroAfterFreeAllocator>` (allocator zeroes memory on free in the original C++ design; in Rust, this crate expects an equivalent `ZeroAfterFreeAllocator`).
  - `n_read_pos` marker separates *consumed* from *unconsumed* bytes.
  - Mutation APIs closely mirror C++ `CDataStream` semantics (insert, erase, compact, rewind, xor, etc.).

- **Unsafe but explicit pointer I/O**
  - `write{le,be}{16,32,64}` and `read{le,be}{16,32,64}` operate directly on raw pointers.
  - Intended for **performance-sensitive** codecs and when integrating with foreign memory layouts.

- **OverrideStream**
  - Thin, unsafe wrapper over a `*mut Stream: Backend` pointer with explicit `type` and `version` fields.
  - Used to emulate Bitcoin Core’s pattern of constructing temporary streams with overridden type/version for selective serialization.

- **Observability**
  - All core operations are annotated with `#[instrument]` and `tracing::{info, debug, trace, error}` calls.
  - Ideal for debugging protocol encoding bugs and boundary conditions.

---

## Core Types

### `trait Backend`

```rust
pub trait Backend {
    fn size(&self) -> usize;
    fn ignore(&mut self, amount: usize);
}
```

A minimal API that backends for `OverrideStream` must implement. A `Backend` is conceptually:

- A byte stream with a **known size**, and
- The ability to **skip** bytes (`ignore`).

You typically combine `Backend` with `Read`/`Write` for fully functional streams.

---

### `OverrideStream<Stream>`

```rust
pub struct OverrideStream<Stream> {
    stream:    *mut Stream,
    n_type:    i32,
    n_version: i32,
}
```

Implements:

- `GetType`, `GetVersion`
- `StreamItems` (placeholder, `todo!()`)
- `StreamInto` (placeholder, `todo!()`)

And, when `Stream: Backend`:

- `new(stream_ptr: *mut Stream, n_type_in: i32, n_version_in: i32)`
- `write(&mut self, pch: *const u8, n_size: usize)`
- `read(&mut self, pch: *mut u8, n_size: usize)`
- `size(&self) -> usize`
- `ignore(&mut self, amount: usize)`

**Usage pattern** (unsafe by design):

```rust
use bitcoin_bitstream::{Backend, OverrideStream};
use std::io::{Read, Write};

struct MyBuf {
    buf: Vec<u8>,
    pos: usize,
}

impl Backend for MyBuf {
    fn size(&self) -> usize { self.buf.len() - self.pos }
    fn ignore(&mut self, amount: usize) { self.pos = (self.pos + amount).min(self.buf.len()); }
}

impl Read for MyBuf {
    fn read(&mut self, dst: &mut [u8]) -> std::io::Result<usize> {
        let n = dst.len().min(self.size());
        dst[..n].copy_from_slice(&self.buf[self.pos..self.pos + n]);
        self.pos += n;
        Ok(n)
    }
}

impl Write for MyBuf {
    fn write(&mut self, src: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(src);
        Ok(src.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

let mut backend = MyBuf { buf: Vec::new(), pos: 0 };
let ptr = &mut backend as *mut MyBuf;
let mut ovr = OverrideStream::new(ptr, /*type=*/0, /*version=*/1);

let data = [0x01u8, 0x02, 0x03];
unsafe {
    ovr.write(data.as_ptr(), data.len());
}
```

**Safety**: `OverrideStream` stores a raw `*mut Stream` and dereferences it under `unsafe`. You must guarantee the pointer is:

- Non-null,
- Properly aligned,
- Valid for the lifetime of the `OverrideStream`, and
- Not simultaneously aliased in ways that violate Rust’s aliasing rules.

---

### `BitStreamReader<IStream>`

```rust
pub struct BitStreamReader<IStream: Default + Read> {
    istream: Rc<RefCell<IStream>>,
    buffer:  u8,
    offset:  i32,
}
```

`BitStreamReader` exposes `read(&mut self, nbits: i32) -> u64`, reading **`nbits` most-significant bits** from the underlying `Read` stream, up to 64.

- `offset` is the number of **consumed bits** in `buffer` (`0..=8`).
- When `offset == 8`, the next byte is fetched from the underlying stream.
- Bits are read MSB-first from each byte (consistent with Bitcoin’s Huffman-like bit encodings).

Example:

```rust
use bitcoin_bitstream::BitStreamReader;
use std::{cell::RefCell, io::Cursor, rc::Rc};

let data = [0b1011_0010u8, 0b0110_0000u8];
let cursor = Cursor::new(&data[..]);
let rc = Rc::new(RefCell::new(cursor));

let mut rdr = BitStreamReader::new(rc);

// Read the first 4 bits: 1011 == 0b1011 == 11
let first = rdr.read(4);
assert_eq!(first, 0b1011);

// Next 4 bits: 0010 == 2
let second = rdr.read(4);
assert_eq!(second, 0b0010);

// Cross-byte reads are handled transparently.
let third = rdr.read(4); // 0110 == 6
assert_eq!(third, 0b0110);
```

This is appropriate for:

- Compact integer encodings
- Prefix codes
- Flag bitfields embedded in the Bitcoin network protocol

Mathematically, the reader maintains a sliding window over the bitstream, equivalent to a left-fold of the form

\[
R_{k+1} := (R_k \ll b_k) \;\vert\; M_k,
\]

where \( R_k \) is the accumulated result, \( b_k \) is the number of bits consumed on iteration \( k \), and \( M_k \) is the extracted bit mask from `buffer` at the current offset.

---

### `BitStreamWriter<OStream>`

```rust
pub struct BitStreamWriter<OStream: Default + Write> {
    ostream: Rc<RefCell<OStream>>,
    buffer:  u8,
    offset:  i32,
}
```

Provides:

- `new(ostream: Rc<RefCell<OStream>>) -> Self`
- `write(&mut self, data: u64, nbits: i32)`
- `flush(&mut self)`
- `Drop` implementation that auto-flushes remaining bits

Semantics mirror `BitStreamReader`, but in reverse: bits from `data` are streamed out MSB-first into full bytes.

```rust
use bitcoin_bitstream::BitStreamWriter;
use std::{cell::RefCell, io::Cursor, rc::Rc};

let buf = Cursor::new(Vec::<u8>::new());
let rc = Rc::new(RefCell::new(buf));

{
    let mut w = BitStreamWriter::new(rc.clone());
    // write 3 bits 0b101 and then 5 bits 0b10011
    w.write(0b101, 3);
    w.write(0b10011, 5);
    // Drop will flush the final partial byte
}

let inner = Rc::try_unwrap(rc).unwrap().into_inner();
assert_eq!(inner.into_inner(), vec![0b1011_0011]);
```

This is suitable for implementing Bitcoin’s compact bit encodings and low-level codecs where you want deterministic bit layout without depending on higher-level formats.

---

### `DataStream`

```rust
pub struct DataStream {
    vch:        SerializeData,  // Vec<u8, ZeroAfterFreeAllocator>
    n_read_pos: u32,
    n_type:     i32,
    n_version:  i32,
}
```

`DataStream` is a buffer-centric abstraction designed to replicate C++ `CDataStream` behavior. The stream is conceptually:

- `vch[..n_read_pos]` — already consumed
- `vch[n_read_pos..]` — unread / available

Key methods:

- **Construction**
  - `new(n_type, n_version)` – empty buffer
  - `new_with_slice(sp, n_type, n_version)` – copy from existing slice
  - `new_with_args(n_type, n_version, args)` – placeholder for multi-argument serialize

- **Inspection**
  - `size() -> usize` – unconsumed size
  - `empty() -> bool`
  - `str_() -> String` – UTF‑8 lossy view of unconsumed bytes
  - `get_type()`, `set_type()`, `get_version()`, `set_version()`

- **Cursor / lifetime control**
  - `rewind(n: Option<usize>) -> bool` – move `n_read_pos` backward or reset to 0
  - `compact()` – physically drop consumed prefix
  - `clear()`
  - `eof() -> bool` – `size() == 0`

- **Raw access**
  - `as_slice() -> &[u8]`, `as_mut_slice() -> &mut [u8]`
  - `data() -> *const u8`, `data_mut() -> *mut u8`
  - `Index` / `IndexMut<usize>` – random access from `n_read_pos`

- **Reading / writing by pointer**
  - `read(pch: *mut u8, n_size: usize)` – copies from `vch[n_read_pos..]` to `pch`, advances `n_read_pos`
  - `write(pch: *const u8, n_size: usize)` – extend `vch` by copying from `pch`
  - `ignore(n_size: i32)` – advance `n_read_pos`, panics on overflow

- **Structural modifications**
  - `insert_item(it, x) -> ZeroAfterFreeVecIter`
  - `insert_multi(it, n, x)`
  - `insert_with_iterator_range(it, first, last)`
  - `insert_with_pointer_range(it, first, last)`
  - `erase(it) -> ZeroAfterFreeVecIter`
  - `erase_range(first, last) -> ZeroAfterFreeVecIter`

- **Utility**
  - `xor(&mut self, key: &Vec<u8>)` – XORs unread portion with repeating key (typically for simple obfuscation)

Example: basic use as a serialization buffer:

```rust
use bitcoin_bitstream::DataStream;

let mut ds = DataStream::new(/*type=*/0, /*version=*/1);

// write some bytes
let payload = b"hello";
unsafe {
    ds.write(payload.as_ptr(), payload.len());
}
assert_eq!(ds.size(), 5);

// read them back
let mut out = [0u8; 5];
unsafe {
    ds.read(out.as_mut_ptr(), out.len());
}
assert_eq!(&out, b"hello");
assert!(ds.empty());
```

Because `DataStream` is pointer-based in several APIs, all pointer-safety considerations apply: you must ensure the destination buffers are large enough and valid for writes.

---

## Endian Helpers and Bit Counting

The crate exposes a series of free functions for direct integer encoding/decoding at fixed endianness, all operating on raw pointers:

```rust
pub fn writebe16(ptr: *mut u8, x: u16);
pub fn writele16(ptr: *mut u8, x: u16);
pub fn writebe32(ptr: *mut u8, x: u32);
pub fn writele32(ptr: *mut u8, x: u32);
pub fn writebe64(ptr: *mut u8, x: u64);
pub fn writele64(ptr: *mut u8, x: u64);

pub fn readbe16(ptr: *const u8) -> u16;
pub fn readle16(ptr: *const u8) -> u16;
pub fn readbe32(ptr: *const u8) -> u32;
pub fn readle32(ptr: *const u8) -> u32;
pub fn readbe64(ptr: *const u8) -> u64;
pub fn readle64(ptr: *const u8) -> u64;
```

These functions:

- Use `to_{le,be}_bytes` / `from_{le,be}_bytes` internally
- Call `std::ptr::copy_nonoverlapping` for the actual move
- Are heavily instrumented with `tracing`; suitable for debugging buffer boundaries

Simple usage:

```rust
use bitcoin_bitstream::{writele32, readle32};

let mut buf = [0u8; 4];
unsafe {
    writele32(buf.as_mut_ptr(), 0xDEADBEEF);
    let x = readle32(buf.as_ptr());
    assert_eq!(x, 0xDEADBEEF);
}
```

The crate also provides:

```rust
pub fn count_bits(x: u64) -> u64;
```

This returns

\[
\min\{ n \in \mathbb{N} : (x \gg n) = 0 \}
\]

or equivalently `0` if `x == 0`, else `64 - leading_zeros(x)`. In other words, the **position of the highest set bit plus one**, which is the base-2 bit length of `x`.

---

## Error Handling and Panics

This crate intentionally mirrors C++ behavior and **panics** on several runtime violations:

- `BitStreamReader::read` and `BitStreamWriter::write` panic if `nbits` is not in `0..=64`.
- `DataStream::read` panics on out-of-bounds reads (`end of data`).
- `DataStream::ignore` panics on negative lengths or skipping past the end.
- `OverrideStream::{read, write, ignore}` panic on null backend pointer or if the underlying `Read`/`Write`/`Backend` operations fail.

In production systems, you may wish to wrap these abstractions in fallible layers or pre-validate buffer boundaries.

---

## Logging / Tracing

All major operations are annotated with `#[instrument(level = "trace")]` and emit via the `tracing` crate:

- `info!` for high-level lifecycle events (construction, major calls),
- `debug!` for return values and internal decisions,
- `trace!` for hot-path details, and
- `error!` for logic violations prior to panicking.

Integrate with your subscriber of choice:

```rust
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    // use bitcoin-bitstream normally, logs will be emitted to stdout
}
```

This makes it straightforward to diagnose bit/byte-level mismatches between your implementation and a Bitcoin Core reference.

---

## Safety Considerations

- **Raw pointers**: Several APIs (`OverrideStream`, `DataStream::read`/`write`, endian helpers) accept or return raw pointers. Misuse can cause memory unsafety.
- **Allocator semantics**: The semantics of `ZeroAfterFreeAllocator` (and `SerializeData`) are assumed to be consistent with Bitcoin Core (zero memory on free). Verify the actual implementation in this repository if you rely on this for security.
- **Aliasing**: When working with `Rc<RefCell<IStream>>` and raw pointers, ensure you do not create aliasing patterns that violate Rust’s safety requirements.

This crate is targeted at advanced users building protocol-level infrastructure who are comfortable explicitly managing such constraints.

---

## Integration in `Cargo.toml`

```toml
[dependencies]
bitcoin-bitstream = "0.1.19"
tracing = "0.1"   # for instrumentation output
```

The crate uses Rust **edition 2021** and is distributed under the **MIT** license.

The upstream repository, which may include additional context such as `ZeroAfterFreeAllocator` and related types, is:

- <https://github.com/klebs6/bitcoin-rs>

---

## License

This project is licensed under the **MIT License**. See the upstream repository for the canonical license text.
