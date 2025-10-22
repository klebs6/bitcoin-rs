// ---------------- [ File: bitcoin-tx-confirm-stats/src/estimator_state.rs ]
crate::ix!();

#[derive(Debug,Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub(crate) struct FeeRateEstimatorState {

    /* Counters for the current bucket range */

    /// Number of tx's confirmed within the confTarget
    n_conf:    f64,

    /// Total number of tx's that were ever confirmed
    total_num: f64,

    /// Number of tx's that were never confirmed but removed from the mempool after confTarget
    fail_num:  f64,

    /// Number of tx's still in mempool for confTarget or longer
    extra_num: i32,

    // --- Range trackers ---
    //
    // We'll combine buckets until we have enough samples.
    //
    // The near and far variables will define the range we've combined
    //
    // The best variables are the last range we saw which still had a high enough confirmation rate
    // to count as success.
    //
    // The cur variables are the current range we're counting.
    //
    cur_near_bucket:  usize,
    cur_far_bucket:   usize,
    best_near_bucket: usize,
    best_far_bucket:  usize,

    // --- Control Flags ---
    new_bucket_range: bool,
    passing:          bool,
    found_answer:     bool,

    // --- Snapshots for pass/fail ranges ---
    pass_bucket: FeeRateEstimatorBucket,
    fail_bucket: FeeRateEstimatorBucket,
}

impl FeeRateEstimatorState {

    pub fn reset_counters(&mut self) {
        self.n_conf    = 0.0;
        self.total_num = 0.0;
        self.fail_num  = 0.0;
        self.extra_num = 0;
    }

    pub fn new(max_bucket_index: usize) -> Self {
        Self {
            n_conf: 0.0,
            total_num: 0.0,
            extra_num: 0,
            fail_num: 0.0,

            cur_near_bucket:  max_bucket_index,
            cur_far_bucket:   max_bucket_index,
            best_near_bucket: max_bucket_index,
            best_far_bucket:  max_bucket_index,

            new_bucket_range: true,
            passing: true,
            found_answer: false,

            pass_bucket: FeeRateEstimatorBucket::default(),
            fail_bucket: FeeRateEstimatorBucket::default(),
        }
    }

    #[inline]
    pub fn begin_or_extend_range(&mut self, b: usize) {
        if self.new_bucket_range {
            self.cur_near_bucket = b;
            self.new_bucket_range = false;
        }
        self.cur_far_bucket = b;
    }

    #[inline]
    pub fn is_passing(&self) -> bool { self.passing }

    #[inline]
    pub fn best_range_minmax(&self) -> (usize, usize) {
        let near = self.best_near_bucket;
        let far  = self.best_far_bucket;
        (near.min(far), near.max(far))
    }

    // -------- State evolution helpers (keep borrows internal to these methods) --------

    /// Accumulate tx/conf/fail deltas from stats for bucket `b`.
    #[inline]
    pub fn accumulate_from_stats(&mut self, stats: &TxConfirmStats, period_target: usize, b: usize) {
        self.n_conf    += stats.conf_avg()[period_target - 1][b];
        self.total_num += stats.tx_ct_avg()[b];
        self.fail_num  += stats.fail_avg()[period_target - 1][b];
    }

    /// Add extra unconfirmed from stats context into this state.
    #[inline]
    pub fn add_extra_from_stats(
        &mut self,
        stats: &TxConfirmStats,
        b: usize,
        conf_target: usize,
        n_block_height: u32,
        bins: usize,
    ) {
        self.extra_num += stats.calc_extra_unconfirmed(b, conf_target, n_block_height, bins);
    }

    /// Delegate sufficient sample check using this state's totals.
    #[inline]
    pub fn has_sufficient(&self, sufficient_tx_val: f64, decay: f64) -> bool {
        has_sufficient_samples(self.total_num, sufficient_tx_val, decay)
    }

    /// Compute the current success ratio using the state's counters.
    #[inline]
    pub fn cur_pct(&self) -> f64 {
        compute_success_ratio(self.n_conf, self.total_num, self.fail_num, self.extra_num)
    }

