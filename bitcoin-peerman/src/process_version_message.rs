// ---------------- [ File: bitcoin-peerman/src/process_version_message.rs ]
crate::ix!();

/**
  | vote for a local address
  |
  */
pub fn seen_local(addr: &Service) -> bool {
    
    todo!();
        /*
            LOCK(cs_mapLocalHost);
        const auto it = mapLocalHost.find(addr);
        if (it == mapLocalHost.end()) return false;
        ++it->second.nScore;
        return true;
        */
}

impl PeerManager {

    pub fn process_version_message(
        self:               Arc<Self>, 
        peer:               &mut Option<Peer>,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if pfrom.n_version() != 0 {
            log_print!(LogFlags::NET, "redundant version message from peer=%d\n", pfrom.get_id());
            return;
        }

        let mut n_time = i64::default();

        let mut addr_me = Service::default();

        let mut n_nonce: u64 = 1;

        let mut n_services = ServiceFlags::default();

        let mut n_version = i32::default();

        let mut clean_sub_ver = String::default();

        let mut starting_height: i32 = -1;

        let mut relay: bool = true;

        recv.stream_into(&mut n_version);
        recv.stream_into(&mut CustomUintFormatter::<ServiceFlags, 8>{ item: &mut n_services });
        recv.stream_into(&mut n_time);

        if n_time < 0 {
            n_time = 0;
        }

        // Ignore the addrMe service bits sent by
        // the peer
        recv.ignore(8);

        recv.stream_into(&mut addr_me);

        if !pfrom.is_inbound_conn() {
            self.addrman.get_mut()
                .set_services(
                    pfrom.service(), 
                    n_services
                );
        }

        if pfrom.expect_services_from_conn() 
        && !has_all_desirable_service_flags(n_services) 
        {
            log_print!(
                LogFlags::NET, 
                "peer=%d does not offer the expected services (%08x offered, %08x expected); disconnecting\n", 
                pfrom.get_id(), 
                n_services, 
                get_desirable_service_flags(n_services)
            );

            pfrom.mark_for_disconnect();

            return;
        }

        if n_version < MIN_PEER_PROTO_VERSION {

            // disconnect from peers older
            // than this proto version
            log_print!(
                LogFlags::NET,
                "peer=%d using obsolete version %i; disconnecting\n",
                pfrom.get_id(),
                n_version
            );

            pfrom.mark_for_disconnect();

            return;
        }

        if !recv.empty() {
            // The version message includes
            // information about the sending
            // node which we don't use:
            //
            //    - 8 bytes (service bits)
            //    - 16 bytes (ipv6 address)
            //    - 2 bytes (port)
            recv.ignore(26);
            recv.stream_into(&mut n_nonce);
        }

        if !recv.empty() {

            let mut str_sub_ver = String::default();

            let mut limited_str = limited_string!(&mut str_sub_ver,MAX_SUBVERSION_LENGTH);

            recv.stream_into(&mut limited_str);

            clean_sub_ver = sanitize_string(&str_sub_ver, None);
        }

        if !recv.empty() {
            recv.stream_into(&mut starting_height);
        }

        if !recv.empty() {
            recv.stream_into(&mut relay);
        }

        // Disconnect if we connected to
        // ourself
        if pfrom.is_inbound_conn() 
        && !self.connman.get_mut().check_incoming_nonce(n_nonce) 
        {
            log_printf!(
                "connected to self at %s, disconnecting\n", 
                pfrom.addr.to_string()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        if pfrom.is_inbound_conn() && addr_me.is_routable() {
            seen_local(&addr_me);
        }

        // Inbound peers send us their version
        // message when they connect.
        //
        // We send our version message in
        // response.
        if pfrom.is_inbound_conn() {
            self.push_node_version(pfrom, &get_adjusted_datetime());
        }

        //  Change version
        let greatest_common_version: i32 = min(n_version,PROTOCOL_VERSION);

        pfrom.set_common_version(greatest_common_version);
        pfrom.set_n_version(n_version);

        let msg_maker: NetMsgMaker = NetMsgMaker::new(greatest_common_version);

        if greatest_common_version >= WTXID_RELAY_VERSION {
            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(NetMsgType::WTXIDRELAY, &[])
            );
        }

        // Signal ADDRv2 support (BIP155).
        if greatest_common_version >= 70016 {

            // BIP155 defines addrv2 and
            // sendaddrv2 for all protocol
            // versions, but some implementations
            // reject messages they don't know. As
            // a courtesy, don't send it to nodes
            // with a version before 70016, as no
            // software is known to support BIP155
            // that doesn't announce at least that
            // protocol version number.
           self.connman.get_mut()
                .push_message(
                    &mut *pfrom, 
                    msg_maker.make(NetMsgType::SENDADDRV2, &[])
                );
        }

        self.connman.get_mut().push_message(
            &mut *pfrom, 
            msg_maker.make(NetMsgType::VERACK, &[])
        );

        pfrom.set_n_services(n_services);

        pfrom.set_addr_local(&addr_me);

        pfrom.set_clean_sub_ver(&clean_sub_ver);

        peer.as_ref().unwrap().starting_height.store(starting_height, atomic::Ordering::Relaxed);

        let is_client = {

            let b0 = (n_services & ServiceFlags::NODE_NETWORK).bits() == 0;
            let b1 = (n_services & ServiceFlags::NODE_NETWORK_LIMITED).bits() == 0;

            b0 && b1
        };

        // set nodes not relaying blocks and tx
        // and not serving (parts) of the
        // historical blockchain as "clients"
        pfrom.set_is_client(is_client);

        let limited_node = {
            let f0 = n_services & ServiceFlags::NODE_NETWORK;
            let f1 = n_services & ServiceFlags::NODE_NETWORK_LIMITED;

            f0.bits() == 0 && f1.bits() != 0
        };

        // set nodes not capable of serving the
        // complete blockchain history as "limited
        // nodes"
        pfrom.set_limited_node(limited_node);

        if pfrom.has_tx_relay() {

            let tx_relay = pfrom.get_tx_relay();

            let mut guard = tx_relay.cs_filter.lock();

            // set to true after we get the first
            // filter* message
            guard.relay_txes = relay;
        }

        if (n_services & ServiceFlags::NODE_WITNESS).bits() != 0 {

            let mut guard = CS_MAIN.lock();

            let cstate = create_state(pfrom.get_id());

            let state = cstate.get();

            state.have_witness.store(true, atomic::Ordering::Relaxed);
        }

        // Potentially mark this peer as
        // a preferred download peer.
        {
            let mut guard = CS_MAIN.lock();

            update_preferred_download(&***pfrom, create_state(pfrom.get_id()));
        }

        // Self advertisement & GETADDR logic
        if !pfrom.is_inbound_conn() 
        && self.clone().setup_address_relay(&***pfrom, peer.as_mut().unwrap()) {

            // For outbound peers, we try to relay
            // our address (so that other nodes
            // can try to find us more quickly, as
            // we have no guarantee that an
            // outbound peer is even aware of how
            // to reach us) and do a one-time
            // address fetch (to help
            // populate/update our addrman). If
            // we're starting up for the first
            // time, our addrman may be pretty
            // empty and no one will know who we
            // are, so these mechanisms are
            // important to help us connect to the
            // network.
            //
            // We skip this for block-relay-only
            // peers. We want to avoid potentially
            // leaking addr information and we do
            // not want to indicate to the peer
            // that we will participate in addr
            // relay.
            if *LISTEN 
            && !self.chainman
                .get()
                .active_chainstate()
                .is_initial_block_download() 
            {
                let mut addr: Address = get_local_address(
                    &pfrom.service().base,
                    pfrom.get_local_services()
                );

                let mut insecure_rand = FastRandomContext::default();

                if addr.is_routable() {

                    log_print!(
                        LogFlags::NET, 
                        "ProcessMessages: advertising address %s\n", 
                        addr.to_string()
                    );

                    peer.as_mut().unwrap().push_address(
                        &addr, 
                        &mut insecure_rand
                    );

                } else {

                    if is_peer_addr_local_good(&mut ***pfrom) {

                        addr.setip(&addr_me.base);

                        log_print!(
                            LogFlags::NET, 
                            "ProcessMessages: advertising address %s\n", 
                            addr.to_string()
                        );

                        peer.as_mut().unwrap().push_address(
                            &addr, 
                            &mut insecure_rand
                        );
                    }
                }
            }

            // Get recent addresses
            self.connman.get_mut().push_message(
                &mut *pfrom, 
                NetMsgMaker::new(greatest_common_version)
                    .make(NetMsgType::GETADDR, &[])
            );

            peer.as_mut().unwrap().getaddr_sent = true;

            // When requesting a getaddr, accept
            // an additional MAX_ADDR_TO_SEND
            // addresses in response (bypassing
            // the
            // MAX_ADDR_PROCESSING_TOKEN_BUCKET
            // limit).
            peer.as_mut().unwrap().addr_token_bucket += MAX_ADDR_TO_SEND as f64;
        }

        if !pfrom.is_inbound_conn() {

            // For non-inbound connections, we
            // update the addrman to record
            // connection success so that addrman
            // will have an up-to-date notion of
            // which peers are online and
            // available.
            //
            // While we strive to not leak
            // information about block-relay-only
            // connections via the addrman, not
            // moving an address to the tried
            // table is also potentially
            // detrimental because new-table
            // entries are subject to eviction in
            // the event of addrman collisions. We
            // mitigate the information-leak by
            // never calling AddrMan::Connected()
            // on block-relay-only peers; see
            // FinalizeNode().
            //
            // This moves an address from New to
            // Tried table in Addrman, resolves
            // tried-table collisions, etc.
            self.addrman.get_mut().good(&pfrom.service(), None);
        }

        let mut remote_addr = String::default();

        if *LOG_IPS {

            remote_addr 
                = ", peeraddr=".to_owned() 
                + &pfrom.addr().to_string();
        }

        log_print!(
            LogFlags::NET, 
            "receive version message: %s: version %d, blocks=%d, us=%s, txrelay=%d, peer=%d%s\n", 
            clean_sub_ver, 
            pfrom.n_version, 
            (*peer).starting_height, 
            addr_me.to_string(), 
            relay, 
            pfrom.get_id(), 
            remote_addr
        );

        let n_time_offset: Duration = OffsetDateTime::from_unix_timestamp(n_time).unwrap() - get_datetime();

        pfrom.set_n_time_offset(Some(n_time_offset));

        add_time_data(
            &pfrom.service().base, 
            n_time_offset
        );

        // If the peer is old enough to have the
        // old alert system, send it the final
        // alert.
        if greatest_common_version <= 70012 {

            let final_alert: DataStream = DataStream::new_with_slice(
                &parse_hex("60010000000000000000000000ffffff7f00000000ffffff7ffeffff7f01ffffff7f00000000ffffff7f00ffffff7f002f555247454e543a20416c657274206b657920636f6d70726f6d697365642c2075706772616465207265717569726564004630440220653febd6410f470f6bae11cad19c48413becb1ac2c17f908fd0fd53bdc3abd5202206d0e9c96fe88d4a0f01ed9dedae2b6f9e00da94cad0fecaae66ecf689bf71b50"), 
                SER_NETWORK, 
                PROTOCOL_VERSION
            );

            self.connman.get_mut().push_message(
                &mut *pfrom, 
                NetMsgMaker::new(greatest_common_version)
                    .make(
                        "alert", 
                        &[&final_alert]
                    )
            );
        }

        // Feeler connections exist only to verify
        // if address is online.
        if pfrom.is_feeler_conn() {

            log_print!(
                LogFlags::NET, 
                "feeler connection completed peer=%d; disconnecting\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();
        }
    }
}
