# bitcoin-fees

High‑precision Bitcoin fee‑rate primitives, estimation modes, and fee filter rounding logic, extracted from Bitcoin Core and exposed as an idiomatic Rust API.

This crate focuses on the *numerical and policy layer* of Bitcoin transaction fees: units (sat/kvB vs sat/vB), conversion and formatting, deterministic rounding rules, and support structures for fee estimation across different confirmation horizons.

---

## Features at a Glance

- **`FeeRate`** – canonical representation of fee rates in *satoshis per kilobyte* (sat/kvB), plus conversion to sat/vB; fully ordered, additive, and serializable.
- **`FeeEstimateMode`** – strong typing for the standard Bitcoin Core estimation modes (`UNSET`, `ECONOMICAL`, `CONSERVATIVE`) plus unit selectors (`BTC_KVB`, `SAT_VB`).
- **`FeeEstimateHorizon`** – identifiers for short/medium/long half‑life horizons used in multi‑timescale fee estimation models.
- **`FeeReason`** – structured reasons explaining *why* a particular fee estimate was chosen (e.g. conservative, mempool minimum, fallback).
- **`FeeFilterRounder`** – geometric bucketing and randomized back‑rounding of fee filter values for mempool announcement privacy, matching Core’s fee filter logic.
- String helpers for error messages and diagnostics (`fee_modes`, `fee_mode_from_string`, etc.).

This crate is intended for implementers of Bitcoin nodes, wallets, fee estimators, and simulation frameworks that want deterministic, Core‑compatible behaviour at the fee policy level.

---

## Core Concepts

### Fee Units and Conversions

Bitcoin Core historically expresses most fee policy in **sat/kvB** (satoshis per 1000 bytes). Modern tooling often reasons in **sat/vB** (satoshis per virtual byte). The relationship is purely scalar:

\[
\text{sat/kvB} = 1000\,\text{sat/vB}
\]

`FeeRate` is intentionally stored as **sat/kvB**, mirroring Core’s internal representation. Conversions are performed explicitly in methods such as `to_string` (when asked for `SAT_VB`) and `new_with_fee_paid` (when using the special `num_bytes == COIN` convention to interpret the result as sat/vB).

The rate is applied linearly over transaction size, with integer truncation towards zero followed by a *minimal non‑zero correction* to avoid hiding sign information. This mirrors Core’s fee rounding semantics:

- Fee is computed as:
  - `fee = floor(rate_sat_per_k * size_bytes / 1000)` with integer arithmetic.
- If this would yield `0` but the rate is non‑zero, the result is adjusted to `±1 sat`.

This behaviour is critical in consensus‑adjacent tooling: you never want an apparently non‑zero rate to silently become zero when applied to a small transaction.

### Fee Estimation Horizons

`FeeEstimateHorizon` labels three separate tracking windows used by advanced fee estimators:

- `SHORT_HALFLIFE` – sensitive to recent mempool conditions; tracks rapid changes.
- `MED_HALFLIFE` – intermediate stability.
- `LONG_HALFLIFE` – slow‑moving, robust to noise; suitable for conservative estimates.

An estimator can maintain separate `TxConfirmStats` or analogous structures for each horizon and choose between them according to the requested `FeeEstimateMode`.

### Fee Reasons

`FeeReason` is a structured explanation of how/why a final fee estimate was selected:

- `NONE` – unspecified.
- `HALF_ESTIMATE` – estimate based on *half* the requested target with a 60% threshold.
- `FULL_ESTIMATE` – estimate directly for the requested target with an 85% threshold.
- `DOUBLE_ESTIMATE` – estimate using a *double* target horizon with a 95% threshold.
- `CONSERVATIVE` – conservative selection using longer horizons and double targets.
- `MEMPOOL_MIN` – clamped to the mempool’s minimum fee.
- `PAYTXFEE` – user‑configured fixed paytxfee was applied.
- `FALLBACK` – fallback fee in absence of estimator data.
- `REQUIRED` – minimum fee required by policy or consensus.

This is primarily for diagnostics, logging, and API transparency.

---

## Crate Layout

The key public items exposed by this crate include:

- Enums:
  - `FeeReason`
  - `FeeEstimateHorizon`
  - `FeeEstimateMode`
- Structs:
  - `FeeRate`
  - `FeeFilterRounder`
  - `FeeCalculation` (result container: estimate + reason + targets)
- Utility functions:
  - `string_for_fee_reason(FeeReason) -> String`
  - `string_for_fee_estimate_horizon(FeeEstimateHorizon) -> String`
  - `fee_mode_map() -> &'static Vec<(String, FeeEstimateMode)>`
  - `fee_modes(delimiter: &str) -> String`
  - `invalid_estimate_mode_error_message() -> String`
  - `fee_mode_from_string(mode_string: &str, &mut FeeEstimateMode) -> bool`

