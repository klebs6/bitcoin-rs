// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_sync.rs ]
crate::ix!();

impl WritableFileSync for PosixWritableFile {

    fn sync(&mut self) -> crate::Status {
        trace!(
            file = %self.filename(),
            fd   = *self.fd(),
            "PosixWritableFile::sync: start"
        );

        // Ensure new files referred to by the manifest are in the filesystem
        // before we flush the manifest itself.
        let mut status = self.sync_dir_if_manifest();
        if !status.is_ok() {
            debug!(
                file       = %self.filename(),
                status_str = %status.to_string(),
                "PosixWritableFile::sync: SyncDirIfManifest failed"
            );
            return status;
        }

        status = self.flush_buffer();
        if !status.is_ok() {
            debug!(
                file       = %self.filename(),
                status_str = %status.to_string(),
                "PosixWritableFile::sync: FlushBuffer failed"
            );
            return status;
        }

        status = Self::sync_fd(*self.fd(), self.filename(), false);

        debug!(
            file = %self.filename(),
            ok   = status.is_ok(),
            "PosixWritableFile::sync: completed"
        );
        status
    }
}
