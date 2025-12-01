# bitcoin-tx-confirm-stats

A direct, bit-for-bit-portable implementation of Bitcoin Core-style transaction confirmation statistics and feerate estimation in Rust.

This crate maintains exponentially decayed statistics for how long transactions with a given feerate take to confirm, and uses those statistics (plus current mempool state) to estimate the feerate required to achieve a target confirmation horizon with a desired probability.

---

## Design overview

At a high level, the crate implements the same conceptual estimator as Bitcoin Core's `TxConfirmStats` and related classes:

1. **Bucketization of feerates**
   - The feerate domain is partitioned into monotonically increasing buckets (e.g. `[0–1] sat/vB, [1–2] sat/vB, ..., [N-1–N]`).
   - Each transaction is assigned to exactly one bucket based on its feerate.

2. **Exponential moving averages**
   - For each bucket and for each *confirmation period* (a contiguous range of block counts), the estimator maintains exponentially decayed counts of:
     - Transactions that confirmed within that period.
     - Transactions that *failed* to confirm quickly enough (left the mempool without confirmation after some horizon).
   - This provides a robust estimate of the probability that a transaction in a given bucket will confirm within a target number of blocks, while reacting adaptively to recent history.

3. **Mempool tracking**
   - Independently of the historical confirmation stats, the estimator tracks transactions currently in the mempool:
     - A circular buffer indexed by block height age records, for each bucket, how many txs have been unconfirmed for `k` blocks.
     - A separate vector tracks *old* unconfirmed transactions that have been in the mempool longer than the circular buffer depth.
   - These live counts are incorporated into estimation as additional evidence ("extra" unconfirmed volume that has not yet confirmed), tightening the success-probability bounds.

4. **Feerate estimation procedure**
   - For a given confirmation target (e.g. `6` blocks), the estimator:
     - Maps the target to a **period index** based on an integer scale factor (e.g. 2 blocks per period).
     - Scans buckets from **highest** feerate to **lowest**, aggregating statistics over contiguous bucket ranges.
     - For each candidate range with sufficient sample size, it computes the **success ratio**:
       \[
       p_{\text{success}} = \frac{n_\text{conf}}{n_\text{total} + n_\text{fail} + n_\text{extra}}
       \]
       where:
       - `n_conf` is the decayed count of transactions that confirmed within the target.
       - `n_total` is the decayed count of transactions observed in the range.
       - `n_fail` is the decayed count of transactions that did not confirm quickly enough (left mempool late).
       - `n_extra` is the number of currently unconfirmed transactions at or beyond the target age.
     - Once it finds the *lowest-feerate* range whose `p_success` exceeds a configurable threshold (e.g. 0.95), it reports a **median feerate** from within that bucket range.

5. **Persistence**
   - Historical state can be serialized and deserialized in a sanity-checked binary format using helper formatters.
   - Corrupted or logically inconsistent files trigger panics, mirroring the behavior in the original C++ implementation.

This architecture is engineered to:
- Avoid overfitting to recent blocks while still adapting to regime changes through exponential decay.
- Provide stable, monotone-in-fee estimates: higher feerates should not yield worse estimated confirmation probabilities.
- Be computationally efficient enough to run continuously on each new block.

---

## Crate features and APIs

### Core types

#### `TxConfirmStats`

```rust
pub struct TxConfirmStats {
    buckets:        Arc<Vec<f64>>,          // bucket boundaries (upper limits)
    bucket_map:     Arc<HashMap<f64, u32>>, // mapping from bucket upper-bound to index
    tx_ct_avg:      Vec<f64>,               // decayed total tx count per bucket
    conf_avg:       Vec<Vec<f64>>,          // [period][bucket] decayed count confirming within period
    fail_avg:       Vec<Vec<f64>>,          // [period][bucket] decayed count failing within period
    feerate_avg:    Vec<f64>,               // decayed sum of feerates per bucket
    decay:          f64,                    // per-block exponential decay factor in (0, 1)
    scale:          u32,                    // blocks per period (e.g. 2 => 1 "period" = 2 blocks)
    unconf_txs:     Vec<Vec<i32>>,          // circular buffer of unconfirmed tx counts [age][bucket]
    old_unconf_txs: Vec<i32>,               // long-lived unconfirmed txs per bucket
}
```

A `TxConfirmStats` instance is the central object. You typically allocate one instance per “policy context” (e.g. per asset or per network) and feed it new data as blocks and mempool transitions occur.

Key characteristics:

