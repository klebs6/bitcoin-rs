// ---------------- [ File: bitcoin-connman/src/socket_handler.rs ]
crate::ix!();

pub fn subroutine_thread_socket_handler(connman: Arc<Connman>)  {
    
    set_syscall_sandbox_policy(SyscallSandboxPolicy::NET);

    while !connman.interrupt_net.get().as_bool() {
        connman.disconnect_nodes();
        connman.notify_num_connections_changed();
        connman.socket_handler();
    }
}

impl Connman {

    pub fn socket_handler(&self)  {
        
        let mut recv_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut send_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut error_set: HashSet::<CSocket> = HashSet::<CSocket>::default();

        self.socket_events(&mut recv_set, &mut send_set, &mut error_set);

        if self.interrupt_net.get().as_bool() {
            return;
        }

        //  Accept new connections
        for h_listen_socket in self.vh_listen_socket.get().iter() {

            if h_listen_socket.socket != INVALID_SOCKET 
            && recv_set.contains(&h_listen_socket.socket)
            {
                self.accept_connection(h_listen_socket);
            }
        }

        //  Service each socket
        let mut nodes_copy: Vec<Amo<Box<dyn NodeInterface>>> = vec![];

        {
            let mut guard = self.cs_v_nodes.get();

            nodes_copy = guard.nodes.clone();

            for pnode in nodes_copy.iter() {

                pnode.get_mut().add_ref();
            }
        }

        for pnode in nodes_copy.iter() {

            let mut node = pnode.get_mut();

            if self.interrupt_net.get().as_bool() {
                return;
            }

            //  Receive
            let mut in_recv_set:  bool = false;;
            let mut in_send_set:  bool = false;
            let mut in_error_set: bool = false;

            {
                let mut guard = node.lock_h_socket();

                if guard.h_socket == INVALID_SOCKET {
                    continue;
                }

                in_recv_set  = recv_set.contains(&guard.h_socket);
                in_send_set  = send_set.contains(&guard.h_socket);
                in_error_set = error_set.contains(&guard.h_socket);
            }

            if in_recv_set || in_error_set {

                // typical socket buffer is 8K-64K
                let mut pch_buf: [u8; 0x10000] = [0; 0x10000];

                let mut n_bytes: i32 = 0;

                {
                    let mut guard = node.lock_h_socket();

                    if guard.h_socket == INVALID_SOCKET {
                        continue;
                    }

                    n_bytes = unsafe {
                        libc::recv(
                            guard.h_socket,
                            pch_buf.as_mut_ptr() as *mut i8 as *mut c_void,
                            size_of_val(&pch_buf),
                            MSG_DONTWAIT.try_into().unwrap()
                        ).try_into().unwrap()
                    };
                }

                if n_bytes > 0 {

                    let mut notify: bool = false;

                    let bytes: &[u8] = unsafe { 
                        std::slice::from_raw_parts(
                            pch_buf.as_ptr(), 
                            n_bytes.try_into().unwrap()
                        )
                    };

                    if !node.receive_msg_bytes(bytes, &mut notify) {
                        node.close_socket_disconnect();
                    }

                    self.record_bytes_recv(n_bytes.try_into().unwrap());

                    if notify {

                        let mut n_size_added: u32 = 0;

                        let mut recv_msg = node.lock_recv_msg();

                        let mut it = recv_msg.iter().enumerate().peekable();

                        while let Some((idx,x)) = it.next() {

                            // vRecvMsg contains
                            // only completed
                            // CNetMessage
                            //
                            // the single possible
                            // partially
                            // deserialized
                            // message are held by
                            // TransportDeserializer
                            n_size_added += x.raw_message_size;
                        }

                        {
                            let mut guard = node.lock_v_process_msg();

                            let idx = it.peek().unwrap().0;

                            let len = guard.process_msg.len();

                            guard.process_msg.splice(len.., recv_msg.drain(0..idx));

                            node.increment_n_process_queue_size(
                                usize::try_from(n_size_added).unwrap()
                            );

                            let n_receive_flood_size = self.n_receive_flood_size();
                            let n_process_queue_size = node.n_process_queue_size();

                            node.set_pause_recv(
                                n_process_queue_size > n_receive_flood_size.try_into().unwrap()
                            );
                        }

                        self.wake_message_handler();
                    }

                } else {

                    if n_bytes == 0 {

                        // socket closed gracefully
                        if !node.marked_for_disconnect() {

                            log_print!(
                                bc_log::NET, 
                                "socket closed for peer={}\n", 
                                node.get_id()
                            );
                        }

                        node.close_socket_disconnect();

                    } else {

                        if n_bytes < 0 {

                            // error
                            let n_err: i32 = *WSA_GET_LAST_ERROR;

                            if n_err != WSAEWOULDBLOCK 
                            && n_err != WSAEMSGSIZE 
                            && n_err != WSAEINTR 
                            && n_err != WSAEINPROGRESS {

                                if !node.marked_for_disconnect() {
                                    log_print!(
                                        bc_log::NET, 
                                        "socket recv error for peer={}: {}\n", 
                                        node.get_id(), 
                                        network_error_string(n_err)
                                    );
                                }

                               node.close_socket_disconnect();
                            }
                        }
                    }
                }
            }

            if in_send_set {

                // Send data
                let bytes_sent: u64 = {

                    let guard = node.lock_v_send();

                    self.socket_send_data(&node).try_into().unwrap()
                };

                if bytes_sent != 0 {
                    self.record_bytes_sent(bytes_sent);
                }
            }

            if self.inactivity_check(&node) {
                node.mark_for_disconnect();
            }
        }

        {
            let mut guard = self.cs_v_nodes.get();

            for pnode in nodes_copy.iter() {
                pnode.get_mut().release();
            }
        }
    }
}
