// ---------------- [ File: bitcoinleveldb-histogram/src/histogram.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/histogram.h]

pub struct Histogram {
    min:         f64,
    max:         f64,
    num:         f64,
    sum:         f64,
    sum_squares: f64,
    buckets:     [f64; HISTOGRAM_NUM_BUCKETS],
}

pub const HISTOGRAM_NUM_BUCKETS: usize = 154;

impl Default for Histogram {
    fn default() -> Self {
        trace!("Constructing default Histogram instance");
        Histogram {
            min: BUCKET_LIMIT[HISTOGRAM_NUM_BUCKETS - 1],
            max: 0.0,
            num: 0.0,
            sum: 0.0,
            sum_squares: 0.0,
            buckets: [0.0; HISTOGRAM_NUM_BUCKETS],
        }
    }
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/histogram.cc]
impl Histogram {
    
    pub fn clear(&mut self) {

        trace!("Clearing histogram state");

        self.min = BUCKET_LIMIT[HISTOGRAM_NUM_BUCKETS - 1];
        self.max = 0.0;
        self.num = 0.0;
        self.sum = 0.0;
        self.sum_squares = 0.0;

        for bucket in self.buckets.iter_mut() {
            *bucket = 0.0;
        }

        debug!(
            "Histogram cleared: min={}, max={}, num={}, sum={}, sum_squares={}",
            self.min,
            self.max,
            self.num,
            self.sum,
            self.sum_squares,
        );
    }
    
    pub fn add(&mut self, value: f64) {
        trace!("Adding value {} to histogram", value);

        // Linear search is fast enough for our usage (matches LevelDB behavior)
        let mut b: usize = 0;
        while b < HISTOGRAM_NUM_BUCKETS - 1 && BUCKET_LIMIT[b] <= value {
            b += 1;
        }

        self.buckets[b] += 1.0;

        if self.num == 0.0 {
            // First sample initializes min/max
            self.min = value;
            self.max = value;
        } else {
            if self.min > value {
                self.min = value;
            }
            if self.max < value {
                self.max = value;
            }
        }

        self.num += 1.0;
        self.sum += value;
        self.sum_squares += value * value;

        debug!(
            "Added sample: value={}, bucket={}, num={}, min={}, max={}",
            value,
            b,
            self.num,
            self.min,
            self.max
        );
    }

    pub fn merge(&mut self, other: &Histogram) {
        trace!("Merging histogram with another histogram");

        if other.num == 0.0 {
            debug!("Other histogram is empty; merge() is a no-op");
            return;
        }

        if self.num == 0.0 {
            // If self is empty, take other's min/max directly
            self.min = other.min;
            self.max = other.max;
        } else {
            if other.min < self.min {
                self.min = other.min;
            }
            if other.max > self.max {
                self.max = other.max;
            }
        }

        self.num += other.num;
        self.sum += other.sum;
        self.sum_squares += other.sum_squares;

        for i in 0..HISTOGRAM_NUM_BUCKETS {
            self.buckets[i] += other.buckets[i];
        }

        debug!(
            "Merge complete: num={}, min={}, max={}, sum={}, sum_squares={}",
            self.num,
            self.min,
            self.max,
            self.sum,
            self.sum_squares
        );
    }
    
    pub fn median(&self) -> f64 {
        trace!("Computing median for histogram");
        self.percentile(50.0)
    }
    
    pub fn percentile(&self, p: f64) -> f64 {
        trace!(
            "Computing percentile p={} for histogram with {} samples",
            p,
            self.num
        );

        if self.num <= 0.0 {
            debug!("Percentile requested on empty histogram; returning 0.0");
            return 0.0;
        }

        if p < 0.0 || p > 100.0 {
            warn!(
                "Percentile outside [0,100] requested (p={}); results will be clamped",
                p
            );
        }

        let clamped_p = if p < 0.0 {
            0.0
        } else if p > 100.0 {
            100.0
        } else {
            p
        };

        let threshold = self.num * (clamped_p / 100.0);
        let mut cumulative = 0.0_f64;

        for (b, bucket_count) in self.buckets.iter().enumerate() {
            if *bucket_count <= 0.0 {
                continue;
            }

            cumulative += *bucket_count;

            if cumulative >= threshold {
                let left_point = if b == 0 { 0.0 } else { BUCKET_LIMIT[b - 1] };
                let right_point = BUCKET_LIMIT[b];

                let left_sum = cumulative - *bucket_count;
                let right_sum = cumulative;
                let width = right_sum - left_sum;

                let pos = if width <= 0.0 {
                    0.0
                } else {
                    (threshold - left_sum) / width
                };

                let mut r = left_point + (right_point - left_point) * pos;

                if r < self.min {
                    r = self.min;
                }
                if r > self.max {
                    r = self.max;
                }

                debug!(
                    "Percentile p={} resolved in bucket {} -> value {}",
                    clamped_p,
                    b,
                    r
                );

                return r;
            }
        }

        debug!(
            "Percentile p={} fell through all buckets; returning max={}",
            clamped_p,
            self.max
        );
        self.max
    }
    
