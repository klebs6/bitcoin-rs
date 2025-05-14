// ---------------- [ File: bitcoin-peerman/src/process_headers_message.rs ]
crate::ix!();

pub trait ProcessHeadersMessage {

    fn process_headers_basic(self: Arc<Self>, 
        pfrom:             &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:              &Peer,
        headers:           &Vec<BlockHeader>,
        via_compact_block: bool);
}
    
impl ProcessHeadersMessage for PeerManager {

    /**
      | Process a single headers message from
      | a peer.
      |
      */
    fn process_headers_basic(self: Arc<Self>, 
        mut pfrom:         &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:              &Peer,
        headers:           &Vec<BlockHeader>,
        via_compact_block: bool)  {

        let msg_maker: NetMsgMaker = NetMsgMaker::new(
            pfrom.get_common_version()
        );

        let n_count: usize = headers.len();

        if n_count == 0 {
            // Nothing interesting. Stop asking
            // this peers for more headers.
            return;
        }

        let mut received_new_header: bool = false;

        let mut pindex_last: Option<Arc<BlockIndex>> = None;

        {
            let mut guard = CS_MAIN.lock();

            let nodestate: Amo<NodeState> = create_state(pfrom.get_id());

            //  If this looks like it could be
            //  a block announcement (nCount
            //  < MAX_BLOCKS_TO_ANNOUNCE), use
            //  special logic for handling headers
            //  that don't connect:
            //
            //  - Send a getheaders message in
            //  response to try to connect the
            //  chain.
            //
            //  - The peer can send up to
            //    MAX_UNCONNECTING_HEADERS in
            //    a row that don't connect before
            //    giving DoS points
            //
            //  - Once a headers message is
            //    received that is valid and does
            //    connect, nUnconnectingHeaders
            //    gets reset back to 0.
            if self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(&headers[0].hash_prev_block).is_none()
            && n_count < MAX_BLOCKS_TO_ANNOUNCE.try_into().unwrap()
            {

                nodestate.get_mut().n_unconnecting_headers.fetch_add(1,atomic::Ordering::Relaxed);

                self.connman.get_mut().push_message(
                    &mut *pfrom, 
                    msg_maker.make(
                        NetMsgType::GETHEADERS, 
                        &[
                            &self.chainman.get().active_chain().get_locator(PINDEX_BEST_HEADER.lock().clone()), 
                            &u256::default()
                        ]
                    )
                );

                log_print!(
                    LogFlags::NET, 
                    "received header %s: missing prev block %s, sending getheaders (%d) to end (peer=%d, nUnconnectingHeaders=%d)\n", 
                    headers[0].get_hash().to_string(), 
                    headers[0].hash_prev_block().to_string(), 
                    (*PINDEX_BEST_HEADER.lock()).n_height, 
                    pfrom.get_id(), 
                    (*nodestate).n_unconnecting_headers
                );

                // Set hashLastUnknownBlock for
                // this peer, so that if we
                // eventually get the headers
                // - even from a different peer
                // - we can use this peer to
                // download.
                self.clone().update_block_availability(
                    pfrom.get_id(),
                    &headers.last().unwrap().get_hash()
                );

                if nodestate.get().n_unconnecting_headers.load(atomic::Ordering::Relaxed) % MAX_UNCONNECTING_HEADERS == 0 {

                    self.misbehaving(
                        pfrom.get_id(), 
                        20, 
                        format!(
                            "{} non-connecting headers",
                            nodestate.get().n_unconnecting_headers.load(atomic::Ordering::Relaxed)
                        ).as_str()
                    );
                }

                return;
            }

            let mut hash_last_block = u256::default();

            for header in headers.iter() {

                if !hash_last_block.is_null() 
                && header.hash_prev_block != hash_last_block 
                {

                    self.misbehaving(
                        pfrom.get_id(), 
                        20, 
                        "non-continuous headers sequence"
                    );

                    return;
                }

                hash_last_block = header.get_hash();
            }

            // If we don't have the last header,
            // then they'll have given us
            // something new (if these headers are
            // valid).
            if self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(&hash_last_block)
                .is_some()
            {
                received_new_header = true;
            }
        }

        let mut state = BlockValidationState::default();

        if !self.chainman.get_mut()
            .process_new_block_headers(
                headers, 
                &mut state, 
                &self.chainparams, 
                pindex_last.clone()) 
        {
            if state.is_invalid() {

                self.maybe_punish_node_for_block(
                    pfrom.get_id(), 
                    &state, 
                    via_compact_block, 
                    Some("invalid header received")
                );

                return;
            }
        }

        {
            let mut guard = CS_MAIN.lock();

            let nodestate: Amo<NodeState> = create_state(pfrom.get_id());

            if nodestate.get().n_unconnecting_headers.load(atomic::Ordering::Relaxed) > 0 {

                log_print!(
                    LogFlags::NET, 
                    "peer=%d: resetting nUnconnectingHeaders (%d -> 0)\n", 
                    pfrom.get_id(), 
                    (*nodestate).n_unconnecting_headers
                );
            }

            nodestate.get_mut().n_unconnecting_headers.store(0,atomic::Ordering::Relaxed);

            assert!(pindex_last.is_some());

            self.clone().update_block_availability(
                pfrom.get_id(), 
                &pindex_last.as_ref().unwrap().get_block_hash()
            );

            // From here, pindexBestKnownBlock
            // should be guaranteed to be
            // non-null, because it is set in
            // UpdateBlockAvailability. Some
            // nullptr checks are still present,
            // however, as belt-and-suspenders.
            if received_new_header 
            && pindex_last.as_ref().unwrap().n_chain_work > self.chainman.get().active_chain().tip().as_ref().unwrap().n_chain_work {
                nodestate.get_mut().last_block_announcement = Some(get_datetime());
            }

            if n_count == MAX_HEADERS_RESULTS.try_into().unwrap() {

                // Headers message had its maximum
                // size; the peer may have more
                // headers.
                //
                // TODO: optimize: if pindexLast
                // is an ancestor of
                // m_chainman.ActiveChain().Tip or
                // pindexBestHeader, continue from
                // there instead.
                log_print!(
                    LogFlags::NET,
                    "more getheaders (%d) to end to peer=%d (startheight:%d)\n",
                    pindex_last.get().n_height,
                    pfrom.get_id(),
                    peer.starting_height
                );

                self.connman.get_mut().push_message(
                    &mut *pfrom, 
                    msg_maker.make(
                        NetMsgType::GETHEADERS, 
                        &[
                            &self.chainman.get().active_chain().get_locator(pindex_last.clone()), 
                            &u256::default()
                        ]
                    )
                );
            }

            // If this set of headers is valid and
            // ends in a block with at least as
            // much work as our tip, download as
            // much as possible.
            if self.clone().can_direct_fetch() 
            && pindex_last.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_TREE)) 
            && self.chainman.get().active_chain().tip().as_ref().unwrap().n_chain_work <= pindex_last.as_ref().unwrap().n_chain_work {

                let mut to_fetch: Vec<Option<Arc<BlockIndex>>> = vec![];

                let mut pindex_walk: Option<Arc<BlockIndex>> = pindex_last.clone();

                // Calculate all the blocks we'd
                // need to switch to pindexLast,
                // up to a limit.
                while pindex_walk.is_some() 
                && !self.chainman.get().active_chain().contains(pindex_walk.clone()) 
                && to_fetch.len() <= MAX_BLOCKS_IN_TRANSIT_PER_PEER.try_into().unwrap() 
                {
                    if (pindex_walk.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) == 0 
                    && !self.is_block_requested(&pindex_walk.as_ref().unwrap().get_block_hash()) 
                    && (
                        !deployment_active_at_with_buried_deployment(
                            pindex_walk.clone().unwrap(),
                            &self.chainparams.get_consensus(),
                            ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT
                        ) 
                        || 
                        create_state(pfrom.get_id()).get().have_witness.load(atomic::Ordering::Relaxed)
                    ) 
                    {
                        //  We don't have this block, and it's not yet in flight.
                        to_fetch.push(pindex_walk.clone());
                    }

                    pindex_walk = pindex_walk.as_ref().unwrap().pprev.clone();
                }

                // If pindexWalk still isn't on
                // our main chain, we're looking
                // at a very large reorg at a time
                // we think we're close to caught
                // up to the main chain -- this
                // shouldn't really happen.  Bail
                // out on the direct fetch and
                // rely on parallel download
                // instead.
                if !self.chainman.get().active_chain().contains(pindex_walk) {

                    log_print!(
                        LogFlags::NET, 
                        "Large reorg, won't direct fetch to %s (%d)\n", 
                        (*pindex_last).get_block_hash().to_string(), 
                        (*pindex_last).n_height
                    );

                } else {

                    let mut get_data: Vec<Inv> = vec![];

                    //  Download as much as possible, from earliest to latest.
                    for pindex in to_fetch.iter().rev() {

                        if nodestate.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) >= MAX_BLOCKS_IN_TRANSIT_PER_PEER {

                            // Can't download any
                            // more from this peer
                            break;
                        }

                        let n_fetch_flags: GetDataMsg = get_fetch_flags(&***pfrom);

                        get_data.push(
                            Inv::new(
                                (GetDataMsg::MSG_BLOCK | n_fetch_flags).bits(),
                                &(*pindex.as_ref().unwrap()).get_block_hash()
                            )
                        );

                        self.clone().block_requested(
                            pfrom.get_id(), 
                            pindex.clone(), 
                            Default::default()
                        );

                        log_print!(
                            LogFlags::NET, 
                            "Requesting block %s from  peer=%d\n", 
                            (*pindex).get_block_hash().to_string(), 
                            pfrom.get_id()
                        );
                    }

                    if get_data.len() > 1 {
                        log_print!(
                            LogFlags::NET, 
                            "Downloading blocks toward %s (%d) via headers direct fetch\n", 
                            (*pindex_last).get_block_hash().to_string(), 
                            (*pindex_last).n_height
                        );
                    }

                    if get_data.len() > 0 {

                        let inner = self.inner.lock();
                        let mbif = inner.map_blocks_in_flight.lock();


                        if !self.ignore_incoming_txs 
                        && nodestate.get().supports_desired_cmpct_version.load(atomic::Ordering::Relaxed) 
                        && get_data.len() == 1 
                        && mbif.len() == 1 

                        && pindex_last
                            .as_ref()
                            .unwrap()
                            .pprev
                            .as_ref()
                            .unwrap()
                            .is_valid(Some(BlockStatus::BLOCK_VALID_CHAIN)) 
                        {
                            // In any case, we
                            // want to download
                            // using a compact
                            // block, not
                            // a regular one
                            get_data[0] = Inv::new(
                                GetDataMsg::MSG_CMPCT_BLOCK.bits(),
                                &get_data[0].hash
                            );
                        }

                        self.connman.get_mut().push_message(
                            &mut *pfrom, 
                            msg_maker.make(NetMsgType::GETDATA, &[&get_data])
                        );
                    }
                }
            }

            // If we're in IBD, we want outbound
            // peers that will serve us a useful
            // chain. Disconnect peers that are on
            // chains with insufficient work.
            if self.chainman.get().active_chainstate().is_initial_block_download() 
            && n_count != MAX_HEADERS_RESULTS.try_into().unwrap() {

                // When nCount
                // < MAX_HEADERS_RESULTS, we know
                // we have no more headers to
                // fetch from this peer.
                if nodestate.get().pindex_best_known_block.is_some() 
                && nodestate.get().pindex_best_known_block.as_ref().unwrap().n_chain_work < *N_MINIMUM_CHAIN_WORK 
                {
                    // This peer has too little
                    // work on their headers chain
                    // to help us sync --
                    // disconnect if it is an
                    // outbound disconnection
                    // candidate.
                    //
                    // Note: We compare their tip
                    // to nMinimumChainWork
                    // (rather than
                    // m_chainman.ActiveChain().Tip())
                    // because we won't start
                    // block download until we
                    // have a headers chain that
                    // has at least
                    // nMinimumChainWork, even if
                    // a peer has a chain past our
                    // tip, as an anti-DoS
                    // measure.
                    if pfrom.is_outbound_or_block_relay_conn() {

                        log_printf!(
                            "Disconnecting outbound peer %d -- headers chain has insufficient work\n", 
                            pfrom.get_id()
                        );

                        pfrom.mark_for_disconnect();
                    }
                }
            }

            // If this is an outbound full-relay
            // peer, check to see if we should
            // protect it from the bad/lagging
            // chain logic.
            //
            // Note that outbound block-relay
            // peers are excluded from this
            // protection, and thus always subject
            // to eviction under the bad/lagging
            // chain logic.
            //
            // See ChainSyncTimeoutState.
            if !pfrom.marked_for_disconnect() 
            && pfrom.is_full_outbound_conn() 
            && nodestate.get().pindex_best_known_block.is_some() 
            {

                let inner = self.inner.lock();

                let chainman = self.chainman.get();

                let oppfd = inner.outbound_peers_with_protect_from_disconnect.load(atomic::Ordering::Relaxed);

                if oppfd < MAX_OUTBOUND_PEERS_TO_PROTECT_FROM_DISCONNECT 

                && nodestate.get().pindex_best_known_block.as_ref().unwrap().n_chain_work 
                    >= chainman.active_chain().tip().as_ref().unwrap().n_chain_work 

                && !nodestate.get().chain_sync.protect 
                {

                    log_print!(
                        LogFlags::NET, 
                        "Protecting outbound peer=%d from eviction\n", 
                        pfrom.get_id()
                    );

                    nodestate.get_mut().chain_sync.protect = true;

                    inner.outbound_peers_with_protect_from_disconnect.fetch_add(1, atomic::Ordering::Relaxed);
                }
            }
        }
    }
}

