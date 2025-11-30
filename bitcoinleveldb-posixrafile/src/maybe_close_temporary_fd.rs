// ---------------- [ File: bitcoinleveldb-posixrafile/src/maybe_close_temporary_fd.rs ]
crate::ix!();

impl PosixRandomAccessFile {

    /// Close a temporary fd if one was opened for this read.
    pub fn maybe_close_temporary_fd(&self, fd_to_use: i32, need_close: bool) {
        if need_close {
            debug_assert!(fd_to_use != *self.fd());
            trace!(
                fd = fd_to_use,
                "PosixRandomAccessFile::maybe_close_temporary_fd: closing temp fd"
            );
            unsafe {
                libc::close(fd_to_use);
            }
        }
    }
}
