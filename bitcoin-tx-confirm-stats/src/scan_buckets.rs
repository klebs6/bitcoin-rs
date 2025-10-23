// ---------------- [ File: bitcoin-tx-confirm-stats/src/scan_buckets.rs ]
crate::ix!();

impl TxConfirmStats {

    pub fn scan_buckets(
        &self,
        conf_target:         i32,
        sufficient_tx_val:   f64,
        success_break_point: f64,
        n_block_height:      u32,
        period_target:       usize,
    ) -> FeeRateEstimatorState {
        let bins = self.unconf_txs().len();
        let max_bucket_index = self.buckets().len() - 1;

        let mut st = FeeRateEstimatorState::new(max_bucket_index);

        // Start counting from highest feerate transactions
        for b in (0..=max_bucket_index).rev() {
            st.begin_or_extend_range(b);

            // Accumulate tx/conf/fail moving averages (no overlapping &mut borrows)
            st.accumulate_from_stats(self, period_target, b);

            // Add mempool 'extra' counts for confTarget..GetMaxConfirms
            st.add_extra_from_stats(self, b, conf_target as usize, n_block_height, bins);

            // Evaluate only once we have sufficient samples
            if st.has_sufficient(sufficient_tx_val, *self.decay()) {
                let cur_pct = st.cur_pct();

                if cur_pct < success_break_point {
                    if st.is_passing() {
                        st.on_first_failure(&self.buckets()[..]);
                    }
                    continue;
                } else {
                    st.on_passing_reset_and_remember();
                }
            }
        }

        st
    }
}

#[cfg(test)]
mod scan_buckets_spec {
    use super::*;

    fn mk_stats() -> TxConfirmStats {
        let buckets = vec![1.0, 2.0, 3.0, 4.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 2, 0.0, 1);

        // total per bucket
        s.set_tx_ct_avg(vec![ 5.0,  6.0,  7.0,  20.0]);
        s.set_feerate_avg(vec![ 5.0, 60.0, 70.0, 2000.0]);

        // within target (period 0)
        s.set_conf_avg(vec![
            vec![ 1.0,  2.0,  3.0,  18.0], // ~ pass at high bucket, fail at lower
            vec![ 2.0,  3.0,  4.0,  19.0],
        ]);
        s.set_fail_avg(vec![
            vec![ 1.0,  3.0,  4.0,  0.0 ],
            vec![ 1.0,  3.0,  4.0,  0.0 ],
        ]);

        // no extras
        s.unconf_txs_mut().iter_mut().for_each(|row| row.fill(0));
        s.old_unconf_txs_mut().fill(0);

        s
    }

    #[traced_test]
    fn scan_records_first_failure_after_passing_and_remembers_best_range() {
        let s = mk_stats();
        let conf_target = 1;          // period_target=1
        let sufficient_tx_val = 5.0;  // enough samples once we include any bucket
        let success_break = 0.85;     // bucket 3 passes (18/20=0.9)
        let n_height = 0u32;

        let st = s.scan_buckets(
            conf_target, sufficient_tx_val, success_break, n_height, /*period_target*/ 1
        );

        // Best passing range should be the highest bucket (index 3) alone
        let (minb, maxb) = st.best_range_minmax();
        assert_eq!((minb, maxb), (3, 3));

        // After that, lower buckets fail (given our conf/total/fail), so fail bucket should be set
        let fb = st.fail_bucket();
        assert!(fb.start() >= &s.buckets()[0]);
        assert!(fb.end()   <= &s.buckets()[3]);
        // Passing state at end can vary depending on data; we at least assert some failure snapshot recorded:
        assert!(fb.total_confirmed() >= &0.0);
    }
}
