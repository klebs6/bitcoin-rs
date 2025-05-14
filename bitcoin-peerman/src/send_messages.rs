// ---------------- [ File: bitcoin-peerman/src/send_messages.rs ]
crate::ix!();

pub type Inventory = Vec<Inv>;

pub enum ControlFlow {
    Return,
    None,
    Break,
    Continue,
}

impl SendMessages for PeerManager {
    
    #[EXCLUSIVE_LOCKS_REQUIRED(pto->cs_sendProcessing)]
    fn send_messages(
        self:      Arc<Self>, 
        pto:       Amo<Box<dyn NodeInterface>>) -> bool {

        let node = pto.get();

        let peer: Amo<Peer> = {

            self.get_peer_ref(node.get_id())
        };

        if peer.is_none() {
            return false;
        }

        let consensus_params: Arc<ChainConsensusParams> 
        = self.chainparams.get_consensus();

        // We must call
        // MaybeDiscourageAndDisconnect first, to
        // ensure that we'll disconnect
        // misbehaving peers even before the
        // version handshake is complete.
        if self.clone().maybe_discourage_and_disconnect(pto.clone(),&mut peer.get_mut()) {
            return true;
        }

        // Don't send anything until the version
        // handshake is complete
        if !node.is_successfully_connected() || node.marked_for_disconnect() {
            return true;
        }

        let common_version = node.get_common_version();

        // If we get here, the outgoing message
        // serialization version is set and can't
        // change.
        let msg_maker: NetMsgMaker = NetMsgMaker::new( common_version );

        let current_time = get_datetime();

        let time_connected = node.get_n_time_connected();

        if node.is_addr_fetch_conn() 
        && current_time - time_connected > 10_i32 * AVG_ADDRESS_BROADCAST_INTERVAL 
        {
            log_print!(
                LogFlags::NET, 
                "addrfetch connection timeout; disconnecting peer=%d\n", 
                node.get_id()
            );

            node.mark_for_disconnect();

            return true;
        }

        self.clone().maybe_send_ping(pto.clone(), peer.clone(), current_time);

        // MaybeSendPing may have marked peer for
        // disconnection
        //
        if node.marked_for_disconnect() {
            return true;
        }
        
        self.clone().maybe_send_addr(
            pto.clone(), 
            peer.getopt_mut().as_mut().unwrap(), 
            current_time
        );

        {
            let mut guard = CS_MAIN.lock();

            self.clone().protected_send_messages(
                pto.clone(), 
                peer, 
                &msg_maker,
                consensus_params
            );

            //  release CS_MAIN
        }

        true
    }
}

impl PeerManager {

    fn expire_old_relay_messages(
        self:   Arc<Self>, 
        txinfo: &TxMemPoolInfo, 
        txid:   &u256,
        wtxid:  &u256) {

        let current_time = get_datetime();

        // Expire old relay messages
        while !self.inner.lock().g_relay_expiration.is_empty() 
        && self.inner.lock().g_relay_expiration[0].0 < current_time 
        {
            self.inner.lock().map_relay.remove(
                &self.inner.lock().g_relay_expiration[0].1.0
            );

            self.inner.lock().g_relay_expiration.pop_front();
        }

        let new_value = txinfo.tx.clone(); //move

        if let Some(old) = self.inner.lock().map_relay.insert(
            txid.clone(), 
            new_value.clone()) 
        {

            //nothing

        } else {

                self.inner.lock().g_relay_expiration.push_back(
                    (
                        current_time + RELAY_TX_CACHE_TIME, 

                        (Arc::new(txid.clone()), new_value.clone())
                    )
                );

            }

        // Add wtxid-based lookup into
        // mapRelay as well, so that peers
        // can request by wtxid
        if let Some(ret2) = self.inner.lock().map_relay.insert(wtxid.clone(), new_value) {

            self.inner.lock().g_relay_expiration.push_back(
                (
                    current_time + RELAY_TX_CACHE_TIME, 
                    (Arc::new(wtxid.clone()), ret2)
                )
            );
        }
    }

