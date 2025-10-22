// ---------------- [ File: bitcoin-fees/src/filter_fee_rounder.rs ]
crate::ix!();

pub struct FeeFilterRounder {
    feeset:        BTreeSet<f64>,
    insecure_rand: FastRandomContext,
}

pub mod fee_filter_rounder {

    pub const MAX_FILTER_FEERATE: f64 = 1e7;

    /// FEE_FILTER_SPACING is just used to provide some quantization of fee filter results.
    /// 
    /// Historically it reused FEE_SPACING, but it is completely unrelated, and was made a separate
    /// constant so the two concepts are not tied together
    /// 
    pub const FEE_FILTER_SPACING: f64 = 1.1;
}

impl FeeFilterRounder {

    /// Create new FeeFilterRounder
    pub fn new(min_incremental_fee: &FeeRate) -> Self {
        use fee_filter_rounder::{FEE_FILTER_SPACING, MAX_FILTER_FEERATE};
        let mut feeset = BTreeSet::new();

        // CAmount minFeeLimit = std::max(CAmount(1), minIncrementalFee.GetFeePerK() / 2);
        let min_fee_limit: Amount =
            cmp::max(1, min_incremental_fee.get_fee_perk() / 2);

        feeset.insert(0.0);
        let mut bucket = min_fee_limit as f64;
        while bucket <= MAX_FILTER_FEERATE {
            feeset.insert(bucket);
            bucket *= FEE_FILTER_SPACING;
        }

        Self {
            feeset,
            insecure_rand: FastRandomContext::new(),
        }
    }
    
    /// Quantize a minimum fee for privacy purpose before broadcast. 
    ///
    /// Not thread-safe due to use of FastRandomContext
    ///
    pub fn round(&mut self, current_min_fee: Amount) -> Amount {
        // std::set<double>::iterator it = feeset.lower_bound(currentMinFee);
        let cur = current_min_fee as f64;
        let mut chosen = if let Some(&v) = self.feeset.range(cur..).next() {
            v
        } else {
            // end() → step back
            *self.feeset.iter().next_back().expect("feeset not empty")
        };

        // If ((it != begin && rand%3!=0) || it == end) it--;
        let at_begin = self.feeset.iter().next().map(|&x| x == chosen).unwrap_or(true);
        let rand_step = (self.insecure_rand.rand32() % 3) != 0;
        if (!at_begin && rand_step) || !self.feeset.contains(&chosen) {
            if let Some(&prev) = self.feeset.range(..chosen).next_back() {
                chosen = prev;
            }
        }

        chosen as Amount
    }
}
