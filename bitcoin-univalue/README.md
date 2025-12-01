# bitcoin-univalue

A near byte-for-byte Rust transcription of Bitcoin Core's `UniValue` JSON subsystem, designed to preserve the original C++ layout, semantics, and test behavior while exposing an idiomatic Rust surface where possible.

---

## Overview

`bitcoin-univalue` provides a minimal, Bitcoin-Core–compatible JSON value type and parser:

- `UniValue`: a dynamically typed JSON value (null, bool, number, string, array, object) mirroring Bitcoin Core's `UniValue` implementation.
- A hand-ported recursive-descent JSON parser with a lexical/token layer.
- Canonical JSON number formatting precisely matching Bitcoin Core’s floating-point semantics, including the historical `FormatSaferDouble` quirks.
- UTF‑8 and UTF‑16 surrogate-pair validation and collation as specified in RFC 4627 / RFC 7159.
- A set of C-style expectation macros and APIs that keep the Rust code mechanically close to upstream C++ for auditability.

The crate is primarily intended as infrastructure for Bitcoin-related tooling that must stay behaviorally identical to upstream Bitcoin Core (e.g., for consensus-adjacent tooling, RPC regression testing, or protocol conformance checks), rather than as a general-purpose JSON library.

---

## Design goals

- **Semantic fidelity to Bitcoin Core**
  - Data model, parser behavior, and formatting must match upstream.
  - Edge cases (floating-point serialization, invalid JSON rejection, UTF‑8/UTF‑16 handling) are ported together with their upstream tests.
- **Auditability**
  - Control flow, names, and state machines remain close to the C++ original.
  - C-style helper macros (`expect!`, `set_expect!`, `clear_expect!`) preserve the original bitmask-based parser logic.
- **Rust ergonomics on the edges**
  - `From<T>` implementations for common scalars.
  - Index operators for object/array access (`uv["key"]`, `uv[0]`).
  - Optional utilities such as `check_object`, `get_obj_map`, and pretty-printing.

If you need a general JSON library, use `serde_json`. If you need *Bitcoin Core–exact* JSON behavior, use `bitcoin-univalue`.

---

## Data model: `UniValue`

```rust
#[derive(Setters, MutGetters, Getters, Clone, Debug)]
pub struct UniValue  {
    typ:    uni_value::VType,
    val:    String,
    keys:   Vec<String>,
    values: Vec<UniValue>,
}

mod uni_value {
    #[derive(Copy, PartialEq, Eq, Clone, Debug)]
    pub enum VType {
        VNULL,
        VOBJ,
        VARR,
        VSTR,
        VNUM,
        VBOOL,
    }
}
``

`UniValue` stores:

- the **type tag** (`VType`),
- a **string payload** for scalars (`val`),
- and for arrays/objects a `values` vector containing the elements and, for objects, a `keys` vector storing the key order.

This layout intentionally mirrors the original C++ `UniValue` structure, down to representation and method naming.

A canonical **null value** is provided as a global static:

```rust
lazy_static! {
    pub static ref NULL_UNI_VALUE: UniValue = UniValue::default();
}
```

By design, a default `UniValue` is `null`:

```rust
impl Default for UniValue {
    fn default() -> Self { Self::null() }
}
```

---

## Construction and conversion

### Constructors

```rust
impl UniValue {
    pub fn null() -> Self;
    pub fn empty_array() -> Self;

    /// Create with a given type and optional initial string content.
    pub fn new(initial_type: uni_value::VType, initial_str: Option<&str>) -> Self;
}
```

Examples:

```rust
use bitcoin_univalue::UniValue;
use bitcoin_univalue::uni_value::VType;

let null = UniValue::null();
let arr  = UniValue::empty_array();

let s = UniValue::new(VType::VSTR, Some("hello"));
let n = UniValue::new(VType::VNUM, Some("42"));
```

### `From<T>` implementations

`UniValue` implements `From` for several scalar types:

```rust
impl From<bool>          for UniValue { .. }
impl From<i32>           for UniValue { .. }
impl From<i64>           for UniValue { .. }
impl From<u64>           for UniValue { .. }
impl From<usize>         for UniValue { .. }
impl From<f64>           for UniValue { .. }
impl From<&str>          for UniValue { .. }
impl From<String>        for UniValue { .. }
impl From<*const u8>     for UniValue { .. }   // C string pointer
impl From<Instant>       for UniValue { .. }   // maps to null (upstream behavior)
impl From<uni_value::VType> for UniValue { .. }
```

Usage:

```rust
let a = UniValue::from(true);
let b = UniValue::from(123_i64);
let c = UniValue::from(3.14_f64);
let d = UniValue::from("string");
```

### Type setters

`UniValue` exposes explicit setters to control type and payload:

```rust
impl UniValue {
    pub fn set_i32(&mut self, val: i32) -> bool;
    pub fn set_i64(&mut self, val: i64) -> bool;
    pub fn set_u64(&mut self, val: u64) -> bool;
    pub fn set_float(&mut self, val: f64) -> bool;

