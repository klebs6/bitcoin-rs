crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Number of blocks of data recorded while
      | fee estimates have been running
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn block_span(&self) -> u32 {
        
        todo!();
        /*
            if (firstRecordedHeight == 0) return 0;
        assert(nBestSeenHeight >= firstRecordedHeight);

        return nBestSeenHeight - firstRecordedHeight;
        */
    }
    
    /**
      | Number of blocks of recorded fee estimate
      | data represented in saved data file
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn historical_block_span(&self) -> u32 {
        
        todo!();
        /*
            if (historicalFirst == 0) return 0;
        assert(historicalBest >= historicalFirst);

        if (nBestSeenHeight - historicalBest > OLDEST_ESTIMATE_HISTORY) return 0;

        return historicalBest - historicalFirst;
        */
    }
}
