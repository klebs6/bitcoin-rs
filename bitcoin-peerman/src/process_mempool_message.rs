// ---------------- [ File: bitcoin-peerman/src/process_mempool_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_mempool_message(self: Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {


        if (pfrom.get_local_services() & ServiceFlags::NODE_BLOOM).bits() == 0 
        && !pfrom.has_permission(NetPermissionFlags::Mempool) {

            if !pfrom.has_permission(NetPermissionFlags::NoBan) {

                log_print!(
                    LogFlags::NET, 
                    "mempool request with bloom filters disabled, disconnect peer=%d\n", 
                    pfrom.get_id()
                );

                pfrom.mark_for_disconnect();
            }

            return;
        }

        if self.connman.get().outbound_target_reached(false) 
        && !pfrom.has_permission(NetPermissionFlags::Mempool) {

            if !pfrom.has_permission(NetPermissionFlags::NoBan) {

                log_print!(
                    LogFlags::NET, 
                    "mempool request with bandwidth limit reached, disconnect peer=%d\n", 
                    pfrom.get_id()
                );

                pfrom.mark_for_disconnect();
            }

            return;
        }

        if pfrom.has_tx_relay() {

            let tx_relay = pfrom.get_tx_relay();

            let mut guard = tx_relay.cs_tx_inventory.lock();

            guard.send_mempool = true;
        }
    }
}
