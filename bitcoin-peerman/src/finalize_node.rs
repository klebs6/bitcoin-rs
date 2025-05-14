// ---------------- [ File: bitcoin-peerman/src/finalize_node.rs ]
crate::ix!();

impl FinalizeNode for PeerManager {
    
    fn finalize_node(&mut self, node: &mut AmoWriteGuard<Box<dyn NodeInterface>>)  {
        
        let nodeid: NodeId = node.get_id();

        let mut misbehavior: i32 = 0;

        {
            let mut guard = CS_MAIN.lock();

            {
                // We remove the PeerRef from
                // g_peer_map here, but we don't
                // always destruct the
                // Peer. Sometimes another thread
                // is still holding a PeerRef, so
                // the refcount is >= 1. Be
                // careful not to do any
                // processing here that assumes
                // Peer won't be changed before
                // it's destructed.
                let peer: Amo<Peer> = self.remove_peer(nodeid);

                assert!(peer.is_some());

                misbehavior = 
                    peer.get().misbehavior.lock().score;
            }

            let state: Amo<NodeState> = create_state(nodeid);

            assert!(state.is_some());

            if state.get().sync_started.load(atomic::Ordering::Relaxed) {
                {
                    let old = self.inner.lock().n_sync_started;
                    self.inner.lock().n_sync_started -= 1;
                    old
                };
            }

            let mut inner = self.inner.lock();

            let mut mbif  = inner.map_blocks_in_flight.lock();

            for entry in state.get().blocks_in_flight.iter() {
                mbif.remove(&entry.pindex.as_ref().unwrap().get_block_hash());
            }

            {
                let mut guard = G_CS_ORPHANS.lock();
                self.orphanage.clone().erase_for_peer(nodeid);
            }

            inner.txrequest.lock().disconnected_peer(nodeid);

            N_PREFERRED_DOWNLOAD.fetch_sub(
                match state.get().preferred_download.load(atomic::Ordering::Relaxed) { true => 1, false => 0 }, 
                atomic::Ordering::Relaxed
            );

            inner.peers_downloading_from.fetch_sub( 
                match state.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) != 0 { true => 1, false => 0 },
                atomic::Ordering::Relaxed
            );

            assert!(inner.peers_downloading_from.load(atomic::Ordering::Relaxed) >= 0);

            inner.outbound_peers_with_protect_from_disconnect.fetch_sub( 
                match state.get().chain_sync.protect { true => 1, false => 0 }, 
                atomic::Ordering::Relaxed
            );

            assert!(inner.outbound_peers_with_protect_from_disconnect.load(atomic::Ordering::Relaxed) >= 0);

            inner.wtxid_relay_peers.fetch_sub( 
                match state.get().wtxid_relay.load(atomic::Ordering::Relaxed) { true => 1, false => 0 },
                atomic::Ordering::Relaxed
            );

            assert!(inner.wtxid_relay_peers.load(atomic::Ordering::Relaxed) >= 0);

            MAP_NODE_STATE.lock().remove(&nodeid);

            if MAP_NODE_STATE.lock().is_empty() {

                // Do a consistency check after
                // the last peer is removed.
                assert!(mbif.is_empty());
                assert!(N_PREFERRED_DOWNLOAD.load(atomic::Ordering::Relaxed) == 0);
                assert!(inner.peers_downloading_from.load(atomic::Ordering::Relaxed) == 0);
                assert!(inner.outbound_peers_with_protect_from_disconnect.load(atomic::Ordering::Relaxed) == 0);
                assert!(inner.wtxid_relay_peers.load(atomic::Ordering::Relaxed) == 0);
                assert!(inner.txrequest.lock().size() == 0);
                assert!(self.orphanage.size() == 0);
            }
        }

        if node.is_successfully_connected()
        && misbehavior == 0 
        && !node.is_block_only_conn() 
        && !node.is_inbound_conn() {

            // Only change visible addrman state
            // for full outbound peers.  We don't
            // call Connected() for feeler
            // connections since they don't have
            // fSuccessfullyConnected set.
            self.addrman.get_mut()
                .connected(node.service(), None);
        }

        log_print!(LogFlags::NET, "Cleared nodestate for peer=%d\n", nodeid);
    }
}
