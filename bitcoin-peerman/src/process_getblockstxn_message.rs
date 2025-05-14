// ---------------- [ File: bitcoin-peerman/src/process_getblockstxn_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_getblockstxn_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut req = BlockTransactionsRequest::default();

        recv.stream_into(&mut req);

        let mut recent_block: Amo<Block> = Amo::<Block>::none();

        {
            let mut guard = CS_MOST_RECENT_BLOCK.get();

            if *MOST_RECENT_BLOCK_HASH.get() == req.blockhash {
                recent_block = MOST_RECENT_BLOCK.clone();
            }

            // Unlock cs_most_recent_block to
            // avoid CS_MAIN lock inversion
        }

        if recent_block.is_some() {

            self.send_block_transactions(
                pfrom, 
                &recent_block.get(), 
                &req
            );

            return;
        }

        {
            let mut guard = CS_MAIN.lock();

            let pindex: Option<Arc<BlockIndex>> 
            = self.chainman.get()
                .inner
                .blockman
                .lookup_block_index(&req.blockhash);

            if pindex.is_none() 
            || (pindex.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) == 0 {

                log_print!(
                    LogFlags::NET, 
                    "Peer %d sent us a getblocktxn for a block we don't have\n", 
                    pfrom.get_id()
                );

                return;
            }

            let active_chain_height = self.chainman.get().active_chain().height().unwrap();

            let target_height = active_chain_height - MAX_BLOCKTXN_DEPTH;

            if pindex.as_ref().unwrap().n_height >= target_height.try_into().unwrap() {

                let mut block = Block::default();

                let ret: bool = read_block_from_disk_with_blockindex(
                    &mut block,
                    pindex.unwrap(),
                    &self.chainparams.get_consensus()
                );

                assert!(ret);

                self.send_block_transactions(pfrom, &block, &req);

                return;
            }
        }

        // If an older block is requested
        // (should never happen in practice,
        // but can happen in tests) send
        // a block response instead of
        // a blocktxn response. Sending a full
        // block response instead of a small
        // blocktxn response is preferable in
        // the case where a peer might
        // maliciously send lots of
        // getblocktxn requests to trigger
        // expensive disk reads, because it
        // will require the peer to actually
        // receive all the data read from disk
        // over the network.
        log_print!(
            LogFlags::NET,
            "Peer %d sent us a getblocktxn for a block > %i deep\n",
            pfrom.get_id(),
            MAX_BLOCKTXN_DEPTH
        );

        let mut inv = Inv::default();

        {
            let mut guard = CS_MAIN.lock();

            let created_state = create_state(pfrom.get_id());

            let state = created_state.get();

            inv.ty = match state.wants_cmpct_witness.load(atomic::Ordering::Relaxed) {
                true   => GetDataMsg::MSG_WITNESS_BLOCK.bits(),
                false  => GetDataMsg::MSG_BLOCK.bits()
            };
        }

        inv.hash = req.blockhash;

        {
            let mut guard = peer.as_ref().unwrap().getdata_requests.lock();

            guard.push_back(inv);
        }

        // The message processing loop will go
        // around again (without pausing) and
        // we'll respond then
    }
}