- **Thread-sharing**: `buckets` and `bucket_map` are `Arc<_>` so multiple estimators can share immutable bucket definitions.
- **Time scaling**: `scale` lets you discretize confirmation horizons coarser than single blocks, reducing state dimensionality.
- **Exponential decay**: `decay` controls the half-life of historical data. After each block, you call `update_moving_averages` to apply decay.

#### `FeeRateEstimatorBucket`

```rust
pub struct FeeRateEstimatorBucket {
    start:           f64,
    end:             f64,
    within_target:   f64,
    total_confirmed: f64,
    in_mempool:      f64,
    left_mempool:    f64,
}
```

Represents aggregated statistics for a contiguous range of feerate buckets.

Fields:
- `start`, `end`: feerate interval `[start, end]` for this aggregated bucket range.
- `within_target`: count of transactions in this range that confirmed within the target horizon.
- `total_confirmed`: total number of observed transactions contributing to this range.
- `in_mempool`: number of still-unconfirmed transactions at or beyond the target ("extra" evidence against faster confirmation).
- `left_mempool`: number of transactions that left the mempool (e.g. eviction, replacement) without confirming within the horizon.

Utility methods:

```rust
impl FeeRateEstimatorBucket {
    pub fn calc_within_target_percentage(&self) -> f64;
    pub fn record_failure_bucket(&mut self, ...);
    pub fn record_passing_bucket(&mut self, ...);
}
```

`calc_within_target_percentage` returns a percentage (0–100). It is used for logging and diagnostics.

#### `FeeRateEstimationResult`

```rust
#[derive(Default)]
pub struct FeeRateEstimationResult {
    pass:  FeeRateEstimatorBucket,
    fail:  FeeRateEstimatorBucket,
    decay: f64,
    scale: u32,
}
```

When you call `TxConfirmStats::estimate_median_val`, you can optionally pass a pointer to a `FeeRateEstimationResult`. If non-null, the estimator fills it with:
- `pass`: the last range that satisfied the success probability target.
- `fail`: the adjacent range that first failed the target (or a trailing failure window if the scan ran out of data).
- `decay`, `scale`: for completeness and downstream debugging.

This gives you a structured, inspectable view of the estimator's decision boundary around the target feerate.

---

## Core functions and methods

### Sample sufficiency and success ratio

The estimator uses two generic helper functions:

```rust
pub fn has_sufficient_samples(total_num: f64, sufficient_tx_val: f64, decay: f64) -> bool {
    total_num >= (sufficient_tx_val / (1.0 - decay))
}

pub fn compute_success_ratio(
    n_conf:    f64,
    total_num: f64,
    fail_num:  f64,
    extra_num: i32,
) -> f64 {
    n_conf / (total_num + fail_num + (extra_num as f64))
}
```

- `has_sufficient_samples` ensures that after applying exponential decay, the effective sample size is large enough: we scale the desired `sufficient_tx_val` by `1 / (1 - decay)` to approximate an infinite-horizon sum.
- `compute_success_ratio` computes a probability-like ratio in `[0, 1]` used to compare against `success_break_point`.

### Feerate median calculation

```rust
impl TxConfirmStats {
    pub fn find_median_feerate(
        &self,
        min_bucket: usize,
        max_bucket: usize,
        mut tx_sum: f64,
    ) -> f64 { ... }
}
```

Given a bucket range `[min_bucket, max_bucket]` and the aggregate `tx_sum` over that range, this method:
- Moves through buckets from `min_bucket` upward, subtracting per-bucket tx counts.
- Locates the bucket where the cumulative count crosses `tx_sum / 2`, and returns:
  \[
  \text{median} \approx \frac{\text{feerate\_avg}[j]}{\text{tx\_ct\_avg}[j]}
  \]
which is the mean feerate of transactions in the median-containing bucket. This is consistent with Bitcoin Core's approximation.

### Estimating a feerate

```rust
impl TxConfirmStats {
    pub fn estimate_median_val(
        &self,
        conf_target:         i32,
        sufficient_tx_val:   f64,
        success_break_point: f64,
        n_block_height:      u32,
        result:              *mut FeeRateEstimationResult,
    ) -> f64 { ... }
}
```

Parameters:
- `conf_target`: desired confirmation horizon in blocks (1-based).
- `sufficient_tx_val`: minimum effective sample size per candidate range (in tx per block units).
- `success_break_point`: minimum acceptable success probability, e.g. `0.95`.
- `n_block_height`: current best chain height; used for indexing the unconfirmed circular buffer.
- `result`: optional out-parameter (raw pointer) for detailed breakdown. Pass `std::ptr::null_mut()` if you do not care.

