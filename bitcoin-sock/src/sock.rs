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
#[derive(Debug,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
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
      | Move‑assignment analogue: grab the socket from `other`
      | and close ours (if set), **exactly** mirroring C++.
      */
    pub fn assign_from(&mut self, mut other: Sock) -> &mut Self {
        use std::mem::{forget, ManuallyDrop};

        self.reset();

        // Preserve C++ semantics: steal the descriptor and leave
        // `other` inert (so that its Drop closes nothing).
        let mut other = ManuallyDrop::new(other);
        self.socket = other.socket;
        other.socket = INVALID_SOCKET;

        trace!(
            new_socket = self.socket,
            "Sock::assign_from – took ownership from peer"
        );
        self
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
    
    /// Close if non‑empty.
    pub fn reset(&mut self) {
        if self.socket != INVALID_SOCKET {
            debug!(socket = self.socket, "Sock::reset – closing");
            close_socket(&mut self.socket); // **fixed: pass `&mut`**
            // `close_socket` already sets the descriptor to `INVALID_SOCKET`.
        } else {
            trace!("Sock::reset – already empty");
        }
    }
    
    /// `send(2)`/`send()` thin wrapper.
    pub fn send(&self, data: *const c_void, len: usize, flags: i32) -> isize {
        #[cfg(target_os = "windows")]
        let ret = unsafe {
            winapi::um::winsock2::send(
                self.socket as winapi::um::winsock2::SOCKET,
                data as *const i8,
                len as i32,
                flags,
            )
        };

        #[cfg(not(target_os = "windows"))]
        let ret = unsafe { libc::send(self.socket, data, len, flags) };

        trace!(ret, bytes = len, "Sock::send");
        ret as isize
    }

    /// `recv(2)`/`recv()` thin wrapper.
    pub fn recv(&self, buf: *mut c_void, len: usize, flags: i32) -> isize {
        #[cfg(target_os = "windows")]
        let ret = unsafe {
            winapi::um::winsock2::recv(
                self.socket as winapi::um::winsock2::SOCKET,
                buf as *mut i8,
                len as i32,
                flags,
            )
        };

        #[cfg(not(target_os = "windows"))]
        let ret = unsafe { libc::recv(self.socket, buf, len, flags) };

        trace!(ret, bytes = len, "Sock::recv");
        ret as isize
    }
   
    /// `connect(2)`/`connect()` thin wrapper.
    pub fn connect(&self, addr: *const SocketAddr, addr_len: libc::socklen_t) -> i32 {
        #[cfg(target_os = "windows")]
        let ret = unsafe {
            winapi::um::winsock2::connect(
                self.socket as winapi::um::winsock2::SOCKET,
                addr as *const _,
                addr_len,
            )
        };

        #[cfg(not(target_os = "windows"))]
        let ret = unsafe { libc::connect(self.socket, addr as *const _, addr_len) };

        trace!(ret, "Sock::connect");
        ret
    }

    /// `getsockopt(2)`/`getsockopt()` thin wrapper.
    pub fn get_sock_opt(
        &self,
        level: i32,
        opt_name: i32,
        opt_val: *mut c_void,
        opt_len: *mut libc::socklen_t,
    ) -> i32 {
        #[cfg(target_os = "windows")]
        let ret = unsafe {
            winapi::um::winsock2::getsockopt(
                self.socket as winapi::um::winsock2::SOCKET,
                level,
                opt_name,
                opt_val as *mut i8,
                opt_len,
            )
        };

        #[cfg(not(target_os = "windows"))]
        let ret =
            unsafe { libc::getsockopt(self.socket, level, opt_name, opt_val, opt_len) };

        trace!(ret, opt = opt_name, "Sock::get_sock_opt");
        ret
    }
}

// -----------------------------------------------------------------------------
// Specification – interface‑level contract tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sock_basic_io_spec {
    use super::*;

    #[cfg(unix)]
    fn make_socket_pair() -> (libc::c_int, libc::c_int) {
        let mut sv = [-1; 2];
        let ret =
            unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv.as_mut_ptr()) };
        assert_eq!(ret, 0, "socketpair() failed");
        (sv[0], sv[1])
    }

    #[traced_test]
    fn assign_from_transfers_descriptor() {

        serialize_fds!();

        #[cfg(unix)]
        {
            use libc::read;

            // Use two independent pairs so replacing `dst` doesn't
            // simultaneously kill the new endpoint's peer.
            let (a, a_peer) = make_socket_pair();
            let (b, _b_peer) = make_socket_pair();
            let mut dst = Sock::from(a);
            let src = Sock::from(b);

            let old_peer = a_peer;
            dst.assign_from(src); // `src` moved; old `a` closed inside reset()

            // Old peer must see EOF -> proves old endpoint was closed.
            let mut tmp = [0u8; 1];
            let eof = unsafe { read(old_peer, tmp.as_mut_ptr() as *mut _, 1) };
            assert_eq!(eof, 0, "old peer should see EOF after reassign");

            // New descriptor is valid and equals `b`.
            assert_eq!(dst.get(), b);
        }

        info!("assign_from_transfers_descriptor passed");
    }

    #[traced_test]
    fn raw_send_and_recv() {
        serialize_fds!(); // <— add this
        #[cfg(unix)]
        {
            let (a, b) = make_socket_pair();
            let sock_a = Sock::from(a);
            let sock_b = Sock::from(b);

            let msg = b"Z";
            let sent = sock_a.send(msg.as_ptr() as *const _, 1, 0);
            assert_eq!(sent, 1);

            let mut buf = [0u8; 1];
            let got = sock_b.recv(buf.as_mut_ptr() as *mut _, 1, 0);
            assert_eq!(got, 1);
            assert_eq!(buf[0], b'Z');
        }

        info!("raw_send_and_recv passed");
    }
}
