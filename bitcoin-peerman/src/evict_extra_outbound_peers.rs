// ---------------- [ File: bitcoin-peerman/src/evict_extra_outbound_peers.rs ]
crate::ix!();
    
pub trait EvictExtraOutboundPeers {

    fn evict_extra_outbound_peers(
        self:            Arc<Self>, 
        time_in_seconds: OffsetDateTime);
}

impl EvictExtraOutboundPeers for PeerManager {

    /**
      | If we have extra outbound peers, try
      | to disconnect the one with the oldest
      | block announcement
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn evict_extra_outbound_peers(
        self:            Arc<Self>, 
        time_in_seconds: OffsetDateTime)  
    {
        // If we have any extra block-relay-only
        // peers, disconnect the youngest unless
        // it's given us a block -- in which case,
        // compare with the second-youngest, and
        // out of those two, disconnect the peer
        // who least recently gave us a block.
        //
        // The youngest block-relay-only peer
        // would be the extra peer we connected to
        // temporarily in order to sync our tip;
        // see net.cpp.
        //
        // Note that we use higher nodeid as
        // a measure for most recent connection.
        if self.connman.get().get_extra_block_relay_count() > 0 {

            let mut youngest_peer:      (NodeId,i64) = (-1,0);
            let mut next_youngest_peer: (NodeId,i64) = (-1,0);

            let mut maybe_update_youngest_peer = move |pnode: Amo<Box<dyn NodeInterface>>| {

                if !pnode.get().is_block_only_conn() || pnode.get().marked_for_disconnect() {
                    return;
                }

                if pnode.get().get_id() > youngest_peer.0 {
                    next_youngest_peer = youngest_peer;
                    youngest_peer.0 = pnode.get().get_id();
                    youngest_peer.1 = pnode.get().n_last_block_time().unwrap().unix_timestamp();
                }
            };

            self.connman
                .get_mut()
                .for_each_node_mut(&mut maybe_update_youngest_peer);

            let mut to_disconnect: NodeId = youngest_peer.0;

            if youngest_peer.1 > next_youngest_peer.1 {

                // Our newest block-relay-only
                // peer gave us a block more
                // recently; disconnect our second
                // youngest.
                to_disconnect = next_youngest_peer.0;
            }

            let mut maybe_mark_disconnect = move |pnode: Amo<Box<dyn NodeInterface>>| {

                // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
                assert_lock_held!(CS_MAIN);

                //  Make sure we're not getting
                //  a block right now, and that
                //  we've been connected long
                //  enough for this eviction to
                //  happen at all.
                //
                //  Note that we only request
                //  blocks from a peer if we learn
                //  of a valid headers chain with
                //  at least as much work as our
                //  tip.
                let node_state: Amo<NodeState> 
                = create_state(pnode.get().get_id());

                if node_state.is_none() 
                || (time_in_seconds - pnode.get().n_time_connected().unwrap() >= MINIMUM_CONNECT_TIME 
                    && node_state.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) == 0)
                {
                    pnode.get().mark_for_disconnect();

                    log_print!(
                        LogFlags::NET, 
                        "disconnecting extra block-relay-only peer=%d (last block received at time %d)\n", 
                        pnode.get_id(), 
                        pnode.n_last_block_time
                    );

                    return true;

                } else {

                    log_print!(
                        LogFlags::NET, 
                        "keeping block-relay-only peer=%d chosen for eviction (connect time: %d, blocks_in_flight: %d)\n", 
                        pnode.get_id(), 
                        pnode.n_time_connected, 
                        (*node_state).n_blocks_in_flight
                    );
                }
                return false;
            };

            self.connman.get_mut().for_node_mut(
                to_disconnect, 
                &mut maybe_mark_disconnect
            );
        };
        
        // Check whether we have too many
        // outbound-full-relay peers
        if self.connman.get().get_extra_full_outbound_count() > 0 {

            // If we have more outbound-full-relay
            // peers than we target, disconnect
            // one.
            //
            // Pick the outbound-full-relay peer
            // that least recently announced us
            // a new block, with ties broken by
            // choosing the more recent connection
            // (higher node id)
            let mut worst_peer: NodeId = -1;

            let mut oldest_block_announcement: Option<OffsetDateTime> 
            = OffsetDateTime::from_unix_timestamp(max_unix_timestamp()).ok();

            let mut maybe_mark_worst_peer = move |pnode: Amo<Box<dyn NodeInterface>>| -> () {

                // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
                assert_lock_held!(CS_MAIN);

                // Only consider
                // outbound-full-relay peers that
                // are not already marked for
                // disconnection
                if !pnode.get().is_full_outbound_conn() 
                || pnode.get().marked_for_disconnect() 
                {
                    return;
                }

                let state: Amo<NodeState> = create_state(pnode.get().get_id());

                // shouldn't be possible, but just
                // in case
                if state.is_none() {
                    return;
                }

                // Don't evict our protected peers
                if state.get().chain_sync.protect {
                    return;
                }

                let last_block_announcement = state.get().last_block_announcement;

                if {
                    let gate0 = last_block_announcement < oldest_block_announcement;

                    let gate1a = last_block_announcement == oldest_block_announcement;
                    let gate1b = pnode.get().get_id() > worst_peer;

                    let gate1 = gate1a && gate1b;
                    
                    gate0 || gate1
                }
                {
                    worst_peer = pnode.get().get_id();

                    oldest_block_announcement = state.get().last_block_announcement;
                }
            };

            self.connman.get_mut().for_each_node_mut(&mut maybe_mark_worst_peer);

            if worst_peer != -1 {

                let mut maybe_mark_disconnect = move |pnode: Amo<Box<dyn NodeInterface>>| {

                    // EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)
                    assert_lock_held!(CS_MAIN);

                    // Only disconnect a peer that
                    // has been connected to us
                    // for some reasonable
                    // fraction of our
                    // check-frequency, to give it
                    // time for new information to
                    // have arrived.
                    //
                    // Also don't disconnect any
                    // peer we're trying to
                    // download a block from.
                    let created_state = create_state(pnode.get().get_id());

                    let state = created_state.get();

                    let time_connected = pnode.get().n_time_connected().unwrap();

                    if time_in_seconds - time_connected > MINIMUM_CONNECT_TIME 
                    && state.n_blocks_in_flight.load(atomic::Ordering::Relaxed) == 0 
                    {
                        log_print!(
                            LogFlags::NET, 
                            "disconnecting extra outbound peer=%d (last block announcement received at time %d)\n", 
                            pnode.get_id(), 
                            oldest_block_announcement
                        );

                        pnode.get().mark_for_disconnect();

                        return true;

                    } else {

                        log_print!(
                            LogFlags::NET, 
                            "keeping outbound peer=%d chosen for eviction (connect time: %d, blocks_in_flight: %d)\n", 
                            pnode.get_id(), 
                            pnode.n_time_connected, 
                            state.n_blocks_in_flight
                        );

                        return false;
                    }
                };

                let disconnected: bool 
                = self.connman.get_mut().for_node_mut(
                    worst_peer, 
                    &mut maybe_mark_disconnect
                );

                if disconnected {

                    // If we disconnected an extra
                    // peer, that means we
                    // successfully connected to
                    // at least one peer after the
                    // last time we detected
                    // a stale tip. Don't try any
                    // more extra peers until we
                    // next detect a stale tip, to
                    // limit the load we put on
                    // the network from these
                    // extra connections.
                    self.connman.get_mut().set_try_new_outbound_peer(false);
                }
            }
        }
    }
}