    pub fn set_null(&mut self) -> bool;
    pub fn set_bool(&mut self, val: bool) -> bool;
    pub fn set_str(&mut self, val: &str) -> bool;

    /// Set from a string representing a JSON number. Validates syntax.
    pub fn set_num_str(&mut self, val: &String) -> bool;

    pub fn set_array(&mut self) -> bool;
    pub fn set_object(&mut self) -> bool;

    /// Generic integer convenience.
    pub fn set_int<T: Debug + std::fmt::Display>(&mut self, val: T) -> bool;

    pub fn clear(&mut self);  // reset to null
}
```

`set_num_str` performs syntactic validation against JSON number rules (no hex, no padding whitespace, etc.), via `parse_prechecks` and `valid_num_str` (not shown here but part of the crate).

---

## Introspection and typed accessors

### Type predicates

```rust
impl UniValue {
    pub fn is_null(&self)   -> bool;
    pub fn is_true(&self)   -> bool;
    pub fn is_false(&self)  -> bool;
    pub fn is_bool(&self)   -> bool;
    pub fn is_str(&self)    -> bool;
    pub fn is_num(&self)    -> bool;
    pub fn is_array(&self)  -> bool;
    pub fn is_object(&self) -> bool;

    pub fn get_type(&self) -> uni_value::VType;
    pub fn ty(&self) -> uni_value::VType; // shorthand

    pub fn empty(&self) -> bool;  // values vector empty
    pub fn size(&self)  -> usize; // number of elements for array/object
}
```

### Accessors (panic on type mismatch)

These mirror Bitcoin Core’s `UniValue` API and intentionally **panic** on incorrect types.

```rust
impl UniValue {
    pub fn get_bool(&self) -> bool;
    pub fn get_str(&self) -> &str;
    pub fn get_str_mut(&mut self) -> &mut str;

    pub fn get_int(&self)   -> i32;
    pub fn get_int64(&self) -> i64;
    pub fn get_real(&self)  -> f64;

    pub fn get_obj(&self)   -> &UniValue;   // ensure object
    pub fn get_array(&self) -> &UniValue;   // ensure array

    pub fn get_val_str(&self)   -> &String; // raw internal value
    pub fn get_keys(&self)      -> &Vec<String>; // only for objects
    pub fn get_values(&self)    -> &Vec<UniValue>; // for objects/arrays
}
```

Use these only when the type is known (e.g., after `check_object` or explicit checks).

---

## Object and array manipulation

### Object API

```rust
impl UniValue {
    /// key existence
    pub fn exists(&self, key: &str) -> bool;

    /// Copy object into a HashMap (no-op if not an object).
    pub fn get_obj_map(&self, kv: &mut HashMap<String, UniValue>);

    /// Schema-like check: every template key must exist with specified VType.
    pub fn check_object(&self, template: &HashMap<String, uni_value::VType>) -> bool;

    /// Low-level: find `key` and fill index in `ret_idx`.
    pub fn find_key(&self, key: &str, ret_idx: &mut usize) -> bool;

    /// Copy every key/value pair from `obj` into `self` (no deduplication).
    pub fn push_kvs(&mut self, obj: &UniValue) -> bool;

    /// Insert or replace key with val (converted via `Into<UniValue>`).
    pub fn pushkv<T: Into<UniValue>>(&mut self, key: &str, val: T) -> bool;
}
```

Example:

```rust
use bitcoin_univalue::UniValue;
use std::collections::HashMap;

let mut obj = UniValue::null();
obj.set_object();

obj.pushkv("answer", 42_i64);
obj.pushkv("flag", true);
obj.pushkv("label", "node-1");

assert!(obj.exists("answer"));
assert_eq!(obj["answer"].get_int64(), 42);

let mut template = HashMap::new();
use bitcoin_univalue::uni_value::VType;

template.insert("answer".to_owned(), VType::VNUM);
template.insert("flag".to_owned(),   VType::VBOOL);

aassert!(obj.check_object(&template));
```

### Array API

```rust
impl UniValue {
    /// Append a single element using runtime type inspection.
    pub fn push_back<T: Any>(&mut self, val: &T) -> bool;

