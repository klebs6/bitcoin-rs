crate::ix!();

impl Connman {

    /**
      | if successful, this moves the passed
      | grant to the constructed node
      |
      */
    pub fn open_network_connection(&self, 
        addr_connect:   &Address,
        count_failure:  bool,
        grant_outbound: Option<&mut SemaphoreGrant>,
        psz_dest:       *const u8,
        conn_type:      ConnectionType)  {
        
        assert!(conn_type != ConnectionType::INBOUND);

        // Initiate outbound network connection

        if self.interrupt_net.get().as_bool() {
            return;
        }

        if !self.network_active.load(atomic::Ordering::Relaxed) {
            return;
        }

        if psz_dest == null_mut() {

            let banned_or_discouraged: bool =
            {
                self.banman.is_some() 
                && {

                    let discouraged = self.banman.get().is_discouraged(&addr_connect.service.base);
                    let banned      = self.banman.get().is_netaddr_banned(&addr_connect.service.base);

                    discouraged || banned
                }
            };

            if is_local(&addr_connect.service) || banned_or_discouraged || self.already_connected_to_address(addr_connect) {
                return;
            }

        } else {

            let sz_dest = unsafe {
                std::ffi::CStr::from_ptr(psz_dest as *const libc::c_char)
                    .to_string_lossy()
                    .to_string()
            };


            if self.find_node_with_addr_name(&sz_dest).is_some() {
                return;
            }
        }

        let pnode = self.connect_node(
            addr_connect.clone(),
            psz_dest,
            count_failure,
            conn_type
        );

        if pnode.is_some() {

            let pnode_clone = pnode.clone();

            let mut node = pnode.get_mut();

            if grant_outbound.is_some() {

                grant_outbound.unwrap().move_to(&mut node.grant_outbound());
            }

            self.msgproc
                .get_mut()
                .initialize_node(&mut node);

            let mut guard = self.cs_v_nodes.get_mut();

            guard.nodes.push(pnode_clone);
        }
    }
}

