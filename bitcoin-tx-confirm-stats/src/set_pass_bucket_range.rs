// ---------------- [ File: bitcoin-tx-confirm-stats/src/set_pass_bucket_range.rs ]
crate::ix!();

impl TxConfirmStats {

    pub fn set_pass_bucket_range(
        &self,
        pass_bucket: &mut FeeRateEstimatorBucket,
        min_bucket: usize,
        max_bucket: usize,
    ) {
        pass_bucket.set_start(if min_bucket > 0 {
            self.buckets()[min_bucket - 1]
        } else {
            0.0
        });

        pass_bucket.set_end(self.buckets()[max_bucket]);
    }
}

#[cfg(test)]
mod set_pass_bucket_range_spec {
    use super::*;

    #[traced_test]
    fn sets_start_to_prev_edge_or_zero_and_end_to_bucket_edge() {
        let buckets = vec![1.0, 2.0, 3.0];
        let s = TxConfirmStats::new(&buckets, &Default::default(), 1, 0.0, 1);

        let mut pass = FeeRateEstimatorBucket::default();
        s.set_pass_bucket_range(&mut pass, 0, 1);
        assert_eq!(pass.start(), &0.0);
        assert_eq!(pass.end(),   &2.0);

        s.set_pass_bucket_range(&mut pass, 2, 2);
        assert_eq!(pass.start(), &buckets[1]);
        assert_eq!(pass.end(),   &buckets[2]);
    }
}
