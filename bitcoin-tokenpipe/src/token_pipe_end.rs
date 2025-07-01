// ---------------- [ File: bitcoin-tokenpipe/src/token_pipe_end.rs ]
crate::ix!();

//--------------------------------------------------------------------
//  Per‑platform I/O shims — **lets us share a single code‑body**.
//--------------------------------------------------------------------

#[cfg(windows)]
use libc::{_write  as sys_write,  _read  as sys_read,  _close as sys_close};

#[cfg(not(windows))]
use libc::{write   as sys_write,  read   as sys_read,   close  as sys_close};

/// One end of a token pipe.
#[derive(Debug, Getters, Builder)]
#[getset(get = "pub(crate)")]
pub struct TokenPipeEnd {
    /// File‑descriptor for this endpoint (`‑1` == closed).
    #[builder(default = "-1")]
    fd: i32,
}

impl Drop for TokenPipeEnd {
    fn drop(&mut self) {
        trace!(fd = self.fd, "TokenPipeEnd::drop");
        self.close();
    }
}

#[cfg(not(windows))]
impl TokenPipeEnd {

    /// Construct a new endpoint (`‑1` ⇒ closed).
    pub fn new(fd: Option<i32>) -> Self {
        let fd = fd.unwrap_or(-1);
        trace!(fd, "TokenPipeEnd::new");
        Self { fd }
    }

    /// Write a single token.
    ///
    /// * `0`  on success  
    /// * `<0` on error (`TS_ERR` / `TS_EOS`)
    pub fn token_write(&mut self, token: u8) -> i32 {
        trace!(fd = self.fd, token, "token_write → start");
        if self.fd == -1 {
            warn!("token_write on closed fd");
            return TokenPipeEndStatus::TS_ERR as i32;
        }

        loop {
            let rc = unsafe {
                sys_write(self.fd, &token as *const _ as *const c_void, 1)
            };
            match rc {
                1 => { trace!("token_write success"); return 0; }
                0 => { info!("peer closed (EOS)");   return TokenPipeEndStatus::TS_EOS as i32; }
                _ if rc < 0 => {
                    let e = last_errno();
                    if e == libc::EINTR { debug!("retry after EINTR"); continue; }
                    error!(e, "token_write error");
                    return TokenPipeEndStatus::TS_ERR as i32;
                }
                _ => unreachable!("write returned unexpected count"),
            }
        }
    }

    /// Read a single token.
    ///
    /// * `≥0` token value on success  
    /// * `<0`  on error (`TS_ERR` / `TS_EOS`)
    pub fn token_read(&mut self) -> i32 {
        trace!(fd = self.fd, "token_read → start");
        if self.fd == -1 {
            warn!("token_read on closed fd");
            return TokenPipeEndStatus::TS_ERR as i32;
        }

        let mut token: u8 = 0;
        loop {
            let rc = unsafe {
                sys_read(self.fd, &mut token as *mut _ as *mut c_void, 1)
            };
            match rc {
                1 => { trace!(token, "token_read success"); return token as i32; }
                0 => { info!("end‑of‑stream");               return TokenPipeEndStatus::TS_EOS as i32; }
                _ if rc < 0 => {
                    let e = last_errno();
                    if e == libc::EINTR { debug!("retry after EINTR"); continue; }
                    error!(e, "token_read error");
                    return TokenPipeEndStatus::TS_ERR as i32;
                }
                _ => unreachable!("read returned unexpected count"),
            }
        }
    }

    /// Explicitly close this endpoint.
    pub fn close(&mut self) {
        trace!(fd = self.fd, "TokenPipeEnd::close");
        if self.fd != -1 {
            // SAFETY: valid fd / ignored if already closed.
            unsafe { sys_close(self.fd); }
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

    // -------------------------------------------------------------
    //  Single‑byte round‑trip
    // -------------------------------------------------------------
    #[traced_test]
    fn roundtrip_single_token() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        assert_eq!(writer.token_write(42), 0);
        assert_eq!(reader.token_read(), 42);
    }

    // -------------------------------------------------------------
    //  Multiple‑byte round‑trip
    // -------------------------------------------------------------
    #[traced_test]
    fn roundtrip_multiple_tokens() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        for b in 0_u8..=15 {
            assert_eq!(writer.token_write(b), 0);
            assert_eq!(reader.token_read(),  b as i32);
        }
    }

    // -------------------------------------------------------------
    //  Writer closes first → reader receives EOS
    // -------------------------------------------------------------
    #[traced_test]
    fn writer_close_signals_eos() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        writer.close();
        assert!(!writer.is_open());

        let rc = reader.token_read();
        assert_eq!(rc, TokenPipeEndStatus::TS_EOS as i32);
    }

    // -------------------------------------------------------------
    //  Reader closes first → subsequent writes error out
    // -------------------------------------------------------------
    #[traced_test]
    fn reader_close_then_write_returns_err() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let mut reader = pipe.take_read_end();
        let mut writer = pipe.take_write_end();

        reader.close();
        assert!(!reader.is_open());

        let rc = writer.token_write(99);
        assert_eq!(rc, TokenPipeEndStatus::TS_ERR as i32);
    }

    // -------------------------------------------------------------
    //  Threaded stress‑test: 0‑100 inclusive
    // -------------------------------------------------------------
    #[traced_test]
    fn threaded_roundtrip_hundred_tokens() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let reader     = pipe.take_read_end();
        let writer     = pipe.take_write_end();

        // Spawn writer thread
        let handle = std::thread::spawn(move || {
            let mut w = writer;
            for b in 0_u8..=100 {
                assert_eq!(w.token_write(b), 0);
            }
            w.close();
        });

        // Reader loop (on main thread)
        let mut r = reader;
        let mut seen = Vec::<u8>::new();
        loop {
            match r.token_read() {
                x if x >= 0 => seen.push(x as u8),
                x => {
                    assert_eq!(x, TokenPipeEndStatus::TS_EOS as i32);
                    break;
                }
            }
        }

        handle.join().expect("writer thread join");
        assert_eq!(seen, (0_u8..=100).collect::<Vec<_>>());
    }
}
