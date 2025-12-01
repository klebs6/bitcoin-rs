# bitcoin-vectorstream

A minimal, allocation-conscious byte stream abstraction over existing `Vec<u8>` buffers, tailored for Bitcoin serialization pipelines.

This crate provides two core types:

- `VectorReader`: an `std::io::Read` implementation over an immutable `Arc<Vec<u8>>` slice with an internal cursor.
- `VectorWriter`: an `std::io::Write`-compatible sink that overwrites and/or appends into an `Rc<RefCell<Vec<u8>>>` buffer.

Both types are designed to integrate with the `bitcoin_serialize` traits (`BtcSerialize`, `BtcUnserialize`, `SerializeMany`, `UnserializeMany`) and to support type- and version-tagged wire formats (e.g., Bitcoin consensus and network encodings) without incidental allocations.

---

## Features at a glance

- Zero-copy **read** over an existing `Arc<Vec<u8>>` with bounds-checked cursor management.
- In-place **write** into an `Rc<RefCell<Vec<u8>>>`, overwriting existing bytes and growing the vector only when needed.
- `Read` / `Write` implementations for drop-in use with standard I/O traits.
- Type and version parameters propagated through the stream, simplifying protocol negotiation and feature-flagged encodings.
- `Builder` + `Getters` (via derive) for ergonomic construction and access.
- Interoperability with `bitcoin_serialize` traits via `<<` / `>>` operators (`Shl` / `Shr` impls).

The design targets high-performance Bitcoin serialization where:

- Ownership of the underlying buffer is **external** to the stream, and
- The same buffer may be shared across multiple components or re-used for multiple encodings.

---

## Core concepts

### VectorReader

`VectorReader` acts as a lightweight, non-owning input stream over an `Arc<Vec<u8>>`:

```rust
use std::io::Read;
use std::sync::Arc;
use bitcoin_vectorstream::VectorReader;

let data: Arc<Vec<u8>> = Arc::new(vec![0x01, 0x02, 0x03, 0x04]);
let mut reader = VectorReader::new(
    /* ty      = */ 0,
    /* version = */ 0,
    data.clone(),
    /* pos     = */ 0,
);

let mut buf = [0u8; 2];
reader.read(&mut buf).unwrap();
assert_eq!(&buf, &[0x01, 0x02]);
assert!(!reader.empty());
assert_eq!(reader.size(), 2); // bytes remaining
```

#### Semantics

- `new(ty, version, data, pos)`
  - `ty`: Serialization type discriminator (e.g., network vs. disk format).
  - `version`: Encoding version / flags (e.g., protocol version, segwit flags).
  - `data`: Shared byte buffer (`Arc<Vec<u8>>`).
  - `pos`: Initial cursor position. Must satisfy `pos <= data.len()`, otherwise the constructor **panics**.

- `size(&self) -> usize`
  - Returns the number of unread bytes: `data.len() - pos`.

- `empty(&self) -> bool`
  - Returns `true` when `pos == data.len()`.

- `impl Read for VectorReader`
  - `read(&mut self, buf: &mut [u8])`:
    - Copies up to `buf.len()` bytes from the current cursor.
    - Advances the internal position.
    - Never allocates; uses `ptr::copy_nonoverlapping` under the hood.

- `impl Default for VectorReader`
  - Produces a trivially empty reader (`Arc<Vec<u8>>::new()`, cursor at `0`). Primarily intended for generic contexts that require a `Default` bound.

#### Bitcoin integration via `>>`

`VectorReader` implements `Shr<&mut T>` where `T: bitcoin_serialize::BtcUnserialize<VectorReader> + ?Sized`. This enables piping deserialization via `>>`:

```rust
use std::sync::Arc;
use bitcoin_vectorstream::VectorReader;

fn deserialize_message<T>(bytes: Arc<Vec<u8>>) -> T
where
    T: bitcoin_serialize::BtcUnserialize<VectorReader> + Default,
{
    let mut msg = T::default();
    let reader = VectorReader::new(0, 0, bytes, 0);
    let _ = reader >> &mut msg; // calls msg.unserialize(&mut reader)
    msg
}
```