    fn protected_send_messages(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>, 
        peer:             Amo<Peer>, 
        msg_maker:        &NetMsgMaker,
        consensus_params: Arc<ChainConsensusParams>,
    ) -> ControlFlow {

        let current_time = get_datetime();

        let node = pnode.get();

        let pstate: Amo<NodeState> 
        = create_state(node.get_id());

        // Start block sync
        if PINDEX_BEST_HEADER.lock().is_none() {

            self.clone().start_block_sync();
        }

        // Download if this is a nice peer, or we
        // have no nice peers and this one might
        // do.
        let fetch: bool 
        = pstate.get().preferred_download.load(atomic::Ordering::Relaxed) 
        || (
            N_PREFERRED_DOWNLOAD.load(atomic::Ordering::Relaxed) == 0 
            && !node.is_client() && !node.is_addr_fetch_conn()
        );

        let state = pstate.get();

        if !state.sync_started.load(atomic::Ordering::Relaxed) 
        && !node.is_client() 
        && !IMPORTING.load(atomic::Ordering::Relaxed) 
        && !REINDEX.load(atomic::Ordering::Relaxed) 
        {
            self.clone().handle_not_sync_started(
                pnode.clone(),
                pstate.clone(),
                msg_maker,
                fetch,
                consensus_params.clone()
            );
        }

        self.clone().try_sending_block_announcements_via_headers(
            pnode.clone(),
            pstate.clone(),
            peer.clone(),
            &msg_maker,
            consensus_params.clone()
        );

        //  Message: inventory
        let mut inventory = self.clone().handle_message_inventory(
            pnode.clone(),
            peer.get(),
            msg_maker
        );

        if node.has_tx_relay() {

            self.clone().handle_tx_relay(
                &mut pnode.get_mut(), 
                pstate.clone(), 
                &mut inventory, 
                msg_maker
            );
        }

        if !inventory.is_empty() {

            self.connman.get_mut()
                .push_message(
                    &mut *pnode.get_mut(), 
                    msg_maker.make(NetMsgType::INV, 
                        &[
                            &inventory
                        ]
                    )
                );
        }

        if pstate.get().detect_stalling(current_time) {

            node.disconnect_on_stall();

            return ControlFlow::Return;
        }

        // In case there is a block that has been
        // in flight from this peer for
        // block_interval * (1 + 0.5 * N) (with
        // N the number of peers from which we're
        // downloading validated blocks),
        // disconnect due to timeout.
        //
        // We compensate for other peers to
        // prevent killing off peers due to our
        // own downstream link being saturated. We
        // only count validated in-flight blocks
        // so peers can't advertise non-existing
        // blockhashes to unreasonably increase
        // our timeout.
        if pstate.get().blocks_in_flight.len() > 0 {

            match self.clone().disconnect_timedout_blocks_in_flight(
                pnode.clone(),
                pstate.clone(),
                consensus_params.clone()
            ) {
                ControlFlow::Return => return ControlFlow::Return,
                ControlFlow::None   => {}
                _                   => panic!("unexpected control flow")
            }
        }

        self.clone().check_for_headers_sync_timeouts(
            pnode.clone(),
            pstate.clone()
        );

        // Check that outbound peers have
        // reasonable chains
        //
        // GetTime() is used by this anti-DoS
        // logic so we can test this using
        // mocktime
        self.clone().consider_eviction(
            pnode.clone(),
            get_datetime()
        );

        // Message: getdata (blocks)
        //
        let mut get_data: Vec<Inv> = vec![];

        if !node.is_client() 
        && ((fetch && !node.is_limited_node()) || !self.chainman.get().active_chainstate().is_initial_block_download()) 
        && pstate.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) < MAX_BLOCKS_IN_TRANSIT_PER_PEER {

            self.clone().handle_message_getdata_blocks(pnode.clone(),pstate.clone(),&mut get_data);
        }

        // Message: getdata (transactions)
        //
        self.clone().handle_message_getdata_transactions(
            pnode.clone(),
            msg_maker,
            &mut get_data
        );

        self.maybe_send_feefilter(
            pnode.clone(), 
            current_time
        );

