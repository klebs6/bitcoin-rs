crate::ix!();

impl PeerManager {

    pub fn process_filterload_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if (pfrom.get_local_services() & ServiceFlags::NODE_BLOOM).bits() == 0 {

            log_print!(
                LogFlags::NET, 
                "filterload received despite not offering bloom services from peer=%d; disconnecting\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        let mut filter = BloomFilter::default();

        recv.stream_into(&mut filter);

        if !filter.is_within_size_constraints() {

            // There is no excuse for sending
            // a too-large filter
            self.misbehaving(pfrom.get_id(),100,"too-large bloom filter");

        } else {

            if pfrom.has_tx_relay() {

                let tx_relay = pfrom.get_tx_relay();

                let mut guard = tx_relay.cs_filter.lock();

                guard.pfilter.replace(BloomFilter::default());
                guard.relay_txes = true;
            }
        }
    }
}
