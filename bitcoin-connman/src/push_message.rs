// ---------------- [ File: bitcoin-connman/src/push_message.rs ]
crate::ix!();

impl Connman {

    pub fn push_message(&mut self, 
        mut node: &mut Box<dyn NodeInterface>,
        mut msg:  SerializedNetMsg) 
    {
        let n_message_size: usize = msg.data.len();

        log_print!(
            bc_log::NET, 
            "sending {} ({} bytes) peer={}\n", 
            msg.ty, 
            n_message_size, 
            node.get_id()
        );

        if G_ARGS.lock().get_bool_arg("-capturemessages", false) {
            capture_message(
                node.addr(), 
                &msg.ty, 
                &msg.data, 
                /* incoming */ false
            );
        }

        trace6!(
            net, 
            outbound_message, 
            node.get_id(), 
            node.addr_name, 
            node.connection_type_as_string(), 
            msg.ty, 
            msg.data.len(), 
            msg.data.data()
        );

        // make sure we use the appropriate
        // network transport format
        let mut serialized_header = Vec::<u8>::default();;

        node.get_transport_serializer_mut()
            .prepare_for_transport(&mut msg, &mut serialized_header);

        let n_total_size: usize = n_message_size + serialized_header.len();

        let mut n_bytes_sent: usize = 0;

        {
            let optimistic_send: bool = {

                let mut guard = node.lock_v_send();

                let optimistic_send = guard.send_msg.is_empty();

                // log total amount of bytes per message type
                *guard.map_send_bytes_per_msg_cmd.entry(msg.ty.clone()).or_insert(0) 
                += u64::try_from(n_total_size).unwrap();

                guard.n_send_size += n_total_size;

                let n_send_buffer_max_size = self.n_send_buffer_max_size.load(atomic::Ordering::Relaxed);

                if guard.n_send_size > n_send_buffer_max_size.try_into().unwrap() {
                    node.set_pause_send(true);
                }

                guard.send_msg.push_back(serialized_header); // want move

                if n_message_size != 0 {
                    guard.send_msg.push_back(msg.data); //want move
                }

                optimistic_send
            };

            // If write queue empty, attempt "optimistic write"
            if optimistic_send {
                n_bytes_sent = self.socket_send_data(&node);
            }
        }

        if n_bytes_sent != 0 {
            self.record_bytes_sent(n_bytes_sent.try_into().unwrap());
        }
    }
}
