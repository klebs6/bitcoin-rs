crate::ix!();

impl BlockPolicyEstimator {
    
    /**
      | Return a specific fee estimate calculation
      | with a given success threshold and time
      | horizon, and optionally return detailed
      | data about calculation
      |
      */
    pub fn estimate_raw_fee(&self, 
        conf_target:       i32,
        success_threshold: f64,
        horizon:           FeeEstimateHorizon,
        result:            *mut EstimationResult) -> FeeRate {
        
        todo!();
        /*
            TxConfirmStats* stats = nullptr;
        double sufficientTxs = SUFFICIENT_FEETXS;
        switch (horizon) {
        case FeeEstimateHorizon::SHORT_HALFLIFE: {
            stats = shortStats.get();
            sufficientTxs = SUFFICIENT_TXS_SHORT;
            break;
        }
        case FeeEstimateHorizon::MED_HALFLIFE: {
            stats = feeStats.get();
            break;
        }
        case FeeEstimateHorizon::LONG_HALFLIFE: {
            stats = longStats.get();
            break;
        }
        } // no default case, so the compiler can warn about missing cases
        assert(stats);

        LOCK(m_cs_fee_estimator);
        // Return failure if trying to analyze a target we're not tracking
        if (confTarget <= 0 || (unsigned int)confTarget > stats->GetMaxConfirms())
            return CFeeRate(0);
        if (successThreshold > 1)
            return CFeeRate(0);

        double median = stats->EstimateMedianVal(confTarget, sufficientTxs, successThreshold, nBestSeenHeight, result);

        if (median < 0)
            return CFeeRate(0);

        return CFeeRate(llround(median));
        */
    }
}
