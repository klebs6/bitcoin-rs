# bitcoin-string

Locale‑independent, consensus‑compatible string, integer, and encoding utilities extracted from the Bitcoin Core codebase and ported to Rust.

This crate focuses on *reproducing the exact control‑flow and edge‑case semantics* of the original C++ helpers that operate on textual data, numeric literals, base encodings, and fixed‑point monetary values. It is designed for projects that:

- interact with Bitcoin data structures or wire formats,
- must remain byte‑for‑byte compatible with existing Bitcoin Core behaviour,
- require constant‑time comparisons or strict parsers for untrusted input.

---

## Features at a Glance

- **ASCII‑only, locale‑independent case conversion**
  - [`ToUpper`] and [`ToLower`] traits implemented for `String` and `u8`.
  - Deterministic, 7‑bit ASCII semantics independent of the process locale.

- **String sanitisation for untrusted text**
  - [`sanitize_string`] with configurable [`SafeChars`] policies mirroring Bitcoin Core:
    - `SAFE_CHARS_DEFAULT` — permissive text & URLs.
    - `SAFE_CHARS_UA_COMMENT` — user‑agent comments.
    - `SAFE_CHARS_FILENAME` — filenames.
    - `SAFE_CHARS_URI` — URI components.

- **Bilingual message abstraction**
  - [`BilingualStr`] carries both `original` (English) and `translated` strings.
  - Centralised translation dispatch via [`set_translation_fn`] and [`bilingual_tr`].
  - `bilingual_format!` macro for dual‑language formatted strings.

- **Robust integer parsing utilities**
  - [`parse_integral`] and type‑specific wrappers (`parse_int32`, `parse_uint64`, …) with explicit success/failure and C‑style out‑pointers.
  - [`to_integral`] — strict `Option<T>` parser with no whitespace, no leading `+`, and full‑string consumption.
  - [`locale_independent_atoi`] — exact behavioural clone of C `atoi`/`atoi64` in the C locale, including handling of "+-".

- **Fixed‑point and monetary parsing/formatting**
  - [`parse_fixed_point`] — parse JSON‑syntax numbers into signed 64‑bit fixed‑point values with bounded exponent and mantissa.
  - [`format_money`] — format `Amount` (Bitcoin amount in satoshis) as decimal coins, trimming redundant trailing zeros while preserving precision constraints.
  - [`parse_money`] — parse coin‑denominated string amounts (e.g. `"0.0034"`) into internal `Amount`, with strict range checks.

- **Base32 / Base64 encoding and decoding**
  - RFC 4648 compliant encoders: [`encode_base32_bytes`], [`encode_base32`], [`encode_base64_bytes`], [`encode_base64`].
  - Multiple decoders mirroring historical Bitcoin and Tor/I2P paths:
    - [`decode_base32_bytes`] — C‑string, padded RFC 4648.
    - [`decode_base32_bytes_strict`] — Rust `&str`, padded RFC 4648.
    - [`decode_base32_bytes_nopad_lower`] — lower‑case alphabet, *no* padding, strict tail‑bit checks.
    - [`decode_base32`] — decode to UTF‑8 `String`.
    - [`decode_base64_bytes`] — C‑string, padded RFC 4648.
    - [`decode_base64`] — decode to UTF‑8 `String`.
  - Shared internal [`convert_bits`] primitive implementing the `FROM → TO` bit‑group mapping logic (as per BIP‑173 and RFC 4648), reusable for custom alphabets.

- **Hex utilities**
  - [`is_hex`] — validate even‑length, fully‑hex byte strings.
  - [`is_hex_number`] — validate optional `0x`/`0X` prefixed hex literals.
  - [`parse_hex`] — parse hex dumps with embedded ASCII whitespace.
  - [`hex_str`] — lower‑case hex encoding of raw bytes.

- **Timing‑resistant comparison**
  - [`timing_resistant_equal`] — constant‑time (in the length of `a`) comparison over any `AsRef<[u8]>` type; suitable for secret material comparisons.

