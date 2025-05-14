// ---------------- [ File: bitcoin-peerman/src/process_messages.rs ]
crate::ix!();

impl ProcessMessages for PeerManager {
    
    fn process_messages(
        self:               Arc<Self>, 
        mut pfrom:          &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        interrupt_msg_proc: &AtomicBool) -> bool {

        let mut more_work: bool = false;

        let peer: Amo<Peer> = self.get_peer_ref((*pfrom).get_id());

        if peer.is_none() {
            return false;
        }

        {
            let guard = peer.get();

            let mut getdata_requests = guard.getdata_requests.lock();

            if !getdata_requests.is_empty() {

                self.clone().process_get_data(
                    pfrom, 
                    peer.clone(), 
                    interrupt_msg_proc
                );
            }
        }

        {
            let mut guard_main    = CS_MAIN.lock();
            let mut guard_orphans = G_CS_ORPHANS.lock();

            if !peer.get().orphan_work_set.is_empty() {

                self.clone().process_orphan_tx(
                    &mut *peer.get_mut().orphan_work_set
                );
            }
        }

        if (*pfrom).marked_for_disconnect() {
            return false;
        }

        //  this maintains the order of responses
        //  and prevents m_getdata_requests to grow unbounded
        {
            let peer = peer.get();

            let mut guard = peer.getdata_requests.lock();

            if !guard.is_empty() {
                return true;
            }
        }

        {
            let mut guard = G_CS_ORPHANS.lock();

            if !peer.get().orphan_work_set.is_empty() {
                return true;
            }
        }

        // Don't bother if send buffer is too full to respond anyway
        if (*pfrom).send_paused() {
            return false;
        }

        let mut msgs: Vec<NetMessage> = vec![];

        {
            let mut guard = (*pfrom).lock_v_process_msg();

            if guard.process_msg.is_empty() {
                return false;
            }

            //  Just take one message
            msgs.splice(0..0, guard.process_msg.drain(0..=0));

            let raw_msg_size: usize 
            = msgs.first().unwrap()
                .raw_message_size
                .try_into().unwrap();

            (*pfrom).decrement_n_process_queue_size(
                raw_msg_size
            );

            let receive_flood_size: usize 
            = self.connman.get()
                .get_receive_flood_size()
                .try_into().unwrap();

            let process_queue_size: usize 
            = (*pfrom).get_n_process_queue_size();

            (*pfrom).set_pause_recv(
                process_queue_size > receive_flood_size
            );

            more_work = !guard.process_msg.is_empty();
        }

        let msg: &mut NetMessage = msgs.first_mut().unwrap();

        trace6!(
            net, 
            inbound_message, 
            (*pfrom).get_id(), 
            (*pfrom).addr_name.str_(), 
            (*pfrom).connection_type_as_string().str_(), 
            msg.command.str_(), 
            msg.recv.size(), 
            msg.recv.data()
        );

        if G_ARGS.lock().get_bool_arg("-capturemessages", false) {
            capture_message(
                (*pfrom).addr(), 
                &msg.command, 
                msg.recv.as_slice(), 
                /* incoming */ true
            );
        }

        msg.set_version((*pfrom).get_common_version());

        let msg_type: &str = &msg.command;

        //  Message size
        let n_message_size: u32 = msg.message_size;

        let mut try_block = || -> TryBlockResult::<_,StdException> {

            self.process_message(
                pfrom, 
                &msg_type.to_string(), 
                &mut msg.recv, 
                msg.time.as_ref().unwrap(), 
                interrupt_msg_proc
            );

            if interrupt_msg_proc.load(atomic::Ordering::Relaxed) {
                return TryBlockResult::Return(false);
            }

            {
                let peer = peer.get();

                let mut guard = peer.getdata_requests.lock();

                if !guard.is_empty() {
                    more_work = true;
                }
            }

            TryBlockResult::Success
        };

        match try_block() {
            TryBlockResult::Return(v)  => return v,
            TryBlockResult::Err(e)  => {

                match e {
                    StdException::Default { what:e }  => {

                        log_print!(
                            LogFlags::NET, 
                            "%s(%s, %u bytes): Exception '%s' (%s) caught\n", 
                            func, 
                            sanitize_string(msg_type), 
                            n_message_size, 
                            e.what(), 
                            e.type_id().name()
                        );
                    }

                    _  => {
                        log_print!(
                            LogFlags::NET, 
                            "%s(%s, %u bytes): Unknown exception caught\n", 
                            func, 
                            sanitize_string(msg_type), 
                            n_message_size
                        );
                    }
                }
            },

            TryBlockResult::Success  => { }
            TryBlockResult::Break  => { }
        }

        more_work
    }
}
