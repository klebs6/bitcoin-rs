crate::ix!();

pub fn read_raw_block_from_disk_with_blockindex(
        block:         Amo<Vec<u8>>,
        pindex:        Arc<BlockIndex>,
        message_start: &MessageHeaderMessageStartChars) -> bool {
    
    todo!();
        /*
            FlatFilePos block_pos;
        {
            LOCK(cs_main);
            block_pos = pindex->GetBlockPos();
        }

        return ReadRawBlockFromDisk(block, block_pos, message_start);
        */
}

pub fn read_raw_block_from_disk(
        block:         Amo<Vec<u8>>,
        pos:           &FlatFilePos,
        message_start: &MessageHeaderMessageStartChars) -> bool {
    
    todo!();
        /*
            FlatFilePos hpos = pos;
        hpos.nPos -= 8; // Seek back 8 bytes for meta header
        CAutoFile filein(OpenBlockFile(hpos, true), SER_DISK, CLIENT_VERSION);
        if (filein.IsNull()) {
            return error("%s: OpenBlockFile failed for %s", __func__, pos.ToString());
        }

        try {
            MessageHeader::MessageStartChars blk_start;
            unsigned int blk_size;

            filein >> blk_start >> blk_size;

            if (memcmp(blk_start, message_start, MessageHeader::MESSAGE_START_SIZE)) {
                return error("%s: Block magic mismatch for %s: %s versus expected %s", __func__, pos.ToString(),
                             HexStr(blk_start),
                             HexStr(message_start));
            }

            if (blk_size > MAX_SIZE) {
                return error("%s: Block data is larger than maximum deserialization size for %s: %s versus %s", __func__, pos.ToString(),
                             blk_size, MAX_SIZE);
            }

            block.resize(blk_size); // Zeroing of memory is intentional here
            filein.read((char*)block.data(), blk_size);
        } catch (const std::exception& e) {
            return error("%s: Read from block file failed: %s for %s", __func__, e.what(), pos.ToString());
        }

        return true;
        */
}

pub trait ProcessGetBlockData {

    fn process_get_block_data(
        self:  Arc<Self>, 
        pfrom: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:  Amo<Peer>,
        inv:   &Inv);
}

impl ProcessGetBlockData for PeerManager {

