// ---------------- [ File: bitcoin-peerman/src/process_filteradd_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_filteradd_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        if (pfrom.get_local_services() & ServiceFlags::NODE_BLOOM).bits() == 0 {

            log_print!(
                LogFlags::NET, 
                "filteradd received despite not offering bloom services from peer=%d; disconnecting\n", 
                pfrom.get_id()
            );

            pfrom.mark_for_disconnect();

            return;
        }

        let mut data: Vec<u8> = vec![];

        recv.stream(&data);

        // Nodes must NEVER send a data item
        // > 520 bytes (the max size for
        // a script data object, and thus, the
        // maximum size any matched object can
        // have) in a filteradd message
        let mut bad: bool = false;

        if data.len() > MAX_SCRIPT_ELEMENT_SIZE {

            bad = true;

        } else {

            if pfrom.has_tx_relay() {

                let tx_relay = pfrom.get_tx_relay_mut();

                let mut guard = tx_relay.cs_filter.lock();

                if guard.pfilter.is_some() {

                    guard
                        .pfilter.as_mut().unwrap()
                        .insert_key(&data);

                } else {

                    bad = true;
                }
            }
        }

        if bad {
            self.misbehaving(pfrom.get_id(), 100, "bad filteradd message");
        }
    }
}
