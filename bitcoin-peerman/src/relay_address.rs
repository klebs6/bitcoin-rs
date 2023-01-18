crate::ix!();

pub trait RelayAddress {

    fn relay_address(self: Arc<Self>, 
        originator: NodeId,
        addr:       &Address,
        reachable:  bool);
}

impl RelayAddress for PeerManager {

    /**
      | Relay (gossip) an address to a few randomly
      | chosen nodes.
      | 
      | -----------
      | @param[in] originator
      | 
      | The id of the peer that sent us the address.
      | We don't want to relay it back.
      | ----------
      | @param[in] addr
      | 
      | Address to relay.
      | ----------
      | @param[in] fReachable
      | 
      | Whether the address' network is reachable.
      | We relay unreachable addresses less.
      |
      */
    fn relay_address(
        self:       Arc<Self>, 
        originator: NodeId,
        addr:       &Address,
        reachable:  bool)  {

        // We choose the same nodes within a given
        // 24h window (if the list of connected
        // nodes does not change) and we don't
        // relay to nodes that already know an
        // address. 
        //
        // So within 24h we will likely relay
        // a given address once. This is to
        // prevent a peer from unjustly giving
        // their address better propagation by
        // sending it to us repeatedly.
        if !reachable && !addr.is_relayable() {
            return;
        }

        // Relay to a limited number of other
        // nodes
        //
        // Use deterministic randomness to send to
        // the same nodes for 24 hours at a time
        // so the m_addr_knowns of the chosen
        // nodes prevent repeats
        let hash_addr: u64 = addr.get_hash();

        let mut hasher: SipHasher = self.connman.get()
            .get_deterministic_randomizer(RANDOMIZER_ID_ADDRESS_RELAY);

        hasher.write_u64(hash_addr << 32);

        hasher.write_u64(
            (u64::try_from(get_datetime().unix_timestamp()).unwrap() + hash_addr) / (24 * 60 * 60)
        );

        let mut insecure_rand = FastRandomContext::default();

        // We choose the same nodes within a given
        // 24h window (if the list of connected
        // nodes does not change) and we don't
        // relay to nodes that already know an
        // address. So within 24h we will likely
        // relay a given address once. This is to
        // prevent a peer from unjustly giving
        // their address better propagation by
        // sending it to us repeatedly.
        if !reachable && !addr.is_relayable() {
            return;
        }

        // Relay to a limited number of other
        // nodes
        //
        // Use deterministic randomness to send to
        // the same nodes for 24 hours at a time
        // so the m_addr_knowns of the chosen
        // nodes prevent repeats
        let hash_addr: u64 = addr.get_hash();

        let mut hasher: SipHasher = self.connman.get()
            .get_deterministic_randomizer(RANDOMIZER_ID_ADDRESS_RELAY);

        hasher.write_u64(hash_addr << 32);

        hasher.write_u64(
            (u64::try_from(get_datetime().unix_timestamp()).unwrap() + hash_addr) / (24 * 60 * 60)
        );

        let mut insecure_rand = FastRandomContext::default();

        // Relay reachable addresses to
        // 2 peers. Unreachable addresses are
        // relayed randomly to 1 or 2 peers.
        let n_relay_nodes: usize = match reachable || (hasher.finish() & 1) != 0 {
            true   => 2,
            false  => 1
        };

        struct Item {
            first:  u64,
            second: Amo<Peer>,
        }

        let mut best: Vec<Item> = vec![
            Item { first: 0, second: amo_none() }, 
            Item { first: 0, second: amo_none() }
        ];

        assert!(n_relay_nodes <= best.len());

        let mut peer_map = self.peer_map.get();

        for (id,peer) in peer_map.iter() {

            if peer.get().addr_relay_enabled.load(atomic::Ordering::Relaxed) 
            && *id != originator 
            && peer.get().is_addr_compatible(addr) 
            {
                let mut siphasher = SipHasher::from(hasher.clone());

                siphasher.write_i64(*id);

                let hash_key: u64 = siphasher.finish();

                for i in 0_usize..n_relay_nodes {

                    if hash_key > best[i].first 
                    {
                        let src = &best[i..n_relay_nodes - 1];

                        let rmin = i + 1;
                        let rmax = n_relay_nodes - 1;

                        let range = 
                            (i + 1)
                            ..
                            (i + 1 + src.len());

                        let other: Vec<Item> = best.drain(rmin..=rmax).collect();

                        best.splice( range, other );

                        best[i] = Item { first: hash_key, second: peer.clone() };

                        break;
                    }
                }
            }
        }

        let mut i: usize = 0;

        while i < n_relay_nodes.try_into().unwrap() && best[i].first != 0 
        {
            best[i].second.get_mut().push_address(
                addr, 
                &mut insecure_rand
            );

            i += 1;
        }
    }
}
