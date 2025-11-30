// ---------------- [ File: bitcoinleveldb-posixenv/src/rename_file.rs ]
crate::ix!();

impl RenameFile for PosixEnv {

    fn rename_file(
        &mut self,
        from: &String,
        to:   &String,
    ) -> crate::Status {
        trace!(
            from = %from,
            to   = %to,
            "PosixEnv::rename_file: renaming file"
        );

        match std::fs::rename(from, to) {
            Ok(()) => {
                debug!(
                    from = %from,
                    to   = %to,
                    "PosixEnv::rename_file: rename succeeded"
                );
                crate::Status::ok()
            }
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    from  = %from,
                    to    = %to,
                    errno,
                    "PosixEnv::rename_file: rename failed"
                );
                posix_error(from, errno)
            }
        }
    }
}
