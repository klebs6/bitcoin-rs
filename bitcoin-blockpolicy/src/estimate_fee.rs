crate::ix!();

impl BlockPolicyEstimator {

    /**
      | DEPRECATED. Return a feerate estimate
      |
      */
    pub fn estimate_fee(&self, conf_target: i32) -> FeeRate {
        
        todo!();
        /*
            // It's not possible to get reasonable estimates for confTarget of 1
        if (confTarget <= 1)
            return CFeeRate(0);

        return estimateRawFee(confTarget, DOUBLE_SUCCESS_PCT, FeeEstimateHorizon::MED_HALFLIFE);
        */
    }
}
