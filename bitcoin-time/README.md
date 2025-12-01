# bitcoin-time

High‑fidelity time utilities and ISO‑8601 formatting/parsing for Bitcoin‑style systems, ported from Bitcoin Core's C++ `std::chrono` and time logic.

---

## Overview

`bitcoin-time` provides a small, focused toolkit for dealing with time in Bitcoin‑like nodes and services:

- Strongly‑typed duration aliases mirroring C++ `std::chrono` typedefs.
- Conversion utilities between Rust `Duration` and C `timeval` for `select()`/FFI.
- Helpers to extract seconds/milliseconds/microseconds as integers or `f64`.
- ISO‑8601 date/time formatting and parsing around Unix timestamps.
- A **mockable** notion of "current time" for deterministic testing.
- A generic **median filter** for time offset consensus, modeled on Bitcoin Core's time adjustment logic.

The design goal is *deterministic, debuggable, and testable time behavior* suitable for consensus‑sensitive or protocol‑sensitive code.

---

## Features at a Glance

- `MillisToTimeval` trait to convert millisecond values into `libc::timeval` (for e.g. `select`, `poll`, or other C APIs).
- Duration typedefs:
  - `Milliseconds`, `Microseconds`, `Seconds`, `SecondsDouble` (all `StdDuration`)
  - `Minutes` (wrapper newtype over `Duration` preserving upstream semantics).
- Global mock time and time offset:
  - `MOCK_TIME: AtomicI64` – mock time in **seconds since Unix epoch**;
  - `TIME_OFFSET: Mutex<i64>` – P2P time offset in seconds.
- Helpers:
  - `count_seconds` / `count_milliseconds` / `count_microseconds` / `count_seconds_double`.
  - `get_time_since_epoch<T>()` – mock‑aware, generic over `From<StdDuration>`.
  - `get_system_time_since_epoch<T>()` – never mockable.
  - `get_time_millis_since_epoch`, `get_time_micros_since_epoch`, `get_time_seconds_since_epoch`.
  - `uninterruptible_sleep` – panic on negative durations, otherwise sleep fully.
- ISO‑8601 conversion:
  - `format_iso8601date_time` – `YYYY‑MM‑DDTHH:MM:SSZ`.
  - `format_iso8601date` – `YYYY‑MM‑DD`.
  - `parse_iso8601date_time` – string → epoch seconds, fall back to `0`.
- `MedianFilter<T>` – generic streaming median over the last `N` values.

All functions are instrumented with structured logging (via tracing macros in upstream code) and are intended to be safe, simple building blocks for higher‑level protocols.

---

## Installation

```toml
[dependencies]
bitcoin-time = "0.1.19"
```

This crate targets Rust **2021 edition**.

---

## Duration Typedefs

The crate mirrors Bitcoin Core's `std::chrono` typedefs so that ported logic can be left structurally intact:

```rust
use bitcoin_time::{Milliseconds, Microseconds, Seconds, SecondsDouble, Minutes};

fn demo(d: Seconds) {
    let s: i64 = bitcoin_time::count_seconds(d);
    let ms: i64 = bitcoin_time::count_milliseconds(d as Milliseconds);
    let us: i64 = bitcoin_time::count_microseconds(d as Microseconds);
    let s_f64: f64 = bitcoin_time::count_seconds_double(d as SecondsDouble);
    println!("s={s}, ms={ms}, us={us}, s_f64={s_f64}");
}
```

Internally these are `std::time::Duration` (or a thin wrapper for `Minutes`), but the explicit aliases encode intent and protect call‑sites from relying on particular underlying units.

---

## Milliseconds → `timeval`

`MillisToTimeval` abstracts conversion from millisecond counts into a `libc::timeval`, suitable for `select()` or other C APIs:

```rust
use bitcoin_time::MillisToTimeval;

fn poll_timeout(ms: i64) -> libc::timeval {
    ms.millis_to_timeval()
}

fn from_duration(d: std::time::Duration) -> libc::timeval {
    d.millis_to_timeval()
}
```

The implementation splits milliseconds into `tv_sec` and `tv_usec` with integer arithmetic, preserving full microsecond resolution up to the limits of `i64` and `libc::time_t`.

---

## ISO‑8601 Formatting and Parsing

For interoperable logs, RPC, and on‑wire protocols, `bitcoin-time` provides deterministic conversion between Unix timestamps and ISO‑8601 strings.

### Formatting

```rust
use bitcoin_time::{format_iso8601date_time, format_iso8601date};

let epoch: i64 = 1_609_459_200; // 2020‑12‑31T00:00:00Z
let ts = format_iso8601date_time(epoch);
let d = format_iso8601date(epoch);

assert_eq!(ts, "2020-12-31T00:00:00Z");
assert_eq!(d,  "2020-12-31");
```

Formatting is total: invalid timestamps (e.g. out of range for the underlying library) degrade to `""` rather than panicking.

### Parsing

```rust
use bitcoin_time::parse_iso8601date_time;

let epoch = parse_iso8601date_time("2020-12-31T00:00:00Z");
assert_eq!(epoch, 1_609_459_200);

// On parse failure we get 0.
assert_eq!(parse_iso8601date_time("nonsense"), 0);
```

Parsing is strict with respect to the `YYYY‑MM‑DDTHH:MM:SSZ` grammar and returns `0` on failure so protocol layers can treat invalid timestamps as sentinel values without panics.

---

## Mock Time and Time Offset

Consensus‑critical systems often differentiate between:

