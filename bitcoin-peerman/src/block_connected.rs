crate::ix!();

impl BlockConnected for PeerManager {

    /**
      | Evict orphan txn pool entries based
      | on a newly connected block, remember
      | the recently confirmed transactions,
      | and delete tracked announcements for
      | them. Also save the time of the last tip
      | update.
      |
      */
    fn block_connected(&mut self, 
        pblock: Arc<Block>,
        pindex: Arc<BlockIndex>)  {
        
        self.orphanage.clone().erase_for_block(&*pblock);

        self.last_tip_update.store(Some(get_datetime()), atomic::Ordering::Relaxed);

        {
            let mut guard = self.recent_confirmed_transactions_mutex.get_mut();

            for ptx in (*pblock).vtx.iter() {

                guard.recent_confirmed_transactions.insert_key(
                    ptx.get().get_hash().as_slice()
                );

                if ptx.get().get_hash() != ptx.get().get_witness_hash() {

                    guard.recent_confirmed_transactions.insert_key(
                        ptx.get().get_witness_hash().as_slice()
                    );

                }
            }
        }

        {
            let mut guard = CS_MAIN.lock();

            for ptx in (*pblock).vtx.iter() {

                let guard = self.inner.lock();

                let mut txreq = guard.txrequest.lock();

                txreq.forget_tx_hash(ptx.get().get_hash());
                txreq.forget_tx_hash(ptx.get().get_witness_hash());
            }
        }
    }
}
