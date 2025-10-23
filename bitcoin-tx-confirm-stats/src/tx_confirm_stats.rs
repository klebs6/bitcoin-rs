// ---------------- [ File: bitcoin-tx-confirm-stats/src/tx_confirm_stats.rs ]
crate::ix!();

/// We will instantiate an instance of this class to track transactions that
/// were included in a block. 
///
/// We will lump transactions into a bucket according to their approximate
/// feerate and then track how long it took for those txs to be included in
/// a block
/// 
/// The tracking of unconfirmed (mempool) transactions is completely independent
/// of the historical tracking of transactions that have been confirmed in
/// a block.
/// 
#[derive(Debug,Getters,Setters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct TxConfirmStats {

    /// Define the buckets we will group transactions into
    /// 
    /// The upper-bound of the range for the bucket (inclusive)
    /// 
    buckets:        Arc<Vec<f64>>,

    /// Map of bucket upper-bound to index into all vectors by bucket
    /// 
    bucket_map:     Arc<HashMap<f64,u32>>,

    /// For each bucket X:
    /// 
    /// Count the total # of txs in each bucket
    /// 
    /// Track the historical moving average of this total over blocks
    /// 
    tx_ct_avg:      Vec<f64>,

    /// Count the total # of txs confirmed within
    /// 
    /// Y blocks in each bucket
    /// 
    /// Track the historical moving average of these totals over blocks
    /// confAvg[Y][X]
    /// 
    conf_avg:       Vec<Vec<f64>>,

    /// Track moving avg of txs which have been evicted from the mempool after
    /// failing to be confirmed within Y blocks failAvg[Y][X]
    /// 
    fail_avg:       Vec<Vec<f64>>,

    /// Sum the total feerate of all tx's in each bucket
    /// 
    /// Track the historical moving average of this total over blocks
    /// 
    feerate_avg:    Vec<f64>,

    /// Combine the conf counts with tx counts to calculate the confirmation
    /// % for each Y,X
    /// 
    /// Combine the total value with the tx counts to calculate the avg feerate
    /// per bucket
    /// 
    decay:          f64,

    /// Resolution (# of blocks) with which confirmations are tracked
    /// 
    scale:          u32,

    /// Mempool counts of outstanding transactions
    /// 
    /// For each bucket X, track the number of transactions in the mempool that
    /// are unconfirmed for each possible confirmation value Y unconfTxs[Y][X]
    ///
    unconf_txs:     Vec<Vec<i32>>,

    /// transactions still unconfirmed after
    /// 
    /// GetMaxConfirms for each bucket
    /// 
    old_unconf_txs: Vec<i32>,
}

impl TxConfirmStats {

    /// Create new TxConfirmStats. 
    ///
    /// This is called by BlockPolicyEstimator's constructor with default
    /// values.
    /// 
    /// -----------
    /// @param defaultBuckets
    /// 
    /// contains the upper limits for the bucket boundaries
    ///
    /// ----------
    /// @param maxPeriods
    /// 
    /// max number of periods to track
    ///
    /// ----------
    /// @param decay
    /// 
    /// how much to decay the historical moving average per block
    ///
    pub fn new(
        default_buckets:    &Vec<f64>,
        // kept for API parity; we compute via buckets
        default_bucket_map: &HashMap<f64, u32>, 
        max_periods:        u32,
        decay:              f64,
        scale:              u32,

    ) -> Self {

        assert!(scale != 0, "_scale must be non-zero");

        let buckets = Arc::new(default_buckets.clone());
        let bucket_map = Arc::new(default_bucket_map.clone());

        let nb = buckets.len();
        let mp = max_periods as usize;

        let mut conf_avg    = vec![vec![0.0f64; nb]; mp];
        let mut fail_avg    = vec![vec![0.0f64; nb]; mp];
        let mut tx_ct_avg   = vec![0.0f64; nb];
        let mut feerate_avg = vec![0.0f64; nb];

        let mut s = Self {
            buckets,
            bucket_map,
            tx_ct_avg,
            conf_avg,
            fail_avg,
            feerate_avg,
            decay,
            scale,
            unconf_txs: vec![],
            old_unconf_txs: vec![],
        };

        s.resize_in_memory_counters(nb);
        s
    }

