crate::ix!();

impl Connman {

    #[cfg(not(USE_POLL))]
    pub fn socket_events(&self, 
        recv_set:  &mut HashSet<CSocket>,
        send_set:  &mut HashSet<CSocket>,
        error_set: &mut HashSet<CSocket>)  {

        let mut recv_select_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut send_select_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut error_select_set: HashSet::<CSocket> = HashSet::<CSocket>::default();

        if !self.generate_select_set(
            &mut recv_select_set,
            &mut send_select_set,
            &mut error_select_set) 
        {
            self.interrupt_net.get_mut().sleep_for(
                Duration::milliseconds(SELECT_TIMEOUT_MILLISECONDS)
            );

            return;
        }

        // Find which sockets have data to
        // receive
        //
        let mut timeout = TimeVal::microseconds(
            //  frequency to poll pnode->vSend
            SELECT_TIMEOUT_MILLISECONDS * 1000
        );

        let mut fdset_recv  = FdSet::new();
        let mut fdset_send  = FdSet::new();
        let mut fdset_error = FdSet::new();

        let mut h_socket_max: CSocket = 0;

        for &h_socket in recv_select_set.iter() {
            fdset_recv.insert(h_socket);
            h_socket_max = max(h_socket_max,h_socket);
        }

        for &h_socket in send_select_set.iter() {
            fdset_send.insert(h_socket);
            h_socket_max = max(h_socket_max,h_socket);
        }

        for &h_socket in error_select_set.iter() {
            fdset_error.insert(h_socket);
            h_socket_max = max(h_socket_max,h_socket);
        }

        let n_select: i32 = select(
            h_socket_max + 1,
            Some(&mut fdset_recv),
            Some(&mut fdset_send),
            Some(&mut fdset_error),
            &mut timeout
        ).unwrap();

        if self.interrupt_net.get().as_bool() {
            return;
        }

        if n_select == SOCKET_ERROR {

            let n_err: i32 = *WSA_GET_LAST_ERROR;

            log_printf!(
                "socket select error {}\n", 
                network_error_string(n_err)
            );

            for i in 0..=h_socket_max {
                fdset_recv.insert(i);
            }

            fdset_send.clear();

            fdset_error.clear();

            if !self.interrupt_net.get_mut().sleep_for(
                Duration::milliseconds(SELECT_TIMEOUT_MILLISECONDS.try_into().unwrap())
            ) {
                return;
            }
        }

        for &h_socket in recv_select_set.iter() {
            if fdset_recv.contains(h_socket) {
                recv_set.insert(h_socket);
            }
        }

        for &h_socket in send_select_set.iter() {
            if fdset_send.contains(h_socket) {
                send_set.insert(h_socket);
            }
        }

        for &h_socket in error_select_set.iter() {
            if fdset_error.contains(h_socket) {
                error_set.insert(h_socket);
            }
        }
    }

    #[cfg(USE_POLL)]
    pub fn socket_events(&self, 
        recv_set:  &mut HashSet<CSocket>,
        send_set:  &mut HashSet<CSocket>,
        error_set: &mut HashSet<CSocket>)  {

        let mut recv_select_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut send_select_set:  HashSet::<CSocket> = HashSet::<CSocket>::default();
        let mut error_select_set: HashSet::<CSocket> = HashSet::<CSocket>::default();

        if !generate_select_set(recv_select_set,send_select_set,error_select_set) {
            interrupt_net.sleep_for_millis(Milliseconds::new(SELECT_TIMEOUT_MILLISECONDS));
            return;
        }

        let mut pollfds = HashMap::<Socket,libc::pollfd>::default();

        for socket_id in recv_select_set.iter() {
            pollfds.fd[socket_id] = socket_id;
            pollfds.events[socket_id] |= POLLIN;
        }

        for socket_id in send_select_set.iter() {
            pollfds.fd[socket_id] = socket_id;
            pollfds.events[socket_id] |= POLLOUT;
        }

        for socket_id in error_select_set.iter() {
            pollfds.fd[socket_id] = socket_id;
            //  These flags are ignored, but we set them for clarity
            pollfds.events[socket_id] |= POLLERR | POLLHUP;
        }

        let mut vpollfds = Vec::<libc::pollfd>::default();

        vpollfds.reserve(pollfds.size());

        for it in pollfds.iter() {
            vpollfds.push_back(it.1); // want move
        }

        if poll(vpollfds.data(),vpollfds.size(),SELECT_TIMEOUT_MILLISECONDS) < 0 {
            return;
        }

        if interrupt_net {
            return;
        }

        for pollfd_entry in vpollfds.iter() {

            if pollfd_entry.revents & POLLIN {
                recv_set.insert(pollfd_entry.fd);
            }

            if pollfd_entry.revents & POLLOUT {
                send_set.insert(pollfd_entry.fd);
            }

            if pollfd_entry.revents & (POLLERR | POLLHUP) {
                error_set.insert(pollfd_entry.fd);
            }
        }
    }
}
