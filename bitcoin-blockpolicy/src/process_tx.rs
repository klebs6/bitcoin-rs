// ---------------- [ File: bitcoin-blockpolicy/src/process_tx.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Process a transaction accepted to the
      | mempool
      |
      */
    pub fn process_transaction(&mut self, 
        entry:              &TxMemPoolEntry,
        valid_fee_estimate: bool)  {
        
        todo!();
        /*
            LOCK(m_cs_fee_estimator);
        unsigned int txHeight = entry.GetHeight();
        uint256 hash = entry.GetTx().GetHash();
        if (mapMemPoolTxs.count(hash)) {
            LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy error mempool tx %s already being tracked\n",
                     hash.ToString());
            return;
        }

        if (txHeight != nBestSeenHeight) {
            // Ignore side chains and re-orgs; assuming they are random they don't
            // affect the estimate.  We'll potentially double count transactions in 1-block reorgs.
            // Ignore txs if BlockPolicyEstimator is not in sync with ActiveChain().Tip().
            // It will be synced next time a block is processed.
            return;
        }

        // Only want to be updating estimates when our blockchain is synced,
        // otherwise we'll miscalculate how many blocks its taking to get included.
        if (!validFeeEstimate) {
            untrackedTxs++;
            return;
        }
        trackedTxs++;

        // Feerates are stored and reported as BTC-per-kb:
        CFeeRate feeRate(entry.GetFee(), entry.GetTxSize());

        mapMemPoolTxs[hash].blockHeight = txHeight;
        unsigned int bucketIndex = feeStats->NewTx(txHeight, (double)feeRate.GetFeePerK());
        mapMemPoolTxs[hash].bucketIndex = bucketIndex;
        unsigned int bucketIndex2 = shortStats->NewTx(txHeight, (double)feeRate.GetFeePerK());
        assert(bucketIndex == bucketIndex2);
        unsigned int bucketIndex3 = longStats->NewTx(txHeight, (double)feeRate.GetFeePerK());
        assert(bucketIndex == bucketIndex3);
        */
    }
}
