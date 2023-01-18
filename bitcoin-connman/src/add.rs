crate::ix!();

impl Connman {

    /**
      | Attempts to open a connection. Currently
      | only used from tests.
      | 
      | -----------
      | @param[in] address
      | 
      | Address of node to try connecting to
      | ----------
      | @param[in] conn_type
      | 
      | ConnectionType::OUTBOUND or ConnectionType::BLOCK_RELAY
      | or ConnectionType::ADDR_FETCH
      | 
      | -----------
      | @return
      | 
      | bool Returns false if there are no available
      | slots for this connection:
      | 
      | - conn_type not a supported ConnectionType
      | 
      | - Max total outbound connection capacity
      | filled
      | 
      | - Max connection capacity for type is
      | filled
      |
      */
    pub fn add_connection(&mut self, 
        address:   &String,
        conn_type: ConnectionType) -> bool {

        let mut max_connections = Option::<i32>::default();

        match conn_type {
            ConnectionType::FEELER 
                | ConnectionType::INBOUND 
                | ConnectionType::MANUAL  => {
                return false;
            },

            ConnectionType::OUTBOUND_FULL_RELAY  => {
                max_connections = Some(self.max_outbound_full_relay.load(atomic::Ordering::Relaxed));
            },

            ConnectionType::BLOCK_RELAY  => {
                max_connections = Some(self.max_outbound_block_relay.load(atomic::Ordering::Relaxed));
            },

            ConnectionType::ADDR_FETCH  => {
                // no limit for ADDR_FETCH because
                // -seednode has no limit either
            },
        }

        //  Count existing connections
        let existing_connections: i32 = {

            let mut guard = self.cs_v_nodes.get();

            guard.nodes
                .iter()
                .filter(|&n| n.get().conn_type() == Some(conn_type) )
                .count().try_into().unwrap()
        };

        // Max connections of specified type already exist
        if max_connections != None 
        && existing_connections >= max_connections.unwrap() {
            return false;
        }

        // Max total outbound connections already
        // exist
        let mut grant: SemaphoreGrant 
        = SemaphoreGrant::new(
            self.sem_outbound.clone(), 
            Some(true)
        );

        let gate: bool = (&grant).into();

        if !gate {
            return false;
        }

        self.open_network_connection(
            &Address::default(),
            false, 
            Some(&mut grant), 
            address.as_ptr() as *const u8, 
            conn_type
        );

        true
    }
}

