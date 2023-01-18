crate::ix!();

pub trait AlreadyHaveTx {

    fn already_have_tx(self: Arc<Self>, gtxid: &GenTxId) -> bool;
}

impl AlreadyHaveTx for PeerManager {

    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn already_have_tx(self: Arc<Self>, gtxid: &GenTxId) -> bool {
        
        if self.chainman.get().active_chain().tip().as_ref().unwrap().get_block_hash() 
        != self.inner.lock().hash_recent_rejects_chain_tip {

            // If the chain tip has changed
            // previously rejected transactions
            // might be now valid, e.g. due to
            // a nLockTime'd tx becoming valid,
            // or a double-spend. Reset the
            // rejects filter and give those txs
            // a second chance.
            self.inner.lock().hash_recent_rejects_chain_tip = 
                self.chainman.get().active_chain().tip().as_ref().unwrap().get_block_hash();

            self.inner.lock().recent_rejects.reset();
        }

        let hash: &u256 = gtxid.get_hash();

        if self.orphanage.have_tx(gtxid) {
            return true;
        }

        {
            let mut guard = self.recent_confirmed_transactions_mutex.get();

            if guard.recent_confirmed_transactions.contains_key(hash.as_slice()) {
                return true;
            }
        }

        self.inner.lock().recent_rejects.contains_key(hash.as_slice()) || self.mempool.get().exists(gtxid)
    }
}
