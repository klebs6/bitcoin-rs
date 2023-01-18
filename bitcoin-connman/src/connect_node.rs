crate::ix!();

impl Connman {

    /**
      | Determine whether we're already connected
      | to a given address, in order to avoid
      | initiating duplicate connections.
      |
      */
    pub fn already_connected_to_address(&self, addr: &Address) -> bool {
        
        let has_node_with_ip: bool 
        = self.find_node_with_ip(&(addr.service.base)).is_some();

        let has_node_with_addr_name: bool 
        = self.find_node_with_addr_name(&addr.service.to_string_ip_port()).is_some();

        has_node_with_ip || has_node_with_addr_name
    }

    pub fn connect_node(&self, 
        mut addr_connect:  Address,
        psz_dest:      *const u8,
        count_failure: bool,
        conn_type:     ConnectionType) -> Amo<Box<dyn NodeInterface>> {

        assert!(conn_type != ConnectionType::INBOUND);

        if psz_dest == std::ptr::null_mut() {

            if is_local(&addr_connect.service) {
                return Amo::<Box<dyn NodeInterface>>::none();
            }

            // Look for an existing connection
            let pnode = self.find_node_with_ip(&addr_connect.service.base);

            if pnode.is_some() {
                log_printf!("Failed to open new connection, already connected\n");
                return Amo::<Box<dyn NodeInterface>>::none();
            }
        }

        // debug print
        log_print!(
            LogFlags::NET,
            "trying connection {} lastseen={:.1}hrs\n",
            match psz_dest {
                true   => psz_dest,
                false  => addr_connect.to_string()
            },
            match psz_dest {
                true   => 0.0,
                false  => (get_adjusted_time() - addr_connect.n_time) as f64 / 3600.0
            }
        );

        // Resolve
        let default_port: u16 = match psz_dest != std::ptr::null_mut() {

            true   => {

                let sz_dest = from_cstr![psz_dest];

                params().get_default_port_from_addr(&sz_dest)
            },

            false  => params().get_default_port()
        };

        if psz_dest != null_mut() {

            let sz_dest = from_cstr![psz_dest];

            let mut resolved = Vec::<Service>::default();

            if lookup_multi(
                &sz_dest,
                &mut resolved,
                default_port,
                DEFAULT_NAME_LOOKUP != 0 && !have_name_proxy(),
                256,
                None
            ) && !resolved.is_empty() {

                let rand_max: u64 = resolved.len().try_into().unwrap();

                let idx: usize = get_rand(rand_max).try_into().unwrap();

                addr_connect = Address::new(resolved[idx].clone(), ServiceFlags::NODE_NONE);

                if !addr_connect.service.base.is_valid() {

                    log_print!(
                        LogFlags::NET, 
                        "Resolver returned invalid address {} for {}\n", 
                        addr_connect.to_string(), 
                        &sz_dest
                    );

                    return Amo::<Box<dyn NodeInterface>>::none();
                }

                // It is possible that we already
                // have a connection to the
                // IP/port pszDest resolved to.
                //
                // In that case, drop the
                // connection that was just
                // created.
                let mut guard = self.cs_v_nodes.get();

                let pnode = self.find_node_with_addr(&addr_connect.service);

                if pnode.is_some() {

                    log_printf!(
                        "Failed to open new connection, already connected\n"
                    );

                    return Amo::<Box<dyn NodeInterface>>::none();
                }
            }
        }

        //  Connect
        let mut connected: bool = false;;

        let mut sock: Option<Box<Sock>> = None;

        let mut proxy     = ProxyType::default();
        let mut addr_bind = Address::default();

        assert!(!addr_bind.service.base.is_valid());

        if addr_connect.service.base.is_valid() {

            let mut proxy_connection_failed: bool = false;

            if addr_connect.service.base.get_network() == Network::NET_I2P 
            && self.i2p_sam_session.is_some() {

                let mut conn = Connection::default();

                if self.i2p_sam_session
                    .get_mut()
                    .connect(&addr_connect.service, &mut conn, &mut proxy_connection_failed) 
                {
                    connected = true;
                    sock = Some(conn.sock); //want move
                    addr_bind = Address::new(conn.me,ServiceFlags::NODE_NONE);
                }

            } else {

                if get_proxy(addr_connect.service.base.get_network(),&mut proxy) {

                    sock = create_socktcp(&proxy.proxy);

                    if sock.is_none() {
                        return Amo::<Box<dyn NodeInterface>>::none();
                    }

                    connected = connect_through_proxy(
                        &proxy,
                        &addr_connect.service.to_string_ip_port(),
                        addr_connect.service.get_port(),
                        &*sock.as_ref().unwrap(),
                        *N_CONNECT_TIMEOUT,
                        &mut proxy_connection_failed
                    );

                } else {

                    // no proxy needed (none set
                    // for target network)
                    sock = create_socktcp(&addr_connect.service);

                    if sock.is_none() {
                        return Amo::<Box<dyn NodeInterface>>::none();
                    }

                    connected = connect_socket_directly(
                        &addr_connect.service,
                        &*sock.as_ref().unwrap(),
                        *N_CONNECT_TIMEOUT,
                        conn_type == ConnectionType::MANUAL
                    );
                }
            }

            if !proxy_connection_failed {

                // If a connection to the node was
                // attempted, and failure (if any)
                // is not caused by a problem
                // connecting to the proxy, mark
                // this as an attempt.
                self.addrman
                    .get_mut()
                    .attempt(&addr_connect.service, count_failure, None);
            }

        } else {

            if psz_dest != null_mut() && get_name_proxy(&mut proxy) {

                sock = create_socktcp(&proxy.proxy);

                if sock.is_none() {
                    return Amo::<Box<dyn NodeInterface>>::none();
                }

                let mut host = String::default();
                let mut port: u16 = default_port;

                let sz_dest = from_cstr![psz_dest];

                split_host_port(sz_dest, &mut port, &mut host);

                let mut proxy_connection_failed = bool::default();

                connected = connect_through_proxy(
                    &proxy,
                    &host,
                    port,
                    &*sock.as_ref().unwrap(),
                    *N_CONNECT_TIMEOUT,
                    &mut proxy_connection_failed
                );
            }
        }

        if !connected {
            return Amo::<Box<dyn NodeInterface>>::none();
        }

        // Add node
        let id: NodeId = self.get_new_node_id();;

        let mut randomizer = self.get_deterministic_randomizer(RANDOMIZER_ID_LOCALHOSTNONCE);

        randomizer.write_i64(id);

        let nonce: u64 = randomizer.finish();

        if !addr_bind.service.base.is_valid() {
            addr_bind = get_bind_address(sock.as_ref().unwrap().get());
        }

        let mut pnode: Amo<Box<dyn NodeInterface>> = Amo::<Box<dyn NodeInterface>>::none();

        pnode.get_mut().add_ref();

        // We're making a new connection, harvest
        // entropy from the time (and our peer
        // count)
        rand_add_event(id as u32);

        pnode
    }
}
