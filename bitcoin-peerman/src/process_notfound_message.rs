// ---------------- [ File: bitcoin-peerman/src/process_notfound_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_notfound_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut inv: Vec<Inv> = vec![];

        recv.stream_into(&mut inv);

        if inv.len() <= (MAX_PEER_TX_ANNOUNCEMENTS + MAX_BLOCKS_IN_TRANSIT_PER_PEER).try_into().unwrap() {

            let mut guard = CS_MAIN.lock();

            for inv in inv.iter() {

                if inv.is_gen_tx_msg() {

                    // If we receive
                    // a NOTFOUND message for
                    // a tx we requested, mark
                    // the announcement for it
                    // as completed in
                    // TxRequestTracker.
                    self.inner.lock().txrequest.lock().received_response(
                        pfrom.get_id(), 
                        &inv.hash
                    );
                }
            }
        }
    }
}
