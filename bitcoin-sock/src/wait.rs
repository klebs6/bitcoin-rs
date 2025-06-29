// ---------------- [ File: bitcoin-sock/src/wait.rs ]
crate::ix!();

impl Sock {
    
    pub fn wait(&self, 
        timeout:   Instant /* millis */,
        requested: SockEvent,
        occurred:  *mut SockEvent) -> bool {
        
        todo!();
        /*
            #ifdef USE_POLL
        pollfd fd;
        fd.fd = m_socket;
        fd.events = 0;
        if (requested & RECV) {
            fd.events |= POLLIN;
        }
        if (requested & SEND) {
            fd.events |= POLLOUT;
        }

        if (poll(&fd, 1, count_milliseconds(timeout)) == SOCKET_ERROR) {
            return false;
        }

        if (occurred != nullptr) {
            *occurred = 0;
            if (fd.revents & POLLIN) {
                *occurred |= RECV;
            }
            if (fd.revents & POLLOUT) {
                *occurred |= SEND;
            }
        }

        return true;
    #else
        if (!IsSelectableSocket(m_socket)) {
            return false;
        }

        fd_set fdset_recv;
        fd_set fdset_send;
        FD_ZERO(&fdset_recv);
        FD_ZERO(&fdset_send);

        if (requested & RECV) {
            FD_SET(m_socket, &fdset_recv);
        }

        if (requested & SEND) {
            FD_SET(m_socket, &fdset_send);
        }

        timeval timeout_struct = MillisToTimeval(timeout);

        if (select(m_socket + 1, &fdset_recv, &fdset_send, nullptr, &timeout_struct) == SOCKET_ERROR) {
            return false;
        }

        if (occurred != nullptr) {
            *occurred = 0;
            if (FD_ISSET(m_socket, &fdset_recv)) {
                *occurred |= RECV;
            }
            if (FD_ISSET(m_socket, &fdset_send)) {
                *occurred |= SEND;
            }
        }

        return true;
    #endif /* USE_POLL */
        */
    }
}
