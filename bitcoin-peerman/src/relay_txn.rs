// ---------------- [ File: bitcoin-peerman/src/relay_txn.rs ]
crate::ix!();

impl RelayTransaction for PeerManager {

    fn relay_transaction(
        self:  Arc<Self>, 
        txid:  &u256,
        wtxid: &u256)  {
        
        let mut guard = CS_MAIN.lock();

        self.clone().relay_transaction_impl(
            txid.clone(), 
            wtxid.clone()
        );
    }
}
