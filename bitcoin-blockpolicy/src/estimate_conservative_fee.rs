// ---------------- [ File: bitcoin-blockpolicy/src/estimate_conservative_fee.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Helper for estimateSmartFee
      |
      | Ensure that for a conservative estimate,
      | the
      | 
      | DOUBLE_SUCCESS_PCT is also met at 2
      | * target for any longer time horizons.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn estimate_conservative_fee(&self, 
        double_target: u32,
        result:        *mut EstimationResult) -> f64 {
        
        todo!();
        /*
            double estimate = -1;
        EstimationResult tempResult;
        if (doubleTarget <= shortStats->GetMaxConfirms()) {
            estimate = feeStats->EstimateMedianVal(doubleTarget, SUFFICIENT_FEETXS, DOUBLE_SUCCESS_PCT, nBestSeenHeight, result);
        }
        if (doubleTarget <= feeStats->GetMaxConfirms()) {
            double longEstimate = longStats->EstimateMedianVal(doubleTarget, SUFFICIENT_FEETXS, DOUBLE_SUCCESS_PCT, nBestSeenHeight, &tempResult);
            if (longEstimate > estimate) {
                estimate = longEstimate;
                if (result) *result = tempResult;
            }
        }
        return estimate;
        */
    }
}
