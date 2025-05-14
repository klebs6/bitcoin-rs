// ---------------- [ File: bitcoin-peerman/src/update_block_availability.rs ]
crate::ix!();

pub trait UpdateBlockAvailability {

    fn update_block_availability(self: Arc<Self>, 
        nodeid: NodeId,
        hash:   &u256);
}

impl UpdateBlockAvailability for PeerManager {

    /**
      | Update tracking information about
      | which blocks a peer is assumed to have.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn update_block_availability(self: Arc<Self>, 
        nodeid: NodeId,
        hash:   &u256)  {
        
        let state: Amo<NodeState> = create_state(nodeid);

        assert!(state.is_some());

        self.clone().process_block_availability(nodeid);

        let pindex: Option<Arc<BlockIndex>> 
        = self.chainman.get()
            .inner
            .blockman
            .lookup_block_index(hash);

        if pindex.is_some() && pindex.as_ref().unwrap().n_chain_work > ArithU256::from(0) {

            // An actually better block was
            // announced.
            if state.get().pindex_best_known_block.is_none() 
            || pindex.as_ref().unwrap().n_chain_work >= state.get().pindex_best_known_block.as_ref().unwrap().n_chain_work 
            {
                state.get_mut().pindex_best_known_block = pindex;
            }

        } else {

            // An unknown block was announced;
            // just assume that the latest one is
            // the best one.
            state.get_mut().hash_last_unknown_block = hash.clone();
        }
    }
}