    pub fn average(&self) -> f64 {
        if self.num == 0.0 {
            debug!("Average requested on empty histogram; returning 0.0");
            return 0.0;
        }

        let avg = self.sum / self.num;
        trace!(
            "Average computed for histogram: num={}, sum={}, average={}",
            self.num,
            self.sum,
            avg
        );
        avg
    }
    
    pub fn standard_deviation(&self) -> f64 {
        if self.num == 0.0 {
            debug!("Standard deviation requested on empty histogram; returning 0.0");
            return 0.0;
        }

        let variance_numerator = self.sum_squares * self.num - self.sum * self.sum;
        let variance_denominator = self.num * self.num;

        let variance = if variance_denominator <= 0.0 {
            0.0
        } else {
            variance_numerator / variance_denominator
        };

        let variance = if variance < 0.0 { 0.0 } else { variance };
        let stddev = variance.sqrt();

        trace!(
            "Standard deviation computed: num={}, sum={}, sum_squares={}, variance={}, stddev={}",
            self.num,
            self.sum,
            self.sum_squares,
            variance,
            stddev
        );

        stddev
    }
   
    pub fn to_string(&self) -> String {
        trace!(
            "Serializing histogram with {} samples to string",
            self.num
        );

        use core::fmt::Write;

        let mut r = String::new();

        let avg = self.average();
        let stddev = self.standard_deviation();
        let min_value = if self.num == 0.0 { 0.0 } else { self.min };
        let median_value = self.median();
        let max_value = self.max;

        let _ = write!(
            &mut r,
            "Count: {:.0}  Average: {:.4}  StdDev: {:.2}\n",
            self.num,
            avg,
            stddev
        );
        let _ = write!(
            &mut r,
            "Min: {:.4}  Median: {:.4}  Max: {:.4}\n",
            min_value,
            median_value,
            max_value
        );
        r.push_str("------------------------------------------------------\n");

        if self.num <= 0.0 {
            debug!("Histogram is empty; string representation contains only header");
            return r;
        }

        let mult = 100.0 / self.num;
        let mut cumulative = 0.0_f64;

        for (b, bucket_count) in self.buckets.iter().enumerate() {
            if *bucket_count <= 0.0 {
                continue;
            }

            cumulative += *bucket_count;

            let left_point = if b == 0 { 0.0 } else { BUCKET_LIMIT[b - 1] };
            let right_point = BUCKET_LIMIT[b];

            let _ = write!(
                &mut r,
                "[ {:7.0}, {:7.0} ) {:7.0} {:7.3}% {:7.3}% ",
                left_point,
                right_point,
                bucket_count,
                mult * bucket_count,
                mult * cumulative
            );

            // Add hash marks based on percentage; 20 marks for 100%.
            let marks_float = 20.0 * (*bucket_count / self.num) + 0.5;
            let marks = if marks_float <= 0.0 {
                0usize
            } else {
                marks_float as usize
            };

            for _ in 0..marks {
                r.push('#');
            }
            r.push('\n');
        }

        debug!(
            "Histogram serialization complete; result has {} characters",
            r.len()
        );

        r
    }
}

#[cfg(test)]
mod histogram_behavior_tests {
    use super::*;

    fn assert_approx_eq(actual: f64, expected: f64, epsilon: f64) {
        let diff = (actual - expected).abs();
        if diff > epsilon {
            error!(
                "approx_eq assertion failed: actual={}, expected={}, diff={}, epsilon={}",
                actual,
                expected,
                diff,
                epsilon
            );
            panic!(
                "approx_eq assertion failed: actual={}, expected={}, diff={}, epsilon={}",
                actual,
                expected,
                diff,
                epsilon
            );
        } else {
            trace!(
                "approx_eq assertion succeeded: actual={}, expected={}, diff={}, epsilon={}",
                actual,
                expected,
                diff,
                epsilon
            );
        }
    }

    #[traced_test]
    fn histogram_empty_state_behaves_as_expected() {
        info!("Starting empty histogram state test");

        let hist = Histogram::default();

        assert_approx_eq(hist.average(), 0.0, 1e-12);
        assert_approx_eq(hist.standard_deviation(), 0.0, 1e-12);
        assert_approx_eq(hist.median(), 0.0, 1e-12);
        assert_approx_eq(hist.percentile(50.0), 0.0, 1e-12);
        assert_approx_eq(hist.percentile(95.0), 0.0, 1e-12);

        let repr = hist.to_string();
        info!("Empty histogram string representation:\n{}", repr);

        assert!(
            repr.contains("Count: 0"),
            "expected header to contain Count: 0"
        );
        assert!(
            repr.contains("Min: 0.0000  Median: 0.0000  Max: 0.0000"),
            "expected header to contain zero min/median/max"
        );
    }

