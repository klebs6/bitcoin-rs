// ---------------- [ File: bitcoin-tokenpipe/src/token_pipe_end_status.rs ]
crate::ix!();

/// Return value constants for TokenWrite and TokenRead.
#[cfg(not(windows))]
#[repr(i32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenPipeEndStatus {
    /// I/O error
    TS_ERR = -1,
    /// End‑of‑stream
    TS_EOS = -2,
}
