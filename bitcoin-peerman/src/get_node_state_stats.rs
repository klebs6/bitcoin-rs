// ---------------- [ File: bitcoin-peerman/src/get_node_state_stats.rs ]
crate::ix!();

impl GetNodeStateStats for PeerManager {

    fn get_node_state_stats(&self, 
        nodeid: NodeId,
        stats:  &mut NodeStateStats) -> bool {

        {
            let mut guard = CS_MAIN.lock();

            let state: Amo<NodeState> = create_state(nodeid);

            if state.is_none() {
                return false;
            }

            stats.n_sync_height = match state.get().pindex_best_known_block {
                Some(ref block) => block.n_height,
                None        => -1
            };

            stats.n_common_height 
                = match state.get().pindex_last_common_block
            {
                Some(ref block) => block.n_height,
                None        => -1
            };

            for queue in state.get().blocks_in_flight.iter() {

                stats.height_in_flight.push(queue.pindex.as_ref().unwrap().n_height);
            }
        }

        let peer: Amo<Peer> = self.get_peer_ref(nodeid);

        if peer.is_none() {
            return false;
        }

        stats.starting_height 
            = peer.get().starting_height.load(atomic::Ordering::Relaxed);

        //  It is common for nodes with good ping
        //  times to suddenly become lagged, due
        //  to a new block arriving or other large
        //  transfer.
        //
        //  Merely reporting pingtime might fool
        //  the caller into thinking the node was
        //  still responsive, since pingtime does
        //  not update until the ping is complete,
        //  which might take a while.
        //
        //  So, if a ping is taking an unusually
        //  long time in flight, the caller can
        //  immediately detect that this is
        //  happening.
        let mut ping_wait: Duration = Duration::microseconds(0);

        if (0 != peer.get().ping_nonce_sent.load(atomic::Ordering::Relaxed)) 
        && (0 != peer.get().ping_start.load(atomic::Ordering::Relaxed).unix_timestamp()) 
        {
            ping_wait 
                = get_datetime() 
                - peer.get().ping_start.load(atomic::Ordering::Relaxed);
        }

        let peer = peer.get();

        stats.ping_wait          = ping_wait;
        stats.addr_processed     = peer.addr_processed.load(atomic::Ordering::Relaxed);
        stats.addr_rate_limited  = peer.addr_rate_limited.load(atomic::Ordering::Relaxed);
        stats.addr_relay_enabled = peer.addr_relay_enabled.load(atomic::Ordering::Relaxed);

        true
    }
}
