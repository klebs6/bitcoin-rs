// ---------------- [ File: bitcoin-connman/src/create_node_from_accepted_socket.rs ]
crate::ix!();

impl Connman {

    /**
      | Create a `Node` object from a socket
      | that has just been accepted and add the
      | node to the `vNodes` member.
      | 
      | -----------
      | @param[in] hSocket
      | 
      | Connected socket to communicate with
      | the peer.
      | ----------
      | @param[in] permissionFlags
      | 
      | The peer's permissions.
      | ----------
      | @param[in] addr_bind
      | 
      | The address and port at our side of the
      | connection.
      | ----------
      | @param[in] addr
      | 
      | The address and port at the peer's side
      | of the connection.
      |
      */
    pub fn create_node_from_accepted_socket(&self, 
        h_socket:         &mut CSocket,
        permission_flags: NetPermissionFlags,
        addr_bind:        &Address,
        addr:             &Address)  {

        let mut n_inbound: i32 = 0;
        let n_max_inbound: i32 = self.n_max_connections.load(atomic::Ordering::Relaxed) - self.max_outbound.load(atomic::Ordering::Relaxed);

        let mut permission_flags = permission_flags;

        self.add_whitelist_permission_flags(&mut permission_flags, &addr.service.base);

        if NetPermissions::has_flag(&permission_flags,NetPermissionFlags::Implicit) {

            NetPermissions::clear_flag(&mut permission_flags, NetPermissionFlags::Implicit);

            if G_ARGS.lock().get_bool_arg("-whitelistforcerelay", DEFAULT_WHITELISTFORCERELAY) {
                NetPermissions::add_flag(&mut permission_flags, NetPermissionFlags::ForceRelay);
            }

            if G_ARGS.lock().get_bool_arg("-whitelistrelay", DEFAULT_WHITELISTRELAY) {
                NetPermissions::add_flag(&mut permission_flags, NetPermissionFlags::Relay);
            }

            NetPermissions::add_flag(&mut permission_flags, NetPermissionFlags::Mempool);
            NetPermissions::add_flag(&mut permission_flags, NetPermissionFlags::NoBan);
        }

        {
            let mut guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                if pnode.get().is_inbound_conn() {
                    n_inbound += 1;
                }
            }
        }

        if !self.network_active.load(atomic::Ordering::Relaxed) {

            log_print!(
                bc_log::NET, 
                "connection from {} dropped: not accepting new connections\n", 
                addr.to_string()
            );

            close_socket(h_socket);
            return;
        }

        if !is_selectable_socket(h_socket) {

            log_printf!(
                "connection from {} dropped: non-selectable socket\n", 
                addr.to_string()
            );

            close_socket(h_socket);
            return;
        }

        // According to the internet TCP_NODELAY
        // is not carried into accepted sockets on
        // all platforms.  Set it again here just
        // to be sure.
        set_socket_no_delay(h_socket);

        // Don't accept connections from banned
        // peers.
        let banned: bool = 
        self.banman.is_some() 
        && self.banman
            .get()
            .is_netaddr_banned(&addr.service.base);;

        if !NetPermissions::has_flag(
            &permission_flags,
            NetPermissionFlags::NoBan) && banned {

            log_print!(
                bc_log::NET, 
                "connection from {} dropped (banned)\n", 
                addr.to_string()
            );

            close_socket(h_socket);
            return;
        }

        // Only accept connections from
        // discouraged peers if our inbound slots
        // aren't (almost) full.
        let discouraged: bool = 
        self.banman.is_some() 
        && self.banman.get().is_discouraged(&addr.service.base);

        if !NetPermissions::has_flag(&permission_flags,NetPermissionFlags::NoBan) 
        && n_inbound + 1 >= n_max_inbound 
        && discouraged 
        {
            log_print!(
                bc_log::NET, 
                "connection from {} dropped (discouraged)\n", 
                addr.to_string()
            );

            close_socket(h_socket);
            return;
        }

        if n_inbound >= n_max_inbound {

            if !self.attempt_to_evict_connection() {

                // No connection to evict,
                // disconnect the new connection
                log_print!(
                    bc_log::NET,
                    "failed to find an eviction candidate - connection dropped (full)\n"
                );

                close_socket(h_socket);

                return;
            }
        }

        let id: NodeId = self.get_new_node_id();

        let mut randomizer = self.get_deterministic_randomizer(RANDOMIZER_ID_LOCALHOSTNONCE);

        randomizer.write_i64(id);

        let nonce: u64 = randomizer.finish();

        let mut node_services: ServiceFlags = self.n_local_services.get().clone();

        if NetPermissions::has_flag(&permission_flags,NetPermissionFlags::BloomFilter) {
            node_services = node_services | ServiceFlags::NODE_BLOOM;
        }

        let inbound_onion: bool = self.onion_binds.get().iter().find(|&x| x == &addr_bind.service).is_some();

        let mut pnode: Amo<Box<dyn NodeInterface>> = Amo::<Box<dyn NodeInterface>>::none();

        let mut node = pnode.get_mut();

        node.add_ref();
        node.set_permission_flags(permission_flags.clone());
        node.set_prefer_evict(discouraged);

        self.msgproc
            .get_mut()
            .initialize_node(&mut node);

        log_print!(
            bc_log::NET, 
            "connection from {} accepted\n", 
            addr.to_string()
        );

        {
            let mut guard = self.cs_v_nodes.get_mut();

            guard.nodes.push(pnode.clone());
        }

        // We received a new connection, harvest entropy
        // from the time (and our peer count)
        rand_add_event(id as u32);
    }
}
