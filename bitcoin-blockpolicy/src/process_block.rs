// ---------------- [ File: bitcoin-blockpolicy/src/process_block.rs ]
crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Process all the transactions that have
      | been included in a block
      |
      */
    pub fn process_block(
        self:           Arc<Self>, 
        n_block_height: u32,
        entries:        &mut Vec<*const TxMemPoolEntry>)  {
        
        todo!();
        /*
            LOCK(m_cs_fee_estimator);
        if (nBlockHeight <= nBestSeenHeight) {
            // Ignore side chains and re-orgs; assuming they are random
            // they don't affect the estimate.
            // And if an attacker can re-org the chain at will, then
            // you've got much bigger problems than "attacker can influence
            // transaction fees."
            return;
        }

        // Must update nBestSeenHeight in sync with ClearCurrent so that
        // calls to removeTx (via processBlockTx) correctly calculate age
        // of unconfirmed txs to remove from tracking.
        nBestSeenHeight = nBlockHeight;

        // Update unconfirmed circular buffer
        feeStats->ClearCurrent(nBlockHeight);
        shortStats->ClearCurrent(nBlockHeight);
        longStats->ClearCurrent(nBlockHeight);

        // Decay all exponential averages
        feeStats->UpdateMovingAverages();
        shortStats->UpdateMovingAverages();
        longStats->UpdateMovingAverages();

        unsigned int countedTxs = 0;
        // Update averages with data points from current block
        for (const auto& entry : entries) {
            if (processBlockTx(nBlockHeight, entry))
                countedTxs++;
        }

        if (firstRecordedHeight == 0 && countedTxs > 0) {
            firstRecordedHeight = nBestSeenHeight;
            LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy first recorded height %u\n", firstRecordedHeight);
        }

        LogPrint(LogFlags::ESTIMATEFEE, "Blockpolicy estimates updated by %u of %u block txs, since last block %u of %u tracked, mempool map size %u, max target %u from %s\n",
                 countedTxs, entries.size(), trackedTxs, trackedTxs + untrackedTxs, mapMemPoolTxs.size(),
                 MaxUsableEstimate(), HistoricalBlockSpan() > BlockSpan() ? "historical" : "current");

        trackedTxs = 0;
        untrackedTxs = 0;
        */
    }
}
