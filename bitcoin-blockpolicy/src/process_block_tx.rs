crate::ix!();

impl BlockPolicyEstimator {
    
    /**
      | Process a transaction confirmed in
      | a block
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(m_cs_fee_estimator)]
    pub fn process_block_tx(&mut self, 
        n_block_height: u32,
        entry:          *const TxMemPoolEntry) -> bool {
        
        todo!();
        /*
            if (!removeTx(entry->GetTx().GetHash(), true)) {
            // This transaction wasn't being tracked for fee estimation
            return false;
        }

        // How many blocks did it take for miners to include this transaction?
        // blocksToConfirm is 1-based, so a transaction included in the earliest
        // possible block has confirmation count of 1
        int blocksToConfirm = nBlockHeight - entry->GetHeight();
        if (blocksToConfirm <= 0) {
            // This can't happen because we don't process transactions from a block with a height
            // lower than our greatest seen height
            LogPrint(BCLog::ESTIMATEFEE, "Blockpolicy error Transaction had negative blocksToConfirm\n");
            return false;
        }

        // Feerates are stored and reported as BTC-per-kb:
        CFeeRate feeRate(entry->GetFee(), entry->GetTxSize());

        feeStats->Record(blocksToConfirm, (double)feeRate.GetFeePerK());
        shortStats->Record(blocksToConfirm, (double)feeRate.GetFeePerK());
        longStats->Record(blocksToConfirm, (double)feeRate.GetFeePerK());
        return true;
        */
    }
}
