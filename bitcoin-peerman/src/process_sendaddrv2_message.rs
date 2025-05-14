// ---------------- [ File: bitcoin-peerman/src/process_sendaddrv2_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_sendaddrv2_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if pfrom.is_successfully_connected() {

            // Disconnect peers that send
            // a SENDADDRV2 message after
            // VERACK.
            log_print!(
                LogFlags::NET,
                "sendaddrv2 received after verack from peer=%d; disconnecting\n",
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        peer.as_ref().unwrap().wants_addrv2.store(true, atomic::Ordering::Relaxed);
    }
}
