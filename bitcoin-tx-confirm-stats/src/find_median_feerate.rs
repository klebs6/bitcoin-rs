// ---------------- [ File: bitcoin-tx-confirm-stats/src/find_median_feerate.rs ]
crate::ix!();

impl TxConfirmStats {

    pub fn find_median_feerate(
        &self,
        min_bucket: usize,
        max_bucket: usize,
        mut tx_sum: f64,
    ) -> f64 {
        // No transactions in this range => no median
        if tx_sum == 0.0 {
            return 0.0;
        }

        tx_sum /= 2.0;
        for j in min_bucket..=max_bucket {
            let tx_ct = self.tx_ct_avg()[j];
            if tx_ct < tx_sum {
                // Consume this bucket and keep searching
                tx_sum -= tx_ct;
            } else {
                // We're in the median-containing bucket; avoid div-by-zero just in case
                if tx_ct == 0.0 {
                    continue;
                }
                return self.feerate_avg()[j] / tx_ct;
            }
        }
        0.0
    }

}

#[cfg(test)]
mod find_median_feerate_spec {
    use super::*;

    fn mk_stats() -> TxConfirmStats {
        let buckets = vec![1.0, 2.0, 3.0, 4.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 1, 0.0, 1);
        s.set_tx_ct_avg(vec![2.0, 1.0, 3.0, 4.0]);
        s.set_feerate_avg(vec![2.0, 5.0, 30.0, 100.0]);
        s
    }

    #[traced_test]
    fn picks_bucket_containing_median_transaction() {
        let s = mk_stats();
        let min_bucket = 1;
        let max_bucket = 3;

        let tx_sum = s.sum_tx_ct_avg(min_bucket, max_bucket); // 1 + 3 + 0? (1,2,3) actually = 1 + 3 + ??? Wait:
        // tx_ct_avg[1..=3] = [1, 3, 4]? From mk_stats(): [2,1,3,4] -> indexes 1..3: [1,3,4]; sum=8
        let median = s.find_median_feerate(min_bucket, max_bucket, tx_sum);

        // Half = 4; iterate j=1: tx[1]=1 <4 => rem=3; j=2: tx[2]=3 !<3 => return feerate_avg[2]/tx_ct_avg[2] = 30/3 = 10
        assert_eq!(median, 10.0);
    }

    #[traced_test]
    fn returns_zero_when_no_bucket_contains_median() {
        let mut s = mk_stats();
        // Make selected range empty wrt tx_ct_avg
        s.tx_ct_avg_mut()[1] = 0.0;
        s.tx_ct_avg_mut()[2] = 0.0;

        let sum = s.sum_tx_ct_avg(1, 2);
        assert_eq!(sum, 0.0);
        assert_eq!(s.find_median_feerate(1, 2, sum), 0.0);
    }
}