Return value:
- Estimated median feerate for the chosen bucket range, or `-1.0` if:
  - `conf_target` maps outside tracked periods, or
  - the scan finds no bucket range with sufficient samples meeting the success threshold.

Algorithm sketch:

1. Map `conf_target` to a period index via:
   \[
   \text{period\_target} = \left\lceil \frac{\text{conf\_target}}{\text{scale}} \right\rceil
   \]
   If `period_target` is zero or exceeds `conf_avg.len()`, the function returns `-1.0`.
2. Call `scan_buckets` to walk buckets from highest feerate downward, accumulating:
   - Confirmed counts within `period_target`;
   - Total counts per bucket;
   - Failures per bucket;
   - Additional unconfirmed transactions via `calc_extra_unconfirmed`.
3. `scan_buckets` tracks the best passing range in a `FeeRateEstimatorState`. Once finished, `estimate_median_val`:
   - Extracts `(min_bucket, max_bucket)` from the state;
   - Computes the total `tx_sum` via `sum_tx_ct_avg`;
   - If an answer was found and `tx_sum > 0`, computes `median` via `find_median_feerate` and records the pass range into the state's `pass_bucket`.
4. `finalize_trailing_failure` infers a trailing failure interval if the scan ended while still passing.
5. If `result` is non-null, the method writes out the `pass` and `fail` buckets, along with `decay` and `scale`.

### Sample accumulation helpers

`TxConfirmStats` offers small helpers to isolate primitive operations:

```rust
impl TxConfirmStats {
    pub fn sum_tx_ct_avg(&self, min_bucket: usize, max_bucket: usize) -> f64;

    pub fn accumulate_bucket_data(
        &self,
        period_target: usize,
        bucket_index:  usize,
        n_conf:        &mut f64,
        total_num:     &mut f64,
        fail_num:      &mut f64,
    );
}
```

These are used both internally and by `FeeRateEstimatorState` to keep borrow lifetimes minimal while computing aggregate stats.

### Unconfirmed transaction accounting

The estimator maintains a ring buffer of unconfirmed tx counts by age:

```rust
impl TxConfirmStats {
    pub fn get_max_confirms(&self) -> u32;
    pub fn resize_in_memory_counters(&mut self, newbuckets: usize);
    pub fn clear_current(&mut self, n_block_height: u32);
    pub fn new_tx(&mut self, n_block_height: u32, val: f64) -> u32;
    pub fn remove_tx(
        &mut self,
        entry_height:       u32,
        n_best_seen_height: u32,
        bucketindex:        u32,
        in_block:           bool,
    );

    pub fn calc_extra_unconfirmed(
        &self,
        bucket_index:   usize,
        conf_target:    usize,
        n_block_height: u32,
        bins:           usize,
    ) -> i32;
}
```

- `get_max_confirms()` returns `scale * conf_avg.len()`: the maximum number of blocks of confirmation horizons tracked.
- `resize_in_memory_counters()` adjusts `unconf_txs` and `old_unconf_txs` after deserialization or bucket changes.
- `clear_current(n_block_height)` rolls the circular buffer for the new block, transferring all unconfirmed counts at the current index into `old_unconf_txs`, and zeroing out that bin.
- `new_tx(n_block_height, val)` records a newly-arriving mempool transaction at the current block height in the appropriate bucket.
- `remove_tx(entry_height, n_best_seen_height, bucketindex, in_block)` is called when a transaction leaves the mempool:
  - It decrements the relevant bucket count either from the circular buffer or from `old_unconf_txs` if older than the buffer window.
  - If `in_block == false` and the transaction has been around for at least one `scale` period, it increments `fail_avg` for the periods which it has exceeded, modeling failure-to-confirm.
- `calc_extra_unconfirmed` is used inside the estimator to quantify `n_extra` for success-ratio computation.

### Bucket scanning state machine

`FeeRateEstimatorState` is an internal helper that encapsulates the bucket scanning process.

```rust
pub(crate) struct FeeRateEstimatorState {
    n_conf:           f64,
    total_num:        f64,
    fail_num:         f64,
    extra_num:        i32,
    cur_near_bucket:  usize,
    cur_far_bucket:   usize,
    best_near_bucket: usize,
    best_far_bucket:  usize,
    new_bucket_range: bool,
    passing:          bool,
    found_answer:     bool,
    pass_bucket:      FeeRateEstimatorBucket,
    fail_bucket:      FeeRateEstimatorBucket,
}
```

