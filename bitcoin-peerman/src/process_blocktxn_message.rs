crate::ix!();

impl PeerManager {

    pub fn process_blocktxn_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        msg_maker:          &NetMsgMaker,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        // Ignore blocktxn received while
        // IMPORTING.load(atomic::Ordering::Relaxed)
        if IMPORTING.load(atomic::Ordering::Relaxed) || REINDEX.load(atomic::Ordering::Relaxed) {

            log_print!(
                LogFlags::NET, 
                "Unexpected blocktxn message received from peer %d\n", 
                pfrom.get_id()
            );

            return;
        }

        let mut resp = BlockTransactions::default();

        recv.stream_into(&mut resp);

        let pblock: Amo::<Block> = amo_none();

        let mut block_read: bool = false;

        {
            let mut guard = CS_MAIN.lock();

            let inner = self.inner.lock();

            let mut mbif = inner.map_blocks_in_flight.lock();

            let mut found_unexpected_block_transaction = false;

            //let mut it = mbif.get_mut(&resp.blockhash) ;

            //HashMap<u256,(NodeId, QueuedBlockIter)>
            if let Some((ref mut nodeid, ref mut queued_block_iter)) = mbif.get_mut(&resp.blockhash) {

                if queued_block_iter.peek().unwrap().1.partial_block.is_some() 
                && *nodeid == pfrom.get_id() {

                    let queued_block_value = queued_block_iter.peek_mut().unwrap();

                    let mut partial_block = queued_block_value.1.partial_block.get_mut();

                    let status: ReadStatus = 
                    partial_block
                        .fill_block(&mut pblock.get_mut(), &resp.txn);

                    if status == ReadStatus::Invalid {

                        // Reset in-flight state in
                        // case Misbehaving does not
                        // result in a disconnect
                        self.remove_block_request(&resp.blockhash);

                        self.misbehaving(
                            pfrom.get_id(), 
                            100, 
                            "invalid compact block/non-matching block transactions"
                        );

                        return;

                    } else {

                        if status == ReadStatus::Failed {

                            // Might have collided,
                            // fall back to getdata
                            // now :(
                            let mut invs: Vec<Inv> = vec![];

                            invs.push(
                                Inv::new(
                                    (GetDataMsg::MSG_BLOCK | get_fetch_flags(&***pfrom)).bits(),
                                    &resp.blockhash
                                )
                            );

                            self.connman.get_mut().push_message(
                                &mut pfrom, 
                                msg_maker.make(NetMsgType::GETDATA, &[&invs])
                            );

                        } else {

                            // Block is either okay, or
                            // possibly we received
                            //
                            // ReadStatus::CheckblockFailed.
                            //
                            // Note that CheckBlock can
                            // only fail for one of a few
                            // reasons:
                            //
                            //  1. bad-proof-of-work
                            //     (impossible here,
                            //     because we've already
                            //     accepted the header)
                            //
                            //  2. merkleroot doesn't
                            //     match the transactions
                            //     given (already caught
                            //     in FillBlock with
                            //     ReadStatus::Failed, so
                            //     impossible here)
                            //
                            //  3. the block is otherwise
                            //     invalid (eg invalid
                            //     coinbase, block is too
                            //     big, too many legacy
                            //     sigops, etc).
                            //
                            //  So if CheckBlock failed,
                            //  #3 is the only
                            //  possibility.
                            //
                            //  Under BIP 152, we don't
                            //  discourage the peer unless
                            //  proof of work is invalid
                            //  (we don't require all the
                            //  stateless checks to have
                            //  been run).  This is
                            //  handled below, so just
                            //  treat this as though the
                            //  block was successfully
                            //  read, and rely on the
                            //  handling in
                            //  ProcessNewBlock to ensure
                            //  the block index is
                            //  updated, etc.
                            self.remove_block_request(&resp.blockhash);

                            // it is now an empty pointer
                            block_read = true;

                            // mapBlockSource is used for
                            // potentially punishing peers
                            // and updating which peers
                            // send us compact blocks, so
                            // the race between here and
                            // CS_MAIN in ProcessNewBlock
                            // is fine. BIP 152 permits
                            // peers to relay compact
                            // blocks after validating the
                            // header only; we should not
                            // punish peers if the block
                            // turns out to be invalid.
                            self.inner.lock().map_block_source.insert(
                                resp.blockhash, 
                                (pfrom.get_id(),false)
                            );
                        }
                    }

                } else {

                    log_print!(
                        LogFlags::NET, 
                        "Peer %d sent us block transactions for block we weren't expecting\n", 
                        pfrom.get_id()
                    );

                    return;
                }

            } else {

                log_print!(
                    LogFlags::NET, 
                    "Peer %d sent us block transactions with unknown hash\n", 
                    pfrom.get_id()
                );

                return;
            }
        }

        // Don't hold CS_MAIN when we call
        // into ProcessNewBlock
        if block_read {

            // Since we requested this block
            // (it was in mapBlocksInFlight),
            // force it to be processed, even
            // if it would not be a candidate
            // for new tip (missing previous
            // block, chain not long enough,
            // etc)
            //
            // This bypasses some anti-DoS
            // logic in AcceptBlock (eg to
            // prevent disk-space attacks),
            // but this should be safe due to
            // the protections in the compact
            // block handler -- see related
            // comment in compact block
            // optimistic reconstruction
            // handling.
            self.process_block(pfrom,pblock.clone(),/*force_processing=*/ true);
        }
    }
}