    /// Bulk-append a vector of UniValues.
    pub fn push_backv(&mut self, vec: &Vec<UniValue>) -> bool;
}
```

The `push_back` method supports:

- `UniValue`
- `bool`, `i32`, `i64`, `u64`, `f64`
- `&str`, `String`

Example:

```rust
use bitcoin_univalue::UniValue;
use std::any::Any;

let mut arr = UniValue::empty_array();

arr.push_back(&true as &dyn Any);
arr.push_back(&123_i64 as &dyn Any);
arr.push_back(&"hello" as &dyn Any);

assert!(arr.is_array());
assert_eq!(arr.size(), 3);
assert!(arr[0].get_bool());
```

> Note: The `push_back` signature is intentionally constrained to maintain compatibility with the upstream logic and to preserve the ability to downcast from a generic `Any` stream.

---

## Indexing

`UniValue` implements `Index<&str>` and `Index<usize>`:

```rust
impl Index<&str> for UniValue {
    type Output = UniValue;

    fn index(&self, key: &str) -> &Self::Output {
        // object-only; otherwise returns &NULL_UNI_VALUE
    }
}

impl Index<usize> for UniValue {
    type Output = UniValue;

    fn index(&self, index: usize) -> &Self::Output {
        // arrays/objects; otherwise &NULL_UNI_VALUE
    }
}
```

Behavior:

- Attempting to index a non-object with `&str` returns a reference to `NULL_UNI_VALUE`.
- For arrays/objects, out-of-bounds indices also return `&NULL_UNI_VALUE`.

This is deliberate: upstream code often relies on a global immutable "null" sentinel rather than panicking or returning `Option`.

Example:

```rust
let mut obj = UniValue::null();
obj.set_object();
obj.pushkv("x", 1_i64);

assert_eq!(obj["x"].get_int64(), 1);
assert!(obj["missing"].is_null());
```

---

## JSON parsing and serialization

### Parsing: `UniValue::read`

The core JSON parser entry point is:

```rust
impl UniValue {
    /// Parse `size` bytes from `raw` into `self`.
    /// Returns `false` on lexical or structural error; `self` is left cleared on failure.
    pub fn read(&mut self, raw: *const u8, size: usize) -> bool;
}
```

This is an almost direct translation of Bitcoin Core’s JSON reader. It uses:

- A token layer (`get_json_token`, `JTokenType`) for lexical analysis.
- A **bitmask of parser expectations** (what token classes are allowed next), encoded in the `ExpectBits` enum.
- An explicit stack of container pointers to walk nested arrays/objects.
- A maximum nesting depth (`MAX_JSON_DEPTH`) matching upstream.

### Expectation bitmask and macros

```rust
#[repr(u32)]
pub enum ExpectBits {
    EXP_OBJ_NAME,
    EXP_COLON,
    EXP_ARR_VALUE,
    EXP_VALUE,
    EXP_NOT_VALUE,
}

#[macro_export]
macro_rules! expect { {
    ($expect_mask:expr,$bit:ident) => {
        ($expect_mask & (ExpectBits::$bit as u32)) != 0
    };
} }

#[macro_export]
macro_rules! set_expect { {
    ($expect_mask:expr,$bit:ident) => {
        $expect_mask |= ExpectBits::$bit as u32;
    };
} }

#[macro_export]
macro_rules! clear_expect { {
    ($expect_mask:expr,$bit:ident) => {
        $expect_mask &= !(ExpectBits::$bit as u32);
    };
} }
```

These macros operate on a `u32` bitmask tracking parser state. They reproduce the original C++ macros to keep the `read` function mechanically transcribable.

Parser example:

```rust
use bitcoin_univalue::UniValue;

let json = br#"{"a": 1, "b": [true, null]}"#;

let mut uv = UniValue::null();
let ok = uv.read(json.as_ptr(), json.len());
assert!(ok);

assert!(uv.is_object());
assert_eq!(uv["a"].get_int64(), 1);
assert!(uv["b"].is_array());
assert!(uv["b"][0].get_bool());
assert!(uv["b"][1].is_null());
```

> Safety: `read` accepts a raw pointer and length to be compatible with upstream code. It is the caller’s responsibility to ensure the lifetime and validity of the buffer.

### Serialization: `UniValue::write`

```rust
impl UniValue {
    /// Serialise this value to JSON.
    /// - pretty_indent == 0 → compact, single-line output.
    /// - indent_level        → current indentation depth.
    pub fn write(&self, pretty_indent: Option<u32>, indent_level: Option<u32>) -> String;
}
```

Strings are escaped via `json_escape`, which uses a precomputed `escapes` lookup table. Pretty-printing uses `indent_str` to append spaces.

Example:

```rust
let mut uv = UniValue::null();
uv.set_object();
uv.pushkv("x", 1_i64);
uv.pushkv("y", "str");