### VectorWriter

`VectorWriter` is a mutable output stream over `Rc<RefCell<Vec<u8>>>`, capable of overwriting existing bytes and appending beyond the current end.

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use bitcoin_vectorstream::VectorWriter;

let backing = Rc::new(RefCell::new(Vec::<u8>::new()));
let mut writer = VectorWriter::new(
    /* n_type    = */ 0,
    /* n_version = */ 0,
    backing.clone(),
    /* n_pos     = */ 0,
);

writer.write(&[0xAA, 0xBB]).unwrap();
writer.write(&[0xCC]).unwrap();

assert_eq!(&*backing.borrow(), &[0xAA, 0xBB, 0xCC]);
```

#### Semantics

- `new(n_type, n_version, vch_data, n_pos)`
  - `n_type`: Serialization type discriminator.
  - `n_version`: Encoding version / flags.
  - `vch_data`: Shared, interior-mutable buffer (`Rc<RefCell<Vec<u8>>>`).
  - `n_pos`: Initial write position.
    - If `n_pos > vch_data.len()`, the underlying vector is resized to `n_pos` (filled with zeros) before any write.

- Write behavior:
  - Writing `n_size` bytes at `n_pos` performs:
    - Overwrite up to `min(n_size, len - n_pos)` bytes in-place.
    - If `n_size` exceeds available overwrite capacity, the excess is appended (the vector grows accordingly).
  - Internal cursor `n_pos` is incremented by `n_size` for each call.

- `impl Write for VectorWriter`
  - `write(&mut self, buf: &[u8]) -> io::Result<usize>` forwards to the raw pointer-based `write` method.
  - `flush(&mut self)` is a no-op (`Ok(())`), appropriate for in-memory buffers.

- `impl Default for VectorWriter`
  - Creates an empty `Rc<RefCell<Vec<u8>>>` with cursor at `0`, suitable for generic streaming code.

- Raw write API:

```rust
impl VectorWriter {
    /// Write `n_size` bytes from `pch` into the stream.
    pub fn write(&mut self, pch: *const u8, n_size: usize) {
        // ...
    }
}
```

This method is used internally and exposed explicitly for high-performance integrations where the caller already has raw pointers and lengths.

#### Bitcoin integration via `<<`

`VectorWriter` implements `Shl<&T>` for `T: bitcoin_serialize::BtcSerialize<VectorWriter> + ?Sized`, enabling expressive serialization pipelines:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use bitcoin_vectorstream::VectorWriter;

fn serialize_message<T>(msg: &T) -> Rc<RefCell<Vec<u8>>>
where
    T: bitcoin_serialize::BtcSerialize<VectorWriter>,
{
    let backing = Rc::new(RefCell::new(Vec::<u8>::new()));
    let writer = VectorWriter::new(0, 0, backing.clone(), 0);
    let _ = writer << msg; // calls msg.serialize(&mut writer)
    backing
}
```

### Batch (de)serialization helpers

Both reader and writer offer `*_with_args` constructors parameterized by `bitcoin_serialize::UnserializeMany` and `SerializeMany` respectively:

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use bitcoin_vectorstream::{VectorReader, VectorWriter};

// Deserialize multiple objects in one pass
fn read_many<Args>(bytes: Arc<Vec<u8>>, args: Args)
where
    Args: bitcoin_serialize::UnserializeMany<VectorReader>,
{
    let _reader = VectorReader::new_with_args(0, 0, bytes, 0, args);
}

