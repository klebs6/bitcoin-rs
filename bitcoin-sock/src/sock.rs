// ---------------- [ File: bitcoin-sock/src/sock.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/sock.h]

/**
  | Maximum time to wait for I/O readiness.
  | 
  | It will take up until this time to break
  | off in case of an interruption.
  |
  */
pub const MAX_WAIT_FOR_IO: Duration = Duration::seconds(1);

/**
  | RAII helper class that manages a socket.
  | Mimics `std::unique_ptr`, but instead
  | of a pointer it contains a socket and
  | closes it automatically when it goes
  | out of scope.
  |
  */
pub struct Sock {

    /**
      | Contained socket. `INVALID_SOCKET`
      | designates the object is empty.
      |
      */
    socket: CSocket,
}

impl Drop for Sock {

    /**
      | Destructor, close the socket or do nothing
      | if empty.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            Reset();
        */
    }
}


pub type SockEvent = u8;

/**
  | If passed to `Wait()`, then it will wait
  | for readiness to read from the socket.
  |
  */
pub const SOCK_RECV: usize = 0b01;

/**
  | If passed to `Wait()`, then it will wait
  | for readiness to send to the socket.
  |
  */
pub const SOCK_SEND: usize = 0b10;

//-------------------------------------------[.cpp/bitcoin/src/util/sock.cpp]

impl Default for Sock {

    /**
      | Default constructor, creates an empty
      | object that does nothing when destroyed.
      |
      */
    fn default() -> Self {
    
        todo!();
        /*
        : socket(INVALID_SOCKET),

        
        */
    }
}

impl From<CSocket> for Sock {
    
    fn from(s: CSocket) -> Self {
    
        todo!();
        /*
        : socket(s),

        
        */
    }
}

impl Sock {
    
    /**
      | Move assignment operator, grab the
      | socket from another object and close
      | ours (if set).
      |
      */
    pub fn assign_from(&mut self, other: Sock) -> &mut Sock {
        
        todo!();
        /*
            Reset();
        m_socket = other.m_socket;
        other.m_socket = INVALID_SOCKET;
        return *this;
        */
    }
    
    pub fn get(&self) -> CSocket {
        
        todo!();
        /*
            return m_socket;
        */
    }
    
    pub fn release(&mut self) -> CSocket {
        
        todo!();
        /*
            const Socket s = m_socket;
        m_socket = INVALID_SOCKET;
        return s;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            CloseSocket(m_socket);
        */
    }
    
    pub fn send(&self, 
        data:  *const c_void,
        len:   usize,
        flags: i32) -> isize {
        
        todo!();
        /*
            return send(m_socket, static_cast<const char*>(data), len, flags);
        */
    }
    
    pub fn recv(&self, 
        buf:   *mut c_void,
        len:   usize,
        flags: i32) -> isize {
        
        todo!();
        /*
            return recv(m_socket, static_cast<char*>(buf), len, flags);
        */
    }
    
    pub fn connect(&self, 
        addr:     *const SocketAddr,
        addr_len: libc::socklen_t) -> i32 {
        
        todo!();
        /*
            return connect(m_socket, addr, addr_len);
        */
    }
    
    pub fn get_sock_opt(&self, 
        level:    i32,
        opt_name: i32,
        opt_val:  *mut c_void,
        opt_len:  *mut libc::socklen_t) -> i32 {
        
        todo!();
        /*
            return getsockopt(m_socket, level, opt_name, static_cast<char*>(opt_val), opt_len);
        */
    }
    
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
    
    pub fn send_complete(&self, 
        data:      &String,
        timeout:   Instant /* millis */,
        interrupt: &mut ThreadInterrupt)  {
        
        todo!();
        /*
            const auto deadline = GetTime<milliseconds>() + timeout;
        size_t sent{0};

        for (;;) {
            const ssize_t ret{Send(data.data() + sent, data.size() - sent, MSG_NOSIGNAL)};

            if (ret > 0) {
                sent += static_cast<size_t>(ret);
                if (sent == data.size()) {
                    break;
                }
            } else {
                const int err{WSAGetLastError()};
                if (IOErrorIsPermanent(err)) {
                    throw std::runtime_error(strprintf("send(): %s", NetworkErrorString(err)));
                }
            }

            const auto now = GetTime<milliseconds>();

            if (now >= deadline) {
                throw std::runtime_error(strprintf(
                    "Send timeout (sent only %u of %u bytes before that)", sent, data.size()));
            }

            if (interrupt) {
                throw std::runtime_error(strprintf(
                    "Send interrupted (sent only %u of %u bytes before that)", sent, data.size()));
            }

            // Wait for a short while (or the socket to become ready for sending) before retrying
            // if nothing was sent.
            const auto wait_time = std::min(deadline - now, milliseconds{MAX_WAIT_FOR_IO});
            (c_void)Wait(wait_time, SEND);
        }
        */
    }
    
