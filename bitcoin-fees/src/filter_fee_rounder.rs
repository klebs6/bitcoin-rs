// ---------------- [ File: bitcoin-fees/src/filter_fee_rounder.rs ]
crate::ix!();

pub struct FeeFilterRounder {
    feeset:        BTreeSet<FloatOrd<f64>>,
    insecure_rand: FastRandomContext,
}

pub mod fee_filter_rounder {
    use super::*;

    pub const MAX_FILTER_FEERATE: FloatOrd<f64> = FloatOrd(1e7);

    /// FEE_FILTER_SPACING is just used to provide some quantization of fee filter results.
    /// Historically it reused FEE_SPACING, but it is completely unrelated, and was made
    /// a separate constant so the two concepts are not tied together.
    pub const FEE_FILTER_SPACING: FloatOrd<f64> = FloatOrd(1.1);
}

impl FeeFilterRounder {
    /// Create new FeeFilterRounder
    pub fn new(min_incremental_fee: &FeeRate) -> Self {
        use fee_filter_rounder::{FEE_FILTER_SPACING, MAX_FILTER_FEERATE};

        let mut feeset: BTreeSet<FloatOrd<f64>> = BTreeSet::new();

        // CAmount minFeeLimit = std::max(CAmount(1), minIncrementalFee.GetFeePerK() / 2);
        let min_fee_limit: Amount = std::cmp::max(1, min_incremental_fee.get_fee_perk() / 2);

        // Always include 0
        feeset.insert(FloatOrd(0.0));

        // Geometric series from min_fee_limit up to MAX_FILTER_FEERATE
        let mut bucket = FloatOrd(min_fee_limit as f64);
        while bucket <= MAX_FILTER_FEERATE {
            feeset.insert(bucket);
            bucket = FloatOrd(bucket.0 * FEE_FILTER_SPACING.0);
        }

        Self {
            feeset,
            insecure_rand: FastRandomContext::new(false),
        }
    }

    /// Quantize a minimum fee for privacy purpose before broadcast.
    /// Not thread-safe due to use of FastRandomContext
    pub fn round(&mut self, current_min_fee: Amount) -> Amount {
        // lower_bound(current_min_fee)
        let cur = FloatOrd(current_min_fee as f64);
        let mut iter = self.feeset.range(cur..);
        let lower = iter.next().copied();
        let at_end = lower.is_none();

        // If end(), choose last element (we will step back unconditionally below).
        let mut chosen = lower
            .unwrap_or_else(|| *self.feeset.iter().next_back().expect("feeset not empty"));

        // If ((it != begin && rand%3!=0) || it == end) it--;
        let at_begin = self
            .feeset
            .iter()
            .next()
            .map(|&first| first == chosen)
            .unwrap_or(true);
        let rand_step_back = (self.insecure_rand.rand32() % 3) != 0;

        if at_end || (!at_begin && rand_step_back) {
            if let Some(prev) = self.feeset.range(..chosen).next_back().copied() {
                chosen = prev;
            }
        }

        chosen.0 as Amount
    }
}
