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

    /// Destructor, close the socket or do nothing if empty.
    fn drop(&mut self) {
        trace!("Sock::drop");
        self.reset();
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

    /// Create an empty object that does nothing when destroyed.
    ///
    /// Default constructor, creates an empty object that does nothing when destroyed.
    fn default() -> Self {
        trace!("Sock::default – constructing empty Sock");
        Self {
            socket: INVALID_SOCKET,
        }
    }
}

impl From<CSocket> for Sock {
    fn from(s: CSocket) -> Self {
        trace!(socket = s, "Sock::from – taking ownership");
        Self { socket: s }
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
        trace!(socket = self.socket, "Sock::get");
        self.socket
    }
    
    /// relinquish ownership to caller.
    pub fn release(&mut self) -> CSocket {
        let s = self.socket;
        self.socket = INVALID_SOCKET;
        trace!(released = s, "Sock::release");
        s
    }
    
    /// close if non‑empty.
    pub fn reset(&mut self) {
        if self.socket != INVALID_SOCKET {
            debug!(socket = self.socket, "Sock::reset – closing");
            close_socket(self.socket);
            self.socket = INVALID_SOCKET;
        } else {
            trace!("Sock::reset – already empty");
        }
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
}
