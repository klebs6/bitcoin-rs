crate::ix!();

pub trait MaybePunishNodeForBlock {

    fn maybe_punish_node_for_block(&self, 
        nodeid:            NodeId,
        state:             &BlockValidationState,
        via_compact_block: bool,
        message:           Option<&str>) -> bool;
}
    
impl MaybePunishNodeForBlock for PeerManager {

    /**
      | Potentially mark a node discouraged
      | based on the contents of a BlockValidationState
      | object
      | 
      | -----------
      | @param[in] via_compact_block
      | 
      | this bool is passed in because net_processing
      | should punish peers differently depending
      | on whether the data was provided in a
      | compact block message or not. If the
      | compact block had a valid header, but
      | contained invalid txs, the peer should
      | not be punished. See BIP 152.
      | 
      | -----------
      | @return
      | 
      | Returns true if the peer was punished
      | (probably disconnected)
      |
      */
    fn maybe_punish_node_for_block(&self, 
        nodeid:            NodeId,
        state:             &BlockValidationState,
        via_compact_block: bool,
        message:           Option<&str>) -> bool {

        let message: &str = message.unwrap_or("");
        
        match state.get_result() {

            BlockValidationResult::BLOCK_RESULT_UNSET  => { },

            BlockValidationResult::BLOCK_CONSENSUS 
                | BlockValidationResult::BLOCK_MUTATED => {

                //  The node is providing invalid data:
                if !via_compact_block {
                    self.misbehaving(nodeid, 100, message);
                    return true;
                }
            },

            BlockValidationResult::BLOCK_CACHED_INVALID  => {

                let mut guard = CS_MAIN.lock();

                let node_state: Amo<NodeState> = create_state(nodeid);

                if node_state.is_some() {

                    // Discourage outbound (but not
                    // inbound) peers if on an invalid
                    // chain.
                    //
                    // Exempt HB compact block
                    // peers. Manual connections are
                    // always protected from
                    // discouragement.
                    if !via_compact_block && !node_state.get().is_inbound.load(atomic::Ordering::Relaxed) {
                        self.misbehaving(nodeid, 100, message);
                        return true;
                    }
                }
            },

            BlockValidationResult::BLOCK_INVALID_HEADER  
                | BlockValidationResult::BLOCK_CHECKPOINT 
                | BlockValidationResult::BLOCK_INVALID_PREV => {

                self.misbehaving(nodeid, 100, message);
                return true;
            },

            // Conflicting (but not necessarily
            // invalid) data or different policy:
            BlockValidationResult::BLOCK_MISSING_PREV  => {

                // TODO: Handle this much more
                // gracefully (10 DoS points is
                // super arbitrary)
                self.misbehaving(nodeid,10,message);
                return true;
            },

            BlockValidationResult::BLOCK_RECENT_CONSENSUS_CHANGE  
                | BlockValidationResult::BLOCK_TIME_FUTURE => { },
        }

        if message != "" {
            log_print!(LogFlags::NET, "peer=%d: %s\n", nodeid, message);
        }

        false
    }
}