- **General string helpers**
  - [`trim_string`] — configurable trim pattern.
  - [`remove_prefix`] — prefix stripping.
  - [`join`] and [`make_unordered_list`] — list composition.
  - [`valid_as_cstring`] — check for absence of embedded NUL (`\0`).
  - [`to_string`] — locale‑independent formatting alias for `format!`.
  - [`has_prefix`] — const‑generic, byte‑slice prefix test.
  - [`capitalize`] — ASCII‑only first‑character capitalization.

All functions are instrumented with [`tracing`] so that call‑site, control‑flow, and error conditions can be observed precisely during testing and production debugging.

---

## Installation

```toml
[dependencies]
bitcoin-string = "0.1.19"
```

This crate targets Rust **edition 2021** and is licensed under **MIT**.

---

## Core Concepts and Guarantees

### Locale‑independent parsing and formatting

Many standard library functions (both in C and some higher‑level languages) exhibit locale‑dependent behaviour for numbers, whitespace, and casing rules. Bitcoin consensus and network code cannot depend on locale, because that would fragment nodes by environment.

`bitcoin-string` intentionally restricts itself to **ASCII and locale‑independent semantics**:

- Numeric parsers (`parse_integral`, `parse_fixed_point`, `parse_money`, `to_integral`, `locale_independent_atoi`) treat input as if processed in the C locale.
- Case transforms operate strictly on ASCII ranges `A–Z` and `a–z` and leave all other bytes untouched.

This constraint is crucial when strings represent consensus‑critical data such as numeric fields in transactions, RPC inputs, or on‑disk configuration.

### Fixed‑point representation and overflow discipline

