crate::ix!();

impl BlockChecked for PeerManager {

    /**
      | Handle invalid block rejection and
      | consequent peer discouragement, maintain
      | which peers announce compact blocks.
      |
      */
    fn block_checked(self: Arc<Self>, 
        block: &Block,
        state: &BlockValidationState)  {

        let mut guard = CS_MAIN.lock();

        let hash: u256 = block.get_hash();

        let cself = self.clone();

        let mut inner = self.inner.lock();

        let it = inner.map_block_source.get(&hash);

        // If the block failed validation, we know
        // where it came from and we're still
        // connected to that peer, maybe punish.
        if state.is_invalid() 
        && it != None 
        && create_state(it.as_ref().unwrap().0).is_some() {

            cself.maybe_punish_node_for_block(

                /*nodeid=*/ 
                it.as_ref().unwrap().0, 

                state, 

                /*via_compact_block=*/ 
                !it.as_ref().unwrap().1,

                None
            );

        } else {

            let mbif = inner.map_blocks_in_flight.lock();

            if state.is_valid() 
            && !cself.chainman.get().active_chainstate().is_initial_block_download() 
            && (mbif.get(&hash).is_some() == (mbif.len() == 1))
            {

                if it.is_some() {

                    cself.maybe_set_peer_as_announcing_header_and_ids(
                        it.as_ref().unwrap().0
                    );
                }
            }
        }

        if it.is_some() {
            inner.map_block_source.remove(&hash);
        }
    }
}
