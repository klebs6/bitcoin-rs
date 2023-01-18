crate::ix!();

pub trait FindNextBlocksToDownload {

    fn find_next_blocks_to_download(self: Arc<Self>, 
        nodeid:       NodeId,
        count:        u32,
        blocks:       &mut Vec<Option<Arc<BlockIndex>>>,
        node_staller: &mut NodeId);
}

impl FindNextBlocksToDownload for PeerManager {

    /**
      | Update pindexLastCommonBlock and
      | add not-in-flight missing successors
      | to vBlocks, until it has at most count
      | entries.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn find_next_blocks_to_download(self: Arc<Self>, 
        nodeid:       NodeId,
        count:        u32,
        blocks:       &mut Vec<Option<Arc<BlockIndex>>>,
        node_staller: &mut NodeId)  {

        if count == 0 {
            return;
        }

        blocks.reserve(blocks.len() + usize::try_from(count).unwrap());

        let state: Amo<NodeState> = create_state(nodeid);

        assert!(state.is_some());

        // Make sure pindexBestKnownBlock is up to
        // date, we'll need it.
        self.clone().process_block_availability(nodeid);

        let best_known_block = &state.get().pindex_best_known_block;

        let active_n_chain_work = {

            self.chainman.get()
                .active_chain()
                .tip()
                .as_ref()
                .unwrap()
                .n_chain_work
                .clone()
        };

        if best_known_block.is_none() 
        || best_known_block.as_ref().unwrap().n_chain_work < active_n_chain_work 
        || best_known_block.as_ref().unwrap().n_chain_work < *N_MINIMUM_CHAIN_WORK 
        {
            // This peer has nothing interesting.
            return;
        }

        if state.get().pindex_last_common_block.is_none() {

            let chainman     = self.chainman.get();
            let active_chain = chainman.active_chain();

            let active_chain_height 
            = active_chain.height().unwrap();

            let best_known_block_height 
            = state.get().pindex_best_known_block.as_ref().unwrap().n_height;

            let common_height = min( 
                best_known_block_height, 
                active_chain_height.try_into().unwrap()
            );

            // Bootstrap quickly by guessing
            // a parent of our best tip is the
            // forking point.
            //
            // Guessing wrong in either direction
            // is not a problem.
            state.get_mut().pindex_last_common_block 
                = active_chain[common_height].clone();
        }

        // If the peer reorganized, our previous
        // pindexLastCommonBlock may not be an
        // ancestor of its current tip anymore. Go
        // back enough to fix that.
        state.get_mut().pindex_last_common_block 
            = last_common_ancestor(
                state.get().pindex_last_common_block.clone(),
                state.get().pindex_best_known_block.clone()
            );

        if state.get().pindex_last_common_block.as_ref().unwrap() 
        == state.get().pindex_best_known_block.as_ref().unwrap() {
            return;
        }

        let consensus_params: Arc<ChainConsensusParams> = self.chainparams.get_consensus();

        let mut to_fetch: Vec<Option<Arc<BlockIndex>>> = vec![];

        let mut pindex_walk: Option<Arc<BlockIndex>> = state.get().pindex_last_common_block.clone();

        let last_common_block_height = 
        state.get()
            .pindex_last_common_block
            .as_ref()
            .unwrap()
            .n_height;

        // Never fetch further than the best block
        // we know the peer has, or more than
        // BLOCK_DOWNLOAD_WINDOW + 1 beyond the
        // last linked block we have in common
        // with this peer. The +1 is so we can
        // detect stalling, namely if we would be
        // able to download that next block if the
        // window were 1 larger.
        let n_window_end: i32 = 
        last_common_block_height
        + i32::try_from(BLOCK_DOWNLOAD_WINDOW).unwrap();

        let n_max_height: i32 = min(

            state.get()
            .pindex_best_known_block
            .as_ref()
            .unwrap()
            .n_height,

            n_window_end + 1
        );

        let mut waitingfor: NodeId = -1;

        while pindex_walk
            .as_ref()
            .unwrap()
            .n_height < n_max_height 
        {
            // Read up to 128 (or more, if more
            // blocks than that are needed)
            // successors of pindexWalk (towards
            // pindexBestKnownBlock) into
            // vToFetch. We fetch 128, because
            // CBlockIndex::GetAncestor may be as
            // expensive as iterating over ~100
            // CBlockIndex* entries anyway.
            let n_to_fetch: i32 = min(

                n_max_height 
                - pindex_walk
                    .as_ref()
                    .unwrap()
                    .n_height,

                max(
                    (count - u32::try_from(blocks.len()).unwrap()).try_into().unwrap(),
                    128
                )
            );

            to_fetch.resize(
                n_to_fetch.try_into().unwrap(), 
                None
            );

            pindex_walk = state
                .get()
                .pindex_best_known_block
                .as_ref()
                .unwrap()
                .clone()
                .get_ancestor(pindex_walk.as_ref().unwrap().n_height + n_to_fetch);

            to_fetch[usize::try_from(n_to_fetch - 1).unwrap()] = pindex_walk.clone();

            for i in ((0 + 1)..=n_to_fetch - 1).rev() {

                to_fetch[usize::try_from(i - 1).unwrap()] 
                    = to_fetch[usize::try_from(i).unwrap()].as_ref().unwrap().pprev.clone();
            }

            // Iterate over those blocks in
            // vToFetch (in forward direction),
            // adding the ones that are not yet
            // downloaded and not in flight to
            // vBlocks. In the meantime, update
            // pindexLastCommonBlock as long as
            // all ancestors are already
            // downloaded, or if it's already part
            // of our chain (and therefore don't
            // need it even if pruned).
            for pindex in to_fetch.iter() {

                if !pindex.as_ref().unwrap().is_valid(Some(BlockStatus::BLOCK_VALID_TREE)) {

                    // We consider the chain that
                    // this peer is on invalid.
                    return;
                }

                if !create_state(nodeid).get().have_witness.load(atomic::Ordering::Relaxed) 
                && deployment_active_at_with_buried_deployment(
                    pindex.clone().unwrap(),
                    &consensus_params,
                    ConsensusBuriedDeployment::DEPLOYMENT_SEGWIT
                ) 
                {
                    // We wouldn't download this
                    // block or its descendants
                    // from this peer.
                    return;
                }

                if (pindex.as_ref().unwrap().n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) != 0
                || self.chainman.get().active_chain().contains(pindex.clone()) 
                {

                    if pindex.as_ref().unwrap().have_txs_downloaded() {
                        state.get_mut().pindex_last_common_block = pindex.clone();
                    }

                } else {

                    if !self.inner.lock().is_block_requested(
                        &pindex.as_ref().unwrap().get_block_hash()
                    ) {

                        // The block is not
                        // already downloaded, and
                        // not yet in flight.
                        if pindex.as_ref().unwrap().n_height > n_window_end {

                            // We reached the end of the window.
                            if blocks.len() == 0 && waitingfor != nodeid {

                                // We aren't able
                                // to fetch
                                // anything, but
                                // we would be if
                                // the download
                                // window was one
                                // larger.
                                *node_staller = waitingfor;
                            }
                            return;
                        }

                        blocks.push(pindex.clone());

                        if blocks.len() == count.try_into().unwrap() {
                            return;
                        }

                    } else {

                        if waitingfor == -1 {

                            let idx: u256 = pindex
                                .as_ref().unwrap()
                                .get_block_hash();

                            let inner = self.inner.lock();
                            let mbif  = inner.map_blocks_in_flight.lock();

                            // This is the first
                            // already-in-flight
                            // block.
                            waitingfor = mbif[&idx].0;
                        }
                    }
                }
            }
        }
    }
}

