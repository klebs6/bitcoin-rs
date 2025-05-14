// ---------------- [ File: bitcoin-peerman/src/process_feefilter_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_feefilter_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let new_fee_filter: Amount = 0;

        recv.stream(new_fee_filter);

        if money_range(&new_fee_filter) {

            if pfrom.has_tx_relay() {

                pfrom.get_tx_relay()
                    .min_fee_filter.store(
                        new_fee_filter, 
                        atomic::Ordering::Relaxed
                    );
            }

            log_print!(
                LogFlags::NET, 
                "received: feefilter of %s from peer=%d\n", 
                fee_rate(new_fee_filter).to_string(), 
                pfrom.get_id()
            );
        }
    }
}
