// ---------------- [ File: bitcoin-peerman/src/maybe_discourage_and_disconnect.rs ]
crate::ix!();

pub trait MaybeDiscourageAndDisconnect {
    fn maybe_discourage_and_disconnect(self: Arc<Self>, 
        pnode: Amo<Box<dyn NodeInterface>>,
        peer:  &mut Peer) -> bool;
}
    
impl MaybeDiscourageAndDisconnect for PeerManager {

    /**
      | Maybe disconnect a peer and discourage
      | future connections from its address.
      | 
      | -----------
      | @param[in] pnode
      | 
      | The node to check.
      | ----------
      | @param[in] peer
      | 
      | The peer object to check.
      | 
      | -----------
      | @return
      | 
      | True if the peer was marked for disconnection
      | in this function
      |
      */
    fn maybe_discourage_and_disconnect(self: Arc<Self>, 
        pnode: Amo<Box<dyn NodeInterface>>,
        peer:  &mut Peer) -> bool {

        {
            let mut guard = peer.misbehavior.lock();

            // There's nothing to do if the
            // m_should_discourage flag isn't set
            if !guard.should_discourage {
                return false;
            }

            guard.should_discourage = false;
        }

        if pnode.get().has_permission(NetPermissionFlags::NoBan) {
            // We never disconnect or discourage
            // peers for bad behavior if they have
            // NetPermissionFlags::NoBan
            // permission
            log_printf!(
                "Warning: not punishing noban peer %d!\n",
                peer.id
            );

            return false;
        }

        if pnode.get().is_manual_conn() {

            // We never disconnect or discourage
            // manual peers for bad behavior
            log_printf!(
                "Warning: not punishing manually connected peer %d!\n",
                peer.id
            );

            return false;
        }

        if pnode.get().addr().is_local() {

            // We disconnect local peers for bad
            // behavior but don't discourage
            // (since that would discourage all
            // peers on the same local address)
            log_print!(
                LogFlags::NET,
                "Warning: disconnecting but not discouraging %s peer %d!\n",
                match pnode.inbound_onion {
                    true   => "inbound onion",
                    false  => "local"
                },
                peer.id
            );

            pnode.get().mark_for_disconnect();

            return true;
        }

        // Normal case: Disconnect the peer and
        // discourage all nodes sharing the
        // address
        log_print!(
            LogFlags::NET,
            "Disconnecting and discouraging peer %d!\n",
            peer.id
        );

        if self.banman.is_some() {
            self.banman.get_mut().discourage(&pnode.get().service().base);
        }

        let node = pnode.get();

        let netaddr = &node.service().base;

        self.connman.get_mut()
            .disconnect_node_with_netaddr(netaddr);

        true
    }
}
