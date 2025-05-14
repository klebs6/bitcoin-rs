// ---------------- [ File: bitcoin-peerman/src/process_block_availability.rs ]
crate::ix!();

pub trait ProcessBlockAvailability {
    
    fn process_block_availability(self: Arc<Self>, nodeid: NodeId);
}

impl ProcessBlockAvailability for PeerManager {

    /**
      | Check whether the last unknown block
      | a peer advertised is not yet known.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn process_block_availability(self: Arc<Self>, nodeid: NodeId)  {
        
        let state: Amo<NodeState> = create_state(nodeid);

        assert!(state.is_some());

        if !state.get().hash_last_unknown_block.is_null() {

            let pindex: Option<Arc<BlockIndex>> 
            = self.chainman.get().inner.blockman.lookup_block_index(
                &state.get().hash_last_unknown_block
            );

            if pindex.is_some() && pindex.as_ref().unwrap().n_chain_work > ArithU256::from(0) {

                if state.get().pindex_best_known_block.is_none() 
                || pindex.as_ref().unwrap().n_chain_work >= state.get().pindex_best_known_block.as_ref().unwrap().n_chain_work 
                {
                    state.get_mut().pindex_best_known_block = pindex;
                }

                state.get_mut().hash_last_unknown_block.set_null();
            }
        }
    }
}