    #[traced_test]
    fn histogram_clear_resets_state() {
        info!("Starting histogram clear() behavior test");

        let mut hist = Histogram::default();
        for v in 1..=10 {
            hist.add(v as f64);
        }

        hist.clear();

        assert_approx_eq(hist.average(), 0.0, 1e-12);
        assert_approx_eq(hist.standard_deviation(), 0.0, 1e-12);
        assert_approx_eq(hist.median(), 0.0, 1e-12);

        let repr = hist.to_string();
        info!("Histogram after clear() string representation:\n{}", repr);

        assert!(
            repr.contains("Count: 0"),
            "expected Count: 0 after clear()"
        );
    }

    #[traced_test]
    fn histogram_single_value_has_zero_deviation() {
        info!("Starting single-value histogram test");

        let value = 5.0_f64;
        let mut hist = Histogram::default();
        hist.add(value);

        assert_approx_eq(hist.average(), value, 1e-12);
        assert_approx_eq(hist.standard_deviation(), 0.0, 1e-12);
        assert_approx_eq(hist.median(), value, 1e-9);
        assert_approx_eq(hist.percentile(50.0), value, 1e-9);
        assert_approx_eq(hist.percentile(90.0), value, 1e-9);
    }

    #[traced_test]
    fn histogram_computes_average_and_stddev_for_sequence() {
        info!("Starting average/stddev test for sequence 1..=5");

        let mut hist = Histogram::default();
        let values = [1.0_f64, 2.0, 3.0, 4.0, 5.0];

        for &v in &values {
            hist.add(v);
        }

        assert_approx_eq(hist.average(), 3.0, 1e-12);

        let expected_variance = 2.0_f64;
        let expected_stddev = expected_variance.sqrt();
        assert_approx_eq(hist.standard_deviation(), expected_stddev, 1e-12);
    }

    #[traced_test]
    fn histogram_percentiles_match_leveldb_behavior_for_small_sample() {
        info!("Starting percentile behavior test for small sample set");

        let mut hist = Histogram::default();
        let values = [1.0_f64, 2.0, 3.0, 4.0, 5.0];

        for &v in &values {
            hist.add(v);
        }

        // Expected values derived from original LevelDB algorithm for this dataset.
        assert_approx_eq(hist.median(), 3.5, 1e-9);
        assert_approx_eq(hist.percentile(25.0), 2.25, 1e-9);
        assert_approx_eq(hist.percentile(75.0), 4.75, 1e-9);
    }

    #[traced_test]
    fn histogram_merge_combines_statistics_correctly() {
        info!("Starting histogram merge() behavior test");

        let mut hist_a = Histogram::default();
        let mut hist_b = Histogram::default();

        let values_a = [10.0_f64, 20.0, 30.0];
        let values_b = [40.0_f64, 50.0, 60.0];

        for &v in &values_a {
            hist_a.add(v);
        }
        for &v in &values_b {
            hist_b.add(v);
        }

        hist_a.merge(&hist_b);

        let all_values = [10.0_f64, 20.0, 30.0, 40.0, 50.0, 60.0];
        let mean = all_values.iter().sum::<f64>() / (all_values.len() as f64);
        assert_approx_eq(hist_a.average(), mean, 1e-12);

        let variance = {
            let mut sum_sq = 0.0_f64;
            for &v in &all_values {
                let diff = v - mean;
                sum_sq += diff * diff;
            }
            sum_sq / (all_values.len() as f64)
        };
        let stddev = variance.sqrt();
        assert_approx_eq(hist_a.standard_deviation(), stddev, 1e-9);

        let repr = hist_a.to_string();
        info!("Merged histogram string representation:\n{}", repr);
        assert!(
            repr.contains("Count: 6"),
            "expected Count: 6 after merge()"
        );
    }

    #[traced_test]
    fn histogram_to_string_formats_bucket_lines_for_simple_case() {
        info!("Starting histogram to_string() bucket formatting test");

        let mut hist = Histogram::default();

        // All samples in the first bucket [0, 1).
        for _ in 0..10 {
            hist.add(0.5);
        }

        let repr = hist.to_string();
        info!("Histogram with 10 samples in [0,1) bucket:\n{}", repr);

        assert!(
            repr.contains("[       0,       1 )"),
            "expected bucket line for [0,1) in output"
        );
        assert!(
            repr.contains("100.000% 100.000%"),
            "expected 100%% of samples in single bucket"
        );

        let hash_count = repr.chars().filter(|&c| c == '#').count();
        assert!(
            hash_count >= 20,
            "expected at least 20 hash marks, got {}",
            hash_count
        );
    }

    #[traced_test]
    fn histogram_percentile_outputs_are_monotonic() {
        info!("Starting percentile monotonicity test");

        let mut hist = Histogram::default();
        for v in 1..=100 {
            hist.add(v as f64);
        }

        let p10 = hist.percentile(10.0);
        let p50 = hist.percentile(50.0);
        let p90 = hist.percentile(90.0);

        assert!(
            p10 <= p50,
            "10th percentile {} should be <= 50th percentile {}",
            p10,
            p50
        );
        assert!(
            p50 <= p90,
            "50th percentile {} should be <= 90th percentile {}",
            p50,
            p90
        );
    }
}