    /// First failure in a passing streak: snapshot & record fail bucket, mark not passing.
    pub fn on_first_failure(&mut self, buckets: &[f64]) {
        let near = self.cur_near_bucket;
        let far  = self.cur_far_bucket;
        let fail_min = near.min(far);
        let fail_max = near.max(far);

        self.fail_bucket.set_start(if fail_min > 0 { buckets[fail_min - 1] } else { 0.0 });
        self.fail_bucket.set_end(buckets[fail_max]);
        self.fail_bucket.set_within_target(self.n_conf);
        self.fail_bucket.set_total_confirmed(self.total_num);
        self.fail_bucket.set_in_mempool(self.extra_num as f64);
        self.fail_bucket.set_left_mempool(self.fail_num);

        self.set_passing(false);
    }

    /// Passing: reset fail bucket, mark answer found, record pass bucket, reset counters,
    /// memorize the best range, and mark that a new range should begin.
    pub fn on_passing_reset_and_remember(&mut self) {
        self.fail_bucket  = FeeRateEstimatorBucket::default();
        self.found_answer = true;
        self.passing      = true;

        self.pass_bucket.record_passing_bucket(
            self.n_conf, self.total_num, self.fail_num, self.extra_num
        );

        self.reset_counters();

        self.best_near_bucket = self.cur_near_bucket;
        self.best_far_bucket  = self.cur_far_bucket;
        self.new_bucket_range = true;
    }

    /// Set the pass bucket [start, end] from bucket indices.
    #[inline]
    pub fn set_pass_range(&mut self, buckets: &[f64], min_bucket: usize, max_bucket: usize) {

        let start = if min_bucket > 0 { buckets[min_bucket - 1] } else { 0.0 };
        let end   = buckets[max_bucket];

        self.pass_bucket.set_start(start);
        self.pass_bucket.set_end(end);
    }

    /// If we were passing until we ran out of data (and we had a non-empty current range),
    /// record the trailing failing range.
    pub fn finalize_trailing_failure(&mut self, buckets: &[f64]) {
        if self.passing && !self.new_bucket_range {
            let near = self.cur_near_bucket;
            let far  = self.cur_far_bucket;
            let fail_min = near.min(far);
            let fail_max = near.max(far);

            self.fail_bucket.set_start(if fail_min > 0 { buckets[fail_min - 1] } else { 0.0 });
            self.fail_bucket.set_end(buckets[fail_max]);
            self.fail_bucket.set_within_target(self.n_conf);
            self.fail_bucket.set_total_confirmed(self.total_num);
            self.fail_bucket.set_in_mempool(self.extra_num as f64);
            self.fail_bucket.set_left_mempool(self.fail_num);
        }
    }
}

#[cfg(test)]
mod estimator_state_spec {
    use super::*;

    fn mk_stats() -> TxConfirmStats {
        let buckets = vec![1.0, 2.0, 3.0, 4.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 2, 0.2, 2);
        // period 0 and 1, 4 buckets
        s.set_conf_avg(vec![
            vec![3.0, 4.0, 5.0, 6.0],
            vec![6.0, 8.0, 10.0, 12.0],
        ]);
        s.set_fail_avg(vec![
            vec![0.5, 0.6, 0.7, 0.8],
            vec![1.0, 1.2, 1.4, 1.6],
        ]);
        s.set_tx_ct_avg(vec![ 5.0, 10.0, 20.0, 40.0]);
        s.set_feerate_avg(vec![ 5.0, 100.0, 200.0, 400.0]);

        // Put some extras in mempool ring so add_extra_from_stats can observe them
        let bins = s.get_max_confirms() as usize; // 2 periods * scale 2 = 4
        for idx in 0..bins {
            s.unconf_txs_mut()[idx][2] = (idx + 1) as i32; // bucket 2, pattern 1..bins
        }
        s.old_unconf_txs_mut()[2] = 7;
        s
    }

    #[traced_test]
    fn new_initializes_to_sane_defaults() {
        let st = FeeRateEstimatorState::new(9);
        assert_eq!(*st.pass_bucket().start(), -1.0);
        assert!(st.is_passing());
        assert!(st.new_bucket_range());
        assert!(!*st.found_answer());
        assert_eq!(*st.cur_near_bucket(), 9);
        assert_eq!(*st.cur_far_bucket(), 9);
    }

