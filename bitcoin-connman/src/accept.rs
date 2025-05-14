// ---------------- [ File: bitcoin-connman/src/accept.rs ]
crate::ix!();

impl Connman {

    pub fn accept_connection(&self, h_listen_socket: &ConnmanListenSocket)  {

        let mut sockaddr: SockAddrStorage = unsafe { std::mem::zeroed() };

        let mut len: libc::socklen_t 
        = size_of_val(&sockaddr).try_into().unwrap();

        let mut h_socket: CSocket = unsafe { 
            libc::accept(
                h_listen_socket.socket,
                &mut sockaddr as *mut SockAddrStorage as *mut _ as *mut libc::sockaddr,
                &mut len as *mut u32
            )
        };

        let mut addr = Address::default();

        if h_socket == INVALID_SOCKET {

            let n_err: i32 = *WSA_GET_LAST_ERROR;

            if n_err != WSAEWOULDBLOCK {
                log_printf!(
                    "socket error accept failed: {}\n", 
                    network_error_string(n_err)
                );
            }

            return;
        }

        if !addr.service.set_sock_addr(&sockaddr as *const _ as *const SocketAddr) {
            log_printf!("Warning: Unknown socket family\n");
        }

        let addr_bind: Address = get_bind_address(h_socket);
        let mut permission_flags: NetPermissionFlags = NetPermissionFlags::None;

        h_listen_socket.add_socket_permission_flags(&mut permission_flags);

        self.create_node_from_accepted_socket(
            &mut h_socket, 
            permission_flags, 
            &addr_bind, 
            &addr
        );
    }
}
