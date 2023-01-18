crate::ix!();

pub trait FindTxForGetData {

    fn find_tx_for_get_data(self: Arc<Self>, 
        peer:        &dyn NodeInterface,
        gtxid:       &GenTxId,
        mempool_req: Option<OffsetDateTime> /* seconds */,
        now:         OffsetDateTime /* seconds */) -> TransactionRef;
}

impl FindTxForGetData for PeerManager {

    /**
      | Determine whether or not a peer can request
      | a transaction, and return it (or nullptr
      | if not found or not allowed).
      |
      */
    #[LOCKS_EXCLUDED(CS_MAIN)]
    fn find_tx_for_get_data(
        self:        Arc<Self>, 
        peer:        &dyn NodeInterface,
        gtxid:       &GenTxId,
        mempool_req: Option<OffsetDateTime>,
        now:         OffsetDateTime) -> TransactionRef {

        let txinfo = self.mempool.get().info(gtxid);

        if txinfo.tx.is_some() {

            // If a TX could have been INVed in
            // reply to a MEMPOOL request, or is
            // older than
            // UNCONDITIONAL_RELAY_DELAY, permit
            // the request unconditionally.
            if (mempool_req.is_some() && txinfo.time <= mempool_req.unwrap()) 
            || txinfo.time <= now - UNCONDITIONAL_RELAY_DELAY {
                return txinfo.tx /* move */;
            }
        }

        {
            let mut guard = CS_MAIN.lock();

            let state = create_state(peer.get_id());

            // Otherwise, the transaction must
            // have been announced recently.
            if state.get().recently_announced_invs.contains_key(gtxid.get_hash().as_slice()) {

                // If it was, it can be relayed
                // from either the mempool...
                if txinfo.tx.is_some() {
                    return txinfo.tx /* move */;
                }

                let inner = self.inner.lock();

                //  ... or the relay pool.
                let mi = inner.map_relay.get(gtxid.get_hash());

                if mi.is_some() {
                    return mi.unwrap().clone();
                }
            }
        }

        TransactionRef::none()
    }
}

