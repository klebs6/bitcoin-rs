crate::ix!();

/**
  | DisconnectedBlockTransactions
  | 
  | During the reorg, it's desirable to
  | re-add previously confirmed transactions
  | to the mempool, so that anything not
  | re-confirmed in the new chain is available
  | to be mined. However, it's more efficient
  | to wait until the reorg is complete and
  | process all still-unconfirmed transactions
  | at that time, since we expect most confirmed
  | transactions to (typically) still
  | be confirmed in the new chain, and re-accepting
  | to the memory pool is expensive (and
  | therefore better to not do in the middle
  | of reorg-processing).
  | 
  | Instead, store the disconnected transactions
  | (in order!) as we go, remove any that
  | are included in blocks in the new chain,
  | and then process the remaining still-unconfirmed
  | transactions at the end.
  |
  */
pub struct DisconnectedBlockTransactions {
    queued_tx:          disconnected_block_transactions::IndexedDisconnectedTransactions,
    cached_inner_usage: u64, // default = 0
}

pub mod disconnected_block_transactions {
    use super::*;

    pub type IndexedDisconnectedTransactions = Broken;
    pub type IndexedDisconnectedTransactionsIndexInsertionOrderIterator = Broken;

    lazy_static!{
        /*
        typedef boost::multi_index_container<
                CTransactionRef,
                boost::multi_index::indexed_by<
                    // sorted by txid
                    boost::multi_index::hashed_unique<
                        boost::multi_index::tag<txid_index>,
                        mempoolentry_txid,
                        SaltedTxidHasher
                    >,
                    // sorted by order in the blockchain
                    boost::multi_index::sequenced<
                        boost::multi_index::tag<insertion_order>
                    >
                >
            > indexed_disconnected_transactions;
        */
    }
}

impl Drop for DisconnectedBlockTransactions {

    /**
      | It's almost certainly a logic bug if we
      | don't clear out queuedTx before
      | destruction, as we add to it while
      | disconnecting blocks, and then we need to
      | re-process remaining transactions to ensure
      | mempool consistency.
      |
      | For now, assert() that we've emptied out
      | this object on destruction.  This assert()
      | can always be removed if the
      | reorg-processing code were to be refactored
      | such that this assumption is no longer true
      | (for instance if there was some other way
      | we cleaned up the mempool after a reorg,
      | besides draining this object).
      */
    fn drop(&mut self) {
        todo!();
        /*
            assert(queuedTx.empty());
        */
    }
}

impl DisconnectedBlockTransactions {

    /**
      | Estimate the overhead of queuedTx to be
      | 6 pointers + an allocation, as no exact
      | formula for boost::multi_index_contained is
      | implemented.
      */
    pub fn dynamic_memory_usage(&self) -> usize {
        
        todo!();
        /*
            return memusage::MallocUsage(sizeof(CTransactionRef) + 6 * sizeof(c_void*)) * queuedTx.size() + cachedInnerUsage;
        */
    }
    
    pub fn add_transaction(&mut self, tx: &TransactionRef)  {
        
        todo!();
        /*
            queuedTx.insert(tx);
            cachedInnerUsage += RecursiveDynamicUsage(tx);
        */
    }

    /**
      | Remove entries based on txid_index,
      | and update memory usage.
      |
      */
    pub fn remove_for_block(&mut self, vtx: &Vec<TransactionRef>)  {
        
        todo!();
        /*
            // Short-circuit in the common case of a block being added to the tip
            if (queuedTx.empty()) {
                return;
            }
            for (auto const &tx : vtx) {
                auto it = queuedTx.find(tx->GetHash());
                if (it != queuedTx.end()) {
                    cachedInnerUsage -= RecursiveDynamicUsage(*it);
                    queuedTx.erase(it);
                }
            }
        */
    }

    /**
      | Remove an entry by insertion_order
      | index, and update memory usage.
      |
      */
    pub fn remove_entry(&mut self, 
        //entry: disconnected_block_transactions::IndexedDisconnectedTransactions::Index<InsertionOrder>::Iterator)  {
        entry: disconnected_block_transactions::IndexedDisconnectedTransactionsIndexInsertionOrderIterator)  {
        
        todo!();
        /*
            cachedInnerUsage -= RecursiveDynamicUsage(*entry);
            queuedTx.get<insertion_order>().erase(entry);
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            cachedInnerUsage = 0;
            queuedTx.clear();
        */
    }
}
