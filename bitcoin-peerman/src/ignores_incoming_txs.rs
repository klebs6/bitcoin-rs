crate::ix!();

impl IgnoresIncomingTxs for PeerManager {

    fn ignores_incoming_txs(&mut self) -> bool {
        
        self.ignore_incoming_txs
    }
}