    fn process_get_block_data(
        self:      Arc<Self>, 
        mut pfrom: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        peer:      Amo<Peer>,
        inv:       &Inv)  {

        let mut a_recent_block: Amo<Block> = Amo::<Block>::none();

        let mut a_recent_compact_block: Amo::<BlockHeaderAndShortTxIDs> = Amo::<BlockHeaderAndShortTxIDs>::none();

        let mut witnesses_present_in_arecent_compact_block = bool::default();

        {
            let mut guard = CS_MOST_RECENT_BLOCK.get();

            a_recent_block                             = MOST_RECENT_BLOCK.clone();
            a_recent_compact_block                     = MOST_RECENT_COMPACT_BLOCK.clone();
            witnesses_present_in_arecent_compact_block = WITNESSES_PRESENT_IN_MOST_RECENT_COMPACT_BLOCK.load(atomic::Ordering::Relaxed);
        }

        let mut need_activate_chain: bool = false;

        {
            let mut guard = CS_MAIN.lock();

            let pindex: Option<Arc<BlockIndex>> 
            = self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(&inv.hash);

            if pindex.is_some() {

                if pindex.as_ref().unwrap().have_txs_downloaded() 
                && !pindex.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_SCRIPTS)) 
                && pindex.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_TREE)) 
                {
                    // If we have the block and
                    // all of its parents, but
                    // have not yet validated it,
                    // we might be in the middle
                    // of connecting it (ie in the
                    // unlock of CS_MAIN before
                    // ActivateBestChain but after
                    // AcceptBlock).
                    //_mut
                    // In this case, we need to
                    // run ActivateBestChain prior
                    // to checking the relay
                    // conditions below.
                    need_activate_chain = true;
                }
            }
        }

        // release CS_MAIN before calling ActivateBestChain
        if need_activate_chain {

            let mut state = BlockValidationState::default();

            if !self.chainman
                .get()
                .active_chainstate()
                .activate_best_chain(&mut state, a_recent_block.clone()) 
            {
                log_print!(
                    LogFlags::NET, 
                    "failed to activate chain (%s)\n", 
                    state.to_string()
                );
            }
        }

        let mut guard = CS_MAIN.lock();

        let pindex: Option<Arc<BlockIndex>> 
        = self.chainman.get()
            .inner
            .blockman
            .lookup_block_index(&inv.hash);

        if pindex.is_none() {
            return;
        }

        if !self.clone().block_request_allowed(pindex.clone()) {
            log_print!(LogFlags::NET, "%s: ignoring request from peer=%i for old block that isn't in the main chain\n", func, pfrom.get_id());
            return;
        }

        let msg_maker: NetMsgMaker = NetMsgMaker::new(pfrom.get_common_version());

        // disconnect node in case we have reached
        // the outbound limit for serving
        // historical blocks
        if self.connman.get().outbound_target_reached(true) 
        && (
            (
                PINDEX_BEST_HEADER.lock().is_some() && 
                (
                    OffsetDateTime::from_unix_timestamp(PINDEX_BEST_HEADER.lock().as_ref().unwrap().get_block_time()).unwrap() 
                    - OffsetDateTime::from_unix_timestamp(pindex.as_ref().unwrap().get_block_time()).unwrap() 
                    > HISTORICAL_BLOCK_AGE
                )
            ) 
            || inv.is_msg_filtered_blk()
        ) 
        && !pfrom.has_permission(NetPermissionFlags::Download) 
        {
            // nodes with the download permission may exceed target
            log_print!(LogFlags::NET,"historical block serving limit reached, disconnect peer=%d\n",pfrom.get_id());

            pfrom.mark_for_disconnect();

            return;
        }

        // Avoid leaking prune-height by never
        // sending blocks below the
        // NODE_NETWORK_LIMITED threshold
        if !pfrom.has_permission(NetPermissionFlags::NoBan) 
        && ((
            ((pfrom.get_local_services() & ServiceFlags::NODE_NETWORK_LIMITED) == ServiceFlags::NODE_NETWORK_LIMITED) 
            && ((pfrom.get_local_services() & ServiceFlags::NODE_NETWORK) != ServiceFlags::NODE_NETWORK) 
            && (
            self.chainman.get().active_chain().tip().as_ref().unwrap().n_height 
            - pindex.as_ref().unwrap().n_height > 
            /* add two blocks buffer extension for possible races */
            NODE_NETWORK_LIMITED_MIN_BLOCKS as i32 + 2
        ))) {

            log_print!(LogFlags::NET, "Ignore block request below NODE_NETWORK_LIMITED threshold, disconnect peer=%d\n", pfrom.get_id());

            // disconnect node and prevent it from
            // stalling (would otherwise wait for
            // the missing block)
            pfrom.mark_for_disconnect();

            return;
        }

        // Pruned nodes may have deleted the
        // block, so check whether it's available
        // before trying to send.
        if (pindex.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) == 0 {
            return;
        }

        let mut pblock: Amo<Block> = Amo::<Block>::none();

        if a_recent_block.is_some() && a_recent_block.get().get_hash() == pindex.as_ref().unwrap().get_block_hash() {

            pblock = a_recent_block;

        } else {

            if inv.is_msg_witness_blk() {

                // Fast-path: in this case it is
                // possible to serve the block
                // directly from disk, as the
                // network format matches the
                // format on disk
                let mut block_data: Amo<Vec<u8>> = Amo::<Vec<u8>>::none();

                if !read_raw_block_from_disk_with_blockindex(
                    block_data.clone(),
                    pindex.clone().unwrap(),
                    self.chainparams.message_start()
                ) {
                    panic!("cannot load block from disk");
                }

                self.connman.get_mut().push_message(
                    &mut *pfrom, 
                    msg_maker.make(
                        NetMsgType::BLOCK, 
                        &[
                            &block_data
                        ]
                    )
                );

                //  Don't set pblock as we've sent the block

            } else {

                //  Send block from disk
                let pblock_read: Amo::<Block> = Amo::<Block>::none();

                if !read_block_from_disk_with_blockindex(
                    &mut pblock_read.get_mut(),
                    pindex.clone().unwrap(),
                    &self.chainparams.get_consensus()
                ) {
                    panic!("cannot load block from disk");
                }

                pblock = pblock_read.clone();
            }
        }

        if pblock.is_some() {

            if inv.is_msg_blk() {

                self.connman
                    .get_mut()
                    .push_message(
                        &mut *pfrom, 
                        msg_maker.make_with_flags(
                            SERIALIZE_TRANSACTION_NO_WITNESS, 
                            NetMsgType::BLOCK, 
                            &[
                                &pblock
                            ]
                        )
                    );

            } else {

                if inv.is_msg_witness_blk() {

                    self.connman.get_mut().push_message(
                        &mut *pfrom, 
                        msg_maker.make(
                            NetMsgType::BLOCK, 
                            &[
                                &pblock
                            ]
                        )
                    );

                } else {

                    if inv.is_msg_filtered_blk() {

                        let mut send_merkle_block: bool = false;

                        let mut merkle_block = MerkleBlock::default();

                        if pfrom.has_tx_relay() {

                            let tx_relay = pfrom.get_tx_relay();

                            let mut guard = tx_relay.cs_filter.lock();

                            if guard.pfilter.is_some() {

                                send_merkle_block = true;

                                merkle_block = MerkleBlock::new_with_block_and_filter(
                                    pblock.clone(),
                                    guard.pfilter.as_mut().unwrap()
                                );
                            }
                        }

                        if send_merkle_block {

                            self.connman.get_mut()
                                .push_message(
                                    &mut *pfrom, 
                                    msg_maker.make(
                                        NetMsgType::MERKLEBLOCK, 
                                        &[
                                            &merkle_block
                                        ]
                                    )
                                );

                            //  CMerkleBlock just
                            //  contains hashes,
                            //  so also push any
                            //  transactions in
                            //  the block the
                            //  client did not see
                            //
                            //  This avoids
                            //  hurting
                            //  performance by
                            //  pointlessly
                            //  requiring
                            //  a round-trip
                            //
                            //  Note that there is
                            //  currently no way
                            //  for a node to
                            //  request any single
                            //  transactions we
                            //  didn't send here
                            //  - they must either
                            //  disconnect and
                            //  retry or request
                            //  the full block.
                            //
                            //  Thus, the protocol
                            //  spec specified
                            //  allows for us to
                            //  provide duplicate
                            //  txn here, however
                            //  we MUST always
                            //  provide at least
                            //  what the remote
                            //  peer needs
                            pub type PairType = (u32,u256);

                            for pair in merkle_block.matched_txn.iter() {
                                self.connman.get_mut()
                                    .push_message(
                                        &mut *pfrom, 
                                        msg_maker.make_with_flags(
                                            SERIALIZE_TRANSACTION_NO_WITNESS, 
                                            NetMsgType::TX, 
                                            &[
                                                &pblock.get().vtx[pair.0 as usize]
                                            ]
                                        )
                                    );
                            }
                        }

                        //  else no response
                    } else {

                        if inv.is_msg_cmpct_blk() {

                            // If a peer is asking
                            // for old blocks, we're
                            // almost guaranteed they
                            // won't have a useful mempool
                            // to match against a compact
                            // block, and we don't feel
                            // like constructing the
                            // object for them, so instead
                            // we respond with the full,
                            // non-compact block.
                            let peer_wants_witness: bool 
                            = create_state(pfrom.get_id()).get().wants_cmpct_witness.load(atomic::Ordering::Relaxed);

                            let n_send_flags: i32 = match peer_wants_witness {
                                true   => 0,
                                false  => SERIALIZE_TRANSACTION_NO_WITNESS
                            };

                            let active_chain_height = self.chainman.get().active_chain().height().unwrap();

                            let node_height = pindex.as_ref().unwrap().n_height;

                            if self.clone().can_direct_fetch() 
                            && node_height >= (active_chain_height - MAX_CMPCTBLOCK_DEPTH).try_into().unwrap() {

                                if (peer_wants_witness || !witnesses_present_in_arecent_compact_block) 
                                && a_recent_compact_block.is_some() 
                                && a_recent_compact_block.get().header.get_hash() == pindex.as_ref().unwrap().get_block_hash() 
                                {
                                    self.connman.get_mut()
                                        .push_message(
                                            &mut *pfrom, 
                                            msg_maker.make_with_flags(
                                                n_send_flags, 
                                                NetMsgType::CMPCTBLOCK, 
                                                &[
                                                    &a_recent_compact_block.clone()
                                                ]
                                            )
                                        );

                                } else {

                                    let cmpctblock: BlockHeaderAndShortTxIDs 
                                    = BlockHeaderAndShortTxIDs::new(
                                        pblock.clone(), 
                                        peer_wants_witness
                                    );

                                    self.connman.get_mut()
                                        .push_message(
                                            &mut *pfrom, 
                                            msg_maker.make_with_flags(
                                                n_send_flags, 
                                                NetMsgType::CMPCTBLOCK, 
                                                &[
                                                    &cmpctblock
                                                ]
                                            )
                                        );
                                }

                            } else {

                                self.connman.get_mut()
                                    .push_message(
                                        &mut *pfrom, 
                                        msg_maker.make_with_flags(
                                            n_send_flags, 
                                            NetMsgType::BLOCK, 
                                            &[
                                                &pblock.clone()
                                            ]
                                        )
                                    );
                            }
                        }
                    }
                }
            }
        }

        {
            let gpeer = peer.get();

            let mut guard = gpeer.block_inv_mutex.lock();

            // Trigger the peer node to send
            // a getblocks request for the next
            // batch of inventory
            if inv.hash == gpeer.block_inv_mutex.lock().continuation_block {

                // Send immediately. This must
                // send even if redundant, and we
                // want it right after the last
                // block so they don't wait for
                // other stuff first.
                let mut inv: Vec<Inv> = vec![];

                inv.push(
                    Inv::new(
                        GetDataMsg::MSG_BLOCK.bits(),
                        &self.chainman.get().active_chain().tip().as_ref().unwrap().get_block_hash()
                    )
                );

                self.connman.get_mut()
                    .push_message(
                        &mut *pfrom, 
                        msg_maker.make(
                            NetMsgType::INV, 
                            &[
                            &inv
                            ]
                        )
                    );

                gpeer.block_inv_mutex.lock()
                    .continuation_block.set_null();
            }
        }
    }
}
