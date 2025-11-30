// ---------------- [ File: bitcoinleveldb-posixrafile/src/read.rs ]
crate::ix!();

impl RandomAccessFileRead for PosixRandomAccessFile {

    fn read(
        &self,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {

        trace!(
            offset,
            n,
            has_permanent_fd = self.has_permanent_fd(),
            fd = self.fd(),
            "PosixRandomAccessFile::read: start"
        );

        // 1. Decide which fd to use (permanent vs temporary).
        let (fd_to_use, need_close) = match self.open_fd_for_read(result) {
            Ok(pair) => pair,
            Err(status) => {
                trace!(
                    ok = false,
                    "PosixRandomAccessFile::read: open_fd_for_read failed"
                );
                return status;
            }
        };

        debug_assert!(fd_to_use != -1);

        // 2. Perform the pread and map the result into a Slice.
        let (status, actual_len) =
            self.pread_into_slice(fd_to_use, offset, n, result, scratch);

        // 3. Clean up the temporary fd if needed.
        self.maybe_close_temporary_fd(fd_to_use, need_close);

        trace!(
            ok = status.is_ok(),
            bytes = actual_len,
            "PosixRandomAccessFile::read: completed"
        );

        status
    }
}