    /// Return the max number of confirms we're tracking
    /// 
    #[inline]
    pub fn get_max_confirms(&self) -> u32 {
        // return scale * confAvg.size();
        self.scale * (self.conf_avg.len() as u32)
    }
   
    pub fn resize_in_memory_counters(&mut self, newbuckets: usize) {
        // newbuckets must be passed in because the buckets referred to during Read have not been updated yet.
        // unconfTxs.resize(GetMaxConfirms()); each row has `newbuckets`
        let maxc            = self.get_max_confirms() as usize;
        self.unconf_txs     = vec![vec![0i32; newbuckets]; maxc];
        self.old_unconf_txs = vec![0i32; newbuckets];
    }

    /**
      | Roll the circular buffer for unconfirmed
      | txs
      |
      */
    pub fn clear_current(&mut self, n_block_height: u32) {

        let bins = self.unconf_txs.len();
        let idx  = (n_block_height as usize) % bins;

        for j in 0..self.buckets.len() {
            self.old_unconf_txs[j] += self.unconf_txs[idx][j];
            self.unconf_txs[idx][j] = 0;
        }
    }
    
    /// Record a new transaction data point in the current block stats
    /// 
    /// -----------
    /// @param blocksToConfirm
    /// 
    /// the number of blocks it took this transaction to confirm
    ///
    /// ----------
    /// @param val
    /// 
    /// the feerate of the transaction
    /// 
    /// -----------
    /// @warning
    /// 
    /// blocksToConfirm is 1-based and has to be >= 1
    /// 
    pub fn record(&mut self, blocks_to_confirm: i32, feerate: f64) {
        // blocksToConfirm is 1-based
        if blocks_to_confirm < 1 {
            return;
        }
        let scale = self.scale as i32;
        let periods_to_confirm = (blocks_to_confirm + scale - 1) / scale;

        // bucketindex = bucketMap.lower_bound(feerate)->second;
        let bucketindex = self.bucket_index_for(feerate);

        for i in (periods_to_confirm as usize)..=self.conf_avg.len() {
            self.conf_avg[i - 1][bucketindex] += 1.0;
        }
        self.tx_ct_avg[bucketindex] += 1.0;
        self.feerate_avg[bucketindex] += feerate;
    }

    /// Update our estimates by decaying our historical moving average and
    /// updating with the data gathered from the current block
    /// 
    pub fn update_moving_averages(&mut self) {
        assert_eq!(self.conf_avg.len(), self.fail_avg.len());
        let decay = self.decay;
        for j in 0..self.buckets.len() {
            for i in 0..self.conf_avg.len() {
                self.conf_avg[i][j] *= decay;
                self.fail_avg[i][j] *= decay;
            }
            self.feerate_avg[j] *= decay;
            self.tx_ct_avg[j] *= decay;
        }
    }

    /**
      | Record a new transaction entering the
      | mempool
      |
      */
    pub fn new_tx(&mut self, n_block_height: u32, val: f64) -> u32 {
        // bucketindex = bucketMap.lower_bound(val)->second;  (via buckets)
        let bucketindex = self.bucket_index_for(val) as u32;
        let idx = (n_block_height as usize) % self.unconf_txs.len();
        self.unconf_txs[idx][bucketindex as usize] += 1;
        bucketindex
    }

    #[inline]
    fn bucket_index_for(&self, val: f64) -> usize {
        // first bucket upper-bound >= val; else last bucket
        match self.buckets.iter().position(|&ub| val <= ub) {
            Some(idx) => idx,
            None => self.buckets.len() - 1,
        }
    }
}

#[cfg(test)]
mod tx_confirm_stats_spec {
    use super::*;

