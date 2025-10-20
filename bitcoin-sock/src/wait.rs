// ---------------- [ File: bitcoin-sock/src/wait.rs ]
crate::ix!();

impl Sock {
    /// Block until the socket becomes ready for the requested events.
    ///
    /// *Uses `poll(2)` on Linux when `USE_POLL` is enabled, otherwise
    /// falls back to a classic `select(2)` implementation that honours
    /// the `is_selectable_socket` constraint.*
    pub fn wait(
        &self,
        timeout: Duration,
        requested: SockEvent,
        occurred: *mut SockEvent,
    ) -> bool {
        #[cfg(all(target_os = "linux", feature = "use_poll"))]
        {
            use libc::{poll, pollfd, POLLIN, POLLOUT};

            let mut fd = pollfd {
                fd: self.socket(),
                events: 0,
                revents: 0,
            };
            if requested & SOCK_RECV as u8 != 0 {
                fd.events |= POLLIN;
            }
            if requested & SOCK_SEND as u8 != 0 {
                fd.events |= POLLOUT;
            }

            let ms = timeout.as_millis() as libc::c_int;
            let ret = unsafe { poll(&mut fd, 1, ms) };

            if ret == SOCKET_ERROR {
                return false;
            }

            unsafe {
                if !occurred.is_null() {
                    *occurred = 0;
                    if fd.revents & POLLIN != 0 {
                        *occurred |= SOCK_RECV as u8;
                    }
                    if fd.revents & POLLOUT != 0 {
                        *occurred |= SOCK_SEND as u8;
                    }
                }
            }

            true
        }

        #[cfg(not(all(target_os = "linux", feature = "use_poll")))]
        {
            use libc::{
                fd_set, select, timeval, FD_ISSET, FD_SET, FD_SETSIZE, FD_ZERO, timeval as TimeVal,
            };

            if !is_selectable_socket(self.socket()) {
                return false;
            }

            unsafe {
                let mut recv_set: fd_set = std::mem::zeroed();
                let mut send_set: fd_set = std::mem::zeroed();
                FD_ZERO(&mut recv_set);
                FD_ZERO(&mut send_set);

                if requested & SOCK_RECV as u8 != 0 {
                    FD_SET(*self.socket(), &mut recv_set);
                }
                if requested & SOCK_SEND as u8 != 0 {
                    FD_SET(*self.socket(), &mut send_set);
                }

                let mut tv = TimeVal {
                    tv_sec: timeout.as_seconds_f32() as libc::time_t,
                    tv_usec: (timeout.subsec_microseconds()) as libc::suseconds_t,
                };

                let ret = select(
                    *self.socket() + 1,
                    &mut recv_set,
                    &mut send_set,
                    std::ptr::null_mut(),
                    &mut tv,
                );

                if ret == SOCKET_ERROR {
                    return false;
                }

                if !occurred.is_null() {
                    *occurred = 0;
                    if FD_ISSET(*self.socket(), &mut recv_set) {
                        *occurred |= SOCK_RECV as u8;
                    }
                    if FD_ISSET(*self.socket(), &mut send_set) {
                        *occurred |= SOCK_SEND as u8;
                    }
                }

                true
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Specification – edge‑to‑edge readiness tests
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sock_wait_spec {
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
    fn wait_detects_readiness() {
        serialize_fds!(); // <— add this
        #[cfg(unix)]
        {
            use std::ptr;

            let (a, b) = make_socket_pair();
            let sock_a = Sock::from(a);
            let sock_b = Sock::from(b);

            // Prepare: write a byte so that `sock_a` becomes readable.
            let one = b"X";
            let sent = sock_b.send(one.as_ptr() as *const _, 1, 0);
            assert_eq!(sent, 1);

            // Now wait for RECV on `sock_a`.
            let mut occurred: SockEvent = 0;
            let ok = sock_a.wait(Duration::seconds(1), SOCK_RECV as u8, &mut occurred);
            assert!(ok, "wait() should succeed");
            assert!(occurred & SOCK_RECV as u8 != 0, "read readiness expected");
        }

        info!("wait_detects_readiness passed");
    }
}
