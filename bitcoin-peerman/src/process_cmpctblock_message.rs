crate::ix!();

impl PeerManager {

    pub fn process_cmpctblock_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          &NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        // Ignore cmpctblock received while
        // IMPORTING.load(atomic::Ordering::Relaxed)
        if IMPORTING.load(atomic::Ordering::Relaxed) || REINDEX.load(atomic::Ordering::Relaxed) {

            log_print!(
                LogFlags::NET, 
                "Unexpected cmpctblock message received from peer %d\n", 
                pfrom.get_id()
            );

            return;
        }

        let mut cmpctblock = BlockHeaderAndShortTxIDs::default();

        recv.stream_into(&mut cmpctblock);

        let mut received_new_header: bool = false;

        {
            let mut guard = CS_MAIN.lock();

            if self.chainman
                .get()
                .inner
                .blockman
                .lookup_block_index(&cmpctblock.header.hash_prev_block)
                .is_none() {

                // Doesn't connect (or is
                // genesis), instead of DoSing
                // in AcceptBlockHeader,
                // request deeper headers
                if !self.chainman
                    .get()
                    .active_chainstate()
                    .is_initial_block_download() {

                    self.connman.get_mut().push_message(
                        &mut pfrom, 
                        msg_maker.make(
                            NetMsgType::GETHEADERS, 
                            &[
                                &self.chainman.get_mut().active_chain().get_locator(PINDEX_BEST_HEADER.lock().clone()), 
                                &u256::default()
                            ]
                        )
                    );
                }

                return;
            }

            if self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(&cmpctblock.header.get_hash())
                .is_none() 
            {
                received_new_header = true;
            }
        }

        let mut pindex: Option<Arc<BlockIndex>> = None;

        let mut state = BlockValidationState::default();

        if !self.chainman.get_mut().process_new_block_headers(
            &vec![cmpctblock.header.clone()], 
            &mut state, 
            &self.chainparams, 
            pindex.clone()
        ) 
        {
            if state.is_invalid() {

                self.maybe_punish_node_for_block(
                    pfrom.get_id(), 
                    &state, 
                    /*via_compact_block*/ true, 
                    Some("invalid header via cmpctblock")
                );

                return;
            }
        }

        // When we succeed in decoding
        // a block's txids from a cmpctblock
        // message we typically jump to the
        // BLOCKTXN handling code, with
        // a dummy (empty) BLOCKTXN message,
        // to re-use the logic there in
        // completing processing of the
        // putative block (without CS_MAIN).
        let mut processblocktxn: bool = false;

        let mut block_txn_msg: DataStream = DataStream::new(SER_NETWORK, PROTOCOL_VERSION);

        // If we end up treating this as
        // a plain headers message, call that
        // as well without CS_MAIN.
        let mut revert_to_header_processing: bool = false;

        // Keep a CBlock for "optimistic"
        // compactblock reconstructions (see
        // below)
        let pblock: Amo::<Block> = amo_none();

        let mut block_reconstructed: bool = false;

