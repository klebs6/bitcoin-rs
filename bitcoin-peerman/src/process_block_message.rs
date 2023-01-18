crate::ix!();

impl PeerManager {

    pub fn process_block_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        // Ignore block received while
        // IMPORTING.load(atomic::Ordering::Relaxed)
        if IMPORTING.load(atomic::Ordering::Relaxed) || REINDEX.load(atomic::Ordering::Relaxed) {

            log_print!(
                LogFlags::NET, 
                "Unexpected block message received from peer %d\n", 
                pfrom.get_id()
            );

            return;
        }

        let pblock: Amo::<Block> = amo_none();

        recv.stream_into(&mut pblock.get_mut());

        log_print!(
            LogFlags::NET, 
            "received block %s peer=%d\n", 
            (*pblock).get_hash().to_string(), 
            pfrom.get_id()
        );

        let mut force_processing: bool = false;

        let hash: u256 = pblock.get().get_hash();

        {
            let mut guard = CS_MAIN.lock();

            // Always process the block if we
            // requested it, since we may need
            // it even when it's not
            // a candidate for a new best tip.
            force_processing = self.is_block_requested(&hash);

            self.remove_block_request(&hash);

            // mapBlockSource is only used for
            // punishing peers and setting
            // which peers send us compact
            // blocks, so the race between
            // here and CS_MAIN in
            // ProcessNewBlock is fine.
            self.inner.lock().map_block_source.insert(hash, (pfrom.get_id(),true));
        }

        self.process_block(pfrom, pblock.clone(), force_processing);
    }
}
