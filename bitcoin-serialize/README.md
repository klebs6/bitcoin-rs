# bitcoin-serialize

Low-level, allocation-conscious (de)serialization primitives that mirror Bitcoin Core's C++ serialization framework, implemented in Rust.

The crate targets protocol implementers and systems engineers who need **byte-for-byte compatibility** with Bitcoin Core's wire format, including CompactSize, VarInt, fixed-width integer encodings, IEEE-754 doubles, vectors, maps, and custom formatter-based serialization.

---

## Design goals

- **Compatibility with Bitcoin Core**
  - Match the semantics of C++ `Serialize` / `Unserialize`, `CSizeComputer`, `CompactSize`, `VarInt`, `Using<Formatter>`, and `VectorFormatter`.
  - Preserve DoS-safety constraints, especially incremental vector allocation with configurable maximum block size.

- **Zero-/low-cost abstractions**
  - Traits and const generics are used to resolve most decisions at compile time.
  - `SizeComputer` pretends to be an `io::Write` sink, allowing size computation to reuse normal `BtcSerialize` implementations.

- **Formatter-based extensibility**
  - Explicit separation between **what** is serialized (your types) and **how** bytes are laid out (formatters like `VarIntFormatter`, `CompactSizeFormatter`, `CustomUintFormatter`, etc.).
  - Formatters compose (
    e.g. `VectorFormatter<VarIntFormatter<...>>`, nested `VectorFormatter` for matrices, or wrapper-based adapters).

- **Type safety with const generics**
  - `VarIntMode` and `CustomUintFormatter<const BYTES: i32, const BIG_ENDIAN: bool>` enforce invariants at compile time.
  - `ModeConstraint` ensures that only admissible integer domains are used for a given `VarIntMode`.

---

## Core concepts

### 1. `BtcSerialize` / `BtcUnserialize`

The crate defines its own serialization traits instead of using `serde`:

```rust
pub trait BtcSerialize<Stream> {
    fn serialize(&self, s: &mut Stream);
}

pub trait BtcUnserialize<Stream> {
    fn unserialize(&mut self, s: &mut Stream);
}
``

- `Stream` is usually something that implements `std::io::Write` or `std::io::Read`, but can also be `SizeComputer` or other custom streams.
- A wide range of standard types has built-in implementations, e.g. primitive integers, `bool`, `String`, arrays of `u8`, `Vec<T>`, `HashMap`, `HashSet`, `Box`, `Arc`, tuples, and a Bitcoin-like `PreVector`.

This trait design deliberately mirrors Bitcoin Core's design, enabling transposition of protocol code from C++ to Rust with minimal structural changes.

### 2. Formatters and `ValueFormatter<T>`

A **formatter** encapsulates *how* a value of type `T` is physically encoded:

```rust
pub trait ValueFormatter<T> {
    fn ser<S: Write>(&mut self, s: &mut S, value: &T);
    fn unser<S: Read>(&mut self, s: &mut S, value: &mut T);
}
```

Key formatters provided:

- `VarIntFormatter<const Mode: VarIntMode>` – Bitcoin's VarInt encoding with pluggable modes.
- `CompactSizeFormatter<const RangeCheck: bool>` – Bitcoin's CompactSize length encoding with optional maximum-size enforcement.
- `EncodedDoubleFormatter` – bit-exact IEEE-754 `f64` codec using the raw `u64` payload.
- `VectorFormatter<F>` – collection formatter parameterized by an element formatter `F`.
- `CustomUintFormatter<const BYTES: i32, const BIG_ENDIAN: bool>` – fixed-width unsigned integer formatter with compile-time width and endianness.
- `DefaultFormatter` – adapter connecting any `T` that already implements `BtcSerialize`/`BtcUnserialize` to the formatter interface.
- `LimitedStringFormatter<'a, const Limit: usize>` – string formatter with compile-time maximum length.

These formatters underpin the `Wrapper<'a, F, T>` type and the provided macros.

### 3. `Wrapper` and `using`

`Wrapper<'a, F, T>` directs serialization of `T` through a specific formatter `F` while reusing the generic `BtcSerialize` / `BtcUnserialize` machinery:

```rust
pub struct Wrapper<'a, F, T> {
    object:  &'a mut T,
    _marker: PhantomData<F>,
}

pub fn using<'a, F, T>(t: &'a mut T) -> Wrapper<'a, F, T> {
    Wrapper::new(t)
}
```

The crate implements `BtcSerialize` and `BtcUnserialize` for `Wrapper<'a, F, T>` whenever `F: ValueFormatter<T>`.

This is analogous to `Using<Formatter>(obj)` in Bitcoin Core and allows you to write:

```rust
use bitcoin_serialize::{using, VarIntFormatter, VarIntMode, BtcSerialize};

fn write_varint_u64<W: std::io::Write>(w: &mut W, value: &mut u64) {
    w.serialize(&using::<VarIntFormatter<{ VarIntMode::Default }>, _>(value));
}
```

In practice, you often use the macro sugar instead.

### 4. VarInt modes and constraints

Bitcoin's VarInt encoding is specialized and slightly nontrivial. This crate models supported encodings via a const-generic enum:

```rust
#[derive(Clone, Debug, ConstParamTy, PartialEq, Eq)]
pub enum VarIntMode {
    Default,
    NonNegativeSigned,
}

pub trait ModeConstraint<const M: VarIntMode, I> {}
```

`ModeConstraint` is implemented selectively:

```rust
impl<I: Unsigned> ModeConstraint<{ VarIntMode::Default }, I> for () {}
impl<I: Signed>   ModeConstraint<{ VarIntMode::NonNegativeSigned }, I> for () {}
```

This statically prevents misuse; for example, you cannot accidentally pass a signed integer to `VarIntMode::Default`.

Encoding/decoding is implemented by:

```rust
pub fn write_var_int<Stream, I, const Mode: VarIntMode>(os: &mut Stream, n: I) where
    (): ModeConstraint<Mode, I>,
    Stream: Write,
    I: Into<u128> + Copy
{ /* ... */ }

pub fn read_var_int<Stream, I, const Mode: VarIntMode>(is: &mut Stream) -> I where
    (): ModeConstraint<Mode, I>,
    Stream: Read,
    I: TryFrom<u128>,
    <I as TryFrom<u128>>::Error: core::fmt::Debug
{ /* ... */ }
```

Mathematically, the encoding uses a **prefix-decremented base-128 representation** (matching Bitcoin Core), where all but the final byte have the high continuation bit set and an adjusted magnitude to maintain canonical encoding.

`VarIntFormatter<Mode>` hooks this into the formatter interface and further supports the `varint!` and `varint_mode!` macros.

### 5. CompactSize encoding

Bitcoin’s `CompactSize` is a non-linear variable-length integer encoding optimized for the typical case of small lengths but able to represent up to 64-bit values:

- `n < 253`      → single byte `n`.
- `n ≤ 0xFFFF`   → `0xFD` followed by `u16` little-endian.
- `n ≤ 0xFFFF_FFFF` → `0xFE` followed by `u32` little-endian.
- else           → `0xFF` followed by `u64` little-endian.

The crate exposes both pure functions and a formatter interface:

```rust
pub fn get_size_of_compact_size(n_size: u64) -> u32;
pub fn write_compact_size<Stream: Write>(os: &mut Stream, n_size: u64);
pub fn read_compact_size<Stream: Read>(is: &mut Stream, range_check: Option<bool>) -> u64;

pub struct CompactSizeFormatter<const RangeCheck: bool>;
```

`RangeCheck = true` enforces `n_size <= MAX_SIZE` (a DoS resilience guard, as in Bitcoin Core) and panics otherwise.

---

## Helper macros

This crate provides a macro DSL that mirrors Bitcoin Core’s C++ macro layer.

### `varint!` and `varint_mode!`

```rust
#[macro_export]
macro_rules! varint_mode {{
    ($obj:expr, $mode:ident) => {
        $crate::var_int_formatter::VarIntFormatter::<{
            $crate::var_int_mode::VarIntMode::$mode
        }>::new($obj)
    };
}}

#[macro_export]
macro_rules! varint {{
    ($obj:expr) => {
        $crate::var_int_formatter::VarIntFormatter::<{
            $crate::var_int_mode::VarIntMode::Default
        }>::new($obj)
    };
}}
```

Usage:

```rust
use bitcoin_serialize::{varint, BtcSerialize};

fn write_version<W: std::io::Write>(w: &mut W, version: &mut u32) {
    w.serialize(&varint!(version));
}
```

### `compactsize!`

```rust
#[macro_export]
macro_rules! compactsize {{
    ($obj:expr) => {
        $crate::compact_size_formatter::CompactSizeFormatter::<true>::new($obj)
    };
}}
```

`RangeCheck = true` is wired in; this is appropriate for length fields.

Usage:

```rust
use bitcoin_serialize::{compactsize, BtcSerialize};

fn write_len<W: std::io::Write>(w: &mut W, len: &mut u64) {
    w.serialize(&compactsize!(len));
}
```

### `limited_string!`

```rust
#[macro_export]
macro_rules! limited_string {{
    ($obj:expr, $n:ident) => {
        $crate::limited_string_formatter::LimitedStringFormatter::<$n> { item: $obj }
    };
}}
```

Allows enforcing a compile-time upper bound on decoded string length.

### `readwrite!`, `readwriteas!`, `ser_read!`, `ser_write!`, and `formatter_methods!`

These macros help you implement C++-style composite serialization logic for your own types:

```rust
#[macro_export]
macro_rules! formatter_methods {{
    ($cls:ident, $obj:ident) => {
        fn ser<Stream: std::io::Write>(&self, s: &mut Stream) {
            Self::serialization_ops(self, s, $crate::action::SerActionSerialize {})
        }
        fn unser<Stream: std::io::Read>(&mut self, s: &mut Stream) {
            Self::serialization_ops(self, s, $crate::action::SerActionUnserialize {})
        }
        fn serialization_ops<Stream, Op>(
            &mut self,
            s: &mut Stream,
            ser_action: Op,
        ) where
            Stream: std::io::Read + std::io::Write,
            Op: crate::action::SerActionSerialize + crate::action::SerActionUnserialize,
        {
            let $obj = self;
            // user-supplied body follows the macro invocation
        }
    };
}}
```

Example – implementing a custom formatter and plugging it into `BtcSerialize`:

```rust
use bitcoin_serialize::{
    formatter_methods,
    serialize_methods,
    readwrite,
    varint,
    BtcSerialize,
    BtcUnserialize,
};

#[derive(Clone, Default)]
struct MyHeader {
    version: i32,
    timestamp: u32,
}

struct MyHeaderFormatter;

impl MyHeaderFormatter {
    formatter_methods!(MyHeaderFormatter, obj);
}

// Provide the serialization body:
impl MyHeaderFormatter {
    fn serialization_ops<Stream, Op>(
        &mut self,
        s: &mut Stream,
        op: Op,
    ) where
        Stream: std::io::Read + std::io::Write,
        Op: bitcoin_serialize::SerActionSerialize + bitcoin_serialize::SerActionUnserialize,
    {
        let obj = self; // already set by macro in the real code
        // Example layout: VarInt version, then raw timestamp
        readwrite!(s, op, varint!(&mut obj.version), &mut obj.timestamp);
    }
}

// Tie it to BtcSerialize / BtcUnserialize for the type of interest:
serialize_methods!(MyHeader, obj);
```

> Note: The exact macro composition in real code is slightly different; refer to the examples and the source for idiomatic patterns.

### `serialize_methods!`

```rust
#[macro_export]
macro_rules! serialize_methods {{
    ($cls:ident, $obj:ident) => {
        impl<Stream: std::io::Write> $crate::serialize::BtcSerialize<Stream> for $cls {
            fn serialize(&self, s: &mut Stream) {
                let mut me = self.clone();
                me.ser(s);
            }
        }
        impl<Stream: std::io::Read> $crate::unserialize::BtcUnserialize<Stream> for $cls {
            fn unserialize(&mut self, s: &mut Stream) {
                self.unser(s);
            }
        }
    };
}}
```

This macro bridges a type that exposes `ser` and `unser` methods into `BtcSerialize` / `BtcUnserialize` via cloning for the serialize phase.

---

## Size computation and `SizeComputer`

Computing serialized sizes without actually emitting bytes is needed for resource planning and fee computation. This crate provides a faithful reimplementation of `CSizeComputer`:

