// ---------------- [ File: bitcoin-sock/src/recv_until_terminator.rs ]
crate::ix!();

impl Sock {

    /// Read until `terminator` without consuming past it.
    pub fn recv_until_terminator(
        &self,
        terminator: u8,
        timeout: chrono::Duration,
        interrupt: &mut ThreadInterrupt,
        max_data: usize,
    ) -> String {
        let deadline = Instant::now()
            + timeout
                .to_std()
                .unwrap_or_else(|_| std::time::Duration::from_secs(u64::MAX));

        let mut data = Vec::<u8>::new();
        let mut terminator_found = false;

        // We must not consume any bytes past the terminator from the socket.
        //
        // One option is to read one byte at a time and check if we have read
        // a terminator.
        //
        // However that is very slow. 
        //
        // Instead, we peek at what is in the socket and only read as many bytes
        // as possible without crossing the terminator.
        //
        // Reading 64 MiB of random data with 262526 terminator chars takes 37
        // seconds to read one byte at a time VS 0.71 seconds with the "peek"
        // solution below. 
        //
        // Reading one byte at a time is about 50 times slower.

        while !terminator_found {
            if data.len() >= max_data {
                panic!(
                    "Received too many bytes without a terminator ({})",
                    data.len()
                );
            }

            let mut buf = [0u8; 512];
            let peek_len = min(buf.len(), max_data - data.len());
            let peek_ret =
                self.recv(buf.as_mut_ptr() as *mut c_void, peek_len, msg_peek_const());

            match peek_ret {
                -1 => {
                    let err = last_socket_error();
                    if io_error_is_permanent(err) {
                        panic!("recv(): {}", network_error_string(err));
                    }
                }
                0 => panic!("Connection unexpectedly closed by peer"),
                n if n > 0 => {
                    let slice = &buf[..n as usize];
                    if let Some(pos) = slice.iter().position(|&b| b == terminator) {
                        // Terminator is within slice -> read up to *and including* it.
                        self.read_exact(slice, pos + 1, &mut data);
                        terminator_found = true;
                    } else {
                        self.read_exact(slice, slice.len(), &mut data);
                    }
                }
                _ => unreachable!(),
            }

            // Timeout or interrupt?
            if Instant::now() >= deadline {
                panic!(
                    "Receive timeout (received {} bytes without terminator before that)",
                    data.len()
                );
            }
            if interrupt.as_bool() {
                panic!(
                    "Receive interrupted (received {} bytes without terminator before that)",
                    data.len()
                );
            }

            // Short bounded wait before retrying.
            let wait_dur = compute_bounded_wait(deadline);
            let _ = self.wait(wait_dur, SOCK_RECV as u8, core::ptr::null_mut());
        }

        // Terminator consumed – omit it in the returned string.
        if terminator_found && data.last() == Some(&terminator) {
            data.pop();
        }
        let s = String::from_utf8(data).expect("socket stream must be valid UTF‑8");
        trace!(len = s.len(), "recv_until_terminator – finished");
        s
    }

    /// Consume exactly `len` bytes from the socket, appending to `out`.
    fn read_exact(&self, _peek: &[u8], len: usize, out: &mut Vec<u8>) {
        let mut scratch = vec![0u8; len];
        let mut read_total = 0;
        while read_total < len {
            let n = self.recv(
                scratch[read_total..].as_mut_ptr() as *mut c_void,
                len - read_total,
                0,
            );
            if n <= 0 {
                let err = last_socket_error();
                panic!(
                    "recv() failed while reading {} of {} bytes: {}",
                    read_total, len, network_error_string(err)
                );
            }
            read_total += n as usize;
        }
        out.extend_from_slice(&scratch[..len]);
    }
}

// -----------------------------------------------------------------------------
// Specification
// -----------------------------------------------------------------------------
#[cfg(test)]
mod recv_until_terminator_spec {
    use super::*;

    #[traced_test]
    fn reads_up_to_newline() {
        serialize_fds!();

        #[cfg(unix)]
        {
            let (a, b) = make_socket_pair();
            let sock_rx = Sock::from(a);
            let sock_tx = Sock::from(b);

            let line = b"hello world\nsome more";
            let _ = sock_tx.send(line.as_ptr() as *const c_void, line.len(), 0);

            let mut intr = ThreadInterrupt::default();
            let got = sock_rx.recv_until_terminator(
                b'\n',
                chrono::Duration::seconds(1),
                &mut intr,
                1024,
            );
            assert_eq!(got, "hello world");
        }
    }
}
