// ---------------- [ File: bitcoinleveldb-posixmmaprfile/src/drop.rs ]
crate::ix!();

impl Drop for PosixMmapReadableFile {

    fn drop(&mut self) {
        use libc::{munmap, c_void};

        trace!(
            file = %self.filename(),
            base = ?self.mmap_base(),
            len  = self.length(),
            limiter = ?self.mmap_limiter(),
            "PosixMmapReadableFile::drop: unmapping region and releasing mmap token"
        );

        // Unmap the region if it looks valid.
        if !self.mmap_base().is_null() && *self.length() > 0 {
            let rc = unsafe {
                munmap(*self.mmap_base() as *mut c_void, *self.length())
            };
            if rc != 0 {
                // We can't recover here, but log for diagnostics.
                let err = std::io::Error::last_os_error();
                warn!(
                    file = %self.filename(),
                    rc,
                    %err,
                    "PosixMmapReadableFile::drop: munmap failed"
                );
            }
        }

        // Release our slot in the mmap limiter, if present.
        if !self.mmap_limiter().is_null() {
            // Safety: the caller guaranteed that `mmap_limiter` outlives this
            // object and that we successfully "acquired" exactly one mmap slot
            // at construction time.
            unsafe {
                let limiter: &Limiter = &*(*self.mmap_limiter());
                limiter.release();
            }
        }

        debug!(
            file = %self.filename(),
            "PosixMmapReadableFile::drop: completed"
        );
    }
}