- **Physical time** – OS clock, never overridden.
- **Mock time** – controlled value for tests and simulations.
- **P2P time offset** – correction derived from peers' clocks.

`bitcoin-time` encodes exactly these three:

### Mock Time

```rust
use bitcoin_time::{set_mock_time, get_mock_time_since_epoch};
use std::time::Instant;

// Set mock time (seconds since epoch) relative to an `Instant`.
set_mock_time(Instant::now());

// Read it back as a Duration since epoch.
let mock = get_mock_time_since_epoch();
assert!(mock.as_secs() > 0);
```

`MOCK_TIME` is **global** and expressed as seconds since Unix epoch. A value of `0` denotes "no mock time" and triggers fallback to real system time.

### Time Since Epoch

```rust
use bitcoin_time::{
    get_time_since_epoch,
    get_system_time_since_epoch,
    get_time_millis_since_epoch,
    get_time_micros_since_epoch,
    get_time_seconds_since_epoch,
};
use std::time::Duration as StdDuration;

// Mock‑aware time since epoch.
let d: StdDuration = get_time_since_epoch();

// Real (non‑mockable) time.
let real_d: StdDuration = get_system_time_since_epoch();

let ms: i64 = get_time_millis_since_epoch();
let us: i64 = get_time_micros_since_epoch();
let s:  i64 = get_time_seconds_since_epoch();
```

`get_time_since_epoch<T>()` and `get_system_time_since_epoch<T>()` are generic over `T: From<StdDuration>`, letting you choose your own duration wrapper or fixed‑point representation.

### P2P Time Offset and Adjusted Time

```rust
use bitcoin_time::{get_time_offset, get_adjusted_time, get_adjusted_datetime};

let offset_s: i64 = get_time_offset();
let adjusted_epoch: i64 = get_adjusted_time();
let adjusted_dt = get_adjusted_datetime();
```

The **time offset** is usually derived from the median of peer clocks and is meant to be small (on the order of seconds). `get_adjusted_time()` adds this offset to the current physical time to produce P2P time.

---

## MedianFilter

`MedianFilter<T>` implements a sliding window median over the last `N` observations. This is used in Bitcoin Core to robustly estimate clock offsets under adversarial or noisy peer behavior.

Mathematically, for a multiset \( S = \{x_1, \dots, x_n\} \) sorted in nondecreasing order, the median is:

- \( x_{(n+1)/2} \) if \( n \) is odd;
- \( \frac{1}{2}(x_{n/2} + x_{n/2+1}) \) if \( n \) is even.

`MedianFilter` realizes this definition on a bounded window by retaining at most `n_size` values, copying and sorting them for each update.

### Type Bounds

```rust
impl<T> MedianFilter<T>
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Div<Output = T>
        + From<u8>
```

The constraints enable ordering and arithmetic on `T`, and allow division by `2` for the even‑length case (`T::from(2u8)`). Typical choices are `i32`, `i64`, `f32`, `f64`.

### Usage Example

```rust
use bitcoin_time::MedianFilter;

let mut f = MedianFilter::new(5, 0_i64);

for v in [1, 100, 2, 3, 1000] {
    f.input(v);
}

let m = f.median();
assert_eq!(m, 3); // median of [1, 2, 3, 100, 1000]
assert_eq!(f.size(), 5);
```

The implementation characteristics:

- Bounded memory: stores at most `n_size` items.
- On insertion, if full, drops the oldest element (FIFO).
- Complexity is `O(n log n)` per insertion due to sorting; this is entirely adequate for small `n` (typical P2P consensus windows are <= a few hundred elements).

---

## Chrono Sanity Check

`chrono_sanity_check()` validates that Rust's `UNIX_EPOCH` constant aligns with the Unix epoch (time `0`) by verifying that `UNIX_EPOCH.duration_since(UNIX_EPOCH)` equals `Duration::ZERO`.

```rust
let ok = bitcoin_time::chrono_sanity_check();
assert!(ok);
```

This function is intended as a defensive assertion for environments with potentially misconfigured or exotic time bases.

---

## Uninterruptible Sleep

`uninterruptible_sleep` enforces a non‑negative duration and sleeps once for the full period:

```rust
use std::time::Duration;
use bitcoin_time::uninterruptible_sleep;

uninterruptible_sleep(Duration::from_millis(250));
```

Internally this:

- Rejects negative durations (`try_into()` on the underlying type will panic).
- Delegates to `std::thread::sleep`.

This is useful when the semantics require that a sleep either completes fully or aborts the program rather than returning early.

---

## Design Notes

- **Epoch alignment** – All epoch computations assume the Unix epoch and are checked via `chrono_sanity_check()`.
- **Mock vs physical** – Functions are explicit about whether they obey `MOCK_TIME` (`get_time_since_epoch`) or bypass it (`get_system_time_since_epoch`, `get_time_millis_since_epoch`, etc.).
- **Integer conversions** – Conversions from `Duration` to `i64` intentionally truncate toward zero (`as_secs`, `as_millis`, `as_micros`), as required by consensus code ports.
- **Error handling** – Parsing and formatting favor *total* behavior without panics, returning sentinel values (`0`, empty strings) on failure, while logging through the tracing infrastructure in the original codebase.

---

## Repository and License

This crate is part of the `bitcoin-rs` project:

- Repository: <https://github.com/klebs6/bitcoin-rs>
- License: MIT

Contributions should preserve deterministic behavior and compatibility with the upstream Bitcoin Core time semantics.