Several types (`Amount`, `FastRandomContext`, constants like `COIN`, `CURRENCY_UNIT`, `CURRENCY_ATOM`, `FEE_FILTER_SPACING`, `MAX_FILTER_FEERATE`, and the `FeeRateEstimationResult` inside `FeeCalculation`) are defined elsewhere in the `bitcoin-rs` workspace. This crate focuses on the *fee‑specific* logic.

---

## Installation

```toml
[dependencies]
bitcoin-fees = "0.1.21"
```

This crate targets **Rust 1.56+ with edition 2021**, and is licensed under **MIT**.

---

## `FeeRate`: Construction, Ordering, and Usage

### Creating Fee Rates

```rust
use bitcoin_fees::{FeeRate, FeeEstimateMode};

// 1. Explicit sat/kvB constructor
let rate = FeeRate::new(2_500u64); // 2_500 sat/kvB

// 2. From fee paid and transaction size in bytes
let fee_paid: i64 = 15_000;       // satoshis
let tx_size: u32 = 250;           // bytes
let rate2 = FeeRate::new_with_fee_paid(&fee_paid, tx_size);

// 3. Default (0 sat/kvB)
let zero = FeeRate::default();
assert_eq!(zero.get_fee_perk(), 0);
```

The `new_with_fee_paid` constructor follows Bitcoin Core’s dual‑unit convention: if `num_bytes == COIN (1e8)`, it interprets the resulting rate as sat/vB. This nuance matters if you are interoperating with Core’s fee estimation RPCs or low‑level policy logic.

### Computing Fees

```rust
use bitcoin_fees::FeeRate;

let rate = FeeRate::new(10_000u64); // 10_000 sat/kvB

// Fee for a 250‑byte tx
let fee_250 = rate.get_fee(250);   // integer arithmetic with truncation

// Fee per kvB (exactly 1000 bytes)
let per_k = rate.get_fee_perk();   // == 10_000 sat
```

The truncation‑toward‑zero plus ±1 sat correction ensures monotonicity with respect to the sign of the rate while preserving Core equivalence.

### Comparison and Aggregation

`FeeRate` implements `Ord`, `PartialOrd`, `Clone`, `Copy`, `Eq`, and `AddAssign<&FeeRate>`:

```rust
use bitcoin_fees::FeeRate;

let mut a = FeeRate::new(1_000u64);
let b = FeeRate::new(2_000u64);

assert!(a < b);

a += &b; // now a is 3_000 sat/kvB
```

You can use `FeeRate` directly in ordered containers (`BTreeMap`, `BTreeSet`) or as keys in priority queues for fee‑based mempool policies.

### Human‑Readable Formatting

```rust
use bitcoin_fees::{FeeRate, FeeEstimateMode};

let rate = FeeRate::new(150_000u64); // 150k sat/kvB

// Default formatting (BTC/kvB)
let s_default = rate.to_string(None);
// e.g., "0.00150000 BTC/kvB" depending on COIN and CURRENCY_UNIT

// Explicit sat/vB formatting
let s_sat_vb = rate.to_string(Some(&FeeEstimateMode::SAT_VB));
// e.g., "150.000 SAT/vB"

// Display impl delegates to to_string(None)
println!("{}", rate); // BTC/kvB representation
```

The output strings use the global `CURRENCY_UNIT` (e.g. `BTC`) and `CURRENCY_ATOM` (e.g. `sat`) constants from the broader `bitcoin-rs` workspace.

---

## Fee Estimation Modes and Parsing

`FeeEstimateMode` encodes both policy modes and unit choices:

- `UNSET` – default; the caller did not specify a mode.
- `ECONOMICAL` – prefers lower fees, willing to accept more confirmation risk.
- `CONSERVATIVE` – prefers higher fees, targeting high confirmation probability.
- `BTC_KVB` – explicit BTC/kvB display/interpretation.
- `SAT_VB` – explicit sat/vB display/interpretation.

The crate provides lookup helpers to parse user input into these modes:

```rust
use bitcoin_fees::{FeeEstimateMode, fee_mode_from_string, fee_modes, invalid_estimate_mode_error_message};

let mut mode = FeeEstimateMode::UNSET;

assert!(fee_mode_from_string("economical", &mut mode));
assert_eq!(mode, FeeEstimateMode::ECONOMICAL);

assert!(fee_mode_from_string("Conservative", &mut mode));
assert_eq!(mode, FeeEstimateMode::CONSERVATIVE);

// Listing available modes suitable for CLI docs or RPC errors
let modes_csv = fee_modes(", "); // "unset, economical, conservative"

// Standardized error message
let msg = invalid_estimate_mode_error_message();
// "Invalid estimate_mode parameter, must be one of: \"unset\", \"economical\", \"conservative\""
```

This design is useful for implementing RPC endpoints or CLIs that mirror Bitcoin Core’s `estimate_mode` parameter.

