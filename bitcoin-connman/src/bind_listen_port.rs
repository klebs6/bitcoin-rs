// ---------------- [ File: bitcoin-connman/src/bind_listen_port.rs ]
crate::ix!();

impl Connman {
       
    pub fn bind_listen_port(&self, 
        addr_bind:   &Service,
        str_error:   &mut BilingualStr,
        permissions: NetPermissionFlags) -> bool {

        let n_one: i32 = 1;

        // Create socket for listening for incoming connections
        let mut sockaddr: SockAddrStorage = unsafe { std::mem::zeroed() };

        let mut len: libc::socklen_t 
        = size_of_val(&sockaddr).try_into().unwrap();

        if !addr_bind.get_sock_addr(&mut sockaddr as *mut _ as *mut SocketAddr, &mut len) {

            *str_error = untranslated(
                &format!{
                    "Error: Bind address family for {} not supported",
                    addr_bind.to_string()
                }
            );

            log_printf!("{}\n", str_error.original);

            return false;
        }

        let sock: Option::<Box<Sock>> = create_socktcp(addr_bind);

        if sock.is_none() {

            *str_error = untranslated(
                &format!(
                    "Error: Couldn't open socket for incoming connections (socket returned error {})",
                    network_error_string(*WSA_GET_LAST_ERROR)
                )
            );

            log_printf!("{}\n", str_error.original);

            return false;
        }

        let mut sock: Box<Sock> = sock.unwrap();

        // Allow binding if the port is still in
        // TIME_WAIT state after the program was
        // closed and restarted.
        unsafe {
            libc::setsockopt(
                sock.get(),
                libc::SOL_SOCKET,
                libc::SO_REUSEADDR,
                &n_one as *const _ as *const c_void,
                size_of::<i32>().try_into().unwrap()
            );
        }

        // some systems don't have IPV6_V6ONLY but
        // are always v6only; others do have the
        // option and enable it by default or
        // not. Try to enable it, if possible.
        if addr_bind.base.is_ipv6() {

            #[cfg(IPV6_V6ONLY)]
            {
                libc::setsockopt(
                    sock.get(), 
                    libc::IPPROTO_IPV6, 
                    libc::IPV6_V6ONLY, 
                    &n_one as SockoptArgType, 
                    size_of::<i32>()
                );
            }

            #[cfg(WIN32)]
            {
                let n_prot_level: i32 = PROTECTION_LEVEL_UNRESTRICTED;

                libc::setsockopt(
                    sock.get(), 
                    libc::IPPROTO_IPV6, 
                    libc::IPV6_PROTECTION_LEVEL, 
                    &n_prot_level as *const i8, 
                    size_of::<i32>()
                );
            }
        }

        if unsafe { 
            libc::bind(
                sock.get(),
                &mut sockaddr as *mut _ as *mut SockAddr,
                len
            ) 
        } == SOCKET_ERROR
        {
            let n_err: i32 = *WSA_GET_LAST_ERROR;

            if n_err == WSAEADDRINUSE {

                *str_error = untranslated(
                    &format!(
                        "Unable to bind to {} on this computer. {} is probably already running.",
                        addr_bind.to_string(),
                        PACKAGE_NAME
                    )
                );

            } else {

                *str_error = untranslated(
                    &format!(
                        "Unable to bind to {} on this computer (bind returned error {})",
                        addr_bind.to_string(),
                        network_error_string(n_err)
                    )
                );
            }

            log_printf!("{}\n", str_error.original);

            return false;
        }

        log_printf!("Bound to {}\n", addr_bind.to_string());

        // Listen for incoming connections
        if unsafe { libc::listen(sock.get(), libc::SOMAXCONN) } == SOCKET_ERROR {

            *str_error = BilingualStr::from(
                format!(
                    "Error: Listening for incoming connections failed (listen returned error {})",
                    network_error_string(*WSA_GET_LAST_ERROR)
                ).as_str()
            );

            log_printf!("{}\n", str_error.original);

            return false;
        }

        self.vh_listen_socket.get_mut().push(
            ConnmanListenSocket::new(
                sock.release(),
                permissions
            )
        );

        true
    }
}