        {
            let mut guard_main    = CS_MAIN.lock();
            let mut guard_orphans = G_CS_ORPHANS.lock();

            // If AcceptBlockHeader returned
            // true, it set pindex
            assert!(pindex.is_some());

            self.clone().update_block_availability(
                pfrom.get_id(), 
                &pindex.as_ref().unwrap().get_block_hash()
            );

            let nodestate: Amo<NodeState> = create_state(pfrom.get_id());

            // If this was a new header with
            // more work than our tip, update
            // the peer's last block
            // announcement time
            if received_new_header 
            && pindex.as_ref().unwrap().n_chain_work > self.chainman.get().active_chain().tip().as_ref().unwrap().n_chain_work {

                nodestate.get_mut().last_block_announcement = Some(get_datetime());
            }

            let inner = self.inner.lock();
            let mbif  = inner.map_blocks_in_flight.lock();

            let block_in_flight_it = mbif.get(&pindex.as_ref().unwrap().get_block_hash());

            let already_in_flight: bool = block_in_flight_it.is_some();

            if (pindex.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) != 0 {
                //  Nothing to do here
                return;
            }

            /* We know something better */
            if pindex.as_ref().unwrap().n_chain_work <= self.chainman.get().active_chain().tip().as_ref().unwrap().n_chain_work 
            || pindex.as_ref().unwrap().n_tx != 0 {

                // We had this block at some
                // point, but pruned it
                if already_in_flight {

                    // We requested this block
                    // for some reason, but
                    // our mempool will
                    // probably be useless so
                    // we just grab the block
                    // via normal getdata
                    let mut inv: Vec<Inv> = Vec::<Inv>::with_capacity(1);

                    inv[0] = Inv::new(
                        (GetDataMsg::MSG_BLOCK | get_fetch_flags(&***pfrom)).bits(),
                        &cmpctblock.header.get_hash()
                    );

                    self.connman
                        .get_mut()
                        .push_message(
                            &mut pfrom, 
                            msg_maker.make(NetMsgType::GETDATA, &[&inv])
                        );
                }

                return;
            }

            // If we're not close to tip yet,
            // give up and let parallel block
            // fetch work its magic
            if !already_in_flight && !self.clone().can_direct_fetch() {
                return;
            }

            if deployment_active_at_with_buried_deployment(
                pindex.as_ref().unwrap().clone(),
                &self.chainparams.get_consensus(),
                ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT
            ) 
            && !nodestate.get().supports_desired_cmpct_version.load(atomic::Ordering::Relaxed) 
            {
                // Don't bother trying to
                // process compact blocks from
                // v1 peers after segwit
                // activates.
                return;
            }

            let active_chain_height 
                = self.chainman
                .get_mut()
                .active_chain()
                .height()
                .unwrap();

            // We want to be a bit
            // conservative just to be extra
            // careful about DoS possibilities
            // in compact block processing...
            if pindex.as_ref().unwrap().n_height 
            <= (active_chain_height + 2).try_into().unwrap() 
            {
                if (!already_in_flight && nodestate.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) < MAX_BLOCKS_IN_TRANSIT_PER_PEER) 
                || (
                    already_in_flight 
                    && block_in_flight_it.unwrap().0 == pfrom.get_id()
                ) 
                {
                    let mut queued_block_it: Amo::<QueuedBlockIter> = Amo::<QueuedBlockIter>::none();

                    if !self.clone().block_requested(
                        pfrom.get_id(),
                        pindex.clone(),
                        queued_block_it.clone()) 
                    {
                        if (*queued_block_it.get_mut().peek().unwrap()).1.partial_block.is_none() {

                            (*queued_block_it.get_mut().peek().unwrap()).1.partial_block.replace(
                                &PartiallyDownloadedBlock::new(amo_none())
                            );

                        } else {


                            // The block was
                            // already in
                            // flight using
                            // compact blocks
                            // from the same
                            // peer
                            log_print!(
                                LogFlags::NET,
                                "Peer sent us compact block we were already syncing!\n"
                            );

                            return;
                        }
                    }

                    let mut queued_block_handle = queued_block_it.get_mut();

                    let mut partial_block = 
                    queued_block_handle.peek().as_mut().unwrap()
                        .1
                        .partial_block
                        .get_mut();

                    let mut status: ReadStatus 
                    = partial_block.init_data(
                        &cmpctblock, 
                        &self.orphan_data.extra_txn_for_compact.lock()
                    );

                    if status == ReadStatus::Invalid {

                        // Reset in-flight
                        // state in case
                        // Misbehaving does
                        // not result in
                        // a disconnect
                        self.remove_block_request(
                            &pindex.as_ref().unwrap().get_block_hash()
                        );

                        self.misbehaving(pfrom.get_id(), 100, "invalid compact block");

                        return;

                    } else {

                        if status == ReadStatus::Failed {

                            // Duplicate
                            // txindexes, the
                            // block is now
                            // in-flight, so
                            // just request it
                            let mut inv: Vec<Inv> = Vec::<Inv>::with_capacity(1);

                            inv[0] = Inv::new(
                                (GetDataMsg::MSG_BLOCK | get_fetch_flags(&***pfrom)).bits(),
                                &cmpctblock.header.get_hash()
                            );

                            self.connman.get_mut().push_message(
                                &mut pfrom, 
                                msg_maker.make(NetMsgType::GETDATA, &[&inv])
                            );

                            return;
                        }
                    }

                    let mut req = BlockTransactionsRequest::default();

                    for i in 0..cmpctblock.block_tx_count() {

                        if !partial_block.is_tx_available(i) {
                            req.indexes.push(i.try_into().unwrap());
                        }
                    }

                    if req.indexes.is_empty() {

                        // Dirty hack to jump
                        // to BLOCKTXN code
                        // (TODO: move message
                        // handling into their
                        // own functions)
                        let mut txn = BlockTransactions::default();

                        txn.blockhash = cmpctblock.header.get_hash();

                        block_txn_msg.stream(txn);

                        processblocktxn = true;

                    } else {

                        req.blockhash = pindex
                            .as_ref()
                            .unwrap()
                            .get_block_hash();

                        self.connman.get_mut().push_message(
                            &mut pfrom, 
                            msg_maker.make(NetMsgType::GETBLOCKTXN, &[&req])
                        );
                    }

                } else {
                    // This block is either
                    // already in flight from
                    // a different peer, or
                    // this peer has too many
                    // blocks outstanding to
                    // download from.
                    //
                    // Optimistically try to
                    // reconstruct anyway
                    // since we might be able
                    // to without any round
                    // trips.
                    let mut temp_block: Amo<PartiallyDownloadedBlock> 
                    = Amo::from(
                        PartiallyDownloadedBlock::new(
                            self.mempool.clone()
                        )
                    );

                    let mut status: ReadStatus = temp_block.get_mut().init_data(
                        &cmpctblock, 
                        &self.orphan_data.extra_txn_for_compact.lock()
                    );

                    if status != ReadStatus::Ok {
                        //  TODO: don't ignore failures
                        return;
                    }

                    let mut dummy: Vec<TransactionRef> = vec![];

                    status = temp_block.get_mut().fill_block(&mut pblock.get_mut(), &dummy);

                    if status == ReadStatus::Ok {
                        block_reconstructed = true;
                    }
                }

            } else {

                if already_in_flight {

                    // We requested this
                    // block, but its far into
                    // the future, so our
                    // mempool will probably
                    // be useless - request
                    // the block normally
                    let mut inv: Vec<Inv> = Vec::<Inv>::with_capacity(1);

                    inv[0] = Inv::new(
                        (GetDataMsg::MSG_BLOCK | get_fetch_flags(&***pfrom)).bits(),
                        &cmpctblock.header.get_hash()
                    );

                    self.connman.get_mut()
                        .push_message(
                            &mut pfrom, 
                            msg_maker.make(NetMsgType::GETDATA, &[&inv])
                        );

                    return;

                } else {

                    // If this was an
                    // announce-cmpctblock, we
                    // want the same treatment as
                    // a header message
                    revert_to_header_processing = true;
                }
            }
        }

        if processblocktxn {

            return self.process_message(
                &mut pfrom,
                NetMsgType::BLOCKTXN,
                &mut block_txn_msg,
                time_received,
                interrupt_msg_proc
            );
        }

        if revert_to_header_processing {

            let headers = vec![cmpctblock.header.clone()];

            // Headers received from HB compact
            // block peers are permitted to be
            // relayed before full validation (see
            // BIP 152), so we don't want to
            // disconnect the peer if the header
            // turns out to be for an invalid
            // block.
            //
            // Note that if a peer tries to build
            // on an invalid chain, that will be
            // detected and the peer will be
            // disconnected/discouraged.
            return self.process_headers_basic(
                pfrom,
                peer.as_ref().unwrap(),
                &headers,
                /*via_compact_block=*/ true
            );
        }

        if block_reconstructed {

            // If we got here, we were able to
            // optimistically reconstruct a block
            // that is in flight from some other
            // peer.
            {
                let mut guard = CS_MAIN.lock();

                self.inner.lock().map_block_source.insert(
                    pblock.get().get_hash(), 
                    (pfrom.get_id(),false)
                );
            }

            // Setting force_processing to true
            // means that we bypass some of our
            // anti-DoS protections in
            // AcceptBlock, which filters
            // unrequested blocks that might be
            // trying to waste our resources (eg
            // disk space). Because we only try to
            // reconstruct blocks when we're close
            // to caught up (via the
            // CanDirectFetch() requirement above,
            // combined with the behavior of not
            // requesting blocks until we have
            // a chain with at least
            // nMinimumChainWork), and we ignore
            // compact blocks with less work than
            // our tip, it is safe to treat
            // reconstructed compact blocks as
            // having been requested.
            self.clone().process_block(
                pfrom,
                pblock.clone(),
                /*force_processing=*/ true
            );

            // hold CS_MAIN for
            // CBlockIndex::IsValid()
            let mut guard = CS_MAIN.lock();

            if pindex.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_TRANSACTIONS)) {

                // Clear download state for this
                // block, which is in process from
                // some other peer.  We do this
                // after calling
                //
                // ProcessNewBlock so that
                // a malleated cmpctblock
                // announcement can't be used to
                // interfere with block relay.
                self.remove_block_request(&pblock.get().get_hash());
            }
        }
    }
}

