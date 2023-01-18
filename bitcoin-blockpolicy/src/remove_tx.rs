crate::ix!();

impl BlockPolicyEstimator {

    /**
      | Remove a transaction from the mempool
      | tracking stats
      |
      | This function is called from
      | CTxMemPool::removeUnchecked to ensure txs
      | removed from the mempool for any reason are no
      | longer tracked. Txs that were part of a block
      | have already been removed in processBlockTx to
      | ensure they are never double tracked, but it is
      | of no harm to try to remove them again.
      */
    pub fn remove_tx(&mut self, 
        hash:     u256,
        in_block: bool) -> bool {
        
        todo!();
        /*
            LOCK(m_cs_fee_estimator);
        std::map<uint256, TxStatsInfo>::iterator pos = mapMemPoolTxs.find(hash);
        if (pos != mapMemPoolTxs.end()) {
            feeStats->removeTx(pos->second.blockHeight, nBestSeenHeight, pos->second.bucketIndex, inBlock);
            shortStats->removeTx(pos->second.blockHeight, nBestSeenHeight, pos->second.bucketIndex, inBlock);
            longStats->removeTx(pos->second.blockHeight, nBestSeenHeight, pos->second.bucketIndex, inBlock);
            mapMemPoolTxs.erase(hash);
            return true;
        } else {
            return false;
        }
        */
    }
}
