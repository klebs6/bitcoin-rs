// ---------------- [ File: bitcoin-sock/src/is_connected.rs ]
crate::ix!();

impl Sock {
    /// Determine whether the socket is still connected.
    ///
    /// Mirrors the original C++ logic by issuing a one‑byte `MSG_PEEK`
    /// read and classifying the outcome.
    pub fn is_connected(&self, errmsg: &mut String) -> bool {
        // Empty holder ⇒ definitely disconnected.
        if *self.socket() == INVALID_SOCKET {
            errmsg.clear();
            errmsg.push_str("not connected");
            return false;
        }

        let mut byte: u8 = 0;
        let ret = self.recv(
            &mut byte as *mut _ as *mut c_void,
            1,
            msg_peek_const(),
        );

        match ret {
            -1 => {
                let err = last_socket_error();
                if io_error_is_permanent(err) {
                    *errmsg = network_error_string(err);
                    return false;
                }
                true // transient ⇒ assume still connected
            }
            0 => {
                errmsg.clear();
                errmsg.push_str("closed");
                false
            }
            _ => true, // any positive value ⇒ data ready ⇒ connected
        }
    }
}

/// Platform `MSG_PEEK`.
#[inline(always)]
const fn msg_peek_const() -> i32 {
    #[cfg(target_os = "windows")]
    {
        winapi::um::winsock2::MSG_PEEK
    }
    #[cfg(not(target_os = "windows"))]
    {
        libc::MSG_PEEK
    }
}

#[inline(always)]
fn last_socket_error() -> i32 {
    #[cfg(target_os = "windows")]
    {
        unsafe { winapi::um::winsock2::WSAGetLastError() }
    }
    #[cfg(not(target_os = "windows"))]
    {
        last_errno()
    }
}

#[cfg(test)]
mod is_connected_spec {
    use super::*;

    #[cfg(unix)]
    fn make_socket_pair() -> (libc::c_int, libc::c_int) {
        let mut sv = [-1; 2];
        let ret =
            unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, sv.as_mut_ptr()) };
        assert_eq!(ret, 0);
        (sv[0], sv[1])
    }

    #[traced_test]
    fn detects_closed_peer() {
        serialize_fds!(); // <— add this
        #[cfg(unix)]
        {
            let (a, b) = make_socket_pair();
            let sock_a = Sock::from(a);
            let mut sock_b = Sock::from(b);

            // Close peer `b`, then probe from `a`.
            sock_b.reset();

            let mut msg = String::new();
            assert!(!sock_a.is_connected(&mut msg));
            assert_eq!(msg, "closed");
        }
    }
}
