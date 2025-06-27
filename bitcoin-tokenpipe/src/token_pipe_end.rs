// ---------------- [ File: bitcoin-tokenpipe/src/token_pipe_end.rs ]
crate::ix!();

/// One end of a token pipe.
#[cfg(not(windows))]
#[derive(Debug, Getters, Builder)]
#[getset(get = "pub(crate)")]
pub struct TokenPipeEnd {
    /// File descriptor for this endpoint (`-1` == closed).
    #[builder(default = "-1")]
    fd: i32,
}

#[cfg(not(windows))]
impl Drop for TokenPipeEnd {
    fn drop(&mut self) {
        trace!(fd = self.fd, "TokenPipeEnd::drop");
        self.close();
    }
}

#[cfg(not(windows))]
impl TokenPipeEnd {

    /// Construct a new endpoint.  `fd = -1` ⇒ closed.
    pub fn new(fd: Option<i32>) -> Self {
        let fd = fd.unwrap_or(-1);
        trace!(fd, "TokenPipeEnd::new");
        Self { fd }
    }

    /// Write a single token.
    ///
    /// * **return 0** on success  
    /// * **return <0** on error (`TS_ERR` / `TS_EOS`)
    ///
    /// TS_ERR If an error happened.
    ///
    /// TS_EOS If end of stream happened.
    ///
    pub fn token_write(&mut self, token: u8) -> i32 {
        trace!(fd = self.fd, token, "token_write → start");
        if self.fd == -1 {
            warn!("token_write on closed fd");
            return TokenPipeEndStatus::TS_ERR as i32;
        }

        loop {
            // SAFETY: we pass a valid pointer to one byte of data.
            let rc = unsafe {
                libc::write(
                    self.fd,
                    &token as *const u8 as *const c_void,
                    1,
                )
            };
            match rc {
                1 => {
                    trace!("token_write success");
                    return 0;
                }
                0 => {
                    info!("token_write: peer closed (EOS)");
                    return TokenPipeEndStatus::TS_EOS as i32;
                }
                _ if rc < 0 => {
                    let errno = unsafe { *libc::__errno_location() };
                    if errno == libc::EINTR {
                        debug!("token_write interrupted → retry");
                        continue;
                    }
                    error!(errno, "token_write error");
                    return TokenPipeEndStatus::TS_ERR as i32;
                }
                _ => unreachable!("write returned unexpected count"),
            }
        }
    }

    /// Read a single token from an endpoint.
    ///
    /// * **return ≥0** token value on success  
    /// * **return <0** on error (`TS_ERR` / `TS_EOS`)
    ///
    /// TS_ERR If an error happened.
    ///
    /// TS_EOS If end of stream happened.
    ///
    pub fn token_read(&mut self) -> i32 {
        trace!(fd = self.fd, "token_read → start");
        if self.fd == -1 {
            warn!("token_read on closed fd");
            return TokenPipeEndStatus::TS_ERR as i32;
        }

        let mut token: u8 = 0;
        loop {
            // SAFETY: we pass a valid mutable byte buffer.
            let rc = unsafe {
                libc::read(
                    self.fd,
                    &mut token as *mut u8 as *mut c_void,
                    1,
                )
            };
            match rc {
                1 => {
                    trace!(token, "token_read success");
                    return token as i32;
                }
                0 => {
                    info!("token_read: end‑of‑stream");
                    return TokenPipeEndStatus::TS_EOS as i32;
                }
                _ if rc < 0 => {
                    let errno = unsafe { *libc::__errno_location() };
                    if errno == libc::EINTR {
                        debug!("token_read interrupted → retry");
                        continue;
                    }
                    error!(errno, "token_read error");
                    return TokenPipeEndStatus::TS_ERR as i32;
                }
                _ => unreachable!("read returned unexpected count"),
            }
        }
    }

    /// Close this endpoint explicitly.
    pub fn close(&mut self) {
        trace!(fd = self.fd, "TokenPipeEnd::close");
        if self.fd != -1 {
            // SAFETY: valid fd / or ignored by OS if already closed.
            unsafe { libc::close(self.fd) };
            self.fd = -1;
        }
    }

    /// Is the endpoint still open?
    pub fn is_open(&mut self) -> bool {
        self.fd != -1
    }

    //---- Move‑only helpers (match original C++ semantics) ------------

    /// Move‑construct from `other`, leaving `other` closed.
    pub fn new_from_other(mut other: TokenPipeEnd) -> Self {
        trace!(fd = other.fd, "TokenPipeEnd::new_from_other");
        let fd = other.fd;
        other.fd = -1;
        Self { fd }
    }

    /// Assign from `other`, closing the current endpoint first.
    pub fn assign_from(&mut self, mut other: TokenPipeEnd) -> &mut TokenPipeEnd {
        trace!(old_fd = self.fd, new_fd = other.fd, "TokenPipeEnd::assign_from");
        self.close();
        self.fd = other.fd;
        other.fd = -1;
        self
    }
}

#[cfg(all(test, not(windows)))]
mod tokenpipe_end_behavior {
    use super::*;

    #[traced_test]
    fn roundtrip_single_token() {

        let mut pipe = match TokenPipe::make() {
            Some(p) => p,
            None => {
                panic!("Failed to create TokenPipe");
            }
        };

        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        // Write a single byte.
        assert_eq!(writer.token_write(42), 0);

        // Read it back.
        let rcv = reader.token_read();
        assert_eq!(rcv, 42);

        // Clean‑up.
        writer.close();
        reader.close();
    }

    #[traced_test]
    fn writer_close_signals_eos() {

        let mut pipe = TokenPipe::make().expect("pipe must be creatable");

        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        // Close writer immediately.
        writer.close();
        assert!(!writer.is_open());

        // Reader should observe EOS.
        let rc = reader.token_read();
        assert_eq!(rc, TokenPipeEndStatus::TS_EOS as i32);
    }
}
