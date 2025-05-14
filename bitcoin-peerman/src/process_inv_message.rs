// ---------------- [ File: bitcoin-peerman/src/process_inv_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_inv_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          &NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut inv: Vec<Inv> = vec![];

        recv.stream_into(&mut inv);

        if inv.len() > MAX_INV_SZ.try_into().unwrap() {

            self.misbehaving(
                pfrom.get_id(), 
                20, 
                format!("inv message size = {}",inv.len()).as_str()
            );

            return;
        }

        // Reject tx INVs when the -blocksonly
        // setting is enabled, or this is
        // a block-relay-only peer
        let mut reject_tx_invs: bool = self.ignore_incoming_txs || !pfrom.has_tx_relay();

        // Allow peers with relay permission to
        // send data other than blocks in blocks
        // only mode
        if pfrom.has_permission(NetPermissionFlags::Relay) {
            reject_tx_invs = false;
        }

        let mut main_guard = CS_MAIN.lock();

        let current_time = get_datetime();

        let mut best_block: Option<&u256> = None;

        for inv in inv.iter() {

            if interrupt_msg_proc.load(atomic::Ordering::Relaxed) {
                return;
            }

            // Ignore INVs that don't match
            // wtxidrelay setting.
            //
            // Note that orphan parent fetching
            // always uses MSG_TX GETDATAs
            // regardless of the wtxidrelay
            // setting.
            //
            // This is fine as no INV messages are
            // involved in that process.
            if create_state(pfrom.get_id()).get().wtxid_relay.load(atomic::Ordering::Relaxed) {

                if inv.is_msg_tx() {
                    continue;
                }

            } else {
                if inv.is_msg_wtx() {
                    continue;
                }
            }

            if inv.is_msg_blk() {

                let already_have: bool 
                = self.clone().already_have_block(&inv.hash);

                log_print!(
                    LogFlags::NET, 
                    "got inv: %s  %s peer=%d\n", 
                    inv.to_string(), 
                    match already_have {
                        true   => "have",
                        false  => "new"
                    }, 
                    pfrom.get_id()
                );

                self.clone().update_block_availability(
                    pfrom.get_id(), 
                    &inv.hash
                );

                if !already_have 
                && !IMPORTING.load(atomic::Ordering::Relaxed) 
                && !REINDEX.load(atomic::Ordering::Relaxed) 
                && !self.is_block_requested(&inv.hash) 
                {
                    // Headers-first is the
                    // primary method of
                    // announcement on the
                    // network. If a node fell
                    // back to sending blocks by
                    // inv, it's probably for
                    // a re-org. The final block
                    // hash provided should be the
                    // highest, so send
                    // a getheaders and then fetch
                    // the blocks we need to catch
                    // up.
                    best_block = Some(&inv.hash);
                }

            } else {

                if inv.is_gen_tx_msg() {

                    if reject_tx_invs {

                        log_print!(
                            LogFlags::NET, 
                            "transaction (%s) inv sent in violation of protocol, disconnecting peer=%d\n", 
                            inv.hash.to_string(), 
                            pfrom.get_id()
                        );

                        pfrom.mark_for_disconnect();

                        return;
                    }

                    let gtxid:     GenTxId = (inv.clone()).into();
                    let already_have: bool = self.clone().already_have_tx(&gtxid);

                    log_print!(
                        LogFlags::NET, 
                        "got inv: %s  %s peer=%d\n", 
                        inv.to_string(), 
                        match already_have {
                            true   => "have",
                            false  => "new"
                        }, 
                        pfrom.get_id()
                    );

                    pfrom.add_known_tx(&inv.hash);

                    if !already_have 
                    && !self.chainman.get().active_chainstate().is_initial_block_download() {
                        self.clone().add_tx_announcement(pfrom, &gtxid, current_time);
                    }

                } else {

                    log_print!(
                        LogFlags::NET, 
                        "Unknown inv type \"%s\" received from peer=%d\n", 
                        inv.to_string(), 
                        pfrom.get_id()
                    );
                }
            }
        }

        if best_block.is_some() {

            self.connman.get_mut().push_message(
                &mut *pfrom, 
                msg_maker.make(
                    NetMsgType::GETHEADERS, 
                    &[
                        &self.chainman.get().active_chain().get_locator(PINDEX_BEST_HEADER.lock().clone()), 
                        best_block.unwrap()
                    ]
                )
            );

            log_print!(
                LogFlags::NET, 
                "getheaders (%d) %s to peer=%d\n", 
                (*PINDEX_BEST_HEADER.lock()).n_height, 
                best_block.unwrap().to_string(), 
                pfrom.get_id()
            );
        }
    }
}
