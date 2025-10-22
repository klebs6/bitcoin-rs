// ---------------- [ File: bitcoin-tx-confirm-stats/src/estimate_median_val.rs ]
crate::ix!();

impl TxConfirmStats {

    /// Calculate a feerate estimate. 
    ///
    /// Find the lowest value bucket (or range of buckets to make sure we have
    /// enough data points) whose transactions still have sufficient likelihood
    /// of being confirmed within the target number of confirmations
    /// 
    /// -----------
    /// @param confTarget
    /// 
    /// target number of confirmations
    ///
    /// ----------
    /// @param sufficientTxVal
    /// 
    /// required average number of transactions per block in a bucket range
    ///
    /// ----------
    /// @param minSuccess
    /// 
    /// the success probability we require
    ///
    /// ----------
    /// @param nBlockHeight
    /// 
    /// the current block height
    /// 
    /// returns -1 on error conditions
    /// 
    pub fn estimate_median_val(
        &self,
        conf_target:         i32,
        sufficient_tx_val:   f64,
        success_break_point: f64,
        n_block_height:      u32,
        result:              *mut FeeRateEstimationResult,
    ) -> f64 {

        // Map conf_target to the period index
        let period_target = ((conf_target + *self.scale() as i32 - 1) / *self.scale() as i32) as usize;

        if period_target == 0 || period_target > self.conf_avg().len() {
            return -1.0; // out of range — Core returns -1 on error
        }

        // Run the core scan
        let mut st = self.scan_buckets(
            conf_target,
            sufficient_tx_val,
            success_break_point,
            n_block_height,
            period_target,
        );

        // Compute the “median” feerate as in Core (average in the bucket
        // containing the median tx)
        //
        let mut median = -1.0;

        // Snapshot best range as VALUES to end immutable borrows before we take &mut to st
        let (min_bucket, max_bucket) = {
            let near = *st.best_near_bucket();
            let far  = *st.best_far_bucket();
            (near.min(far), near.max(far))
        };

        let tx_sum = self.sum_tx_ct_avg(min_bucket, max_bucket);

        if *st.found_answer() && tx_sum != 0.0 {
            median = self.find_median_feerate(min_bucket, max_bucket, tx_sum);
            st.set_pass_range(&self.buckets(), min_bucket, max_bucket);
        }

        // If we were passing until we ran out of data, record the trailing failing range
        st.finalize_trailing_failure(&self.buckets());

        // (Optional) logging — identical content/ordering to Core
        let passed_within_target_perc = st.pass_bucket().calc_within_target_percentage();
        let failed_within_target_perc = st.fail_bucket().calc_within_target_percentage();

        trace!(
            target: "estimatefee",
            "FeeEst: {} > {:.0}% decay {:.5}: feerate: {:.6} from ({:.6} - {:.6}) {:.2}% \
            {:.1}/({:.1} {:.1} mem {:.1} out) \
            Fail: ({:.6} - {:.6}) {:.2}% {:.1}/({:.1} {:.1} mem {:.1} out)",
            conf_target,
            100.0 * success_break_point,
            self.decay(),
            median,
            st.pass_bucket().start(),
            st.pass_bucket().end(),
            passed_within_target_perc,
            st.pass_bucket().within_target(),
            st.pass_bucket().total_confirmed(),
            st.pass_bucket().in_mempool(),
            st.pass_bucket().left_mempool(),
            st.fail_bucket().start(),
            st.fail_bucket().end(),
            failed_within_target_perc,
            st.fail_bucket().within_target(),
            st.fail_bucket().total_confirmed(),
            st.fail_bucket().in_mempool(),
            st.fail_bucket().left_mempool(),
            );

        // Export results if requested (bit-matched to C++)
        if !result.is_null() {
            unsafe {
                (*result).set_pass(*st.pass_bucket());
                (*result).set_fail(*st.fail_bucket());
                (*result).set_decay(*self.decay());
                (*result).set_scale(*self.scale());
            }
        }

        median
    }
}

#[cfg(test)]
mod estimate_median_val_spec {
    use super::*;

    fn mk_stats() -> TxConfirmStats {
        // 3 buckets: [1.0, 2.0, 3.0]; scale=1; periods=3 -> bins = 3
        let buckets = vec![1.0, 2.0, 3.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.0, 1);

        // tx_ct_avg: emphasis on highest bucket (index 2)
        s.set_tx_ct_avg(vec![5.0, 7.0, 20.0]);
        s.set_feerate_avg(vec![  5.0, 70.0, 2000.0]); // => per-bucket avg: [1.0, 10.0, 100.0]

        // conf_avg for period_target=1 (index 0)
        s.set_conf_avg(vec![
            vec![ 1.0,  3.0, 19.0], // within target (1 block)
            vec![ 2.0,  5.0, 19.0], // >=2 blocks
            vec![ 4.0,  6.0, 20.0], // >=3 blocks
        ]);
        s.set_fail_avg(vec![
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
            vec![0.0, 0.0, 0.0],
        ]);

        // no mempool extras
        s.unconf_txs_mut().iter_mut().for_each(|row| row.fill(0));
        s.old_unconf_txs_mut().fill(0);

        s
    }

    #[traced_test]
    fn returns_error_for_out_of_range_target() {
        let s = mk_stats();
        let mut out = FeeRateEstimationResult::default();

        // conf_target=0 -> period_target=0 -> -1 sentinel
        let m = s.estimate_median_val(0, 1.0, 0.8, 0, &mut out as *mut _);
        assert_eq!(m, -1.0);

        // too large => map beyond s.conf_avg.len()
        let m2 = s.estimate_median_val(10_000, 1.0, 0.8, 0, &mut out as *mut _);
        assert_eq!(m2, -1.0);
    }

    #[traced_test]
    fn computes_median_from_best_passing_range_and_exports_result() {
        let s = mk_stats();
        let conf_target = 1;            // period_target=1
        let sufficient_tx_val = 5.0;    // threshold = 5/(1-decay)=5 (decay=0)
        let success_break = 0.85;
        let n_height = 123;

        let mut res = FeeRateEstimationResult::default();
        let median = s.estimate_median_val(
            conf_target,
            sufficient_tx_val,
            success_break,
            n_height,
            &mut res as *mut _,
        );

        // We engineered bucket 2 to pass with high success and enough samples.
        // Its per-bucket feerate avg is 2000/20 = 100.
        assert_eq!(median, 100.0);

        // Best passing range should be exactly [bucket 2, bucket 2]
        let pass = res.pass();
        assert_eq!(pass.start(), &s.buckets()[1]); // start at previous bucket edge
        assert_eq!(pass.end(),   &s.buckets()[2]);

        // No failing txs in our synthetic setup => fail bucket will be either default or
        // may represent trailing failure if logic collected one. We assert pass stats at least:
        assert!(pass.total_confirmed() >= &20.0);
        assert_eq!(pass.in_mempool(), &0.0);
        assert_eq!(pass.left_mempool(), &0.0);

        // Decay and scale echoed back
        assert_eq!(res.decay(), s.decay());
        assert_eq!(res.scale(), s.scale());
    }
}
