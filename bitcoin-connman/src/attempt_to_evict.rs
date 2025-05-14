// ---------------- [ File: bitcoin-connman/src/attempt_to_evict.rs ]
crate::ix!();

impl Connman {

    /**
      | Try to find a connection to evict when
      | the node is full.
      | 
      | Extreme care must be taken to avoid opening
      | the node to attacker triggered network
      | partitioning.
      | 
      | The strategy used here is to protect
      | a small number of peers for each of several
      | distinct characteristics which are
      | difficult to forge.
      | 
      | In order to partition a node the attacker
      | must be simultaneously better at all
      | of them than honest peers.
      |
      */
    pub fn attempt_to_evict_connection(&self) -> bool {

        let mut eviction_candidates = Vec::<NodeEvictionCandidate>::default();

        {
            let mut guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let mut node = pnode.get();

                unsafe {

                    if node.has_permission(NetPermissionFlags::NoBan) {
                        continue;
                    }

                    if !node.is_inbound_conn() {
                        continue;
                    }

                    if node.marked_for_disconnect() {
                        continue;
                    }

                    let mut peer_relay_txes:      bool = false;
                    let mut peer_filter_not_null: bool = false;

                    if node.has_tx_relay() {

                        let relay = node.get_tx_relay();

                        let guard = relay.cs_filter.lock();

                        peer_relay_txes      = guard.relay_txes;
                        peer_filter_not_null = guard.pfilter.is_some();
                    }

                    let n_time_connected = match node.n_time_connected() {
                        Some(t) => Some(t),
                        None    => None,
                    };

                    let candidate = NodeEvictionCandidate {
                        id:                 node.get_id(),
                        n_time_connected:   n_time_connected,
                        min_ping_time:      node.min_ping_time(),
                        n_last_block_time:  node.n_last_block_time(),
                        n_last_tx_time:     node.n_last_tx_time(),
                        relevant_services:  has_all_desirable_service_flags(node.n_services()),
                        relay_txes:         peer_relay_txes,
                        bloom_filter:       peer_filter_not_null,
                        n_keyed_net_group:  node.n_keyed_net_group(),
                        prefer_evict:       node.prefer_evict(),
                        is_local:           node.service().base.is_local(),
                        network:            node.connected_through_network()
                    };

                    eviction_candidates.push(candidate);
                }
            }
        }

        let node_id_to_evict: Option::<NodeId> = select_node_to_evict(eviction_candidates);

        if node_id_to_evict.is_none() {
            return false;
        }

        let mut guard = self.cs_v_nodes.get();

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {

                if Some(node.get_id()) == node_id_to_evict {

                    log_print!(
                        LogFlags::NET, 
                        "selected {} connection for eviction peer={}; disconnecting\n", 
                        node.connection_type_as_string(), 
                        node.get_id()
                    );

                    node.mark_for_disconnect();

                    return true;
                }
            }
        }

        false
    }
}
