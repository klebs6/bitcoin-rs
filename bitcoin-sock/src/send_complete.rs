// ---------------- [ File: bitcoin-sock/src/send_complete.rs ]
crate::ix!();

impl Sock {
    /// Send the entire buffer, retrying on transient errors.
    ///
    /// On failure this panics with a message identical in spirit to the
    /// C++ `std::runtime_error`.
    pub fn send_complete(
        &self,
        data: &String,
        timeout: chrono::Duration,
        interrupt: &mut ThreadInterrupt,
    ) {
        let deadline = Instant::now()
            + timeout
                .to_std()
                .unwrap_or_else(|_| std::time::Duration::from_secs(u64::MAX));

        let bytes = data.as_bytes();
        let mut sent = 0;

        loop {
            if sent == bytes.len() {
                debug!(total = sent, "send_complete – finished");
                return;
            }

            let ret = self.send(
                bytes[sent..].as_ptr() as *const c_void,
                bytes.len() - sent,
                msg_nosignal_const(),
            );

            if ret > 0 {
                sent += ret as usize;
                continue;
            }

            // Error handling
            let err = last_socket_error();
            if io_error_is_permanent(err) {
                panic!("send(): {}", network_error_string(err));
            }

            // Timeout?
            if Instant::now() >= deadline {
                panic!(
                    "Send timeout (sent only {} of {} bytes before that)",
                    sent,
                    bytes.len()
                );
            }

            // Interrupt?
            if interrupt.is_interrupted() {
                panic!(
                    "Send interrupted (sent only {} of {} bytes before that)",
                    sent,
                    bytes.len()
                );
            }

            // Wait a bounded amount before retrying.
            let wait_ns = min(
                (deadline - Instant::now()).as_nanos(),
                MAX_WAIT_FOR_IO.num_nanoseconds().unwrap_or(1_000_000_000),
            );
            let _ = self.wait(
                chrono::Duration::nanoseconds(wait_ns as i64),
                SOCK_SEND as u8,
                std::ptr::null_mut(),
            );
        }
    }
}

#[inline(always)]
const fn msg_nosignal_const() -> i32 {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        libc::MSG_NOSIGNAL
    }
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        0
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

// -----------------------------------------------------------------------------
// Specification
// -----------------------------------------------------------------------------
#[cfg(test)]
mod send_complete_spec {
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
    fn transmits_entire_payload() {
        #[cfg(unix)]
        {
            let (a, b) = make_socket_pair();
            let sock_tx = Sock::from(a);
            let sock_rx = Sock::from(b);

            let payload = "rust‑bitcoin!".repeat(256); // sizeable buffer
            let mut interrupt = ThreadInterrupt::default();

            sock_tx.send_complete(
                &payload,
                chrono::Duration::seconds(1),
                &mut interrupt,
            );

            let mut buf = vec![0u8; payload.len()];
            let mut got = 0;
            while got < buf.len() {
                let n = sock_rx.recv(
                    buf[got..].as_mut_ptr() as *mut c_void,
                    buf.len() - got,
                    0,
                );
                got += n as usize;
            }
            assert_eq!(buf, payload.as_bytes());
        }
    }
}
