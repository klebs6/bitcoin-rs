// ---------------- [ File: bitcoin-tx-confirm-stats/src/accumulate_bucket_data.rs ]
crate::ix!();

impl TxConfirmStats {

    /// Compute the sum of tx count averages between the given bucket range (inclusive).
    ///
    /// # Arguments
    /// * `min_bucket` - inclusive lower index
    /// * `max_bucket` - inclusive upper index
    ///
    /// # Returns
    /// The summed tx count average across the specified bucket range.
    pub fn sum_tx_ct_avg(&self, min_bucket: usize, max_bucket: usize) -> f64 {
        self.tx_ct_avg()[min_bucket..=max_bucket].iter().sum()
    }

    /// Accumulate confirmation and failure statistics for a given bucket index.
    pub fn accumulate_bucket_data(
        &self,
        period_target: usize,
        bucket_index: usize,
        n_conf: &mut f64,
        total_num: &mut f64,
        fail_num: &mut f64,
    ) {
        *n_conf    += self.conf_avg()[period_target - 1][bucket_index];
        *total_num += self.tx_ct_avg()[bucket_index];
        *fail_num  += self.fail_avg()[period_target - 1][bucket_index];
    }
}

#[cfg(test)]
mod accumulate_bucket_data_spec {
    use super::*;

    fn sample_stats() -> TxConfirmStats {
        let buckets = vec![1.0, 2.0, 3.0, 4.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.5, 2);
        // Shape: conf_avg[period][bucket]
        s.set_conf_avg(vec![
            vec![10.0, 11.0, 12.0, 13.0],
            vec![20.0, 21.0, 22.0, 23.0],
            vec![30.0, 31.0, 32.0, 33.0],
        ]);
        s.set_fail_avg(vec![
            vec![1.0, 1.1, 1.2, 1.3],
            vec![2.0, 2.1, 2.2, 2.3],
            vec![3.0, 3.1, 3.2, 3.3],
        ]);
        s.set_tx_ct_avg(vec![100.0, 200.0, 300.0, 400.0]);
        s
    }

    #[traced_test]
    fn sum_tx_ct_avg_inclusive_ranges() {
        let s = sample_stats();
        assert_eq!(s.sum_tx_ct_avg(0, 0), 100.0);
        assert_eq!(s.sum_tx_ct_avg(1, 2), 200.0 + 300.0);
        assert_eq!(s.sum_tx_ct_avg(0, 3), 1000.0);
    }

    #[traced_test]
    fn accumulate_bucket_data_adds_correct_components() {
        let s = sample_stats();
        let period_target = 2; // uses index [1] in conf_avg/fail_avg
        let b = 3;

        let mut n_conf = 0.0;
        let mut total_num = 0.0;
        let mut fail_num = 0.0;

        s.accumulate_bucket_data(period_target, b, &mut n_conf, &mut total_num, &mut fail_num);

        assert_eq!(n_conf,    s.conf_avg()[period_target - 1][b]);
        assert_eq!(total_num, s.tx_ct_avg()[b]);
        assert_eq!(fail_num,  s.fail_avg()[period_target - 1][b]);
    }
}