// Serialize multiple objects in one pass
fn write_many<Args>(backing: Rc<RefCell<Vec<u8>>>, args: Args)
where
    Args: bitcoin_serialize::SerializeMany<VectorWriter>,
{
    let _writer = VectorWriter::new_with_args(0, 0, backing, 0, args);
}
```

These constructors encapsulate a common pattern in Bitcoin protocol code: construct the stream, immediately (de)serialize a set of fields, and then retain (or drop) the stream.

---

## Safety, invariants, and performance

### Invariants

- `VectorReader::new` panics if `pos > data.len()`. Callers must ensure the starting offset is within bounds.
- `VectorWriter::new` ensures that after construction, `n_pos <= vch_data.len()` via a resize if needed.
- `VectorWriter::write` asserts that `n_pos <= vec_ref.len()` before performing pointer arithmetic.

These invariants allow the implementations to safely employ `unsafe` pointer operations for maximal throughput while staying memory-safe at the API boundary.

### Complexity

- All operations are (O(n)) in the number of bytes copied; there is no additional per-byte overhead beyond the intrinsic cost of `ptr::copy_nonoverlapping`.
- No extra allocations occur during reads.
- Writes only allocate when growing the backing vector; overwrites are in-place and do not allocate.

### Concurrency and sharing

- `VectorReader` uses `Arc<Vec<u8>>`, enabling cheap cloning and safe cross-thread sharing of immutable data.
- `VectorWriter` uses `Rc<RefCell<Vec<u8>>>`, which is strictly single-threaded but allows interior mutability and logical sharing within a thread or task.

In typical Bitcoin applications, large serialized messages are shared read-only across components (favoring `Arc`), while mutable buffers are confined to a single execution context (favoring `Rc<RefCell<_>>`).

---

## Integration with bitcoin-rs

This crate resides in the `bitcoin-rs` repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>

It is likely used internally by higher-level modules implementing consensus serialization for blocks, transactions, P2P messages, and related structures.

You can treat `bitcoin-vectorstream` as the low-level streaming substrate and build your own domain-specific codecs on top of it, provided they implement the `bitcoin_serialize` traits.

---

## Usage examples

### Appending to an existing buffer

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use bitcoin_vectorstream::VectorWriter;

let backing = Rc::new(RefCell::new(vec![0x00, 0x11, 0x22]));
// Start writing at the current end -> append
let start_pos = backing.borrow().len();
let mut writer = VectorWriter::new(0, 0, backing.clone(), start_pos);

writer.write(&[0x33, 0x44]).unwrap();
assert_eq!(&*backing.borrow(), &[0x00, 0x11, 0x22, 0x33, 0x44]);
```

### Overwriting part of a buffer in-place

```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use bitcoin_vectorstream::VectorWriter;

let backing = Rc::new(RefCell::new(vec![0xDE, 0xAD, 0xBE, 0xEF]));
// Overwrite the middle two bytes
let mut writer = VectorWriter::new(0, 0, backing.clone(), 1);
writer.write(&[0x01, 0x02]).unwrap();

assert_eq!(&*backing.borrow(), &[0xDE, 0x01, 0x02, 0xEF]);
```

### Partial reads without slicing

```rust
use std::sync::Arc;
use std::io::Read;
use bitcoin_vectorstream::VectorReader;

let data = Arc::new(vec![10u8, 20, 30, 40, 50]);
let mut reader = VectorReader::new(0, 0, data.clone(), 2); // start at index 2

let mut buf = [0u8; 3];
let n = reader.read(&mut buf).unwrap();
assert_eq!(n, 3);
assert_eq!(&buf, &[30, 40, 50]);
assert!(reader.empty());
```

---

## Error handling and panics

- Constructors and write operations may panic when invariants are violated (e.g., inconsistent cursor vs. buffer length). These conditions typically indicate bugs at the call site and are not expected in well-formed protocol code.
- `Read`/`Write` trait methods themselves return `io::Result<usize>` / `io::Result<()>` but currently perform no fallible I/O beyond the bounds and invariants enforced by the constructors and internal logic.

In performance-critical serialization code, panics on invariant violation are often preferable to silently truncated or corrupted messages.

---

## Logging

The code uses macros such as `trace!`, `debug!`, and `error!` to instrument stream construction and raw writes. These are typically provided by a logging facade like `tracing` or `log` in the parent crate. Consult `bitcoin-rs` for the concrete logging backend and configuration.

---

## License

This crate is distributed under the MIT License.

See the root repository for full licensing details:

- <https://github.com/klebs6/bitcoin-rs>
