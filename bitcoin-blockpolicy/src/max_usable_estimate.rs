crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Calculation of highest target that
      | reasonable estimate can be provided
      | for
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn max_usable_estimate(&self) -> u32 {
        
        todo!();
        /*
            // Block spans are divided by 2 to make sure there are enough potential failing data points for the estimate
        return std::min(longStats->GetMaxConfirms(), std::max(BlockSpan(), HistoricalBlockSpan()) / 2);
        */
    }
}
