crate::ix!();

/**
  | Dump the anchor IP address database
  | (anchors.dat)
  | 
  | Anchors are last known outgoing block-relay-only
  | peers that are tried to re-connect to
  | on startup.
  |
  */
pub fn dump_anchors(
        anchors_db_path: &Path,
        anchors:         &Vec<Address>)  {
    
    log_time_seconds!(
        format!("Flush {} outbound block-relay-only peer addresses to anchors.dat",anchors.size())
    );

    serialize_filedb(
        "anchors", 
        anchors_db_path, 
        anchors, 
        CLIENT_VERSION | ADDRV2_FORMAT
    );
}

impl Connman {
    
    pub fn join_threads(self: Arc<Self>)  {

        macro_rules! join { ($t:tt) => { self.$t.take().unwrap().join(); } }

        join!{thread_i2p_accept_incoming};
        join!{thread_message_handler};
        join!{thread_open_connections};
        join!{thread_open_added_connections};
        join!{thread_dns_address_seed};
        join!{thread_socket_handler};
    }
    
    pub fn stop_nodes(self: Arc<Self>)  {

        if self.addresses_initialized.load(atomic::Ordering::Relaxed) {

            subroutine_dump_addresses(self.clone());

            self.addresses_initialized.store(false, atomic::Ordering::Relaxed);

            if self.use_addrman_outgoing.load(atomic::Ordering::Relaxed) {

                // Anchor connections are only
                // dumped during clean shutdown.
                let mut anchors_to_dump: Vec::<Address> = self.get_current_block_relay_only_conns();

                if anchors_to_dump.len() > MAX_BLOCK_RELAY_ONLY_ANCHORS {
                    anchors_to_dump.resize(MAX_BLOCK_RELAY_ONLY_ANCHORS, Default::default());
                }

                let mut anchors_dbfile = G_ARGS
                    .lock()
                    .get_data_dir_net();

                anchors_dbfile.push(ANCHORS_DATABASE_FILENAME);

                dump_anchors(&anchors_dbfile, &anchors_to_dump);
            }
        }

        // Delete peer connections.
        let mut nodes = Vec::<Amo<Box<dyn NodeInterface>>>::default();

        {
            let mut guard = self.cs_v_nodes.get_mut();

            guard.nodes = nodes.clone();
        }

        for pnode in nodes.drain(..) {

            pnode.get_mut().close_socket_disconnect();

            self.delete_node(pnode);
        }

        // Close listening sockets.
        for h_listen_socket in self.vh_listen_socket.get_mut().iter_mut() {

            if h_listen_socket.socket != INVALID_SOCKET {

                if !close_socket(&mut h_listen_socket.socket) {
                    log_printf!(
                        "CloseSocket(hListenSocket) failed with error %s\n", 
                        network_error_string(wsa_get_last_error())
                    );
                }
            }
        }

        let to_delete: Vec<Amo<Box<dyn NodeInterface>>> 
        = self.nodes_disconnected.get_mut().drain(..).collect();

        for pnode in to_delete.iter() {
            self.delete_node(pnode.clone());
        }

        self.nodes_disconnected.get_mut().clear();
        self.vh_listen_socket.get_mut().clear();
        self.sem_outbound.take();
        self.sem_addnode.take();
    }

    pub fn stop(self: Arc<Self>)  {
        
        self.clone().join_threads();
        self.clone().stop_nodes();
    }
}