```rust
pub struct SizeComputer {
    n_size:    usize,
    n_version: i32,
}

impl SizeComputer {
    pub fn new(n_version_in: i32) -> Self;
    pub fn add_bytes(&mut self, n_size: usize);
    pub fn write_ptr(&mut self, _psz: *const u8, n_size: usize);
    pub fn seek(&mut self, n_size: usize);
    pub fn size(&self) -> usize;
    pub fn get_version(&self) -> i32;
}

impl Write for SizeComputer { /* increments counters only */ }

impl<'a, T> std::ops::Shl<&'a T> for SizeComputer
where
    T: BtcSerialize<SizeComputer>,
{
    type Output = SizeComputer;
    fn shl(mut self, rhs: &'a T) -> Self::Output { /* ... */ }
}
```

Convenience functions:

```rust
pub fn get_serialize_size<T>(t: &T, n_version: Option<i32>) -> usize
where
    T: BtcSerialize<SizeComputer>;

pub fn get_serialize_size_many<Args>(n_version: i32, args: &Args) -> usize
where
    Args: SerializeMany<SizeComputer>;
```

This allows you to compute sizes for arbitrary objects and tuples that implement `BtcSerialize` or `SerializeMany` without code duplication.

Example:

```rust
use bitcoin_serialize::{BtcSerialize, SizeComputer, get_serialize_size};

#[derive(Clone)]
struct TxHeader { /* fields with BtcSerialize implementations */ }

fn size_of_header(h: &TxHeader) -> usize {
    get_serialize_size(h, Some(0))
}
```

---

## Vector and matrix helpers for `f64`

The crate includes dedicated helpers for serialization of vectors and matrices of `f64`, using `EncodedDoubleFormatter` under the hood:

```rust
pub fn write_vec_f64_fmt<S: Write>(s: &mut S, v: &Vec<f64>);
pub fn read_vec_f64_fmt<S: Read>(s: &mut S) -> Vec<f64>;

pub fn write_matrix_f64_fmt<S: Write>(s: &mut S, m: &Vec<Vec<f64>>);
pub fn read_matrix_f64_fmt<S: Read>(s: &mut S) -> Vec<Vec<f64>>;
```

Serialization layout:

- A `Vec<T>` is encoded as a CompactSize length, followed by each element encoded by the chosen formatter.
- `Vec<f64>` therefore becomes: `CompactSize(len)`, then `len` encoded doubles.
- A `Vec<Vec<f64>>` (matrix) nests `VectorFormatter<EncodedDoubleFormatter>`, resulting in a CompactSize row count, then each row as `Vec<f64>`.

This design is suitable for deterministic numerical serialization, where exact IEEE-754 bit patterns must be preserved for reproducibility (e.g. consensus-critical simulations, deterministic randomization, or cryptographic precomputation tables).

---

## Low-level primitives and utilities

The crate also exposes several low-level, Bitcoin-compatible primitives:

- **Fixed-endian integer IO**:

  ```rust
  pub fn write_u32_le<W: Write>(w: &mut W, v: u32) -> IoResult<()>;
  pub fn read_u32_le<R: Read>(r: &mut R) -> IoResult<u32>;

  pub fn ser_writedata8<Stream: Write>(s: &mut Stream, obj: u8);
  pub fn ser_writedata16<Stream: Write>(s: &mut Stream, obj: u16);
  pub fn ser_writedata32<Stream: Write>(s: &mut Stream, obj: u32);
  pub fn ser_writedata64<Stream: Write>(s: &mut Stream, obj: u64);

  pub fn ser_readdata8<Stream: Read>(s: &mut Stream) -> u8;
  pub fn ser_readdata16<Stream: Read>(s: &mut Stream) -> u16;
  pub fn ser_readdata32<Stream: Read>(s: &mut Stream) -> u32;
  pub fn ser_readdata64<Stream: Read>(s: &mut Stream) -> u64;
  ```

- **IEEE-754 double encoding**:

  ```rust
  pub fn encode_double(v: f64) -> u64;
  pub fn decode_double(x: u64) -> f64;

  pub fn write_encoded_f64<W: Write>(w: &mut W, v: f64) -> IoResult<()>;
  pub fn read_encoded_f64<R: Read>(r: &mut R) -> IoResult<f64>;
  ```