let compact = uv.write(Some(0), Some(0));
// {"x":1,"y":"str"}

let pretty = uv.write(Some(2), Some(0));
// {
//   "x": 1,
//   "y": "str"
// }
```

---

## UTF‑8 / UTF‑16 handling: `JSONUTF8StringFilter`

The parser must correctly implement RFC 4627 / RFC 7159 rules for JSON strings, notably:

- All JSON strings are sequences of Unicode scalar values.
- `\uXXXX` escapes represent UTF‑16 code units; surrogate pairs must be combined into a single scalar value.

`JSONUTF8StringFilter` is a small streaming filter that:

- Accepts raw bytes (`push_back`) that may be part of a multi-byte UTF‑8 sequence.
- Accepts UCS‑4 codepoints via `push_back_u` (often from decoded `\uXXXX` sequences).
- Combines surrogate pairs `(high, low)` into a single scalar value.
- Tracks invalid sequences (bad continuation bytes, dangling surrogates, etc.).

### Interface

```rust
pub struct JSONUTF8StringFilter  {
    str_:      Rc<RefCell<String>>,
    is_valid:  bool,
    codepoint: u32,
    state:     u8,
    surpair:   u32,
}

impl JSONUTF8StringFilter {
    pub fn new(target: Rc<RefCell<String>>) -> Self;

    /// Finalise: no open sequences or surrogate pairs allowed.
    pub fn finalize(&mut self) -> bool;

    /// Feed a single UTF‑8 byte.
    pub fn push_back(&mut self, ch: u8);

