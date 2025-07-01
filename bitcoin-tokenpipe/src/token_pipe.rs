// ---------------- [ File: bitcoin-tokenpipe/src/token_pipe.rs ]
crate::ix!();

/// An interprocess or interthread pipe for sending tokens (one-byte values) over.
#[derive(Debug, Getters, Builder)]
#[getset(get = "pub(crate)")]
pub struct TokenPipe {
    /// `[0]` = read end, `[1]` = write end (`-1` == closed)
    #[builder(default = "[-1, -1]")]
    fds: [i32; 2],
}

impl Drop for TokenPipe {
    fn drop(&mut self) {
        trace!(fds = ?self.fds, "TokenPipe::drop");
        self.close();
    }
}

impl TokenPipe {
    /// Construct directly from raw FDs.
    pub fn new(fds: [i32; 2]) -> Self {
        trace!(fds = ?fds, "TokenPipe::new");
        Self { fds }
    }

    /// Create a fresh pipe.
    ///
    /// * `Some(pipe)` on success  
    /// * `None`       on error
    pub fn make() -> Option<Self> {
        trace!("TokenPipe::make → start");

        //―――― POSIX (non‑Windows) ――――
        #[cfg(not(windows))]
        {
            let mut fds = [-1; 2];
            if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
                error!(errno = last_errno(), "pipe(2) failed");
                return None;
            }
            info!(fds = ?fds, "pipe(2) created");
            return Some(Self { fds });
        }

        //―――― Windows (msvcrt `_pipe`) ――――
        #[cfg(windows)]
        {
            let mut fds = [-1; 2];
            const BUFSZ: libc::c_int = 4096;
            const O_BINARY: libc::c_int = 0x8000; // _O_BINARY
            if unsafe { libc::_pipe(fds.as_mut_ptr(), BUFSZ, O_BINARY) } != 0 {
                error!(errno = last_errno(), "_pipe failed");
                return None;
            }
            info!(fds = ?fds, "_pipe created");
            return Some(Self { fds });
        }
    }

    /// Move out the read end (may be called once).
    pub fn take_read_end(&mut self) -> TokenPipeEnd {
        trace!(fd = self.fds[0], "TokenPipe::take_read_end");
        let fd = self.fds[0]; 
        self.fds[0] = -1; 
        TokenPipeEnd::new(Some(fd))
    }

    /// Move out the write end (may be called once).
    pub fn take_write_end(&mut self) -> TokenPipeEnd {
        trace!(fd = self.fds[1], "TokenPipe::take_write_end");
        let fd = self.fds[1]; 
        self.fds[1] = -1; 
        TokenPipeEnd::new(Some(fd))
    }

    /// Close any still‑owned endpoints.
    pub fn close(&mut self) {
        trace!(fds = ?self.fds, "TokenPipe::close");
        for fd in &mut self.fds {
            if *fd != -1 {
                #[cfg(windows)]
                unsafe { libc::_close(*fd); }
                #[cfg(not(windows))]
                unsafe { libc::close(*fd); }
                *fd = -1;
            }
        }
    }

    //―――― move‑only helpers ――――
    pub fn new_from_other(mut other: Self) -> Self {
        trace!(fds = ?other.fds, "TokenPipe::new_from_other");
        let mut fds = [-1; 2];
        for (d, s) in fds.iter_mut().zip(other.fds.iter_mut()) { 
            *d = *s; 
            *s = -1; 
        }
        Self { fds }
    }
    pub fn assign_from(&mut self, mut other: Self) -> &mut Self {
        trace!(self_fds = ?self.fds, other_fds = ?other.fds, "TokenPipe::assign_from");
        self.close();
        for (d, s) in self.fds.iter_mut().zip(other.fds.iter_mut()) { 
            *d = *s; 
            *s = -1; 
        }
        self
    }
}

#[cfg(all(test, not(windows)))]
mod tokenpipe_pipe_behavior {
    use super::*;

    // -------------------------------------------------------------
    //  take_read_end / take_write_end are move‑only
    // -------------------------------------------------------------
    #[traced_test]
    fn endpoints_are_moved_only_once() {

        let mut pipe = TokenPipe::make().expect("pipe creation");

        // First call succeeds
        let _r1 = pipe.take_read_end();
        let _w1 = pipe.take_write_end();

        // Subsequent calls hand back closed endpoints
        let mut r2 = pipe.take_read_end();
        let mut w2 = pipe.take_write_end();
        assert_eq!(r2.token_read(), TokenPipeEndStatus::TS_ERR as i32);
        assert_eq!(w2.token_write(1), TokenPipeEndStatus::TS_ERR as i32);
    }

    // -------------------------------------------------------------
    //  assign_from moves FD ownership
    // -------------------------------------------------------------
    #[traced_test]
    fn assign_from_transfers_fd() {

        let mut pipe   = TokenPipe::make().expect("pipe creation");
        let mut writer1 = pipe.take_write_end();       // valid writer
        let mut writer2 = TokenPipeEnd::new(None);     // closed endpoint

        // Transfer ownership
        writer2.assign_from(writer1);

        // Roundtrip through new owner
        let mut reader = pipe.take_read_end();
        assert_eq!(writer2.token_write(0xAB), 0);
        assert_eq!(reader.token_read(),       0xAB);
    }

    // -------------------------------------------------------------
    //  Closing a pipe before taking endpoints
    // -------------------------------------------------------------
    #[traced_test]
    fn close_then_take_returns_closed_endpoints() {

        let mut pipe = TokenPipe::make().expect("pipe creation");
        pipe.close();

        let mut r = pipe.take_read_end();
        let mut w = pipe.take_write_end();
        assert_eq!(r.token_read(),            TokenPipeEndStatus::TS_ERR as i32);
        assert_eq!(w.token_write(77),         TokenPipeEndStatus::TS_ERR as i32);
    }
}
