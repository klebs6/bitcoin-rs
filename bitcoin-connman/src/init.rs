crate::ix!();

impl Connman {
    
    pub fn init(&self, conn_options: &mut ConnmanOptions)  {

        *self.n_local_services.get_mut() = conn_options.n_local_services;

        self.n_max_connections.store(
            conn_options.n_max_connections, 
            atomic::Ordering::Relaxed
        );

        self.max_outbound_full_relay.store(
            min(conn_options.max_outbound_full_relay,conn_options.n_max_connections), 
            atomic::Ordering::Relaxed
        );

        self.max_outbound_block_relay.store(
            conn_options.max_outbound_block_relay, 
            atomic::Ordering::Relaxed
        );

        self.use_addrman_outgoing.store(
            conn_options.use_addrman_outgoing, 
            atomic::Ordering::Relaxed
        );

        self.n_max_addnode.store(
            conn_options.n_max_addnode, 
            atomic::Ordering::Relaxed
        );

        self.n_max_feeler.store(
            conn_options.n_max_feeler, 
            atomic::Ordering::Relaxed
        );

        self.max_outbound.store(
            self.max_outbound_full_relay.load(atomic::Ordering::Relaxed) 
            + self.max_outbound_block_relay.load(atomic::Ordering::Relaxed) 
            + self.n_max_feeler.load(atomic::Ordering::Relaxed), 

            atomic::Ordering::Relaxed
        );

        self.client_interface.load(conn_options.ui_interface.take());
        self.banman.load(conn_options.banman.take());
        self.msgproc.load(conn_options.msgproc.take());

        self.n_send_buffer_max_size.store(
            conn_options.n_send_buffer_max_size, 
            atomic::Ordering::Relaxed
        );

        self.n_receive_flood_size.store(
            conn_options.n_receive_flood_size, 
            atomic::Ordering::Relaxed
        );

        *self.peer_connect_timeout.get_mut() = conn_options.peer_connect_timeout;

        self.cs_total_bytes_sent
            .get_mut()
            .n_max_outbound_limit = conn_options.n_max_outbound_limit.clone();

        self.whitelisted_range.load(conn_options.whitelisted_range.take());

        self.cs_v_added_nodes
            .get_mut()
            .added_nodes = conn_options.added_nodes.clone();

        self.onion_binds.load(Some(conn_options.onion_binds.clone()));
    }
}
