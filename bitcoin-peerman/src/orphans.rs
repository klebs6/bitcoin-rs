crate::ix!();

pub struct PeerOrphans {

    /**
      | Set of txids to reconsider once their
      | parent transactions have been accepted
      | 
      |
      */
    orphan_work_set:        HashSet<u256>,
}

impl Deref for PeerOrphans {

    type Target = HashSet<u256>;

    fn deref(&self) -> &Self::Target {
        &self.orphan_work_set
    }
}

impl DerefMut for PeerOrphans {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.orphan_work_set
    }
}

impl PeerOrphans {
    delegate!{
        to self.orphan_work_set {
            pub fn is_empty(&self) -> bool;
        }
    }
}

pub struct PeerManagerOrphans {

    /**
      | Orphan/conflicted/etc transactions
      | that are kept for compact block reconstruction.
      | 
      | The last -blockreconstructionextratxn/DEFAULT_BLOCK_RECONSTRUCTION_EXTRA_TXN
      | of these are kept in a ring buffer
      | 
      |
      */
    pub extra_txn_for_compact:             Arc<Mutex<Vec<Option<(u256,TransactionRef)>>>>,

    /**
      | Offset into vExtraTxnForCompact to
      | insert the next tx
      | 
      |
      */
    pub extra_txn_for_compact_it:          AtomicUsize, // default = 0
}