---

## Horizons and Reasons as Strings

For logging and user‑facing APIs you can convert horizons and reasons to strings:

```rust
use bitcoin_fees::{
    FeeEstimateHorizon, FeeReason,
    string_for_fee_estimate_horizon, string_for_fee_reason,
};

let h = FeeEstimateHorizon::LONG_HALFLIFE;
assert_eq!(string_for_fee_estimate_horizon(h), "long");

let r = FeeReason::CONSERVATIVE;
let desc = string_for_fee_reason(r);
// "Conservative Double Target longer horizon"
```

These are intended to be stable, human‑interpretable descriptors that can appear in logs, JSON responses, or monitoring dashboards.

---

## `FeeFilterRounder`: Privacy‑Preserving Fee Filters

Bitcoin nodes advertise a **fee filter** to peers, indicating the minimum fee rate of transactions they are interested in receiving. Broadcasting the exact minimum fee can leak information about local mempool state and policy. To mitigate this, Bitcoin Core uses a *geometric series of allowed buckets* and a slight randomization when rounding.

`FeeFilterRounder` implements this mechanism:

- Constructed from a `min_incremental_fee` (`FeeRate`).
- Builds a `BTreeSet<FloatOrd<f64>>` of bucket boundaries:
  - Starts from `max(1 sat, min_incremental_fee.get_fee_perk() / 2)`.
  - Geometric progression via a spacing factor `FEE_FILTER_SPACING`.
  - Truncated at `MAX_FILTER_FEERATE`.
  - Always includes `0`.
- `round(current_min_fee)` selects the bucket *at or above* `current_min_fee` and then, with 2/3 probability (and unless already at the smallest bucket), steps back by one bucket. If `current_min_fee` exceeds all buckets, the maximum bucket is chosen and the stepping‑back logic still applies.

This yields a **quantized and stochastically perturbed** fee filter, reducing the information content of a single advertised value.

### Usage

```rust
use bitcoin_fees::{FeeFilterRounder, FeeRate};

// Example: construct from a 1 sat/vB incremental fee.
// Suppose 1 sat/vB corresponds to 1000 sat/kvB internally.
let incremental = FeeRate::new(1_000u64);
let mut rounder = FeeFilterRounder::new(&incremental);

// Round an internal minimum fee
let internal_min_fee: i64 = 12_345; // satoshis
let filter_value = rounder.round(internal_min_fee);

// `filter_value` is quantized and randomized according to Core’s rules.
```

**Concurrency note:** `round` is *not* thread‑safe due to the use of `FastRandomContext`. Instantiate separate `FeeFilterRounder` instances per thread or guard access with external synchronization if you require concurrent usage.

---

## `FeeCalculation`: Carrying Estimation Results

`FeeCalculation` bundles the outcome of a fee estimation:

- `est: FeeRateEstimationResult` – estimator output (defined elsewhere in the workspace).
- `reason: FeeReason` – structured reason for the selected estimate.
- `desired_target: i32` – requested confirmation target (e.g. N blocks).
- `returned_target: i32` – target that the estimator actually used.

This struct is convenient for exposing rich estimation results through internal APIs or RPC layers, while remaining decoupled from any particular estimator implementation.

---

## Integration Patterns

### Wallet Fee Selection

A minimal wallet integration might:

1. Choose `FeeEstimateMode` based on user preference (economical vs conservative).
2. Query an estimator that returns a `FeeRate` and `FeeReason`.
3. Apply `FeeRate::get_fee(tx_vsize)` to obtain fee in satoshis.
4. Present both the computed fee and the `FeeReason` description to the user.

### Node Fee Filter Logic

A node implementation can:

1. Track a mempool‑derived minimum fee internally.
2. Maintain a `FeeFilterRounder` seeded from local fee policy.
3. Periodically call `round(min_fee)` to produce updated fee filter values.
4. Advertise these values via peer protocol messages.

### Simulation and Research

For fee‑market modelling or backtesting:

- Use `FeeEstimateHorizon` to index datasets by decay half‑life.
- Use `FeeRate` for consistent units and rounding when computing transaction inclusion thresholds.
- Use `FeeReason` to introspect the trade‑off between aggressive and conservative policies.

---

## Relationship to Bitcoin Core

This crate is derived from and intended to be behaviourally aligned with **Bitcoin Core**’s fee handling logic. The integer arithmetic, choice of 1000‑byte kilobyte, truncation rules, and fee filter bucketing all matter for exact equivalence.

If you require strict compatibility with Core for wallet fee selection, mempool policies, or RPC emulation, you should:

- Treat this crate’s behaviour as the authoritative reference for fee‑rate math.
- Keep the crate version synchronized with the upstream `bitcoin-rs` repository (`https://github.com/klebs6/bitcoin-rs`).

---

## License

This crate is distributed under the **MIT** license.

---