impl PeerManager {

    pub fn process_headers_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          NetMsgMaker,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        // Ignore headers received while
        // IMPORTING.load(atomic::Ordering::Relaxed)
        if IMPORTING.load(atomic::Ordering::Relaxed) || REINDEX.load(atomic::Ordering::Relaxed) {

            log_print!(
                LogFlags::NET, 
                "Unexpected headers message received from peer %d\n", 
                pfrom.get_id()
            );

            return;
        }

        let mut headers: Vec<BlockHeader> = vec![];

        // Bypass the normal CBlock
        // deserialization, as we don't want
        // to risk deserializing 2000 full
        // blocks.
        let n_count: usize = read_compact_size(recv, None).try_into().unwrap();

        if n_count > MAX_HEADERS_RESULTS.try_into().unwrap() {

            self.misbehaving(
                pfrom.get_id(), 
                20, 
                format!("headers message size = {}",n_count).as_str()
            );

            return;
        }

        headers.resize(n_count, Default::default());

        for n in 0_usize..n_count {

            recv.stream_into(&mut headers[n]);

            // ignore tx count; assume it is
            // 0.
            read_compact_size(recv, None);
        }

        return self.process_headers_basic(
            pfrom,
            peer.as_ref().unwrap(),
            &headers,
            /*via_compact_block=*/ false
        );
    }
}
