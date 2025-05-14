// ---------------- [ File: bitcoin-peerman/src/process_pong_message.rs ]
crate::ix!();

impl PeerManager {

    pub fn process_pong_message(
        self:               Arc<Self>, 
        peer:               &Option<Peer>,
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        msg_type:           &str,
        recv:               &mut DataStream,
        time_received:      &OffsetDateTime /* micros */,
        interrupt_msg_proc: &AtomicBool)  {

        let ping_end = time_received;
        let mut nonce: u64 = 0;
        let n_avail: usize = recv.in_avail().try_into().unwrap();
        let mut ping_finished: bool = false;

        let mut problem = String::default();

        if n_avail >= size_of_val(&nonce) {

            recv.stream_into(&mut nonce);

            // Only process pong message if
            // there is an outstanding ping
            // (old ping without nonce should
            // never pong)
            if peer.as_ref().unwrap().ping_nonce_sent.load(atomic::Ordering::Relaxed) != 0 {

                if nonce == peer.as_ref().unwrap().ping_nonce_sent.load(atomic::Ordering::Relaxed) {

                    // Matching pong received,
                    // this ping is no longer
                    // outstanding
                    ping_finished = true;

                    let ping_start = 
                    peer.as_ref().unwrap()
                        .ping_start.load(atomic::Ordering::Relaxed);

                    let ping_time = *ping_end - ping_start;

                    if ping_time.as_seconds_f64() >= 0.0_f64 {

                        // Let connman know
                        // about this
                        // successful
                        // ping-pong
                        pfrom.pong_received(ping_time);

                    } else {

                        // This should never happen
                        problem = "Timing mishap".to_string();
                    }

                } else {

                    // Nonce mismatches are
                    // normal when pings are
                    // overlapping
                    problem = "Nonce mismatch".to_string();

                    if nonce == 0 {

                        // This is most likely
                        // a bug in another
                        // implementation
                        // somewhere; cancel
                        // this ping
                        ping_finished = true;
                        problem = "Nonce zero".to_string();
                    }
                }

            } else {
                problem = "Unsolicited pong without ping".to_string();
            }

        } else {

            // This is most likely a bug in
            // another implementation
            // somewhere; cancel this ping
            ping_finished = true;
            problem = "Short payload".to_string();
        }

        if !(problem.is_empty()) {
            log_print!(
                LogFlags::NET, 
                "pong peer=%d: %s, %x expected, %x received, %u bytes\n", 
                pfrom.get_id(), 
                problem, 
                peer.as_ref().unwrap().ping_nonce_sent, 
                nonce, 
                n_avail
            );
        }

        if ping_finished {
            peer.as_ref().unwrap().ping_nonce_sent.store(0, atomic::Ordering::Relaxed);
        }
    }
}
