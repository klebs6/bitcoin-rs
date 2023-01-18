crate::ix!();

impl Connman {

    pub fn generate_select_set(&self, 
        recv_set:  &mut HashSet<CSocket>,
        send_set:  &mut HashSet<CSocket>,
        error_set: &mut HashSet<CSocket>) -> bool {

        for h_listen_socket in self.vh_listen_socket.get().iter() {
            recv_set.insert(
                h_listen_socket.socket.try_into().unwrap()
            );
        }

        {
            let mut guard = self.cs_v_nodes.get();

            for pnode in guard.nodes.iter() {

                let node = pnode.get();

                unsafe {

                    //  Implement the following logic:
                    //
                    //  * If there is data to send,
                    //    select() for sending
                    //    data. As this only happens
                    //    when optimistic write
                    //    failed, we choose to first
                    //    drain the write buffer in
                    //    this case before receiving
                    //    more. This avoids needlessly
                    //    queueing received data, if
                    //    the remote peer is not
                    //    themselves receiving
                    //    data. This means properly
                    //    utilizing TCP flow control
                    //    signalling.
                    //
                    //  * Otherwise, if there is space
                    //    left in the receive buffer,
                    //    select() for receiving data.
                    //
                    //  * Hand off all complete
                    //    messages to the processor,
                    //    to be handled without
                    //    blocking here.
                    let select_recv: bool = !node.pause_recv();

                    let mut select_send = bool::default();

                    {
                        let mut guard = node.lock_v_send();

                        select_send = !guard.send_msg.is_empty();
                    }

                    {
                        let mut guard = node.lock_h_socket();

                        if guard.h_socket == INVALID_SOCKET {
                            continue;
                        }

                        error_set.insert(
                            guard.h_socket.try_into().unwrap()
                        );

                        if select_send {
                            send_set.insert(guard.h_socket.try_into().unwrap());
                            continue;
                        }

                        if select_recv {
                            recv_set.insert(guard.h_socket.try_into().unwrap());
                        }
                    }
                }
            }
        }

        !recv_set.is_empty() 
        || !send_set.is_empty() 
        || !error_set.is_empty()
    }
}
