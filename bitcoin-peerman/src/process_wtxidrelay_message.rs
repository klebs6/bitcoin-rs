crate::ix!();

impl PeerManager {

    pub fn process_wtxidrelay_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if pfrom.is_successfully_connected() {

            // Disconnect peers that send
            // a wtxidrelay message after
            // VERACK.
            log_print!(
                LogFlags::NET,
                "wtxidrelay received after verack from peer=%d; disconnecting\n",
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        if pfrom.get_common_version() >= WTXID_RELAY_VERSION {

            let mut main_guard = CS_MAIN.lock();

            let cstate = create_state(pfrom.get_id());

            let state = cstate.get();

            if !state.wtxid_relay.load(atomic::Ordering::Relaxed) 
            {

                let cstate = create_state(pfrom.get_id());

                let state = cstate.get();

                state.wtxid_relay.store(true, atomic::Ordering::Relaxed);

                self.inner.lock().wtxid_relay_peers.fetch_add(1, atomic::Ordering::Relaxed);

            } else {

                log_print!(
                    LogFlags::NET, 
                    "ignoring duplicate wtxidrelay from peer=%d\n", 
                    pfrom.get_id()
                );
            }

        } else {

            log_print!(
                LogFlags::NET, 
                "ignoring wtxidrelay due to old common version=%d from peer=%d\n", 
                pfrom.get_common_version(), 
                pfrom.get_id()
            );
        }
    }
}
