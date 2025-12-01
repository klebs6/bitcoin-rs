# bitcoin-get-json-token

A minimal, allocation-aware JSON lexer extracted from the Bitcoin Core codebase and re-engineered in safe(ish) Rust. It focuses on lexing – not building an AST – and is optimised for incremental, streaming inspection of JSON text in security‑sensitive and latency‑sensitive contexts (e.g. P2P protocol handling, RPC servers, and log ingestion).

---

## Design intent

This crate exposes a low‑level tokeniser for JSON, suitable when:

- You already have allocation and ownership policies and only need a lexer.
- You want deterministic, spec‑compliant classification of JSON tokens without committing to a full DOM/AST.
- You are processing untrusted data (e.g. network payloads) and require strong guarantees around numeric syntax, escape semantics, and whitespace handling.
- You are porting / interoperating with Bitcoin Core infrastructure and want a functionally equivalent lexer in Rust.

No `serde`, no dynamic dispatch, no hidden allocations beyond explicit `String` outputs for token literals.

---

## Core concepts

The lexer operates on raw byte pointers `*const u8` delimited by `[raw, end)`. It performs a single forward pass and emits one token at a time. The caller is responsible for advancing the cursor using the returned `consumed` byte count.

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JTokenType {
    JTOK_ERR,
    JTOK_NONE,
    JTOK_OBJ_OPEN,
    JTOK_OBJ_CLOSE,
    JTOK_ARR_OPEN,
    JTOK_ARR_CLOSE,
    JTOK_COLON,
    JTOK_COMMA,
    JTOK_KW_NULL,
    JTOK_KW_TRUE,
    JTOK_KW_FALSE,
    JTOK_NUMBER,
    JTOK_STRING,
}
```

The lexer is **intentionally primitive**:

- No implicit buffering beyond caller‑owned `String`s.
- No recursion, no tree building.
- Operates on byte slices and raw pointers for maximal control.

This makes it especially suitable for:

- Network protocol parsers.
- Constrained runtimes where you want to avoid heavyweight JSON frameworks.
- Interfacing with foreign memory (FFI) where you primarily need syntax validation and token classification.

---

## High‑level API overview

### `get_json_token`

```rust
pub fn get_json_token(
    token_val: &mut String,
    consumed:  &mut u32,
    raw:       *const u8,
    end:       *const u8,
) -> JTokenType
```

Lexes the **next JSON token** starting at `raw` and not past `end`.

Behaviour:

1. Skips leading JSON whitespace (`SP`, `HT`, `LF`, `CR`) and trailing `NUL` padding.
2. Attempts to lex, in order:
   - Structural tokens: `{ } [ ] : ,`
   - Keywords: `null`, `true`, `false`
   - Numbers (RFC 7159 compliant; strict about leading zeros, exponents, etc.)
   - Strings, including full handling of `\uXXXX` and surrogate pairs
3. On success:
   - Returns the corresponding `JTokenType`.
   - Writes the token byte length into `*consumed` (excluding leading whitespace).
   - For `JTOK_NUMBER` and `JTOK_STRING`, writes the decoded literal into `token_val`.
4. On reaching the end of input, emits `JTOK_NONE`.
5. On syntactic violation, emits `JTOK_ERR`.

#### Example: single‑pass tokenisation

```rust
use bitcoin_get_json_token::{
    get_json_token, JTokenType,
};

fn lex_all(input: &str) -> Vec<(JTokenType, String)> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut offset = 0usize;

    while offset < bytes.len() {
        let mut val = String::new();
        let mut consumed = 0u32;
        let raw = unsafe { bytes.as_ptr().add(offset) };
        let end = unsafe { bytes.as_ptr().add(bytes.len()) };

        let kind = get_json_token(&mut val, &mut consumed, raw, end);

        if kind == JTokenType::JTOK_NONE {
            break; // EOF
        }
        if kind == JTokenType::JTOK_ERR {
            panic!("lexing error at byte {}", offset);
        }

        tokens.push((kind, val));
        offset += consumed as usize;
    }

    tokens
}

