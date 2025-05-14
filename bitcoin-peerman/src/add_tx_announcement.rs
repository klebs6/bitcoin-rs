// ---------------- [ File: bitcoin-peerman/src/add_tx_announcement.rs ]
crate::ix!();

pub trait AddTxAnnouncement {

    fn add_tx_announcement(self: Arc<Self>, 
        node:         &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        gtxid:        &GenTxId,
        current_time: OffsetDateTime /* micros */);
}

impl AddTxAnnouncement for PeerManager {

    /**
      | Register with TxRequestTracker that
      | an INV has been received from a peer.
      | The announcement parameters are decided
      | in PeerManager and then passed to TxRequestTracker.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::CS_MAIN)]
    fn add_tx_announcement(
        self:         Arc<Self>, 
        node:         &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        gtxid:        &GenTxId,
        current_time: OffsetDateTime /* micros */)  {
        
        //  For m_txrequest
        assert_lock_held!(CS_MAIN);

        let nodeid: NodeId = node.get_id();

        if !node.has_permission(NetPermissionFlags::Relay) 
        && self.inner.lock().txrequest.lock().count(nodeid) >= MAX_PEER_TX_ANNOUNCEMENTS.try_into().unwrap() {
            //  Too many queued announcements from this peer
            return;
        }

        let state: Amo<NodeState> = create_state(nodeid);

        //  Decide the TxRequestTracker parameters for this announcement:
        //  - "preferred": if fPreferredDownload is set (= outbound, or NetPermissionFlags::NoBan permission)
        //  - "reqtime": current time plus delays for:
        //    - NONPREF_PEER_TX_DELAY for announcements from non-preferred connections
        //    - TXID_RELAY_DELAY for txid announcements while wtxid peers are available
        //    - OVERLOADED_PEER_TX_DELAY for announcements from peers which have at least
        //      MAX_PEER_TX_REQUEST_IN_FLIGHT requests in flight (and don't have NetPermissionFlags::Relay).
        let mut delay = Duration::microseconds(0);
        let preferred: bool = state.get().preferred_download.load(atomic::Ordering::Relaxed);

        if !preferred {
            delay += NONPREF_PEER_TX_DELAY;
        }

        if !gtxid.is_wtxid() && self.inner.lock().wtxid_relay_peers.load(atomic::Ordering::Relaxed) > 0 {
            delay += TXID_RELAY_DELAY;
        }

        let overloaded: bool = 
        !node.has_permission(NetPermissionFlags::Relay) 
        && self.inner.lock().txrequest.lock().count_in_flight(nodeid) >= MAX_PEER_TX_REQUEST_IN_FLIGHT.try_into().unwrap();

        if overloaded {
            delay += OVERLOADED_PEER_TX_DELAY;
        }

        self.inner.lock().txrequest.lock().received_inv(nodeid, gtxid, preferred, current_time + delay);
    }
}