- **Fast uniform mapping** (`map_into_range`): Given a 64-bit uniformly random `x` in `[0, 2^64)`, map it uniformly into `[0, n)` by taking the upper 64 bits of `x * n`:

  ```rust
  pub fn map_into_range(x: u64, n: u64) -> u64;
  ```

  This is based on Lemire's fast modulo reduction without bias:

  \[
  \text{map}(x, n) = \left\lfloor \frac{x \cdot n}{2^{64}} \right\rfloor
  \]

  On platforms with `__int128` support, this is implemented directly; otherwise it falls back to a 32×32→64-bit decomposition.

---

## Tuple serialization (`SerializeMany` / `UnserializeMany`)

To emulate Bitcoin Core's variadic templates for serializing multiple arguments in lockstep, the crate defines:

```rust
pub trait SerializeMany<Stream> {
    fn serialize_many(&self, s: &mut Stream);
}

pub trait UnserializeMany<Stream> {
    fn unserialize_many(&mut self, s: &mut Stream);
}
```

Implementations are provided for tuples up to arity 4 (and can be extended). This is what powers macros like `readwrite!` and enables `get_serialize_size_many`.

Example:

```rust
use bitcoin_serialize::{SerializeMany, UnserializeMany, BtcSerialize, BtcUnserialize};

fn write_pair<S: std::io::Write, A, B>(s: &mut S, a: &A, b: &B)
where
    S: std::io::Write,
    A: BtcSerialize<S>,
    B: BtcSerialize<S>,
{
    (a, b).serialize_many(s);
}
```

---

## Safety and failure modes

- **Panics on malformed data**: For consensus-like usage, it is often preferable to abort rather than attempt recovery on structurally invalid data. Functions such as `read_compact_size`, `read_var_int`, and `CustomUintFormatter::*` will panic on non-canonical encodings or out-of-range values.
- **Bounds checks**: `CompactSize` and vector deserialization apply range checks and incremental allocation limits (`MAX_VECTOR_ALLOCATE`) to reduce memory-exhaustion attack surface.
- **`unsafe` usage**: Restricted to performance-critical string handling (`String::as_mut_vec()` length manipulation) and slice reinterpretation for `&[u8]` deserialization. These sections are carefully constrained and mirror the C++ semantics.

When integrating into external systems, consider whether panics are acceptable or whether additional defensive wrappers should be introduced.

---

## Example: serializing a simple struct

Suppose you have a basic header struct and want a Bitcoin-like serialization layout:

```rust
use bitcoin_serialize::{
    BtcSerialize, BtcUnserialize,
    write_compact_size, read_compact_size,
    ser_writedata32, ser_readdata32,
};

#[derive(Clone, Default)]
struct SimpleHeader {
    version: u32,
    payload_size: u64,
}

impl<Stream: std::io::Write> BtcSerialize<Stream> for SimpleHeader {
    fn serialize(&self, s: &mut Stream) {
        ser_writedata32(s, self.version);
        write_compact_size(s, self.payload_size);
    }
}

impl<Stream: std::io::Read> BtcUnserialize<Stream> for SimpleHeader {
    fn unserialize(&mut self, s: &mut Stream) {
        self.version = ser_readdata32(s);
        self.payload_size = read_compact_size(s, Some(true));
    }
}
```

With this in place, `SimpleHeader` can be serialized to any stream that implements `Write`, or to a `SizeComputer` for size estimation.

---

## When to use this crate

Use `bitcoin-serialize` when:

- You need **exact compatibility** with Bitcoin Core's wire-level serialization.
- You are porting or reusing C++ serialization logic and want a **nearly isomorphic Rust API**, including macros and formatters.
- You need deterministic, low-level control over the encoded byte layout, including advanced constructs like partial streams, size pre-computation, custom allocators, or incremental vector decoding.

If you only require high-level schema-based serialization and do not care about Bitcoin Core compatibility, `serde` with a suitable data format might be more appropriate. This crate is specialized for Bitcoin-style protocols and the engineering constraints that accompany them.

---

## Repository and license

- Repository: <https://github.com/klebs6/bitcoin-rs>
- Crate: `bitcoin-serialize`, version `0.1.19`
- License: MIT

Consult the repository for integration examples, tests, and the latest API extensions.
