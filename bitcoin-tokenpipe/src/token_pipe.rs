// ---------------- [ File: bitcoin-tokenpipe/src/token_pipe.rs ]
crate::ix!();

/// An interprocess or interthread pipe for sending tokens (one-byte values) over.
#[cfg(not(windows))]
#[derive(Debug, Getters, Builder)]
#[getset(get = "pub(crate)")]
pub struct TokenPipe {
    /// `[0]` = read end, `[1]` = write end (`-1` == closed)
    #[builder(default = "[-1, -1]")]
    fds: [i32; 2],
}

#[cfg(not(windows))]
impl Drop for TokenPipe {
    fn drop(&mut self) {
        trace!(fds = ?self.fds, "TokenPipe::drop");
        self.close();
    }
}

#[cfg(not(windows))]
impl TokenPipe {
    /// Construct directly from raw fds.
    pub fn new(fds: [i32; 2]) -> Self {
        trace!(fds = ?fds, "TokenPipe::new");
        Self { fds }
    }

    /// Create a brand‑new pipe (uses `pipe(2)`).
    ///
    /// * **`Some(pipe)`** on success  
    /// * **`None`**        on error
    pub fn make() -> Option<TokenPipe> {
        trace!("TokenPipe::make → start");
        let mut fds = [-1; 2];
        // SAFETY: we give `pipe` a valid pointer to two ints.
        let rc = unsafe { libc::pipe(fds.as_mut_ptr()) };
        if rc != 0 {
            let errno = unsafe { *libc::__errno_location() };
            error!(errno, "pipe(2) failed");
            return None;
        }
        info!(fds = ?fds, "pipe(2) created successfully");
        Some(Self { fds })
    }

    /// Take (move out) the read end.
    ///
    /// Take the read end of this pipe. 
    /// This can only be called once, as the object will be moved out.
    ///
    pub fn take_read_end(&mut self) -> TokenPipeEnd {
        trace!(fd = self.fds[0], "TokenPipe::take_read_end");
        let fd = self.fds[0];
        self.fds[0] = -1;
        TokenPipeEnd::new(Some(fd))
    }

    /// Take (move out) the write end.
    ///
    /// Take the write end of this pipe. 
    /// This should only be called once, as the object will be moved out.
    ///
    pub fn take_write_end(&mut self) -> TokenPipeEnd {
        trace!(fd = self.fds[1], "TokenPipe::take_write_end");
        let fd = self.fds[1];
        self.fds[1] = -1;
        TokenPipeEnd::new(Some(fd))
    }

    /// Close any endpoint that has not been moved out.
    pub fn close(&mut self) {
        trace!(fds = ?self.fds, "TokenPipe::close");
        for fd in &mut self.fds {
            if *fd != -1 {
                // SAFETY: valid fd / or ignored by OS if already closed.
                unsafe { libc::close(*fd) };
                *fd = -1;
            }
        }
    }

    //---- Move‑only helpers ------------------------------------------

    pub fn new_from_other(mut other: TokenPipe) -> Self {
        trace!(fds = ?other.fds, "TokenPipe::new_from_other");
        let mut fds = [-1; 2];
        for (dst, src) in fds.iter_mut().zip(other.fds.iter_mut()) {
            *dst = *src;
            *src = -1;
        }
        Self { fds }
    }

    pub fn assign_from(&mut self, mut other: TokenPipe) -> &mut TokenPipe {
        trace!(
            self_fds = ?self.fds,
            other_fds = ?other.fds,
            "TokenPipe::assign_from"
        );
        self.close();
        for (dst, src) in self.fds.iter_mut().zip(other.fds.iter_mut()) {
            *dst = *src;
            *src = -1;
        }
        self
    }
}
