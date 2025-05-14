// ---------------- [ File: bitcoin-blockpolicy/src/highest_target_tracked.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Calculation of highest target that
      | estimates are tracked for
      |
      */
    pub fn highest_target_tracked(&self, horizon: FeeEstimateHorizon) -> u32 {
        
        let guard = self.cs_fee_estimator.lock();

        return match horizon {
            FeeEstimateHorizon::SHORT_HALFLIFE => guard.short_stats.get_max_confirms(),
            FeeEstimateHorizon::MED_HALFLIFE   => guard.fee_stats.get_max_confirms(),
            FeeEstimateHorizon::LONG_HALFLIFE  => guard.long_stats.get_max_confirms(),
        }
    }
}
