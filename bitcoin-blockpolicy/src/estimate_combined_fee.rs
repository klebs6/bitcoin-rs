// ---------------- [ File: bitcoin-blockpolicy/src/estimate_combined_fee.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Helper for estimateSmartFee
      |
      | Return a fee estimate at the required
      | successThreshold from the shortest
      | time horizon which tracks confirmations
      | up to the desired target. If checkShorterHorizon
      | is requested, also allow short time
      | horizon estimates for a lower target
      | to reduce the given answer
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn estimate_combined_fee(&self, 
        conf_target:           u32,
        success_threshold:     f64,
        check_shorter_horizon: bool,
        result:                *mut EstimationResult) -> f64 {
        
        todo!();
        /*
            double estimate = -1;
        if (confTarget >= 1 && confTarget <= longStats->GetMaxConfirms()) {
            // Find estimate from shortest time horizon possible
            if (confTarget <= shortStats->GetMaxConfirms()) { // short horizon
                estimate = shortStats->EstimateMedianVal(confTarget, SUFFICIENT_TXS_SHORT, successThreshold, nBestSeenHeight, result);
            }
            else if (confTarget <= feeStats->GetMaxConfirms()) { // medium horizon
                estimate = feeStats->EstimateMedianVal(confTarget, SUFFICIENT_FEETXS, successThreshold, nBestSeenHeight, result);
            }
            else { // long horizon
                estimate = longStats->EstimateMedianVal(confTarget, SUFFICIENT_FEETXS, successThreshold, nBestSeenHeight, result);
            }
            if (checkShorterHorizon) {
                EstimationResult tempResult;
                // If a lower confTarget from a more recent horizon returns a lower answer use it.
                if (confTarget > feeStats->GetMaxConfirms()) {
                    double medMax = feeStats->EstimateMedianVal(feeStats->GetMaxConfirms(), SUFFICIENT_FEETXS, successThreshold, nBestSeenHeight, &tempResult);
                    if (medMax > 0 && (estimate == -1 || medMax < estimate)) {
                        estimate = medMax;
                        if (result) *result = tempResult;
                    }
                }
                if (confTarget > shortStats->GetMaxConfirms()) {
                    double shortMax = shortStats->EstimateMedianVal(shortStats->GetMaxConfirms(), SUFFICIENT_TXS_SHORT, successThreshold, nBestSeenHeight, &tempResult);
                    if (shortMax > 0 && (estimate == -1 || shortMax < estimate)) {
                        estimate = shortMax;
                        if (result) *result = tempResult;
                    }
                }
            }
        }
        return estimate;
        */
    }
}
