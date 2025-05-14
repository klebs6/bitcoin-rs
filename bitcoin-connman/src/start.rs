// ---------------- [ File: bitcoin-connman/src/start.rs ]
crate::ix!();

pub fn deserialize_filedb<Data>(
        path:    &Path,
        data:    &mut Data,
        version: i32)  {

    todo!();
        /*
            // open input file, and associate with CAutoFile
        FILE* file = fsbridge::fopen(path, "rb");
        CAutoFile filein(file, SER_DISK, version);
        if (filein.IsNull()) {
            throw DbNotFoundError{};
        }
        DeserializeDB(filein, data);
        */
}

/**
  | Read the anchor IP address database
  | (anchors.dat)
  | 
  | Deleting anchors.dat is intentional
  | as it avoids renewed peering to anchors
  | after an unclean shutdown and thus potential
  | exploitation of the anchor peer policy.
  |
  */
pub fn read_anchors(anchors_db_path: &Path) -> Vec<Address> {

    let mut anchors: Vec<Address> = vec![];

    let mut try_block = || -> TryBlockResult::<_,StdException> {

        deserialize_filedb(anchors_db_path, &mut anchors, CLIENT_VERSION | ADDRV2_FORMAT);

        log_printf!(
            "Loaded {} addresses from {}\n", 
            anchors.len(), 
            fs::quoted(fs::path_to_string(anchors_db_path.filename()))
        );

        TryBlockResult::Success
    };

    match try_block() {

        TryBlockResult::Return(v)  => return v,

        TryBlockResult::Err(e)  => {
            anchors.clear();
        },

        TryBlockResult::Break    => { }
        TryBlockResult::Success  => { }
    }

    fs::remove_file(anchors_db_path);

    anchors
}

impl Connman {

    pub fn start_extra_block_relay_peers(&mut self)  {
        
        log_print!(
            LogFlags::NET, 
            "net: enabling extra block-relay-only peers\n"
        );

        self.start_extra_block_relay_peers.store(true,atomic::Ordering::Relaxed);
    }
    
