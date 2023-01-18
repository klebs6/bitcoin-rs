crate::ix!();

impl Connman {
    
    pub fn get_new_node_id(&self) -> NodeId {
        
        self.n_last_node_id.fetch_add(1, atomic::Ordering::Relaxed)
    }
    
    pub fn get_node_count(&self, flags: ConnectionDirection) -> usize {
        
        let guard = self.cs_v_nodes.get();

        if flags == ConnectionDirection::Both {

            // Shortcut if we want total
            return guard.nodes.len();
        }

        let mut n_num: i32 = 0;

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {

                let dir = match node.is_inbound_conn() {
                    true   => ConnectionDirection::In,
                    false  => ConnectionDirection::Out
                };

                if (flags & dir).bits() != 0 {
                    n_num += 1;
                }
            }
        }

        n_num.try_into().unwrap()
    }
    
    pub fn get_node_stats(&self, vstats: &mut Vec<NodeStats>)  {
        
        vstats.clear();

        let guard = self.cs_v_nodes.get();

        vstats.reserve(guard.nodes.len());

        for pnode in guard.nodes.iter() {

            let mut node = pnode.get_mut();

            unsafe {

                vstats.push(Default::default());

                node.copy_stats(&mut vstats.last_mut().unwrap());

                vstats.last_mut().unwrap().mapped_as 
                    = node.service().base.get_mappedas(self.addrman.get().get_asmap());
            }
        }
    }

    pub fn get_max_outbound_target(&self) -> u64 {
        
        let guard = self.cs_total_bytes_sent.get();

        guard.n_max_outbound_limit
    }
    
    pub fn get_max_outbound_timeframe(&self) -> Duration {
        
        MAX_UPLOAD_TIMEFRAME
    }

    /**
      | returns the time left in the current
      | max outbound cycle in case of no limit,
      | it will always return 0
      |
      */
    pub fn get_max_outbound_time_left_in_cycle(&self) -> Duration {

        let guard = self.cs_total_bytes_sent.get();

        if guard.n_max_outbound_limit == 0 {
            return Duration::seconds(0);
        }

        if guard.n_max_outbound_cycle_start_time == None {
            return MAX_UPLOAD_TIMEFRAME;
        }

        let cycle_end_time: Instant = guard.n_max_outbound_cycle_start_time.unwrap() + MAX_UPLOAD_TIMEFRAME;

        let now = Instant::now();

        match (cycle_end_time < now) {
            true   => Duration::seconds(0),
            false  => cycle_end_time - now
        }
    }

    /**
      | Return vector of current BLOCK_RELAY
      | peers.
      |
      */
    pub fn get_current_block_relay_only_conns(&self) -> Vec<Address> {
        
        let mut ret = Vec::<Address>::default();

        let guard = self.cs_v_nodes.get();

        for pnode in guard.nodes.iter() {

            let node = pnode.get();

            unsafe {
                if node.is_block_only_conn() {
                    ret.push(node.addr().clone());
                }
            }
        }

        ret
    }

    /**
      | Count the number of block-relay-only
      | peers we have over our limit.
      |
      */
    pub fn get_extra_block_relay_count(&self) -> i32 {
        
        let mut block_relay_peers: i32 = 0;

        {
            let guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let node = pnode.get();

                unsafe {
                    if node.successfully_connected() 
                    && !node.marked_for_disconnect() 
                    && node.is_block_only_conn() 
                    {
                        block_relay_peers += 1;
                    }
                }
            }
        }

        max(block_relay_peers - self.max_outbound_block_relay.load(atomic::Ordering::Relaxed),0)
    }

    /**
      | Return the number of peers we have over our
      | outbound connection limit
      |
      | Exclude peers that are marked for disconnect,
      | or are going to be disconnected soon (eg
      | ADDR_FETCH and FEELER)
      |
      | Also exclude peers that haven't finished
      | initial connection handshake yet (so that we
      | don't decide we're over our desired connection
      | limit, and then evict some peer that has
      | finished the handshake)
      |
      | Return the number of outbound peers we have
      | in excess of our target (eg, if we
      | previously called
      | SetTryNewOutboundPeer(true), and have since
      | set to false, we may have extra peers that
      | we wish to disconnect). This may return
      | a value less than (num_outbound_connections
      | - num_outbound_slots) in cases where some
      | outbound connections are not yet fully
      | connected, or not yet fully disconnected.
      */
    pub fn get_extra_full_outbound_count(&self) -> i32 {
        
        let mut full_outbound_peers: i32 = 0;

        {
            let guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let node = pnode.get();

                unsafe {
                    if node.successfully_connected() 
                    && !node.marked_for_disconnect() 
                    && node.is_full_outbound_conn() 
                    {
                        full_outbound_peers += 1;
                    }
                }
            }
        }

        max(full_outbound_peers - self.max_outbound_full_relay.load(atomic::Ordering::Relaxed), 0)
    }

    pub fn get_try_new_outbound_peer(&self) -> bool {
        
        self.try_another_outbound_peer.load(atomic::Ordering::Relaxed)
    }

    pub fn get_network_active(&self) -> bool {
        
        self.network_active.load(atomic::Ordering::Relaxed)
    }
    
    pub fn get_use_addrman_outgoing(&self) -> bool {
        
        self.use_addrman_outgoing.load(atomic::Ordering::Relaxed)
    }

    /**
      | response the bytes left in the current
      | max outbound cycle in case of no limit,
      | it will always response 0
      |
      */
    pub fn get_outbound_target_bytes_left(&self) -> u64 {
        
        let guard = self.cs_total_bytes_sent.get();

        if guard.n_max_outbound_limit == 0 {
            return 0;
        }

        match guard.n_max_outbound_total_bytes_sent_in_cycle >= guard.n_max_outbound_limit {
            true   => 0,
            false  => guard.n_max_outbound_limit - guard.n_max_outbound_total_bytes_sent_in_cycle
        }
    }
    
    pub fn get_total_bytes_recv(&self) -> u64 {
        
        let guard = self.cs_total_bytes_recv.get();
        guard.n_total_bytes_recv
    }
    
    pub fn get_total_bytes_sent(&self) -> u64 {
        
        let guard = self.cs_total_bytes_sent.get();
        guard.n_total_bytes_sent
    }
    
    /**
      | Used to convey which local services we are
      | offering peers during node connection.
      |
      | The data returned by this is used in Node
      | construction, which is used to advertise
      | which services we are offering that peer
      | during
      | `net_processing.cpp:PushNodeVersion()`.
      */
    pub fn get_local_services(&self) -> ServiceFlags {
        
        self.n_local_services.get().clone()
    }
    
    pub fn get_receive_flood_size(&self) -> u32 {
        
        self.n_receive_flood_size.load(atomic::Ordering::Relaxed)
    }

    /**
      | Get a unique deterministic randomizer.
      |
      */
    pub fn get_deterministic_randomizer(&self, id: u64) -> SipHasher {
        
        let mut hasher = SipHasher::new_with_keys(self.n_seed0,self.n_seed1);

        hasher.write_u64(id);

        hasher
    }
}
