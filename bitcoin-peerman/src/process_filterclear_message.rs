// ---------------- [ File: bitcoin-peerman/src/process_filterclear_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_filterclear_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if (pfrom.get_local_services() & ServiceFlags::NODE_BLOOM).bits() == 0 {

            log_print!(
                LogFlags::NET, 
                "filterclear received despite not offering bloom services from peer=%d; disconnecting\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        if !pfrom.has_tx_relay() {
            return;
        }

        let tx_relay = pfrom.get_tx_relay();

        let mut guard = tx_relay.cs_filter.lock();

        guard.pfilter    = None;
        guard.relay_txes = true;
    }
}