    #[traced_test]
    fn new_and_get_max_confirms_and_resize() {
        let buckets = vec![1.0, 2.0, 3.0, 4.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.9, 2);
        assert_eq!(s.get_max_confirms(), 6);
        assert_eq!(s.unconf_txs.len(), s.get_max_confirms() as usize);
        assert_eq!(s.unconf_txs[0].len(), buckets.len());

        // Resize in-memory (same count), just to exercise
        s.resize_in_memory_counters(buckets.len());
        assert_eq!(s.unconf_txs.len(), 6usize);
    }

    #[traced_test]
    fn clear_current_moves_ring_row_into_old() {
        let buckets = vec![1.0, 2.0, 3.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 2, 0.0, 2);
        let bins = s.unconf_txs.len();
        let h = 5u32;
        let idx = (h as usize) % bins;

        s.unconf_txs[idx][0] = 2;
        s.unconf_txs[idx][1] = 3;

        s.clear_current(h);
        assert_eq!(s.unconf_txs[idx][0], 0);
        assert_eq!(s.unconf_txs[idx][1], 0);
        assert_eq!(s.old_unconf_txs[0], 2);
        assert_eq!(s.old_unconf_txs[1], 3);
    }

    #[traced_test]
    fn record_updates_conf_totals_and_feerate_avg() {
        let buckets = vec![1.0, 2.0, 3.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 3, 0.0, 2);

        // bucket_index_for: val <= upper bound chooses first matching
        // Use feerate 2.0 -> bucket 1
        s.record(/*blocks_to_confirm*/ 3, /*feerate*/ 2.0);
        // scale=2 => periods_to_confirm = ceil(3/2) = 2
        // Increment conf_avg[i-1][bucket] for i in 2..=len (i.e., rows 1 and 2)
        assert_eq!(s.conf_avg[0][1], 0.0);
        assert_eq!(s.conf_avg[1][1], 1.0);
        assert_eq!(s.conf_avg[2][1], 1.0);

        assert_eq!(s.tx_ct_avg[1], 1.0);
        assert_eq!(s.feerate_avg[1], 2.0);
    }

    #[traced_test]
    fn update_moving_averages_decays_all_components() {
        let buckets = vec![1.0, 2.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 2, 0.5, 1);
        s.conf_avg    = vec![vec![2.0, 4.0], vec![6.0, 8.0]];
        s.fail_avg    = vec![vec![1.0, 3.0], vec![5.0, 7.0]];
        s.feerate_avg = vec![10.0, 20.0];
        s.tx_ct_avg   = vec![4.0, 8.0];

        s.update_moving_averages();

        assert_eq!(s.conf_avg[0][0], 1.0);
        assert_eq!(s.conf_avg[1][1], 4.0);
        assert_eq!(s.fail_avg[0][1], 1.5);
        assert_eq!(s.feerate_avg[1], 10.0);
        assert_eq!(s.tx_ct_avg[0], 2.0);
    }

    #[traced_test]
    fn new_tx_places_entry_in_current_ring_row() {
        let buckets = vec![1.0, 2.5, 5.0];
        let mut s = TxConfirmStats::new(&buckets, &Default::default(), 2, 0.0, 1);
        let h = 7u32;
        let idx = (h as usize) % s.unconf_txs.len();

        let bucket = s.new_tx(h, 2.4f64);
        assert_eq!(bucket, 1);
        assert_eq!(s.unconf_txs[idx][1], 1);
    }

    #[traced_test]
    fn bucket_index_for_uses_first_upper_bound_or_last_bucket() {
        let buckets = vec![1.0, 2.0, 3.0];
        let s = TxConfirmStats::new(&buckets, &Default::default(), 1, 0.0, 1);

        // <= ub -> pick leftmost matching
        assert_eq!(s.bucket_index_for(0.5), 0);
        assert_eq!(s.bucket_index_for(1.0), 0);
        assert_eq!(s.bucket_index_for(2.0), 1);
        assert_eq!(s.bucket_index_for(10.0), 2); // fallback to last bucket
    }
}
