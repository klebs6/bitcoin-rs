crate::ix!();

pub trait MaybePunishNodeForTx {

    fn maybe_punish_node_for_tx(self: Arc<Self>, 
        nodeid:  NodeId,
        state:   &TxValidationState,
        message: Option<&str>) -> bool;
}

impl MaybePunishNodeForTx for PeerManager {

    /**
      | Potentially disconnect and discourage
      | a node based on the contents of a TxValidationState
      | object
      | 
      | 
      | -----------
      | @return
      | 
      | Returns true if the peer was punished
      | (probably disconnected)
      |
      */
    fn maybe_punish_node_for_tx(self: Arc<Self>, 
        nodeid:  NodeId,
        state:   &TxValidationState,
        message: Option<&str>) -> bool {

        let message: &str = message.unwrap_or("");
        
        match state.get_result() {
            TxValidationResult::TX_RESULT_UNSET  => { },

            TxValidationResult::TX_CONSENSUS  => {
                //  The node is providing invalid data:
                self.misbehaving(nodeid,100,message);
                return true;
            },

            // Conflicting (but not necessarily
            // invalid) data or different policy:
            TxValidationResult::TX_RECENT_CONSENSUS_CHANGE  
                | TxValidationResult::TX_INPUTS_NOT_STANDARD
                | TxValidationResult::TX_NOT_STANDARD
                | TxValidationResult::TX_MISSING_INPUTS
                | TxValidationResult::TX_PREMATURE_SPEND
                | TxValidationResult::TX_WITNESS_MUTATED
                | TxValidationResult::TX_WITNESS_STRIPPED
                | TxValidationResult::TX_CONFLICT
                | TxValidationResult::TX_MEMPOOL_POLICY
                => { },
        }

        if message != "" {
            log_print!(LogFlags::NET, "peer=%d: %s\n", nodeid, message);
        }

        false
    }
}