fn main() {
    let json = r#"{"x": 1.23e4, "y": "hello"}"#;
    let tokens = lex_all(json);
    for (kind, val) in tokens {
        println!("{:?} => {:?}", kind, val);
    }
}
```

---

## Numeric validation

### `valid_num_str`

```rust
pub fn valid_num_str(s: &str) -> bool
```

Returns **true** iff `s` is _exactly_ one syntactically valid JSON number per RFC 7159.

Internally, this simply calls `get_json_token` on the string and checks:

- that the token type is `JTOK_NUMBER`, and
- that the token consumes the entire string.

This function is useful when you:

- Accept numeric fields as strings (e.g. to avoid precision loss) but still want strict JSON numeric syntax.
- Need to guard a numeric parser that assumes well‑formed input.

#### Example

```rust
use bitcoin_get_json_token::valid_num_str;

assert!(valid_num_str("0"));
assert!(valid_num_str("-0.1"));
assert!(valid_num_str("1e10"));
assert!(!valid_num_str("01"));      // leading zero
assert!(!valid_num_str("1."));      // missing fractional digits
assert!(!valid_num_str("1e"));      // incomplete exponent
assert!(!valid_num_str("-"));       // incomplete sign
```

---

## String and escape handling

### `lex_string`

```rust
pub fn lex_string(
    token_val: &mut String,
    p: *const u8,
    end: *const u8,
) -> Option<(JTokenType, *const u8, u32)>
```

Lexes a JSON string beginning with `"` at `p` and decodes escape sequences into `token_val`:

- Simple escapes: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`.
- Unicode escapes: `\uXXXX` with correct handling of UTF‑16 surrogate pairs.
- Rejects control characters `< 0x20` and unterminated strings.

The returned tuple contains:

- `JTokenType::JTOK_STRING` on success.
- Pointer after the closing quote.
- Bytes consumed for the token.

### `parse_string_escape`

```rust
pub fn parse_string_escape(
    backslash: *const u8,
    end: *const u8,
) -> Option<(String, *const u8)>
```

Low‑level helper that parses a **single** escape sequence starting at a backslash:

- `backslash` must point at `b'\'`.
- On success, returns the decoded segment and the pointer after the escape.
- On error (truncated, bad hex, invalid surrogate structure), returns `None`.

This is useful if you are building a custom string parser but want to reuse the robust escape decoding logic, including surrogate‑pair composition:

```rust
use bitcoin_get_json_token::parse_string_escape;

fn decode_escape_sequence(input: &str, idx_of_backslash: usize) -> Option<(String, usize)> {
    let bytes = input.as_bytes();
    let backslash = unsafe { bytes.as_ptr().add(idx_of_backslash) };
    let end = unsafe { bytes.as_ptr().add(bytes.len()) };
    let (decoded, ptr_after) = parse_string_escape(backslash, end)?;
    let new_offset = (ptr_after as usize) - (bytes.as_ptr() as usize);
    Some((decoded, new_offset))
}
```

---

## Structural and keyword lexers

These are composable primitives used by `get_json_token`. You can call them directly if you are assembling a custom state machine.

### `lex_structural`

```rust
pub fn lex_structural(p: *const u8, end: *const u8) -> Option<(JTokenType, *const u8, u32)>;
```

Recognises one of:

- `{` → `JTOK_OBJ_OPEN`
- `}` → `JTOK_OBJ_CLOSE`
- `[` → `JTOK_ARR_OPEN`
- `]` → `JTOK_ARR_CLOSE`
- `:` → `JTOK_COLON`
- `,` → `JTOK_COMMA`

Returns `None` if `p` is at `end` or if the byte is not a recognised structural character.

### `lex_keyword`

```rust
pub fn lex_keyword(p: *const u8, end: *const u8) -> Option<(JTokenType, *const u8, u32)>;
```

Recognises:

- `null`  → `JTOK_KW_NULL`
- `true`  → `JTOK_KW_TRUE`
- `false` → `JTOK_KW_FALSE`

This function assumes ASCII input and does exact byte comparison; there is no case folding, as required by JSON.

---

## Character classification and helpers

### `json_isspace`

```rust
pub fn json_isspace(ch: i32) -> bool
```

Returns `true` if `ch` is one of the four JSON whitespace characters:

- space (U+0020)
- horizontal tab (U+0009)
- line feed (U+000A)
- carriage return (U+000D)

### `json_isdigit`

```rust
pub fn json_isdigit(ch: i32) -> bool
```

Returns `true` if `ch` is an ASCII digit `0..=9`.

### `skip_ws_nul`

```rust
pub fn skip_ws_nul(mut p: *const u8, end: *const u8) -> *const u8
```

Skips JSON whitespace per `json_isspace` **and** any `NUL` padding bytes (`0x00`), returning the first non‑whitespace, non‑NUL pointer.

This is particularly useful for handling buffers padded with zeroes or assembled from fixed‑length frames.

### `json_token_is_value`

```rust
pub fn json_token_is_value(jtt: JTokenType) -> bool
```

Returns `true` if `jtt` is one of the **terminal** JSON value tokens:

- `JTOK_KW_NULL`
- `JTOK_KW_TRUE`
- `JTOK_KW_FALSE`
- `JTOK_NUMBER`
- `JTOK_STRING`

This is a tiny convenience for parsers that distinguish container structure from scalar values.

### `bytes_consumed`

```rust
pub fn bytes_consumed(start: *const u8, after: *const u8) -> u32
```

Computes the difference in bytes between two pointers in the same buffer. All lexers use this for book‑keeping.

### `single_byte_token`

```rust
pub fn single_byte_token(
    p:        &mut *const u8,
    start:    *const u8,
    consumed: &mut u32,
    kind:     JTokenType,
) -> JTokenType
```

Utility for emitting a token corresponding to a single byte and advancing the pointer.

---

## Hex parsing primitive

### `hatoui`

```rust
pub fn hatoui(first: *const u8, last: *const u8, out: &mut u32) -> *const u8
```

Converts a hexadecimal slice `[first, last)` into a `u32`.

- Parses contiguous hex digits `0‑9`, `a‑f`, `A‑F`.
- Stops at the first non‑hex byte or at `last`.
- Writes the parsed value into `*out`.
- Returns a pointer to the first **unconsumed** byte.

This is used internally for `\uXXXX` decoding and is independently useful for other protocol‑level hex parsing tasks.

---

## Safety and `unsafe` considerations

The API exposes several functions that accept raw pointers. The caller must ensure:

- `raw` / `first` / `backslash` / `p` / `start` and `end` / `last` refer to the same valid allocated buffer.
- `raw <= end`, `first <= last`, and all intermediate pointer arithmetic stays within bounds or at most one‑past‑the‑end, as per Rust's pointer rules.
- The underlying data is immutable while parsing.

The crate itself attempts to:

- Restrict `unsafe` usage to clear, small regions where pointer arithmetic is required.
- Avoid undefined behaviour by adhering to Rust's aliasing and lifetime constraints.

If you prefer a fully safe interface, you can wrap these primitives with slice‑based abstractions (`&[u8]`) and manage indices rather than raw pointers.

---

## Tracing and observability

Many functions are annotated with `#[instrument]` and emit `trace!` events. This assumes you have `tracing` configured in your application. The emitted fields include:

- Token kind and byte counts.
- For numbers and strings, the parsed literal.
- Decision points (e.g. why a candidate number or string was rejected).

This makes it straightforward to inspect malformed input from untrusted peers and to profile tokenisation behaviour.

---

## When to use this crate vs general JSON libraries

Use this crate when you:

- Already have a bespoke parser or state machine and just need a **lexical** front‑end.
- Need full transparency into numeric and string syntax acceptance.
- Care about tight control over allocations and do not want an implicit object model.

Prefer `serde_json` or similar when you:

- Want high‑level `Deserialize` / `Serialize` drives with minimal manual code.
- Do not need raw pointer‑level, incremental control.

Both approaches can coexist: you can pre‑screen or partially tokenise with this crate, and then hand selected slices to a higher‑level deserialiser.

---

## License and metadata

- **Crate name:** `bitcoin-get-json-token`
- **Version:** 0.1.1
- **Edition:** 2024
- **License:** MIT
- **Author:** `klebs6 <tpk3.mx@gmail.com>`

Contributions and issue reports are encouraged, especially around edge‑case compatibility with Bitcoin Core's original lexer semantics.