    pub fn start(
        self:         Arc<Self>, 
        scheduler:    &mut Scheduler,
        mut conn_options: ConnmanOptions) -> bool {

        self.init(&mut conn_options);

        if *LISTEN && !self.init_binds(&conn_options) {

            let mut ci = &self.client_interface;

            if ci.is_some() {

                ci.get_mut().thread_safe_message_box(
                        &untranslated("Failed to listen on any port. Use -listen=0 if you want this."), 
                        &"".to_string(), 
                        MessageBoxFlags::MSG_ERROR.bits()
                    );
            }

            return false;
        }

        let mut i2p_sam = ProxyType::default();

        if get_proxy(Network::NET_I2P, &mut i2p_sam) {

            let mut i2p_private_key_dir = G_ARGS
                .lock()
                .get_data_dir_net();

            i2p_private_key_dir.push("i2p_private_key");

            let mut sam_session = self.i2p_sam_session.get_mut();

            *sam_session = Box::<SAMSession>::new(SAMSession::new(
                &i2p_private_key_dir,
                &i2p_sam.proxy,
                self.interrupt_net.clone()
            ));
        }

        for str_dest in conn_options.seed_nodes.iter() {
            self.add_addr_fetch(str_dest);
        }

        if self.use_addrman_outgoing.load(atomic::Ordering::Relaxed) {

            let anchor_database_filename = {
                let mut builder = G_ARGS.lock().get_data_dir_net();
                builder.push(ANCHORS_DATABASE_FILENAME);
                builder
            };

            let mut anchors = self.anchors.get_mut();

            // Load addresses from anchors.dat
            *anchors = read_anchors(&anchor_database_filename);

            if anchors.len() > MAX_BLOCK_RELAY_ONLY_ANCHORS {
                anchors.resize(MAX_BLOCK_RELAY_ONLY_ANCHORS, Default::default());
            }

            log_printf!(
                "{} block-relay-only anchors will be tried for connections.\n", 
                anchors.len()
            );
        }

        let ci = &self.client_interface;

        if ci.is_some() {
            ci.get_mut().init_message("Starting network threads…");
        }

        self.addresses_initialized.store(true, atomic::Ordering::Relaxed);

        {
            let outbound = &self.sem_outbound;

            if outbound.is_none() {

                let sem_init: i32 = min(
                    self.max_outbound.load(atomic::Ordering::Relaxed),
                    self.n_max_connections.load(atomic::Ordering::Relaxed)
                );

                // initialize semaphore
                *outbound.get_mut() = Semaphore::new(sem_init);
            }
        }

        {
            let addnode = &self.sem_addnode;

            if addnode.is_none() {

                // initialize semaphore
                *addnode.get_mut() = Semaphore::new(self.n_max_addnode.load(atomic::Ordering::Relaxed));
            }
        }

        //  Start threads
        //
        assert!(self.msgproc.is_some());

        interrupt_socks5(false);

        self.interrupt_net.get_mut().reset();

        self.flag_interrupt_msg_proc
            .store(false, atomic::Ordering::Relaxed);


        {
            let mut guard = self.mutex_msg_proc.get();

            guard.msg_proc_wake.store(false, atomic::Ordering::Relaxed);
        }

        let self_handle = self.clone();

        // Send and receive from sockets, accept connections
        let mut join_handle = self.thread_socket_handler.get_mut();

        *join_handle = launch_traced_thread!{"net", || { 
            subroutine_thread_socket_handler(self_handle); 
        }}.unwrap();

        if !G_ARGS.lock().get_bool_arg("-dnsseed", DEFAULT_DNSSEED) {

            log_printf!("DNS seeding disabled\n");

        } else {

            let self_handle = self.clone();

            let mut join_handle = self.thread_dns_address_seed.get_mut();

            *join_handle = launch_traced_thread!{"dnsseed", || {
                subroutine_thread_dns_address_seed(self_handle); 
            }}.unwrap();
        }

        let self_handle = self.clone();

        let mut join_handle = self.thread_open_added_connections.get_mut();

        // Initiate manual connections
        *join_handle = launch_traced_thread!{"addcon", || { 
            subroutine_thread_open_added_connections(self_handle); 
        }}.unwrap();

        if conn_options.use_addrman_outgoing 
        && !conn_options.specified_outgoing.is_empty() {

            let mut ci = &self.client_interface;

            if ci.is_some() {

                ci.get_mut()
                    .thread_safe_message_box(
                        &untranslated("Cannot provide specific connections and have addrman find outgoing connections at the same."), 
                        "", 
                        MessageBoxFlags::MSG_ERROR.bits()
                    );
            }

            return false;
        }

        if conn_options.use_addrman_outgoing 
        || !conn_options.specified_outgoing.is_empty() {

            let self_handle = self.clone();

            let mut join_handle = self.thread_open_connections.get_mut();

            *join_handle = launch_traced_thread!{"opencon", || { 
                let connect = conn_options.specified_outgoing; 
                subroutine_thread_open_connections(self_handle,connect); 
            }}.unwrap();
        }

        let self_handle = self.clone();

        //  Process messages
        let mut join_handle = self.thread_message_handler.get_mut();

        *join_handle = launch_traced_thread!{"msghand", || { 
            subroutine_thread_message_handler(self_handle); 
        }}.unwrap();

        if conn_options.i2p_accept_incoming && self.i2p_sam_session.is_some() {

            let self_handle = self.clone();

            let mut join_handle = self.thread_i2p_accept_incoming.get_mut();

            *join_handle = launch_traced_thread!{"i2paccept", || { 
                subroutine_thread_i2p_accept_incoming(self_handle); 
            }}.unwrap();
        }

        let self_handle = self.clone();

        let dump_handle = Box::new(move || {
            subroutine_dump_addresses(self_handle.clone());
        });

        //  Dump network addresses
        scheduler.schedule_every(
            dump_handle, 
            DUMP_PEERS_INTERVAL
        );

        true
    }
}