`parse_fixed_point` interprets a JSON number (roughly the grammar from <https://json.org/number.gif>) and maps it into a signed 64‑bit integer with a caller‑supplied decimal *scale* (`decimals`).

Conceptually, it computes

\[
\text{mantissa} = \left\lfloor x \times 10^{d} \right\rfloor
\]

subject to the invariant

\[
|\text{mantissa}| < 10^{18}
\]

where `x` is the input real and `d` is `decimals`. Internally it:

1. Parses sign, integral, fractional, and exponential parts.
2. Accumulates an integer mantissa by scaling the previous mantissa by 10 and adding the new digit, while counting trailing zeros separately to delay scaling (see [`process_mantissa_digit`]).
3. Applies exponent and decimal point offsets, producing an effective power of 10.
4. Rejects any underflow or overflow: invalid ranges, exponent outside `[0, 17]`, or mantissa exceeding `10^18 − 1` in magnitude.

This careful control‑flow is engineered to match the original Bitcoin Core implementation, so that any system using these helpers in Rust remains consistent with C++ peers.

### Timing‑attack resistance

[`timing_resistant_equal`] compares two secrets `a` and `b` such that the running time depends solely on the length of `a`, **not** on the first differing byte position. It does this by:

- XOR‑ing the lengths of `a` and `b` into an accumulator;
- XOR‑ing every byte of `a` with the corresponding byte of `b` (cycling through `b` if shorter) and OR‑ing the result into the accumulator;
- returning `acc == 0`.

This prevents simple timing attacks that exploit early‑exit comparisons.

> **Important**: While this function is designed to be timing‑resistant at the library level, overall application security depends on compiler, CPU, and surrounding control‑flow. Use in conjunction with other standard hardening techniques.

---

## Usage Examples

### Case conversion and sanitisation

```rust
use bitcoin_string::{ToUpper, ToLower, sanitize_string, SafeChars};

let s = "Hello, World!".to_string();
assert_eq!(s.to_upper(), "HELLO, WORLD!");
assert_eq!(s.to_lower(), "hello, world!");

// Sanitize for generic text / URLs
let raw = "Visit https://example.com/?q=<script>";
let clean = sanitize_string(raw, Some(SafeChars::SAFE_CHARS_URI as i32));
// `clean` contains only characters from the SAFE_CHARS_URI set.
```

### Bilingual strings and translations

```rust
use bitcoin_string::{BilingualStr, untranslated, bilingual_tr, set_translation_fn};

// Mark a string as untranslated: original == translated
let msg: BilingualStr = untranslated("Wallet loaded");
assert_eq!(msg.original(), "Wallet loaded");
assert_eq!(msg.translated(), "Wallet loaded");

// Install a trivial translation function (e.g. from your i18n layer)
set_translation_fn(Some(|s: &str| {
    if s == "Wallet loaded" { "Portefeuille chargé".to_owned() } else { s.to_owned() }
}));

let localized = bilingual_tr("Wallet loaded");
assert_eq!(localized.original(), "Wallet loaded");
assert_eq!(localized.translated(), "Portefeuille chargé");
```

Using the `bilingual_format!` macro (from the `tinyformat` module):

```rust
use bitcoin_string::{BilingualStr, bilingual_tr};
use bitcoin_string::bilingual_format; // macro is exported at crate root

let tmpl = bilingual_tr("Processed {} transactions");
let count = 42;

let formatted = bilingual_format!(&tmpl, count);
// formatted.original()  == "Processed 42 transactions"
// formatted.translated() == translation of that template with `42` substituted
```

### Integer parsing

```rust
use bitcoin_string::{parse_int32, parse_uint64, to_integral, locale_independent_atoi};

// C‑style parser with out‑pointer
let mut v: i32 = 0;
let ok = parse_int32(&"123".to_string(), &mut v as *mut i32);
assert!(ok);
assert_eq!(v, 123);

// Strict parser that forbids whitespace and leading '+'
assert_eq!(to_integral::<i64>("42"), Some(42));
assert_eq!(to_integral::<i64>(" 42"), None);
assert_eq!(to_integral::<i64>("+42"), None);

// `atoi`‑like behaviour
let x: i32 = locale_independent_atoi("  +123");
let y: i32 = locale_independent_atoi("+-5");
assert_eq!(x, 123);
assert_eq!(y, 0); // matches C `atoi("+-5")`
```

### Fixed‑point parsing

```rust
use bitcoin_string::parse_fixed_point;

let mut amount: i64 = 0;
// Interpret as 2 decimal places: "1.23" → 123
let ok = parse_fixed_point("1.23", 2, &mut amount as *mut i64);
assert!(ok);
assert_eq!(amount, 123);

// Out‑of‑range exponents or mantissas are rejected
assert!(!parse_fixed_point("1e20", 0, std::ptr::null_mut()));
```

### Base32 / Base64 encoding and decoding

```rust
use bitcoin_string::{encode_base32, decode_base32_bytes_strict};
use bitcoin_string::{encode_base64, decode_base64};

let data = b"hello";
let b32 = encode_base32(data, Some(true));
let mut invalid = false;
let roundtrip = decode_base32_bytes_strict(&b32, Some(&mut invalid as *mut bool));
assert!(!invalid);
assert_eq!(roundtrip, data);

let s = "hello".to_string();
let encoded = encode_base64(&s);
let decoded = decode_base64(&encoded, None);
assert_eq!(decoded, s);
```

### Hex utilities

```rust
use bitcoin_string::{is_hex, is_hex_number, parse_hex, hex_str};

assert!(is_hex("00ff10"));
assert!(!is_hex("0xff"));
assert!(is_hex_number("0xff"));

let bytes = parse_hex("00 ff 10");
assert_eq!(hex_str(&bytes), "00ff10");
```

### Timing‑resistant equality

```rust
use bitcoin_string::timing_resistant_equal;

let secret_a = b"super_secret_key".to_vec();
let secret_b = b"super_secret_key".to_vec();
let secret_c = b"super_secret_keZ".to_vec();

assert!(timing_resistant_equal(&secret_a, &secret_b));
assert!(!timing_resistant_equal(&secret_a, &secret_c));
```

---

## Integration Notes

- The crate depends on [`tracing`] for structured logging. In a production application, install an appropriate subscriber (e.g. `tracing-subscriber`) to capture diagnostics.
- Monetary helpers rely on `Amount`, `COIN`, and `money_range` from the surrounding Bitcoin domain. When used outside the `bitcoin-rs` tree, ensure that these are wired consistently with Bitcoin Core semantics (e.g. `COIN = 100_000_000` for sats per BTC).
- Many parsers accept raw pointers (for C‑compatibility). In idiomatic Rust code, you can often pass `std::ptr::null_mut()` when you want to check validity but discard the result.

---

## Safety and Security Considerations

- **Pointer arguments**: functions like `parse_integral`, `parse_fixed_point`, `parse_int32`, etc., mirror the original C++ signature pattern and accept raw pointers. The implementations are careful, but the caller is responsible for providing valid, appropriately aligned pointers or `null_mut()`.
- **Constant‑time claims**: `timing_resistant_equal` is structured to avoid early returns and to depend only on the length of `a`, but micro‑architectural details can still leak information. For critical key‑material handling, consider layering it with additional defences.
- **Sanitisation**: [`sanitize_string`] enforces an *allowlist* of characters. Pick the correct [`SafeChars`] rule for your context. For example, URIs and filenames have different constraints.

---

## License

This crate is distributed under the **MIT** license.

The upstream repository is hosted at: <https://github.com/klebs6/bitcoin-rs>

---

## Provenance

This crate is part of the `bitcoin-rs` effort to provide a faithful, well‑structured Rust port of the Bitcoin Core codebase. The implementation choices are driven by the goal of **behavioural equivalence**, not by API minimalism. Consequently, some functions retain C‑style signatures and pointer‑based semantics to preserve compatibility with existing call sites.

If you require different ergonomics (e.g. pure `Result<T, E>` APIs), consider wrapping these primitives in your own higher‑level abstractions while keeping this crate as the compatibility layer.

[`tracing`]: https://docs.rs/tracing
[`SafeChars`]: #features-at-a-glance
[`BilingualStr`]: #bilingual-message-abstraction
[`sanitize_string`]: #case-conversion-and-sanitisation
[`set_translation_fn`]: #bilingual-strings-and-translations
[`bilingual_tr`]: #bilingual-strings-and-translations
[`parse_integral`]: #integer-parsing
[`parse_fixed_point`]: #fixed-point-parsing
[`parse_money`]: #fixed-point-and-monetary-parsingformatting
[`format_money`]: #fixed-point-and-monetary-parsingformatting
[`encode_base32_bytes`]: #base32--base64-encoding-and-decoding
[`encode_base32`]: #base32--base64-encoding-and-decoding
[`encode_base64_bytes`]: #base32--base64-encoding-and-decoding
[`encode_base64`]: #base32--base64-encoding-and-decoding
[`decode_base32_bytes`]: #base32--base64-encoding-and-decoding
[`decode_base32_bytes_strict`]: #base32--base64-encoding-and-decoding
[`decode_base32_bytes_nopad_lower`]: #base32--base64-encoding-and-decoding
[`decode_base32`]: #base32--base64-encoding-and-decoding
[`decode_base64_bytes`]: #base32--base64-encoding-and-decoding
[`decode_base64`]: #base32--base64-encoding-and-decoding
[`convert_bits`]: #base32--base64-encoding-and-decoding
[`timing_resistant_equal`]: #timing-resistant-equality
[`trim_string`]: #general-string-helpers
[`remove_prefix`]: #general-string-helpers
[`join`]: #general-string-helpers
[`make_unordered_list`]: #general-string-helpers
[`valid_as_cstring`]: #general-string-helpers
[`to_string`]: #general-string-helpers
[`has_prefix`]: #general-string-helpers
[`capitalize`]: #general-string-helpers
[`is_hex`]: #hex-utilities
[`is_hex_number`]: #hex-utilities
[`parse_hex`]: #hex-utilities
[`hex_str`]: #hex-utilities
[`process_mantissa_digit`]: #fixed-point-representation-and-overflow-discipline
[`to_integral`]: #integer-parsing
[`locale_independent_atoi`]: #integer-parsing
[`parse_int32`]: #integer-parsing
[`parse_uint64`]: #integer-parsing
[`ToLower`]: #features-at-a-glance
[`ToUpper`]: #features-at-a-glance