Key methods:

- `new(max_bucket_index)` initializes the state at the highest bucket.
- `begin_or_extend_range(b)` begins or extends the current contiguous bucket range.
- `accumulate_from_stats(...)` and `add_extra_from_stats(...)` integrate stats from `TxConfirmStats`.
- `has_sufficient(...)` and `cur_pct()` derive sample sufficiency and success ratio.
- `on_first_failure(...)`, `on_passing_reset_and_remember()`, and `finalize_trailing_failure(...)` maintain `pass_bucket`/`fail_bucket` boundaries and track the best passing range.

The public `scan_buckets` method coordinates this logic:

```rust
impl TxConfirmStats {
    pub fn scan_buckets(
        &self,
        conf_target:         i32,
        sufficient_tx_val:   f64,
        success_break_point: f64,
        n_block_height:      u32,
        period_target:       usize,
    ) -> FeeRateEstimatorState { ... }
}
```

This traverses the feerate domain from high to low, adjusting the candidate range as it encounters buckets whose success ratio crosses the threshold, and returns the final `FeeRateEstimatorState` from which `estimate_median_val` extracts its answer.

---

## Persistence

`TxConfirmStats` supports reading and writing its decayed averages (not the live unconfirmed state) to an arbitrary I/O source implementing `Read` / `Write`.

```rust
impl TxConfirmStats {
    pub fn read<R: Read>(
        &mut self,
        filein:          &mut R,
        _n_file_version: i32,
        num_buckets:     usize,
    );

    pub fn write<W: Write>(&self, fileout: &mut W);
}
```

- `read`:
  - Deserializes `decay`, `scale`, `feerate_avg`, `tx_ct_avg`, `conf_avg`, and `fail_avg` using `EncodedDoubleFormatter` and `VectorFormatter` helpers.
  - Performs a series of strong invariance checks, panicking on mismatches:
    - `decay` must be in `(0, 1)`.
    - `scale` must be non-zero.
    - `feerate_avg.len()`, `tx_ct_avg.len()`, and each row of `conf_avg` / `fail_avg` must match `num_buckets`.
    - `max_confirms = scale * conf_avg.len()` must be in `[1, 6 * 24 * 7]` (1–1008 blocks).
  - Only after successful validation does it commit decoded data to `self` and resize `unconf_txs` / `old_unconf_txs` accordingly.

- `write`:
  - Serializes `decay`, `scale`, `feerate_avg`, `tx_ct_avg`, `conf_avg`, and `fail_avg` to `fileout`.
  - Any I/O errors during `write_u32_le` are surfaced as panics, aligning with Bitcoin Core's behavior.

This separation between persistent historical state and in-memory rolling mempool state allows you to checkpoint estimators periodically without encoding the full mempool, which is typically volatile and reconstructible.

---

## Basic usage pattern

Below is a conceptual example (simplified, error-handling omitted) of how an application might employ the crate.

```rust
use std::collections::HashMap;
use std::sync::Arc;
use bitcoin_tx_confirm_stats::{TxConfirmStats, FeeRateEstimationResult};

fn make_default_buckets() -> (Vec<f64>, HashMap<f64, u32>) {
    // Example: geometric progression from 1 sat/vB up to ~1024 sat/vB
    let mut buckets = Vec::new();
    let mut x = 1.0;
    while x <= 1024.0 {
        buckets.push(x);
        x *= 1.2;
    }

    let mut bucket_map = HashMap::new();
    for (i, &upper) in buckets.iter().enumerate() {
        bucket_map.insert(upper, i as u32);
    }

    (buckets, bucket_map)
}

fn main() {
    let (buckets, bucket_map) = make_default_buckets();

    // Track up to 25 periods; with scale=2, that is up to 50 blocks of confirmation horizons
    let max_periods = 25u32;
    let decay = 0.998_f64; // about 1/(1-decay) ≈ 500 blocks effective window
    let scale = 2u32;      // 2 blocks per period

    let mut stats = TxConfirmStats::new(&buckets, &bucket_map, max_periods, decay, scale);

    // Called when a tx enters the mempool
    fn on_tx_added(stats: &mut TxConfirmStats, best_height: u32, feerate: f64) -> u32 {
        stats.new_tx(best_height, feerate)
    }

    // Called when a tx leaves the mempool
    fn on_tx_removed(
        stats:             &mut TxConfirmStats,
        entry_height:       u32,
        best_seen_height:   u32,
        bucket_index:       u32,
        confirmed_in_block: bool,
        blocks_to_confirm:  Option<i32>,
        feerate:            f64,
    ) {
        // Update unconfirmed bookkeeping and failure stats
        stats.remove_tx(entry_height, best_seen_height, bucket_index, confirmed_in_block);

        if confirmed_in_block {
            let blocks = blocks_to_confirm.expect("confirmed tx must have blocks_to_confirm");
            stats.record(blocks, feerate);
        }
    }

    // Called once per new block
    fn on_new_block(stats: &mut TxConfirmStats, new_height: u32) {
        // Roll mempool age accounting
        stats.clear_current(new_height);

        // Apply exponential decay to historical stats
        stats.update_moving_averages();
    }

    // Estimating a feerate for 6-block target, 95% success
    let conf_target = 6;
    let sufficient_tx_val = 1.0;     // tune for your tolerance
    let success_break_point = 0.95;  // 95% probability
    let current_height = 800_000u32; // example

    let mut details = FeeRateEstimationResult::default();
    let est = stats.estimate_median_val(
        conf_target,
        sufficient_tx_val,
        success_break_point,
        current_height,
        &mut details as *mut _,
    );

    if est > 0.0 {
        println!("Estimated feerate: {est:.2} sat/vB for confirmation within {conf_target} blocks");
        println!(
            "Passing bucket range: [{:.2}, {:.2}] sat/vB (within-target: {:.2}%)",
            details.pass().start(),
            details.pass().end(),
            details.pass().calc_within_target_percentage(),
        );
    } else {
        println!("Insufficient data for a stable estimate");
    }
}
```