    pub fn recv_until_terminator(&self, 
        terminator: u8,
        timeout:    Instant /* millis */,
        interrupt:  &mut ThreadInterrupt,
        max_data:   usize) -> String {
        
        todo!();
        /*
            const auto deadline = GetTime<milliseconds>() + timeout;
        std::string data;
        bool terminator_found{false};

        // We must not consume any bytes past the terminator from the socket.
        // One option is to read one byte at a time and check if we have read a terminator.
        // However that is very slow. Instead, we peek at what is in the socket and only read
        // as many bytes as possible without crossing the terminator.
        // Reading 64 MiB of random data with 262526 terminator chars takes 37 seconds to read
        // one byte at a time VS 0.71 seconds with the "peek" solution below. Reading one byte
        // at a time is about 50 times slower.

        for (;;) {
            if (data.size() >= max_data) {
                throw std::runtime_error(
                    strprintf("Received too many bytes without a terminator (%u)", data.size()));
            }

            char buf[512];

            const ssize_t peek_ret{Recv(buf, std::min(sizeof(buf), max_data - data.size()), MSG_PEEK)};

            switch (peek_ret) {
            case -1: {
                const int err{WSAGetLastError()};
                if (IOErrorIsPermanent(err)) {
                    throw std::runtime_error(strprintf("recv(): %s", NetworkErrorString(err)));
                }
                break;
            }
            case 0:
                throw std::runtime_error("Connection unexpectedly closed by peer");
            default:
                auto end = buf + peek_ret;
                auto terminator_pos = std::find(buf, end, terminator);
                terminator_found = terminator_pos != end;

                const size_t try_len{terminator_found ? terminator_pos - buf + 1 :
                                                        static_cast<size_t>(peek_ret)};

                const ssize_t read_ret{Recv(buf, try_len, 0)};

                if (read_ret < 0 || static_cast<size_t>(read_ret) != try_len) {
                    throw std::runtime_error(
                        strprintf("recv() returned %u bytes on attempt to read %u bytes but previous "
                                  "peek claimed %u bytes are available",
                                  read_ret, try_len, peek_ret));
                }

                // Don't include the terminator in the output.
                const size_t append_len{terminator_found ? try_len - 1 : try_len};

                data.append(buf, buf + append_len);

                if (terminator_found) {
                    return data;
                }
            }

            const auto now = GetTime<milliseconds>();

            if (now >= deadline) {
                throw std::runtime_error(strprintf(
                    "Receive timeout (received %u bytes without terminator before that)", data.size()));
            }

            if (interrupt) {
                throw std::runtime_error(strprintf(
                    "Receive interrupted (received %u bytes without terminator before that)",
                    data.size()));
            }

            // Wait for a short while (or the socket to become ready for reading) before retrying.
            const auto wait_time = std::min(deadline - now, milliseconds{MAX_WAIT_FOR_IO});
            (c_void)Wait(wait_time, RECV);
        }
        */
    }
    
    pub fn is_connected(&self, errmsg: &mut String) -> bool {
        
        todo!();
        /*
            if (m_socket == INVALID_SOCKET) {
            errmsg = "not connected";
            return false;
        }

        char c;
        switch (Recv(&c, sizeof(c), MSG_PEEK)) {
        case -1: {
            const int err = WSAGetLastError();
            if (IOErrorIsPermanent(err)) {
                errmsg = NetworkErrorString(err);
                return false;
            }
            return true;
        }
        case 0:
            errmsg = "closed";
            return false;
        default:
            return true;
        }
        */
    }
}
