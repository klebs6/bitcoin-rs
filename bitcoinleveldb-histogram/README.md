# bitcoinleveldb-histogram

Fast, deterministic histogram statistics for Bitcoin-leveldb style latency and value distributions.

This crate provides a faithful Rust implementation of the histogram used inside LevelDB, adapted for the `bitcoinleveldb` subsystem of [`bitcoin-rs`](https://github.com/klebs6/bitcoin-rs). It is designed for profiling and characterizing distributions (e.g., operation latencies, sizes, or fees) with negligible runtime overhead and a compact internal representation.

---

## Features

- Fixed-bucket histogram with `HISTOGRAM_NUM_BUCKETS` logarithmically (LevelDB-style) spaced bucket limits `BUCKET_LIMIT`.
- Online accumulation of statistics:
  - `min`, `max`
  - sample count `num`
  - linear sum `sum`
  - quadratic sum `sum_squares`
- Derived statistics without storing raw samples:
  - arithmetic mean (average)
  - standard deviation
  - median
  - arbitrary percentile
- Deterministic merge of histograms without re-processing samples.
- Human-readable ASCII representation suitable for logs.
- `no_std`-friendly core (uses `core::fmt::Write`), with logging controlled by the caller's logging setup.

The design is suitable for high-throughput systems where allocating or storing all samples is undesirable. The histogram approximates the distribution using bucket counts and pre-defined bucket limits, providing excellent tradeoffs between precision, memory, and CPU cost.

---

## Data model

```rust
pub struct Histogram {
    min:         f64,
    max:         f64,
    num:         f64,
    sum:         f64,
    sum_squares: f64,
    buckets:     [f64; HISTOGRAM_NUM_BUCKETS],
}
```

Semantics:

- **`min`, `max`**: Smallest and largest sample ever added. When empty, `min` is initialized to the largest bucket limit and `max` is `0.0`.
- **`num`**: Total number of samples (as `f64` to match LevelDB and simplify arithmetic with large counts).
- **`sum`**: Sum of all values.
- **`sum_squares`**: Sum of squares of all values, enabling variance computation without storing samples.
- **`buckets`**: Per-bucket counts; bucket boundaries are provided by `BUCKET_LIMIT` and follow LevelDB.

This is an **online algorithm**: each `add` call updates all sufficient statistics in O(number_of_buckets) worst-case for bucketing (linear scan) but typically small constant time, and O(1) for the other aggregates.

The standard deviation implementation uses the identity:

\[
\operatorname{Var}(X) = E[X^2] - (E[X])^2
\]

with

\[
E[X] = \frac{\text{sum}}{\text{num}},\quad E[X^2] = \frac{\text{sum\_squares}}{\text{num}}
\]

and guarded against small negative values from floating-point error.

---

## Basic usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
bitcoinleveldb-histogram = "0.1.19"
```

Create a histogram and feed samples:

```rust
use bitcoinleveldb_histogram::Histogram;

fn main() {
    let mut h = Histogram::default();

    // Simulate some latency / cost data
    for value in [1.0, 5.0, 10.0, 10.0, 100.0] {
        h.add(value);
    }

    println!("count = {}", h.num);
    println!("min   = {}", h.min);
    println!("max   = {}", h.max);
    println!("avg   = {}", h.average());
    println!("p50   = {}", h.median());
    println!("p99   = {}", h.percentile(99.0));
    println!("std   = {}", h.standard_deviation());

    // Pretty ASCII representation, similar to LevelDB output
    println!("{}", h.to_string());
}
```

Example output (shape only, exact values depend on bucket configuration):

```text
Count: 5  Average: 25.2000  StdDev: 38.42
Min: 1.0000  Median: 10.0000  Max: 100.0000
------------------------------------------------------
[       0,       1 )       0   0.000%   0.000% 
[       1,      10 )       2  40.000%  40.000% ########
[      10,     100 )       2  40.000%  80.000% ########
[     100,    1000 )       1  20.000% 100.000% ####
```

---

## Operations

### Construction and reset

```rust
impl Default for Histogram {
    fn default() -> Self;
}

impl Histogram {
    pub fn clear(&mut self);
}
```

- `Histogram::default()` initializes an **empty** histogram: internal counters zeroed, `min` set to the largest bucket limit, `max` to `0.0`.
- `clear()` resets the histogram to this empty state, retaining the allocated bucket array.

### Adding samples

```rust
impl Histogram {
    pub fn add(&mut self, value: f64);
}
```

- Performs a linear search over `BUCKET_LIMIT` to locate the bucket index `b` whose range contains `value`.
- Increments `buckets[b]`.
- Updates `min`, `max`, `num`, `sum`, and `sum_squares`.

The linear search mirrors LevelDB: the bucket count is small and the branch-predictable loop is faster than a generic binary search for the typical usage patterns.

### Merging histograms

```rust
impl Histogram {
    pub fn merge(&mut self, other: &Histogram);
}
```

- Component-wise aggregation of sufficient statistics:
  - `num += other.num`
  - `sum += other.sum`
  - `sum_squares += other.sum_squares`
  - per-bucket addition: `self.buckets[i] += other.buckets[i]`
- `min`/`max` combined as the global extrema across both histograms.
- If `other.num == 0.0`, merge is a no-op.

This makes histogram aggregation trivial in multi-threaded systems: each worker thread can maintain its own `Histogram`, then a coordinator merges them at the end, approximating the global distribution with no raw data transfer.

### Descriptive statistics

```rust
impl Histogram {
    pub fn median(&self) -> f64;
    pub fn percentile(&self, p: f64) -> f64;
    pub fn average(&self) -> f64;
    pub fn standard_deviation(&self) -> f64;
}
```

#### Average

- If `num == 0.0`, returns `0.0`.
- Otherwise `sum / num`.

#### Standard deviation

- If `num == 0.0`, returns `0.0`.
- Uses the `E[X^2] - (E[X])^2` identity (see above).
- Clamps negative variance from floating-point error to `0.0` before `sqrt`.

Mathematically, this computes the **population** standard deviation, not the Bessel-corrected sample standard deviation.

#### Percentile & median

- `median()` is a shorthand for `percentile(50.0)`.
- `percentile(p)`:
  - Returns `0.0` if `num <= 0.0`.
  - Clamps `p` to `[0.0, 100.0]`.
  - Finds the first bucket where the cumulative count crosses `threshold = num * (p / 100.0)`.
  - Linearly interpolates inside that bucket based on position within its count range.
  - Clamps the interpolated value to `[min, max]` to avoid leaking outside the observed range.
  - If no bucket crosses the threshold due to numerical edge cases, falls back to `max`.

Thus percentiles are approximated from the histogram. When bucket spacing is logarithmic, lower percentiles are more precise for small values; higher percentiles focus resolution where buckets are dense.

### String representation

```rust
impl Histogram {
    pub fn to_string(&self) -> String;
}
```

`to_string()` produces a compact text summary plus an ASCII-art visualization:

- Header line with `Count`, `Average`, and `StdDev`.
- Second line with `Min`, `Median`, and `Max`.
- A row per non-empty bucket:
  - Half-open interval `[left, right)`.
  - Absolute count in that bucket.
  - Bucket-local percentage of total.
  - Cumulative percentage up to and including that bucket.
  - A bar of `#` characters (20 total marks correspond to 100% of the distribution in a single bucket).

This representation is intended for logs and interactive diagnostics.

---

## Logging behavior

The crate uses logging macros (e.g., `trace!`, `debug!`, `warn!`) to expose internal events and edge cases:

- `trace!` at entry/exit of methods, and for detailed computation steps.
- `debug!` for noteworthy but non-fatal situations (e.g., percentile on empty histogram).
- `warn!` if a percentile outside `[0,100]` is requested.

Exact logging backend configuration is left to the embedding application; if no logger is configured, logs will be ignored as usual.

---

## Integration examples

### Profiling latency in a storage engine

```rust
use bitcoinleveldb_histogram::Histogram;
use std::time::{Duration, Instant};

fn profile_ops<F: Fn()>(iterations: usize, op: F) -> Histogram {
    let mut h = Histogram::default();

    for _ in 0..iterations {
        let start = Instant::now();
        op();
        let elapsed = start.elapsed();
        let micros = elapsed.as_secs_f64() * 1_000_000.0;
        h.add(micros);
    }

    h
}

fn main() {
    let histogram = profile_ops(10_000, || {
        // your database or network operation here
    });

    println!("Latency distribution (µs):\n{}", histogram.to_string());
}
```

### Aggregating histograms across threads

```rust
use bitcoinleveldb_histogram::Histogram;
use std::thread;

fn main() {
    let threads = 4;
    let per_thread = 1000;

    let handles: Vec<_> = (0..threads)
        .map(|_| {
            thread::spawn(move || {
                let mut h = Histogram::default();
                for i in 0..per_thread {
                    h.add((i % 100) as f64);
                }
                h
            })
        })
        .collect();

    let mut global = Histogram::default();

    for handle in handles {
        let local = handle.join().expect("thread panicked");
        global.merge(&local);
    }

    println!("Global distribution:\n{}", global.to_string());
}
```

---

## Relationship to LevelDB and bitcoin-rs

This crate mirrors the histogram design from LevelDB, making it straightforward to:

- Port profiling and diagnostic tooling from C++ LevelDB to Rust.
- Interpret distributions in `bitcoin-rs` using the same conventions as upstream components.

If you are interacting directly with LevelDB or with systems already using its histogram output, this crate should produce comparable behavior and text representations, subject to differences in bucket configuration.

---

## License

This crate is distributed under the **MIT** license.

For source code, issues, and contributions, see the parent repository:

- Repository: <https://github.com/klebs6/bitcoin-rs>
