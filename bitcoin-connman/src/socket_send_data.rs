crate::ix!();

impl Connman {

    #[EXCLUSIVE_LOCKS_REQUIRED(node.cs_vSend)]
    pub fn socket_send_data(&self, node: &Box<dyn NodeInterface>) -> usize {
        todo!();
        
        /*
        let it = node.send_msg.begin();

        let n_sent_size: usize = 0;

        while it != node.send_msg.end(){

            let data = *it;

            assert!(data.size() > node.n_send_offset);

            let n_bytes: i32 = 0;

            {
                let guard = node.cs_h_socket.lock().unwrap();

                if node.h_socket == INVALID_SOCKET {
                    break;
                }

                n_bytes = send(
                    node.h_socket,
                    data.data() as *const i8 + node.n_send_offset,
                    data.size() - node.n_send_offset,
                    MSG_NOSIGNAL | MSG_DONTWAIT
                );
            }

            if n_bytes > 0 {
                node.n_last_send = get_time_seconds();
                node.n_send_bytes += n_bytes;
                node.n_send_offset += n_bytes;
                n_sent_size += n_bytes;

                if node.n_send_offset == data.size() {
                    node.n_send_offset = 0;
                    node.n_send_size -= data.size();
                    node.pause_send.store(
                        node.n_send_size > self.n_send_buffer_max_size,
                        atomic::Ordering::Relaxed,
                    );

                    it += 1;

                } else {
                    // could not send full
                    // message; stop sending more
                    break;
                }

            } else {

                if n_bytes < 0 {

                    // error
                    let n_err: i32 = wsa_get_last_error();;

                    if n_err != WSAEWOULDBLOCK 
                    && n_err != WSAEMSGSIZE 
                    && n_err != WSAEINTR 
                    && n_err != WSAEINPROGRESS {

                        log_print!(
                            bc_log::NET, 
                            "socket send error for peer={}: {}\n", 
                            node.get_id(), 
                            network_error_string(n_err)
                        );

                        node.close_socket_disconnect();
                    }
                }

                // couldn't send anything at all
                break;
            }
        }

        if it == None {
            assert!(node.n_send_offset == 0);
            assert!(node.n_send_size == 0);
        }

        node.send_msg.erase(node.send_msg.begin(), it);

        n_sent_size
        */
    }
}
