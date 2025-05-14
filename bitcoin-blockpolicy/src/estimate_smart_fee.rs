// ---------------- [ File: bitcoin-blockpolicy/src/estimate_smart_fee.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Estimate feerate needed to get be included
      | in a block within confTarget blocks.
      | If no answer can be given at confTarget,
      | return an estimate at the closest target
      | where one can be given. 'conservative'
      | estimates are valid over longer time
      | horizons also.
      |
      | estimateSmartFee returns the max of
      | the feerates calculated with a 60% threshold
      | required at target / 2, an 85% threshold
      | required at target and a 95% threshold
      | required at 2 * target.
      | 
      | Each calculation is performed at the
      | shortest time horizon which tracks
      | the required target.
      | 
      | Conservative estimates, however,
      | required the 95% threshold at 2 * target
      | be met for any longer time horizons also.
      |
      */
    pub fn estimate_smart_fee(&self, 
        conf_target:  i32,
        fee_calc:     *mut FeeCalculation,
        conservative: bool) -> FeeRate {
        
        todo!();
        /*
            LOCK(m_cs_fee_estimator);

        if (feeCalc) {
            feeCalc->desiredTarget = confTarget;
            feeCalc->returnedTarget = confTarget;
        }

        double median = -1;
        EstimationResult tempResult;

        // Return failure if trying to analyze a target we're not tracking
        if (confTarget <= 0 || (unsigned int)confTarget > longStats->GetMaxConfirms()) {
            return CFeeRate(0);  // error condition
        }

        // It's not possible to get reasonable estimates for confTarget of 1
        if (confTarget == 1) confTarget = 2;

        unsigned int maxUsableEstimate = MaxUsableEstimate();
        if ((unsigned int)confTarget > maxUsableEstimate) {
            confTarget = maxUsableEstimate;
        }
        if (feeCalc) feeCalc->returnedTarget = confTarget;

        if (confTarget <= 1) return CFeeRate(0); // error condition

        assert(confTarget > 0); //estimateCombinedFee and estimateConservativeFee take unsigned ints
        /** true is passed to estimateCombined fee for target/2 and target so
         * that we check the max confirms for shorter time horizons as well.
         * This is necessary to preserve monotonically increasing estimates.
         * For non-conservative estimates we do the same thing for 2*target, but
         * for conservative estimates we want to skip these shorter horizons
         * checks for 2*target because we are taking the max over all time
         * horizons so we already have monotonically increasing estimates and
         * the purpose of conservative estimates is not to let short term
         * fluctuations lower our estimates by too much.
         */
        double halfEst = estimateCombinedFee(confTarget/2, HALF_SUCCESS_PCT, true, &tempResult);
        if (feeCalc) {
            feeCalc->est = tempResult;
            feeCalc->reason = FeeReason::HALF_ESTIMATE;
        }
        median = halfEst;
        double actualEst = estimateCombinedFee(confTarget, SUCCESS_PCT, true, &tempResult);
        if (actualEst > median) {
            median = actualEst;
            if (feeCalc) {
                feeCalc->est = tempResult;
                feeCalc->reason = FeeReason::FULL_ESTIMATE;
            }
        }
        double doubleEst = estimateCombinedFee(2 * confTarget, DOUBLE_SUCCESS_PCT, !conservative, &tempResult);
        if (doubleEst > median) {
            median = doubleEst;
            if (feeCalc) {
                feeCalc->est = tempResult;
                feeCalc->reason = FeeReason::DOUBLE_ESTIMATE;
            }
        }

        if (conservative || median == -1) {
            double consEst =  estimateConservativeFee(2 * confTarget, &tempResult);
            if (consEst > median) {
                median = consEst;
                if (feeCalc) {
                    feeCalc->est = tempResult;
                    feeCalc->reason = FeeReason::CONSERVATIVE;
                }
            }
        }

        if (median < 0) return CFeeRate(0); // error condition

        return CFeeRate(llround(median));
        */
    }
}
