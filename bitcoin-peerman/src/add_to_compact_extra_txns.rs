// ---------------- [ File: bitcoin-peerman/src/add_to_compact_extra_txns.rs ]
crate::ix!();

pub trait AddtoCompactExtraTransactions {

    fn add_to_compact_extra_transactions(self: Arc<Self>, tx: &TransactionRef);
}

impl AddtoCompactExtraTransactions for PeerManager {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(G_CS_ORPHANS)]
    fn add_to_compact_extra_transactions(self: Arc<Self>, tx: &TransactionRef)  {
        
        let max_extra_txn: usize = G_ARGS.lock()
            .get_int_arg(
                "-blockreconstructionextratxn", 
                DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN.into()
            ).try_into().unwrap();

        if max_extra_txn <= 0 {
            return;
        }

        let mut guard = self.orphan_data.extra_txn_for_compact.lock();

        if guard.len() == 0 {

            guard.resize(
                max_extra_txn, 
                None
            );
        }

        let idx: usize = self.orphan_data.extra_txn_for_compact_it.load(atomic::Ordering::Relaxed);

        guard[idx] = 
            Some( 
                ((*tx).get().get_witness_hash().clone(), tx.clone()) 
            );

        self.orphan_data.extra_txn_for_compact_it.store(
            (self.orphan_data.extra_txn_for_compact_it.load(atomic::Ordering::Relaxed) + 1) % max_extra_txn, 
            atomic::Ordering::Relaxed
        );
    }
}
