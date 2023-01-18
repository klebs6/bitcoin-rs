crate::ix!();

pub trait StartScheduledTasks {

    /**
      | Begin running background tasks, should
      | only be called once
      |
      */
    fn start_scheduled_tasks(self: Arc<Self>, scheduler: Arc<Mutex<Scheduler>>);
}

pub trait IgnoresIncomingTxs {

    /**
      | Whether this node ignores txs received
      | over p2p.
      |
      */
    fn ignores_incoming_txs(&mut self) -> bool;
}

pub trait RelayTransaction {

    /**
      | Relay transaction to all peers.
      |
      */
    fn relay_transaction(
        self:  Arc<Self>,
        txid:  &u256,
        wtxid: &u256);

}

pub trait SendPings {

    /**
      | Send ping message to all peers
      |
      */
    fn send_pings(&mut self);
}

pub trait SetBestHeight {

    /**
      | Set the best height
      |
      */
    fn set_best_height(&mut self, height: i32);
}

pub trait Misbehaving {

    /**
      | Increment peer's misbehavior score.
      | If the new value >= DISCOURAGEMENT_THRESHOLD,
      | mark the node to be discouraged, meaning
      | the peer might be disconnected and added
      | to the discouragement filter.
      | 
      | Public for unit testing.
      |
      */
    fn misbehaving(&self, 
            pnode:   NodeId,
            howmuch: i32,
            message: &str);
}

pub trait CheckForStaleTipAndEvictPeers {

    /**
      | Evict extra outbound peers. If we think
      | our tip may be stale, connect to an extra
      | outbound.
      | 
      | Public for unit testing.
      |
      */
    fn check_for_stale_tip_and_evict_peers(self: Arc<Self>);
}
