// ---------------- [ File: bitcoin-peerman/src/process_getheaders_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_getheaders_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          &NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
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
                "getheaders locator size %lld > %d, disconnect peer=%d\n", 
                locator.have.len(), 
                MAX_LOCATOR_SZ, 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        let mut guard = CS_MAIN.lock();

        if self.chainman.get().active_chainstate().is_initial_block_download() 
        && !pfrom.has_permission(NetPermissionFlags::Download) 
        {
            log_print!(
                LogFlags::NET, 
                "Ignoring getheaders from peer=%d because node is in initial block download\n", 
                pfrom.get_id()
            );

            return;
        }

        let nodestate: Amo<NodeState> = create_state(pfrom.get_id());

        let mut pindex: Option<Arc<BlockIndex>> = None;

        if locator.is_null() {

            // If locator is null, return the
            // hashStop block
            pindex = self.chainman
                .get()
                .inner
                .blockman
                .lookup_block_index(&hash_stop);

            if pindex.is_none() {
                return;
            }

            if !self.clone().block_request_allowed(pindex.clone()) {

                log_print!(
                    LogFlags::NET, 
                    "%s: ignoring request from peer=%i for old block header that isn't in the main chain\n", 
                    func, 
                    pfrom.get_id()
                );

                return;
            }

        } else {

            let blockman = &mut self.chainman.get_mut().inner.blockman;

            // Find the last block the caller
            // has in the main chain
            pindex = find_fork_in_global_index(
                blockman,
                self.chainman.get().active_chain(), 
                &locator
            );

            if pindex.is_some() {

                pindex = self.chainman
                    .get()
                    .active_chain()
                    .next(pindex);
            }
        }

        // we must use CBlocks, as
        // CBlockHeaders won't include the
        // 0x00 nTx count at the end
        let mut headers: Vec<BlockHeader> = vec![];

        let mut n_limit: i32 = MAX_HEADERS_RESULTS.try_into().unwrap();

        log_print!(
            LogFlags::NET, 
            "getheaders %d to %s from peer=%d\n", 
            match pindex {
                Some(pindex) => pindex.lock().n_height,
                None         => -1
            }, 
            match hash_stop.is_null() {
                true   => "end",
                false  => hash_stop.to_string()
            }, 
            pfrom.get_id()
        );

        while pindex.is_some() {

            headers.push(pindex.as_ref().unwrap().get_block_header());

            if {
                n_limit -= 1;
                n_limit
            } <= 0 || pindex.as_ref().unwrap().get_block_hash() == hash_stop {
                break;
            }

            let chainman = self.chainman.get();

            pindex = chainman.active_chain().next(pindex);
        }

        // pindex can be nullptr either if we sent
        // m_chainman.ActiveChain().Tip() OR if
        // our peer has
        // m_chainman.ActiveChain().Tip() (and
        // thus we are sending an empty headers
        // message). In both cases it's safe to
        // update pindexBestHeaderSent to be our
        // tip.
        //
        // It is important that we simply reset
        // the BestHeaderSent value here, and not
        // max(BestHeaderSent, newHeaderSent). We
        // might have announced the
        // currently-being-connected tip using
        // a compact block, which resulted in the
        // peer sending a headers request, which
        // we respond to without the new block. By
        // resetting the BestHeaderSent, we ensure
        // we will re-announce the new block via
        // headers (or compact blocks again) in
        // the SendMessages logic.
        nodestate.get_mut().pindex_best_header_sent = match pindex.is_some() {
            true  => pindex,
            false => self.chainman.get().active_chain().tip()
        };

        self.connman.get_mut().push_message(
            &mut *pfrom, 
            msg_maker.make(NetMsgType::HEADERS, &[&headers])
        );
    }
}
