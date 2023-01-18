crate::ix!();

impl PeerManager {

    pub fn process_sendcmpct_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let mut announce_usingcmpctblock: bool = false;
        let mut n_cmpctblock_version: u64 = 0;

        recv.stream_into(&mut announce_usingcmpctblock);
        recv.stream_into(&mut n_cmpctblock_version);

        if n_cmpctblock_version == 1 || n_cmpctblock_version == 2 {

            let mut guard = CS_MAIN.lock();

            // fProvidesHeaderAndIDs is used to
            // "lock in" version of compact blocks
            // we send (fWantsCmpctWitness)
            if !create_state(pfrom.get_id()).get().provides_header_and_ids.load(atomic::Ordering::Relaxed) {
                create_state(pfrom.get_id()).get().provides_header_and_ids.store(true, atomic::Ordering::Relaxed);
                create_state(pfrom.get_id()).get().wants_cmpct_witness.store(n_cmpctblock_version == 2, atomic::Ordering::Relaxed);
            }

            if create_state(pfrom.get_id()).get().wants_cmpct_witness.load(atomic::Ordering::Relaxed) == (n_cmpctblock_version == 2) {

                let created_state = create_state(pfrom.get_id());

                let state = created_state.get();

                //  ignore later version announces
                state.prefer_header_and_ids.store(announce_usingcmpctblock,atomic::Ordering::Relaxed);

                // save whether peer selects us as
                // BIP152 high-bandwidth peer
                // (receiving sendcmpct(1) signals
                // high-bandwidth, sendcmpct(0)
                // low-bandwidth)
                pfrom.set_bip152_highbandwidth_from(announce_usingcmpctblock);
            }

            if !create_state(pfrom.get_id()).get().supports_desired_cmpct_version.load(atomic::Ordering::Relaxed) {
                create_state(pfrom.get_id()).get().supports_desired_cmpct_version.store(n_cmpctblock_version == 2, atomic::Ordering::Relaxed);
            }
        }
    }
}
