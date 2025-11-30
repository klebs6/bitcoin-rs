// ---------------- [ File: bitcoinleveldb-posixrafile/src/filename_cstring_or_status.rs ]
crate::ix!();

impl PosixRandomAccessFile {

    /// Convert the stored filename into a CString, or return a Status
    /// describing why it failed (interior NUL).
    pub fn filename_cstring_or_status(&self, result: *mut Slice) -> Result<std::ffi::CString, Status> {
        match std::ffi::CString::new(self.filename().as_str()) {
            Ok(cstr) => Ok(cstr),
            Err(_) => {
                // Interior NUL in filename – treat as IO error.
                unsafe {
                    *result = Slice::default();
                }

                let filename = self.filename().clone();
                let ctx      = Slice::from(&filename);
                let msg      = "filename contains interior NUL".to_string();
                let msg_slice = Slice::from(&msg);

                let status = Status::io_error(&ctx, Some(&msg_slice));

                debug!(
                    status_str = %status.to_string(),
                    "PosixRandomAccessFile::filename_cstring_or_status: CString::new failed"
                );

                Err(status)
            }
        }
    }
}