This example assumes that your surrounding system provides:
- Correct tracking of `entry_height` for each mempool entry.
- `best_seen_height` for the moment of removal or confirmation.
- Accurate `blocks_to_confirm` and `feerate` for confirmed transactions.

---

## Mathematical notes

1. **Exponential decay**

   Each block, the estimator multiplies historical counts by a factor `decay` in `(0, 1)`:
   \[
   x_{t+1} = \text{decay} \cdot x_t + \Delta_t
   \]
   where `Δ_t` is the new contribution from the current block.

   The *effective* horizon of such an exponential moving average is approximately:
   \[
   H_{\text{eff}} \approx \frac{1}{1 - \text{decay}}\ \text{blocks}
   \]

   This informs the choice of `sufficient_tx_val` and sampling thresholds.

2. **Success probability**

   The success ratio deliberately includes both past failures and currently unconfirmed outliers:
   \[
   p_{\text{success}} = \frac{n_\text{conf}}{n_\text{conf} + n_\text{fail} + n_\text{extra}}
   \]

   - `n_conf`: decayed count of txs that did confirm within the target.
   - `n_fail`: decayed count of txs that did not confirm in time and left the mempool.
   - `n_extra`: current unconfirmed transactions whose age exceeds the target, a lower bound on additional failures if they keep not confirming.

   This structure makes the estimator conservative: a large backlog of aged unconfirmed txs in a bucket reduces `p_success`, pushing the required feerate upward.

3. **Bucket median approximation**

   Instead of computing a fully continuous conditional distribution of confirmation times given feerate, the estimator uses bucketed histograms and approximates the median by the bucket mean at the halfway point. This is computationally cheap and sufficiently stable for fee policy.

---

## Safety and panics

- Several methods (particularly `read`) perform strict invariance checks and panic on violations. This matches Bitcoin Core's philosophy that a corrupted fee-estimates file is a programmer/operator error, not an expected runtime condition.
- The crate uses raw pointers for `FeeRateEstimationResult` in order to match C++-style ABI expectations. It is the caller's responsibility to:
  - Pass a valid, non-null pointer if they want results, or
  - Pass `std::ptr::null_mut()` if they do not.
- Indices (bucket, period, buffer) are carefully range-checked in the implementation; nonetheless, misuse of public APIs with incorrect dimensions can still result in panics.

---

## When to use this crate

This crate is appropriate when you need:

- **Bitcoin Core–compatible fee estimation behavior**: You want estimates that behave similarly to Core's `estimatesmartfee`, possibly for cross-implementation consistency.
- **Long-lived, low-level fee policy infrastructure**: You are implementing a wallet, miner, or policy engine that directly interacts with mempool-level signals.
- **Deterministic, inspectable fee analytics**: You want both the feerate and the underlying bucket statistics used to derive that number.

It is not a general-purpose statistical toolkit; it encodes a very specific model intended to mirror existing production systems.

---

## License and authorship

- License: **MIT**
- Authors: `YourName <you@example.com>`
