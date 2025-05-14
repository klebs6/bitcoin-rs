// ---------------- [ File: bitcoin-peerman/src/process_sendheaders_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_sendheaders_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        pfrom:              &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut main_guard = CS_MAIN.lock();

        let cstate = create_state(pfrom.get_id());

        let state = cstate.get();

        state.prefer_headers.store(true, atomic::Ordering::Relaxed);
    }
}