        ControlFlow::None
    }

    fn handle_not_sync_started(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>,
        pstate:           Amo<NodeState>,
        msg_maker:        &NetMsgMaker,
        fetch:            bool,
        consensus_params: Arc<ChainConsensusParams>,
    ) {

        let current_time = get_datetime();

        // Only actively request headers from
        // a single peer, unless we're close to
        // today.
        if (self.inner.lock().n_sync_started == 0 && fetch) 
        || (*PINDEX_BEST_HEADER.lock()).as_ref().unwrap().get_block_time() > get_adjusted_time() - 24 * 60 * 60 
        {
            pstate.get().sync_started.store(true, atomic::Ordering::Relaxed);

            // Convert
            // HEADERS_DOWNLOAD_TIMEOUT_PER_HEADER
            // to microseconds before
            // scaling to maintain
            // precision
            pstate.get_mut().headers_sync_timeout 
                = Some(
                    current_time 
                    + HEADERS_DOWNLOAD_TIMEOUT_BASE 
                    + {
                        let timeout    = HEADERS_DOWNLOAD_TIMEOUT_PER_HEADER;
                        let block_time = (*PINDEX_BEST_HEADER.lock()).as_ref().unwrap().get_block_time();
                        let time       = get_adjusted_time() - block_time;
                        let spacing: i32    = consensus_params.n_pow_target_spacing.try_into().unwrap();

                        timeout.checked_mul(time.try_into().unwrap()).unwrap() / spacing
                    }
                );

            {
                let old = self.inner.lock().n_sync_started;
                self.inner.lock().n_sync_started += 1;
                old
            };

            let mut pindex_start: Option<Arc<BlockIndex>> = PINDEX_BEST_HEADER.lock().clone();

            /*
             | If possible, start at the block preceding
             | the currently best known header. This
             | ensures that we always get a non-empty
             | list of headers back as long as the peer
             | is up-to-date. With a non-empty response,
             | we can initialise the peer's known best
             | block. This wouldn't be possible if
             | we requested starting at pindexBestHeader
             | and got back an empty response.
             |
             */
            if pindex_start.as_ref().unwrap().pprev.is_some() {

                pindex_start = pindex_start.as_ref().unwrap().pprev.clone();
            }

            log_print!(
                LogFlags::NET, 
                "initial getheaders (%d) to peer=%d (startheight:%d)\n", 
                pindex_start.unwrap().lock().n_height, 
                (*pto).get_id(), 
                peer.as_ref().unwrap().starting_height
            );

            self.connman.get_mut().push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make(
                    NetMsgType::GETHEADERS, 
                    &[
                        &self.chainman.get().active_chain().get_locator(pindex_start), 
                        &u256::default()
                    ]
                )
            );
        }
    }

    fn handle_message_inventory<'a>(
        self:      Arc<Self>, 
        pnode:     Amo<Box<dyn NodeInterface>>,
        peer:      AmoReadGuard<'a, Peer>,
        msg_maker: &NetMsgMaker
    ) -> Inventory {

        let mut inventory: Vec<Inv> = vec![];

        let mut guard = peer.block_inv_mutex.lock();

        inventory.reserve(
            max(
                peer.block_inv_mutex.lock().blocks_for_inv_relay.len(),
                (*INVENTORY_BROADCAST_MAX).try_into().unwrap()
            )
        );

        //  Add blocks
        for hash in peer.block_inv_mutex.lock().blocks_for_inv_relay.iter() {

            inventory.push(Inv::new(GetDataMsg::MSG_BLOCK.bits(),hash));

            if inventory.len() == MAX_INV_SZ.try_into().unwrap() {

                self.connman.get_mut().push_message(
                    &mut *pnode.get_mut(), 
                    msg_maker.make(
                        NetMsgType::INV, 
                        &[
                            &inventory
                        ]
                    )
                );

                inventory.clear();
            }
        }

        peer
            .block_inv_mutex.lock()
            .blocks_for_inv_relay.clear();

        inventory
    }

    fn handle_tx_relay(
        self:      Arc<Self>, 
        node:      &mut AmoWriteGuard<Box<dyn NodeInterface>>, 
        pstate:    Amo<NodeState>,
        inventory: &mut Vec<Inv>,
        msg_maker: &NetMsgMaker,
    ) {

        let current_time = get_datetime();

        let (send_trickle, send_mempool) = {

            let mut tx_relay = node.get_tx_relay_mut();

            let tx_inventory_guard = tx_relay.cs_tx_inventory.lock();

            // Check whether periodic sends should
            // happen
            let mut send_trickle: bool = node.has_permission(NetPermissionFlags::NoBan);

            let n_next_inv_send = tx_relay.n_next_inv_send.lock().unwrap();

            if n_next_inv_send < current_time {

                self.clone().update_n_next_inv_send(
                    node,
                    &mut send_trickle
                );
            }

            // Time to send but the peer has requested
            // we not relay transactions.
            if send_trickle {

                let mut filter_guard = tx_relay.cs_filter.lock();

                if !filter_guard.relay_txes {
                    tx_relay.set_inventory_tx_to_send.lock().clear();
                }
            }

            let send_mempool = tx_inventory_guard.send_mempool.clone();

            (send_trickle, send_mempool)
        };

        // Respond to BIP35 mempool requests
        if send_trickle && send_mempool {

            self.clone().respond_to_bip35_mempool_requests(
                node,
                pstate.clone(),
                inventory,
                msg_maker
            );
        }

        // Determine transactions to relay
        if send_trickle {

            self.determine_transactions_to_relay(
                node,
                pstate.clone(),
                inventory,
                msg_maker
            );
        }
    }

    fn update_n_next_inv_send(
        self:         Arc<Self>, 
        node:         &AmoWriteGuard<Box<dyn NodeInterface>>, 
        send_trickle: &mut bool) {

        let current_time = get_datetime();

        *send_trickle = true;

        if node.is_inbound_conn() {

            let tx_relay = node.get_tx_relay_mut();

            let mut n_next_inv_send = tx_relay.n_next_inv_send.lock();

            *n_next_inv_send = Some(
                self.connman.get_mut()
                .poisson_next_send_inbound(
                    current_time, 
                    INBOUND_INVENTORY_BROADCAST_INTERVAL
                )
            );

        } else {

            let tx_relay = node.get_tx_relay_mut();

            let mut n_next_inv_send = tx_relay.n_next_inv_send.lock();

            *n_next_inv_send = Some(
                poisson_next_send(
                    current_time,
                    OUTBOUND_INVENTORY_BROADCAST_INTERVAL
                )
            );
        }
    }

    fn handle_message_getdata_transactions(
        self:      Arc<Self>, 
        pnode:     Amo<Box<dyn NodeInterface>>,
        msg_maker: &NetMsgMaker,
        get_data:  &mut Vec<Inv>,
    ) {

        let current_time = get_datetime();

        let mut expired: Amo<Vec<(NodeId,GenTxId)>> = Amo::from(vec![]);

        let requestable = self.inner.lock().txrequest.lock().get_requestable(
            pnode.get().get_id(), 
            current_time, 
            expired.clone()
        );

        for entry in expired.get().iter() {

            log_print!(
                LogFlags::NET, 
                "timeout of inflight %s %s from peer=%d\n", 
                match entry.second.is_wtxid() {
                    true   => "wtx",
                    false  => "tx"
                }, 
                entry.second().get_hash().to_string(), 
                entry.first
            );
        }

        for gtxid in requestable.iter() {

            if !self.clone().already_have_tx(gtxid) {

                self.clone().download_txn(
                    pnode.clone(), 
                    msg_maker, 
                    get_data, 
                    &gtxid
                );

            } else {

                // We have already seen this
                // transaction, no need to
                // download. This is just
                // a belt-and-suspenders, as this
                // should already be called
                // whenever a transaction becomes
                // AlreadyHaveTx().
                self.inner.lock().txrequest.lock().forget_tx_hash(gtxid.get_hash());
            }
        }

        if !get_data.is_empty() {

            self.connman.get_mut().push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make(
                    NetMsgType::GETDATA, 
                    &[
                        get_data
                    ]
                )
            );
        }
    }

    fn handle_message_getdata_blocks(
        self:     Arc<Self>, 
        pnode:    Amo<Box<dyn NodeInterface>>, 
        pstate:   Amo<NodeState>,
        get_data: &mut Vec<Inv>,
    ) {
        let current_time = get_datetime();

        let mut to_download: Vec<Option<Arc<BlockIndex>>> = vec![];

        let mut staller: NodeId = -1;

        self.clone().find_next_blocks_to_download(
            pnode.get().get_id(), 
            (MAX_BLOCKS_IN_TRANSIT_PER_PEER - pstate.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed)).try_into().unwrap(), 
            &mut to_download, 
            &mut staller
        );

        for pindex in to_download.iter() {

            let n_fetch_flags: GetDataMsg 
                = get_fetch_flags(&**pnode.get());

            get_data.push(
                Inv::new(
                    (GetDataMsg::MSG_BLOCK | n_fetch_flags).bits(),
                    &pindex.clone().unwrap().get_block_hash()
                )
            );

            self.clone().block_requested(
                pnode.get().get_id(), 
                pindex.clone(),
                amo_none()
            );

            log_print!(
                LogFlags::NET, 
                "Requesting block %s (%d) peer=%d\n", 
                pindex.unwrap().get_block_hash().to_string(), 
                pindex.unwrap().n_height, 
                (*pto).get_id()
            );
        }

        if pstate.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) == 0 && staller != -1 {

            let state = create_state(staller);

            if pstate.get().stalling_since == OffsetDateTime::from_unix_timestamp(0).ok() {

                state.get_mut().stalling_since = Some(current_time);

                log_print!(
                    LogFlags::NET, 
                    "Stall started peer=%d\n", 
                    staller
                );
            }
        }
    }

    fn download_txn(
        self:      Arc<Self>, 
        pnode:     Amo<Box<dyn NodeInterface>>,
        msg_maker: &NetMsgMaker,
        get_data:  &mut Vec<Inv>,
        gtxid:     &GenTxId,
    ) {

        let current_time = get_datetime();

        log_print!(
            LogFlags::NET, 
            "Requesting %s %s peer=%d\n", 
            match gtxid.is_wtxid() {
                true   => "wtx",
                false  => "tx"
            }, 
            gtxid.get_hash().to_string(), (*pto).get_id()
        );

        get_data.push(
            Inv::new(
                match gtxid.is_wtxid() {
                    true   => GetDataMsg::MSG_WTX.bits(),
                    false  => {
                        let msg_tx = GetDataMsg::MSG_TX;
                        let flags  = get_fetch_flags(&**pnode.get());
                        (msg_tx | flags).bits()
                    }
                }, 
                gtxid.get_hash()
            )
        );

        if get_data.len() >= MAX_GETDATA_SZ.try_into().unwrap() {

            self.connman.get_mut().push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make(
                    NetMsgType::GETDATA, 
                    &[
                        get_data
                    ]
                )
            );

            get_data.clear();
        }

        self.inner.lock().txrequest.lock().requested_tx(
            pnode.get().get_id(), 
            gtxid.get_hash(), 
            current_time + GETDATA_TX_INTERVAL
        );
    }

    fn check_for_headers_sync_timeouts(
        self:   Arc<Self>, 
        pnode:  Amo<Box<dyn NodeInterface>>,
        pstate: Amo<NodeState>) 
    {
        // Check for headers sync timeouts
        if pstate.get().sync_started.load(atomic::Ordering::Relaxed) 
        && pstate.get().headers_sync_timeout.is_some() {

            // Detect whether this is a stalling
            // initial-headers-sync peer
            if (*PINDEX_BEST_HEADER.lock()).as_ref().unwrap().get_block_time() <= get_adjusted_time() - 24 * 60 * 60 {

                self.handle_stalling_initial_headers_sync_peer(pnode,pstate);

            } else {

                // After we've caught up once,
                // reset the timeout so we can't
                // trigger disconnect later.
                pstate.get_mut().headers_sync_timeout = None;
            }
        }
    }

    fn handle_stalling_initial_headers_sync_peer(
        self:   Arc<Self>, 
        pnode:  Amo<Box<dyn NodeInterface>>,
        pstate: Amo<NodeState>) {

        let current_time = get_datetime();

        if current_time > pstate.get().headers_sync_timeout.unwrap() 
        && self.inner.lock().n_sync_started == 1 
        && {

            let n_preferred_dl = N_PREFERRED_DOWNLOAD.load(atomic::Ordering::Relaxed);

            let state_preferred_dl = match pstate.get().preferred_download.load(atomic::Ordering::Relaxed) { true => 1, false => 0 };

            (n_preferred_dl - state_preferred_dl) >= 1
        }
        {
            self.disconnect_peer_if_it_is_our_only_sync_peer_and_we_have_others_we_could_be_using_instead(pnode,pstate);
        }
    }

    fn disconnect_peer_if_it_is_our_only_sync_peer_and_we_have_others_we_could_be_using_instead(
        self:   Arc<Self>, 
        pnode:  Amo<Box<dyn NodeInterface>>, 
        pstate: Amo<NodeState>) -> ControlFlow {

        let node = pnode.get();

        // Disconnect a peer (without
        // NetPermissionFlags::NoBan permission)
        // if it is our only sync peer, and we
        // have others we could be using instead.
        //
        // Note: If all our peers are inbound,
        // then we won't disconnect our sync peer
        // for stalling; we have bigger problems
        // if we can't get any outbound peers.
        if !node.has_permission(NetPermissionFlags::NoBan) {

            log_printf!(
                "Timeout downloading headers from peer=%d, disconnecting\n", 
                (*pto).get_id()
            );

            node.mark_for_disconnect();

            return ControlFlow::Return;

        } else {

            log_printf!(
                "Timeout downloading headers from noban peer=%d, not disconnecting\n", 
                (*pto).get_id()
            );

            // Reset the headers sync state so
            // that we have a chance to try
            // downloading from a different peer.
            //
            // Note: this will also result in at
            // least one more getheaders message
            // to be sent to this peer
            // (eventually).
            pstate.get().sync_started.store(false, atomic::Ordering::Relaxed);

            {
                let old = self.inner.lock().n_sync_started;
                self.inner.lock().n_sync_started -= 1;
                old
            };

            pstate.get_mut()
                .headers_sync_timeout 
                = Some(OffsetDateTime::from_unix_timestamp(0).unwrap());
        }

        ControlFlow::None
    }

    fn start_block_sync(self: Arc<Self>) {

        PINDEX_BEST_HEADER.lock().replace(
            self.chainman.get().active_chain().tip().clone().unwrap()
        );
    }

    fn respond_to_bip35_mempool_requests(
        self:      Arc<Self>, 
        mut node:  &mut AmoWriteGuard<Box<dyn NodeInterface>>, 
        pstate:    Amo<NodeState>,
        inventory: &mut Vec<Inv>,
        msg_maker: &NetMsgMaker
    ) {

        let current_time = get_datetime();

        let vtxinfo = self.mempool.get().info_all();

        node.get_tx_relay_mut().cs_tx_inventory.lock().send_mempool = false;

        let filterrate: FeeRate 
        = FeeRate::new(
            node.get_tx_relay().min_fee_filter.load(atomic::Ordering::Relaxed)
        );

        for txinfo in vtxinfo.iter() {

            let tx = txinfo.tx.get();

            let hash: &u256 = match pstate.get().wtxid_relay.load(atomic::Ordering::Relaxed) {
                true   => tx.get_witness_hash(),
                false  => tx.get_hash()
            };

            let inv: Inv = Inv::new(
                match pstate.get().wtxid_relay.load(atomic::Ordering::Relaxed) {
                    true   => GetDataMsg::MSG_WTX.bits(),
                    false  => GetDataMsg::MSG_TX.bits()
                }, 
                hash
            );

            node.get_tx_relay().set_inventory_tx_to_send.lock().remove(hash);

            // Don't send transactions
            // that peers will not put
            // into their mempool
            if txinfo.fee < filterrate.get_fee(txinfo.vsize.try_into().unwrap()) {
                continue;
            }

            let pfilter = node.get_tx_relay().cs_filter.lock().pfilter.clone();

            if pfilter.is_some()
            {
                if !pfilter.unwrap()
                    .is_relevant_and_update(&txinfo.tx.get()) 
                {
                    continue;
                }
            }

            node.get_tx_relay_mut()
                .cs_tx_inventory
                .lock()
                .filter_inventory_known
                .insert_key(hash.as_slice());

            // Responses to MEMPOOL requests
            // bypass the
            // m_recently_announced_invs filter.
            inventory.push(inv);

            if inventory.len() == MAX_INV_SZ.try_into().unwrap() {

                self.connman.get_mut()
                    .push_message(
                        &mut *node, 
                        msg_maker.make(
                            NetMsgType::INV, 
                            &[
                                inventory
                            ]
                        )
                    );

                inventory.clear();
            }
        }

        node.get_tx_relay()
            .last_mempool_req.store(
                Some(current_time), 
                atomic::Ordering::Relaxed
            );

    }

    fn determine_transactions_to_relay(
        self:      Arc<Self>, 
        mut node:  &mut AmoWriteGuard<Box<dyn NodeInterface>>, 
        pstate:    Amo<NodeState>,
        inventory: &mut Vec<Inv>,
        msg_maker: &NetMsgMaker
    ) {

        // Produce a vector with all candidates
        // for sending
        let mut inv_tx: Vec<u256> = vec![];

        inv_tx.reserve(
            node.get_tx_relay()
                .set_inventory_tx_to_send.lock().len()
        );

        {
            let tx_relay = node.get_tx_relay();

            let inv_lock = tx_relay
                .set_inventory_tx_to_send
                .lock();

            for it in inv_lock.iter() {
                inv_tx.push(it.clone());
            }
        }

        let filterrate: FeeRate = 
        FeeRate::new(
            node.get_tx_relay()
                .min_fee_filter
                .load(atomic::Ordering::Relaxed)
        );

        // Topologically and fee-rate sort the
        // inventory we send for privacy and
        // priority reasons.
        //
        // A heap is used so that not all items
        // need sorting if only a few are being
        // sent.
        let compare_inv_mempool_order: CompareInvMempoolOrder 
        = CompareInvMempoolOrder::new(
            self.mempool.clone(), 
            pstate.get().wtxid_relay.load(atomic::Ordering::Relaxed)
        );

        let mut inv_tx: MaxHeap<u256, CompareInvMempoolOrder> = {

            let mut builder 
            = MaxHeap::with_comparator(
                compare_inv_mempool_order
            );

            builder.extend(
                inv_tx.iter().cloned().collect::<Vec<u256>>()
            );

            builder
        };

        // No reason to drain out at many times
        // the network's capacity, especially
        // since we have many peers and some will
        // draw much shorter delays.
        let mut n_relayed_transactions: u32 = 0;

        while !inv_tx.is_empty() && n_relayed_transactions < *INVENTORY_BROADCAST_MAX {

            // Fetch the top element from the heap
            let it = inv_tx.pop();

            let hash: u256 = it.unwrap();

            let inv: Inv = Inv::new(
                match pstate.get().wtxid_relay.load(atomic::Ordering::Relaxed) {
                    true   => GetDataMsg::MSG_WTX.bits(),
                    false  => GetDataMsg::MSG_TX.bits()
                }, 
                &hash
            );

            // Remove it from the to-be-sent set
            node.get_tx_relay_mut()
                .set_inventory_tx_to_send
                .lock()
                .remove(&hash);

            // Check if not in the filter already
            if node.get_tx_relay()
                .cs_tx_inventory.lock()
                .filter_inventory_known
                .contains_key(hash.as_slice()) 
            {
                continue;
            }

            // Not in the mempool anymore? don't bother sending it.
            let txinfo = self.mempool.get().info(
                &(inv.clone()).into()
            );

            if txinfo.tx.is_none() {
                continue;
            }

            let tx = txinfo.tx.get();

            let txid  = tx.get_hash();
            let wtxid = tx.get_witness_hash();

            // Peer told you to not send
            // transactions at that feerate? Don't
            // bother sending it.
            if txinfo.fee < filterrate.get_fee(txinfo.vsize.try_into().unwrap()) {
                continue;
            }

            if let Some(ref mut filter) = node.get_tx_relay().cs_filter.lock().pfilter {

                if !filter.is_relevant_and_update(&txinfo.tx.get()) {
                    continue;
                }
            }

            let id = node.get_id();

            // Send
            create_state(id)
                .get_mut()
                .recently_announced_invs
                .insert_key(hash.as_slice());

            inventory.push(inv);

            n_relayed_transactions += 1;

            {
                self.clone().expire_old_relay_messages(&txinfo,&txid,&wtxid);
            }

            if inventory.len() == MAX_INV_SZ.try_into().unwrap() {

                self.connman.get_mut()
                    .push_message(
                        &mut *node, 
                        msg_maker.make(
                            NetMsgType::INV, 
                            &[
                                inventory
                            ]
                        )
                    );

                inventory.clear();
            }

            node.get_tx_relay()
                .cs_tx_inventory.lock()
                .filter_inventory_known
                .insert_key(hash.as_slice());

            if hash != *txid {

                // Insert txid into
                // filterInventoryKnown,
                // even for wtxidrelay
                // peers. This
                // prevents re-adding
                // of unconfirmed
                // parents to the
                // recently_announced
                // filter, when
                // a child tx is
                // requested. See
                // ProcessGetData().
                node.get_tx_relay()
                    .cs_tx_inventory.lock()
                    .filter_inventory_known
                    .insert_key(txid.as_slice());
            }
        }
    }

    fn disconnect_timedout_blocks_in_flight(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>, 
        pstate:           Amo<NodeState>,
        consensus_params: Arc<ChainConsensusParams>,
    ) -> ControlFlow {

        let current_time = get_datetime();

        let node = pnode.get();

        let queued_block: &mut QueuedBlock = &mut pstate.get_mut().blocks_in_flight[0];

        let inner = self.inner.lock();

        let n_other_peers_with_validated_downloads: i32 = inner.peers_downloading_from.load(atomic::Ordering::Relaxed) - 1;

        if current_time > 
        pstate.get().downloading_since 
        + Duration::seconds(consensus_params.n_pow_target_spacing) * 
        (BLOCK_DOWNLOAD_TIMEOUT_BASE + BLOCK_DOWNLOAD_TIMEOUT_PER_PEER * (n_other_peers_with_validated_downloads as f64))
        {
            log_printf!(
                "Timeout downloading block %s from peer=%d, disconnecting\n", 
                (*queued_block.pindex()).get_block_hash().to_string(), 
                (*pto).get_id()
            );

            node.mark_for_disconnect();

            return ControlFlow::Return;
        }

        ControlFlow::None
    }

    fn process_headers_prefer_headers(
        self:       Arc<Self>, 
        pnode:      Amo<Box<dyn NodeInterface>>,
        pstate:     Amo<NodeState>,
        msg_maker:  &NetMsgMaker,
        headers:    &Vec<BlockHeader>,
        best_index: Option<Arc<BlockIndex>>,
    ) {

        if headers.len() > 1 {

            log_print!(
                LogFlags::NET, 
                "%s: %u headers, range (%s, %s), to peer=%d\n", 
                func, 
                headers.len(), 
                headers.front().get_hash().to_string(), 
                headers.back().get_hash().to_string(), 
                (*pto).get_id()
            );

        } else {

            log_print!(
                LogFlags::NET, 
                "%s: sending header %s to peer=%d\n", 
                func, 
                headers.front().get_hash().to_string(), 
                (*pto).get_id()
            );
        }

        self.connman.get_mut()
            .push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make(
                    NetMsgType::HEADERS, 
                    &[
                        headers
                    ]
                )
            );

        pstate.get_mut().pindex_best_header_sent = best_index;
    }

    fn process_headers_prefer_headers_and_ids(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>,
        pstate:           Amo<NodeState>,
        msg_maker:        &NetMsgMaker,
        best_index:       Option<Arc<BlockIndex>>,
        consensus_params: Arc<ChainConsensusParams>) 
    {
        // We only send up to
        // 1 block as header-and-ids, as otherwise
        // probably means we're doing an
        // initial-ish-sync or they're slow
        log_print!(
            LogFlags::NET,
            "%s sending header-and-ids %s to peer=%d\n",
            func,
            headers.front().get_hash().to_string(),
            (*pto).get_id()
        );

        let n_send_flags: i32 = match pstate.get().wants_cmpct_witness.load(atomic::Ordering::Relaxed) {
            true   => 0,
            false  => SERIALIZE_TRANSACTION_NO_WITNESS
        };

        let mut got_block_from_cache: bool = false;

        {
            let mut guard = CS_MOST_RECENT_BLOCK.get();

            if *MOST_RECENT_BLOCK_HASH.get() == best_index.as_ref().unwrap().get_block_hash() {

                self.clone().handle_most_recent_blockhash_is_best_index(
                    pnode.clone(), 
                    pstate.clone(), 
                    n_send_flags, 
                    msg_maker
                );

                got_block_from_cache = true;
            }
        }

        if !got_block_from_cache {

            self.clone().handle_noblock_from_cache(
                pnode.clone(), 
                pstate.clone(), 
                best_index.clone(), 
                consensus_params.clone(), 
                n_send_flags, 
                msg_maker
            );
        }

        pstate.get_mut().pindex_best_header_sent = best_index;
    }

    fn process_headers(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>,
        pstate:           Amo<NodeState>,
        msg_maker:        &NetMsgMaker,
        headers:          &Vec<BlockHeader>,
        best_index:       Option<Arc<BlockIndex>>,
        consensus_params: Arc<ChainConsensusParams>,
        revert_to_inv:    &mut bool) 
    {
        if headers.len() == 1 && pstate.get().prefer_header_and_ids.load(atomic::Ordering::Relaxed) {

            self.process_headers_prefer_headers_and_ids(
                pnode,
                pstate,
                msg_maker,
                best_index,
                consensus_params,
            );

        } else {

            if pstate.get().prefer_headers.load(atomic::Ordering::Relaxed) {

                self.process_headers_prefer_headers(
                    pnode, 
                    pstate, 
                    msg_maker, 
                    headers, 
                    best_index
                );

            } else {
                *revert_to_inv = true;
            }
        }
    }

    fn handle_no_starting_header(
        self:                  Arc<Self>, 
        pindex:                Arc<BlockIndex>,
        pstate:                Amo<NodeState>,
        headers:               &mut Vec<BlockHeader>,
        revert_to_inv:         &mut bool,
        found_starting_header: &mut bool) -> ControlFlow 
    {
        if peer_has_header_with_amo(&pstate.get(),pindex.clone()) {

            // keep looking for the first new
            // block
            return ControlFlow::Continue;

        } else {

            if pindex.pprev.is_none() 
            || peer_has_header_with_amo(&pstate.get(),pindex.pprev.clone().unwrap()) {

                // Peer doesn't have this header
                // but they do have the prior one.
                //
                // Start sending headers.
                *found_starting_header = true;

                headers.push(pindex.get_block_header());

            } else {

                // Peer doesn't have this header
                // or the prior one -- nothing
                // will connect, so bail out.
                *revert_to_inv = true;

                return ControlFlow::Break;
            }
        }

        ControlFlow::None
    }

    fn handle_most_recent_blockhash_is_best_index(
        self:         Arc<Self>, 
        pnode:        Amo<Box<dyn NodeInterface>>,
        pstate:       Amo<NodeState>,
        n_send_flags: i32,
        msg_maker:    &NetMsgMaker
    ) {

        if pstate.get().wants_cmpct_witness.load(atomic::Ordering::Relaxed) 
        || !WITNESSES_PRESENT_IN_MOST_RECENT_COMPACT_BLOCK.load(atomic::Ordering::Relaxed) 
        {
            self.connman.get_mut().push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make_with_flags(
                    n_send_flags, 
                    NetMsgType::CMPCTBLOCK, 
                    &[
                        &MOST_RECENT_COMPACT_BLOCK.get()
                    ]
                )
            );

        } else {

            let cmpctblock: BlockHeaderAndShortTxIDs 
            = BlockHeaderAndShortTxIDs::new(
                MOST_RECENT_BLOCK.clone(), 
                pstate.get().wants_cmpct_witness.load(atomic::Ordering::Relaxed)
            );

            self.connman.get_mut().push_message(
                &mut *pnode.get_mut(), 
                msg_maker.make_with_flags(
                    n_send_flags, 
                    NetMsgType::CMPCTBLOCK, 
                    &[
                        &cmpctblock
                    ]
                )
            );
        }
    }

    fn handle_noblock_from_cache(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>,
        pstate:           Amo<NodeState>,
        best_index:       Option<Arc<BlockIndex>>,
        consensus_params: Arc<ChainConsensusParams>,
        n_send_flags:     i32,
        msg_maker:        &NetMsgMaker
    ) {
        let mut block = Amo::<Block>::from(Block::default());

        let ret: bool = read_block_from_disk_with_blockindex(
            &mut block.get_mut(),
            best_index.as_ref().unwrap().clone(),
            &consensus_params
        );

        assert!(ret);

        let cmpctblock: BlockHeaderAndShortTxIDs = BlockHeaderAndShortTxIDs::new(
            block.clone(), 
            pstate.get().wants_cmpct_witness.load(atomic::Ordering::Relaxed)
        );

        self.connman.get_mut().push_message(
            &mut *pnode.get_mut(), 
            msg_maker.make_with_flags(
                n_send_flags, 
                NetMsgType::CMPCTBLOCK, 
                &[
                    &cmpctblock
                ]
            )
        );
    }

    fn try_find_starting_header(
        self:          Arc<Self>,
        peer:          Amo<Peer>,
        pstate:        Amo<NodeState>,
        best_index:    &mut Option<Arc<BlockIndex>>,
        headers:       &mut Vec<BlockHeader>,
        revert_to_inv: &mut bool,
    ) {

        let mut found_starting_header: bool = false;

        // Try to find first header that our peer
        // doesn't have, and then send all headers
        // past that one.  If we come across any
        // headers that aren't on
        // m_chainman.ActiveChain(), give up.
        for hash in peer.get().block_inv_mutex.lock().blocks_for_headers_relay.iter() {

            let pindex: Option<Arc<BlockIndex>> 
            = self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(hash);

            assert!(pindex.is_some());

            if self.chainman.get().active_chain()[pindex.as_ref().unwrap().n_height] != pindex {
                //  Bail out if we reorged away from this block
                *revert_to_inv = true;
                break;
            }

            if best_index.is_some() 
            && pindex.as_ref().unwrap().pprev.as_ref().unwrap() != best_index.as_ref().unwrap() {

                //  This means that the list of
                //  blocks to announce don't
                //  connect to each other.
                //
                //  This shouldn't really be
                //  possible to hit during regular
                //  operation (because reorgs
                //  should take us to a chain that
                //  has some block not on the
                //  prior chain, which should be
                //  caught by the prior check),
                //  but one way this could happen
                //  is by using invalidateblock
                //  / reconsiderblock repeatedly
                //  on the tip, causing it to be
                //  added multiple times to
                //  m_blocks_for_headers_relay.
                //
                //  Robustly deal with this rare
                //  situation by reverting to an
                //  inv.
                *revert_to_inv = true;
                break;
            }

            *best_index = pindex.clone();

            if found_starting_header {

                // add this to the headers message
                headers.push(pindex.as_ref().unwrap().get_block_header());

            } else {

                self.clone().handle_no_starting_header(
                    pindex.as_ref().unwrap().clone(),
                    pstate.clone(),
                    headers,
                    revert_to_inv,
                    &mut found_starting_header
                );
            }
        }
    }

    fn try_to_inv_the_tip(
        self:   Arc<Self>, 
        pstate: Amo<NodeState>,
        peer:   AmoReadGuard<Peer>
    ) {

        // If falling back to using an inv, just
        // try to inv the tip.
        //
        // The last entry in
        // m_blocks_for_headers_relay was our tip
        // at some point in the past.
        if !peer.block_inv_mutex.lock().blocks_for_headers_relay.is_empty() {

            let block_inv_guard = peer.block_inv_mutex.lock();

            let hash_to_announce: &u256 = 
                block_inv_guard
                .blocks_for_headers_relay
                .last()
                .unwrap();

            let pindex: Option<Arc<BlockIndex>> 
            = self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(hash_to_announce);

            assert!(pindex.is_some());

            // Warn if we're announcing a block
            // that is not on the main chain.
            //
            // This should be very rare and could
            // be optimized out.
            //
            // Just log for now.
            if self.chainman.get().active_chain()[pindex.as_ref().unwrap().n_height] != pindex {
                log_print!(
                    LogFlags::NET, 
                    "Announcing block %s not on main chain (tip=%s)\n", 
                    hash_to_announce.to_string(), 
                    (*self.chainman.active_chain().tip()).get_block_hash().to_string()
                );
            }

            // If the peer's chain has this block,
            // don't inv it back.
            if !peer_has_header_with_amo(&pstate.get(),pindex.unwrap()) {

                peer 
                    .block_inv_mutex.lock()
                    .blocks_for_inv_relay.push(hash_to_announce.clone());

                log_print!(
                    LogFlags::NET, 
                    "%s: sending inv peer=%d hash=%s\n", 
                    func, 
                    (*pto).get_id(), 
                    hash_to_announce.to_string()
                );
            }
        }
    }

    // Try sending block announcements via headers
    #[EXCLUSIVE_LOCKS_REQUIRED(pto->cs_sendProcessing)]
    fn try_sending_block_announcements_via_headers(
        self:             Arc<Self>, 
        pnode:            Amo<Box<dyn NodeInterface>>, 
        pstate:           Amo<NodeState>,
        peer:             Amo<Peer>,
        msg_maker:        &NetMsgMaker,
        consensus_params: Arc<ChainConsensusParams>,
    ) {

        // If we have less than
        // MAX_BLOCKS_TO_ANNOUNCE in our
        // list of block hashes we're
        // relaying, and our peer wants
        // headers announcements, then
        // find the first header not yet
        // known to our peer but would
        // connect, and send.
        //
        // If no header would connect, or
        // if we have too many blocks, or
        // if the peer doesn't want
        // headers, just add all to the
        // inv queue.
        let mut guard = peer.get();

        let mut block_inv_mutex = guard.block_inv_mutex.lock();

        let mut headers: Vec<BlockHeader> = vec![];

        let mut revert_to_inv: bool = 
        (
            !pstate.get().prefer_headers.load(atomic::Ordering::Relaxed) && (!pstate.get().prefer_header_and_ids.load(atomic::Ordering::Relaxed) 
            || peer.get().block_inv_mutex.lock().blocks_for_headers_relay.len() > 1)
        ) 
        || peer.get().block_inv_mutex.lock().blocks_for_headers_relay.len() > (MAX_BLOCKS_TO_ANNOUNCE as usize);

        // last header queued for delivery
        let mut best_index: Option<Arc<BlockIndex>> = None;

        //  ensure pindexBestKnownBlock is up-to-date
        self.clone().process_block_availability(pnode.get().get_id());

        if !revert_to_inv {

            self.clone().try_find_starting_header(
                peer.clone(),
                pstate.clone(),
                &mut best_index,
                &mut headers,
                &mut revert_to_inv
            );
        }

        if !revert_to_inv && !headers.is_empty() {

            self.clone().process_headers(
                pnode,
                pstate.clone(),
                &msg_maker,
                &headers,
                best_index.clone(),
                consensus_params,
                &mut revert_to_inv
            );
        }

        if revert_to_inv {

            self.clone().try_to_inv_the_tip(
                pstate.clone(),
                peer.get()
            );
        }

        peer.get().block_inv_mutex.lock().blocks_for_headers_relay.clear();
    }
}