    #[traced_test]
    fn begin_or_extend_range_works_as_documented() {
        let mut st = FeeRateEstimatorState::new(3);
        st.begin_or_extend_range(2);
        assert_eq!(*st.cur_near_bucket(), 2);
        assert_eq!(*st.cur_far_bucket(),  2);
        st.begin_or_extend_range(1);
        assert_eq!(*st.cur_near_bucket(), 2);
        assert_eq!(*st.cur_far_bucket(),  1);
    }

    #[traced_test]
    fn accumulate_and_extra_feed_counters() {
        let s = mk_stats();
        let mut st = FeeRateEstimatorState::new(s.buckets().len() - 1);

        // Take period_target=1 (index 0)
        st.accumulate_from_stats(&s, 1, 2);
        assert_eq!(*st.n_conf(),    5.0);
        assert_eq!(*st.total_num(), 20.0);
        assert_eq!(*st.fail_num(),  0.7);

        // Add extras for conf_target=1, n_block_height=10
        st.add_extra_from_stats(&s, 2, 1, 10, s.unconf_txs().len());
        // The exact value depends on ring pattern but should be positive.
        assert!(*st.extra_num() > 0);
    }

    #[traced_test]
    fn sufficient_and_cur_pct_match_helpers() {
        let mut st = FeeRateEstimatorState::new(1);
        *st.total_num_mut() = 10.0;
        assert!(st.has_sufficient(5.0, 0.5)); // 10 >= 5/(1-0.5) == 10
        *st.n_conf_mut() = 7.0;
        *st.fail_num_mut() = 1.0;
        *st.extra_num_mut() = 2;
        assert_eq!(st.cur_pct(), 7.0 / (10.0 + 1.0 + 2.0));
    }

    #[traced_test]
    fn failure_and_passing_transitions_snapshot_ranges() {
        let s = mk_stats();
        let mut st = FeeRateEstimatorState::new(s.buckets().len() - 1);

        // Simulate current range [3..2]
        st.set_cur_near_bucket(3);
        st.set_cur_far_bucket(2);
        *st.n_conf_mut() = 9.0;
        *st.total_num_mut() = 10.0;
        *st.fail_num_mut() = 1.0;
        *st.extra_num_mut() = 0;
        st.on_first_failure(&s.buckets());

        let fb = st.fail_bucket();
        assert_eq!(fb.start(), &s.buckets()[1]); // min(2,3)=2 => start=buckets[1]
        assert_eq!(fb.end(),   &s.buckets()[3]);

        // Now simulate a passing reset
        *st.n_conf_mut()    = 5.0;
        *st.total_num_mut() = 5.0;
        *st.fail_num_mut()  = 0.0;
        *st.extra_num_mut() = 0;

        st.set_cur_near_bucket(1);
        st.set_cur_far_bucket(1);
        st.on_passing_reset_and_remember();

        assert!(*st.found_answer());
        assert!(st.is_passing());
        let pb = st.pass_bucket();
        assert_eq!(pb.total_confirmed(), &5.0);
        assert_eq!(st.best_range_minmax(), (1,1));
        assert!(st.new_bucket_range);
        assert_eq!(*st.n_conf(), 0.0); // counters reset
    }

    #[traced_test]
    fn set_pass_range_and_finalize_trailing_failure() {
        let s = mk_stats();
        let mut st = FeeRateEstimatorState::new(s.buckets().len() - 1);

        st.set_pass_range(s.buckets(), 2, 3);
        assert_eq!(st.pass_bucket().start(), &s.buckets()[1]);
        assert_eq!(st.pass_bucket().end(),   &s.buckets()[3]);

        // Simulate "passing until we ran out of data" with a current range [0..1]
        st.set_passing(true);
        st.set_new_bucket_range(false);
        st.set_cur_near_bucket(0);
        st.set_cur_far_bucket(1);
        *st.n_conf_mut() = 2.0;
        *st.total_num_mut() = 4.0;
        *st.fail_num_mut() = 1.0;
        *st.extra_num_mut() = 3;

        st.finalize_trailing_failure(&s.buckets());
        let fb = st.fail_bucket();
        assert_eq!(fb.start(), &0.0);
        assert_eq!(fb.end(),   &s.buckets()[1]);
        assert_eq!(fb.within_target(), &2.0);
        assert_eq!(fb.total_confirmed(), &4.0);
        assert_eq!(fb.left_mempool(), &1.0);
        assert_eq!(fb.in_mempool(), &3.0);
    }
}
