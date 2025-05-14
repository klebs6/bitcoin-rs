// ---------------- [ File: bitcoin-peerman/src/process_getdata_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_getdata_message(
        self:               Arc<Self>,
        peer:               Amo<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut inv: Vec<Inv> = vec![];

        recv.stream(&inv);

        if inv.len() > MAX_INV_SZ.try_into().unwrap() {

            self.misbehaving(
                pfrom.get_id(), 
                20, 
                &format!("getdata message size = {}",inv.len())
            );

            return;
        }

        log_print!(
            LogFlags::NET, 
            "received getdata (%u invsz) peer=%d\n", 
            inv.len(), 
            pfrom.get_id()
        );

        if inv.len() > 0 {

            log_print!(
                LogFlags::NET, 
                "received getdata for: %s peer=%d\n", 
                inv[0].to_string(), 
                pfrom.get_id()
            );
        }

        {
            let mut peer_guard = peer.get_mut();

            let mut getdata_requests = peer_guard.getdata_requests.lock();

            getdata_requests.extend(inv.iter().cloned());

            self.process_get_data(
                pfrom, 
                peer.clone(), 
                interrupt_msg_proc
            );
        }
    }
}