    /// Feed a full UCS‑4 codepoint (e.g., from `\uXXXX`).
    pub fn push_back_u(&mut self, cp: u32);
}
```

This state machine is essentially a deterministic finite automaton over DFA states `(state, surpair)`, verifying that:

- Every UTF‑8 multibyte sequence respects the `10xxxxxx` continuation rule.
- Surrogate ranges `0xD800..=0xDFFF` are not emitted directly; instead, high/low pairs are combined into a scalar in the supplementary plane.
- The filter ends in a quiescent state (`state == 0`, `surpair == 0`) before being considered valid.

---

## Canonical floating-point formatting

Bitcoin Core's JSON layer uses a special function `FormatSaferDouble` to avoid surprising rounding artifacts and to be stable across libc implementations. `bitcoin-univalue` reimplements this logic **bit-for-bit** as `UniValue::format_f64_canonical`.

```rust
impl UniValue {
    /// Canonical double formatting ported from Bitcoin Core.
    pub fn format_f64_canonical(val: f64) -> String;
}
```

Algorithm sketch:

1. Use **Ryu** to obtain the shortest finite representation: `short`.
2. Normalise `short`:
   - Strip a trailing `.0` unless the value is `0.0`.
   - If there is no decimal point or an exponent, accept as-is.
   - If there is a single decimal place and the value is exactly an integer × 0.10, pad a `0` to make two decimals (e.g. `-4.9` → `-4.90`).
   - If there are more than 2 decimals, keep Ryu’s output as-is.
3. For the remaining 2-decimal cases, build a `"%#.17g"` C-style representation using `libc::snprintf`.
4. Emulate a **glibc quirk**: toggle the last digit `0 → 1` when doing so still round-trips exactly to the same IEEE-754 value. Keep that variant.

Mathematically, this aims at an injective map from finite `f64` to decimal strings (within the domain used by Bitcoin Core) that is **stable** across platforms, while avoiding spurious trailing zeros or unnecessary exponentials.

The crate's `set_float` method uses this canonical formatter:

```rust
impl UniValue {
    pub fn set_float(&mut self, val: f64) -> bool {
        self.clear();
        self.set_typ(uni_value::VType::VNUM);
        self.set_val(Self::format_f64_canonical(val));
        true
    }
}
```

This is essential for deterministic signature payloads, regression testing, and any other application that persists or hashes the JSON textual form.

---

## Numeric parsing helpers

The crate includes strict numeric prechecks and convenience parsers.

### `parse_prechecks`

```rust
pub fn parse_prechecks(x: &str) -> bool;
```

Checks:

- Non-empty input.
- No leading or trailing JSON whitespace (`json_isspace`).
- No embedded NUL (`\0`) bytes.

This is used as a gate for `parse_double`, `parse_int32`, `parse_int64`, and JSON number string validation.

### Integer and float parsing

```rust
pub fn parse_double(x: &String, out: *mut f64) -> bool;
pub fn parse_int32(str_: &String, out: *mut i32) -> bool;
pub fn parse_int64(str_: &String, out: *mut i64) -> bool;
```

All three functions:

- Call `parse_prechecks` first.
- Reject hexadecimal floats (`0x...`) for `parse_double`.
- Use Rust’s `parse` and perform range checks for `parse_int32`.
- Write into the raw pointer `out` when non-null (mirroring upstream C API style).

Using raw pointers allows matching the exact upstream function signatures and calling conventions from FFI or other low-level glue.

---

## Escape table generation utilities

Bitcoin Core historically generated a JSON escape lookup table at build time. In Rust, the table ships as a `const` array, but the generator is kept for compatibility and tests.

### `generate_escapes_table`

```rust
pub fn generate_escapes_table() -> String;
```

Builds **Rust source code** for a `pub const escapes: [Option<&'static str>; 256]` table, mapping each byte (0–255) to either `None` or a JSON escape string.

### `output_escape` and `univalue_gen_main`

```rust
pub fn output_escape();

pub fn univalue_gen_main(_argc: i32, _argv: &[*mut u8]) -> i32;
```

- `output_escape` writes the generated table source to `stdout`.
- `univalue_gen_main` is a legacy-style `main` function that calls `init_json_escape` and `output_escape` and returns `0`, preserving the original command-line tool behavior.

### `init_json_escape`

```rust
pub fn init_json_escape();
```

In Bitcoin Core this performed expensive one-time table initialization. Here it is an idempotent no-op guarded with `Once`, retained only for API compatibility.

---

## Lookup helpers: `find_value` and `find_value_mut`

```rust
pub fn find_value<'a>(obj: &'a UniValue, name: &'a str) -> &'a UniValue;

pub fn find_value_mut<'a>(obj: &'a mut UniValue, name: &'a str) -> &'a mut UniValue;
```

Semantics:

- If `obj` is not an object, or the key is absent, they return a reference to `NULL_UNI_VALUE`.
- `find_value_mut` uses `unsafe` to cast the immutable global `NULL_UNI_VALUE` to `&mut` when the key is absent, faithfully mirroring the C++ API that returns a mutable reference to a global **null** value.

> This is intentionally unsafe; callers that mutate the returned null value must do so knowingly.

---

## Error type: `StdException`

```rust
#[derive(Debug, Clone)]
pub struct StdException(pub String);

impl std::fmt::Display for StdException { .. }
impl std::error::Error for StdException { }

pub fn runtime_error<T: Into<String>>(msg: T) -> StdException;
```

`StdException` is a lightweight stand-in for C++ `std::runtime_error`, allowing the crate to mirror the upstream exception-based API without committing to a broader error type hierarchy.

---

## Logging and tracing

Many functions are annotated with `#[instrument]` and internally call `trace!` macros. These are typically supplied by the `tracing` ecosystem.

- `#[instrument(level = "trace")]` decorates functions with structured spans, propagating arguments.
- `trace!(...)` attaches key–value diagnostics to events.

In performance-sensitive deployments you can:

- Configure global `tracing` filters to disable `trace`-level events.
- Compile with features that disable logging in production builds, if provided by the surrounding crate workspace.

---

## Safety and invariants

- Parsing via `read` and the numeric helpers requires valid memory passed as raw pointers.
- Mutating the sentinel `NULL_UNI_VALUE` (reachable through `find_value_mut`) is undefined from an application-level semantics perspective, though allowed by the crate to preserve compatibility with the C++ interface.
- Many getters panic on type mismatches, reflecting upstream behavior. Use type predicates or `check_object` for defensive programming.

When integrating with consensus- or security-critical Bitcoin software, keep these invariants in mind and model-test any new code paths.

---

## Use cases

Typical use cases for `bitcoin-univalue` include:

- **Re-implementing Bitcoin Core RPCs in Rust**, while keeping response formatting byte-identical.
- **Regression testing**: comparing Rust-generated JSON against reference Bitcoin Core outputs.
- **Protocol conformance** tooling where JSON serialization must be canonical and stable across versions and platforms.

For general JSON work, `serde` + `serde_json` will usually be more ergonomic and feature-rich; `bitcoin-univalue` is specialized infrastructure.

---

## Repository, license, and edition

- Crate: `bitcoin-univalue`
- Version: `0.1.20`
- License: MIT
- Edition: Rust 2021
- Repository: <https://github.com/klebs6/bitcoin-rs>

The crate is part of the wider `bitcoin-rs` effort to provide a faithful Rust translation of upstream Bitcoin Core.
