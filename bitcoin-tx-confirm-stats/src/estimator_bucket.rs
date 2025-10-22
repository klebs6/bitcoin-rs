// ---------------- [ File: bitcoin-tx-confirm-stats/src/estimator_bucket.rs ]
crate::ix!();

/**
  | Used to return detailed information
  | about a feerate bucket
  |
  */
#[derive(Copy,Clone,Debug,Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct FeeRateEstimatorBucket
{
    start:           f64,
    end:             f64,
    within_target:   f64,
    total_confirmed: f64,
    in_mempool:      f64,
    left_mempool:    f64,
}

impl Default for FeeRateEstimatorBucket {

    fn default() -> Self {
        Self {
            start:          -1.0,
            end:            -1.0,
            within_target:   0.0,
            total_confirmed: 0.0,
            in_mempool:      0.0,
            left_mempool:    0.0,
        }
    }
}

pub fn has_sufficient_samples(total_num: f64, sufficient_tx_val: f64, decay: f64) -> bool {
    total_num >= (sufficient_tx_val / (1.0 - decay))
}

pub fn compute_success_ratio(n_conf: f64, total_num: f64, fail_num: f64, extra_num: i32) -> f64 {
    n_conf / (total_num + fail_num + (extra_num as f64))
}

impl FeeRateEstimatorBucket {

    pub fn record_failure_bucket(
        &mut self,
        cur_near_bucket: usize,
        cur_far_bucket:  usize,
        buckets:         &[f64],
        n_conf:          f64,
        total_num:       f64,
        fail_num:        f64,
        extra_num:       i32,
    ) {
        // First failure — record the failing bucket range
        let fail_min = cur_near_bucket.min(cur_far_bucket);
        let fail_max = cur_near_bucket.max(cur_far_bucket);
        self.start           = if fail_min > 0 { buckets[fail_min - 1] } else { 0.0 };
        self.end             = buckets[fail_max];
        self.within_target   = n_conf;
        self.total_confirmed = total_num;
        self.in_mempool      = extra_num as f64;
        self.left_mempool    = fail_num;
    }

    pub fn record_passing_bucket(
        &mut self,
        n_conf:    f64,
        total_num: f64,
        fail_num:  f64,
        extra_num: i32,
    ) {
        self.within_target   = n_conf;
        self.total_confirmed = total_num;
        self.in_mempool      = extra_num as f64;
        self.left_mempool    = fail_num;
    }

    pub fn calc_within_target_percentage(&self) -> f64 {
        let denom = self.total_confirmed + self.in_mempool + self.left_mempool;
        if denom != 0.0 {
            100.0 * self.within_target / denom
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod estimator_bucket_spec {
    use super::*;

    #[traced_test]
    fn sufficient_samples_threshold_matches_core_formula() {
        // threshold = sufficient / (1 - decay)
        assert!( has_sufficient_samples(10.0,  5.0, 0.5));
        assert!(!has_sufficient_samples( 9.9, 10.0, 0.0));
    }

    #[traced_test]
    fn success_ratio_is_n_conf_over_all_components() {
        assert_eq!(compute_success_ratio(9.0, 1.0, 0.0, 0), 9.0 / 1.0);
        assert_eq!(compute_success_ratio(9.0, 1.0, 1.0, 1), 9.0 / 3.0);
    }

    #[traced_test]
    fn record_failure_and_passing_fill_all_fields() {
        let buckets = [1.0, 2.0, 3.0];

        let mut fail = FeeRateEstimatorBucket::default();
        fail.record_failure_bucket(
            /*cur_near*/ 2, /*cur_far*/ 1,
            &buckets, /*n_conf*/ 7.0, /*total*/ 10.0, /*fail*/ 2.0, /*extra*/ 3,
        );
        assert_eq!(fail.start(), &buckets[0]); // min=1 => start=buckets[0]
        assert_eq!(fail.end(),   &buckets[2]); // max=2 => end=buckets[2]
        assert_eq!(fail.within_target(), &7.0);
        assert_eq!(fail.total_confirmed(), &10.0);
        assert_eq!(fail.in_mempool(), &3.0);
        assert_eq!(fail.left_mempool(), &2.0);

        let mut pass = FeeRateEstimatorBucket::default();
        pass.record_passing_bucket(5.0, 9.0, 1.0, 4);
        assert_eq!(pass.within_target(), &5.0);
        assert_eq!(pass.total_confirmed(), &9.0);
        assert_eq!(pass.left_mempool(), &1.0);
        assert_eq!(pass.in_mempool(), &4.0);
    }

    #[traced_test]
    fn within_target_percentage_handles_zero_denom() {
        let mut b = FeeRateEstimatorBucket::default();
        assert_eq!(b.calc_within_target_percentage(), 0.0);
        b.set_within_target(5.0);
        b.set_total_confirmed(5.0);
        b.set_in_mempool(5.0);
        b.set_left_mempool(0.0);
        // 100 * 5 / (5 + 5 + 0) = 50%
        assert_eq!(b.calc_within_target_percentage(), 50.0);
    }
}
