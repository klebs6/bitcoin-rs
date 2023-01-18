crate::ix!();

impl PeerManager {

    pub fn process_getblocks_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut locator = BlockLocator::default();

        let mut hash_stop = u256::default();

        recv.stream_into(&mut locator);
        recv.stream_into(&mut hash_stop);

        if locator.have.len() > MAX_LOCATOR_SZ.try_into().unwrap() {

            log_print!(
                LogFlags::NET, 
                "getblocks locator size %lld > %d, disconnect peer=%d\n", 
                locator.have.len(), 
                MAX_LOCATOR_SZ, 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        // We might have announced the
        // currently-being-connected tip using
        // a compact block, which resulted in the
        // peer sending a getblocks request, which
        // we would otherwise respond to without
        // the new block.
        //
        // To avoid this situation we simply
        // verify that we are on our best known
        // chain now. This is super overkill, but
        // we handle it better for getheaders
        // requests, and there are no known nodes
        // which support compact blocks but still
        // use getblocks to request blocks.
        {
            let mut a_recent_block: Amo<Block> = Amo::<Block>::none();

            {
                let mut guard = CS_MOST_RECENT_BLOCK.get();

                a_recent_block = MOST_RECENT_BLOCK.clone();
            }

            let mut state = BlockValidationState::default();

            if !self.chainman.get()
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

        let blockman = &mut self.chainman.get_mut().inner.blockman;

        // Find the last block the caller has
        // in the main chain
        let mut pindex: Option<Arc<BlockIndex>>
        = find_fork_in_global_index(
            blockman,
            self.chainman.get().active_chain(), 
            &locator
        );

        // Send the rest of the chain
        if pindex.is_some() {
            pindex = self.chainman.get().active_chain().next(pindex);
        }

        let mut n_limit: i32 = 500;

        log_print!(
            LogFlags::NET, 
            "getblocks %d to %s limit %d from peer=%d\n", 
            match pindex {
                Some(pindex) => pindex.unwrap().lock().n_height,
                None         => -1
            }, 
            match hash_stop.is_null() {
                true   => "end",
                false  => hash_stop.to_string()
            }, 
            n_limit, 
            pfrom.get_id()
        );

        while pindex.is_some() {

            if pindex.as_ref().unwrap().get_block_hash() == hash_stop {

                log_print!(
                    LogFlags::NET, 
                    "  getblocks stopping at %d %s\n", 
                    pindex.unwrap().lock().n_height, 
                    pindex.unwrap().lock().get_block_hash().to_string()
                );

                break;
            }

            // If pruning, don't inv blocks
            // unless we have on disk and are
            // likely to still have for some
            // reasonable time window (1 hour)
            // that block relay might require.
            let n_pruned_blocks_likely_to_have: i32 
            = (
                MIN_BLOCKS_TO_KEEP 
                - usize::try_from(3600 / self.chainparams.get_consensus().n_pow_target_spacing).unwrap()
            ).try_into().unwrap();

            if *PRUNE_MODE 
            && (
            (pindex.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) == 0 
            || pindex.as_ref().unwrap().n_height <= self.chainman.get().active_chain().tip().as_ref().unwrap().n_height - n_pruned_blocks_likely_to_have
        ) {

                log_print!(
                    LogFlags::NET, 
                    " getblocks stopping, pruned or too old block at %d %s\n", 
                    pindex.unwrap().lock().n_height, 
                    pindex.unwrap().lock().get_block_hash().to_string()
                );

                break;
            }

            {
                let mut guard = peer.as_ref().unwrap().block_inv_mutex.lock();

                peer.as_ref().unwrap()
                    .block_inv_mutex
                    .lock()
                    .blocks_for_inv_relay
                    .push(pindex.as_ref().unwrap().get_block_hash());
            }

            if {
                n_limit -= 1;
                n_limit
            } <= 0 {

                // When this block is
                // requested, we'll send an
                // inv that'll trigger the
                // peer to getblocks the next
                // batch of inventory.
                log_print!(
                    LogFlags::NET,
                    "  getblocks stopping at limit %d %s\n",
                    pindex.unwrap().lock().n_height,
                    pindex.unwrap().lock().get_block_hash().to_string()
                );

                {
                    let mut guard = peer.as_ref().unwrap().block_inv_mutex.lock();

                    peer.as_ref()
                        .unwrap()
                        .block_inv_mutex
                        .lock()
                        .continuation_block = pindex.as_ref().unwrap().get_block_hash();
                }

                break;
            }

            pindex = self.chainman.get().active_chain().next(pindex);
        }
    }
}

