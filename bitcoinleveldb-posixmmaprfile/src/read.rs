// ---------------- [ File: bitcoinleveldb-posixmmaprfile/src/read.rs ]
crate::ix!();

impl RandomAccessFileRead for PosixMmapReadableFile {

    fn read(
        &self,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        _scratch: *mut u8,
    ) -> Status {
        trace!(
            file   = %self.filename(),
            offset,
            n,
            len    = self.length(),
            "PosixMmapReadableFile::read: entry"
        );

        // Compute offset+n in a wide type to avoid overflow.
        let length_u128 = *self.length() as u128;
        let offset_u128 = offset as u128;
        let n_u128      = n as u128;

        let end_ok = offset_u128
            .checked_add(n_u128)
            .map(|end| end <= length_u128)
            .unwrap_or(false);

        if !end_ok {
            // Out‑of‑range read: return empty slice and an IO error
            // (original C++ uses PosixError(fname, EINVAL)).
            unsafe {
                *result = Slice::default();
            }

            let fname = self.filename().clone();
            let fname_slice = Slice::from(&fname);
            let status = Status::io_error(&fname_slice, None);

            debug!(
                file       = %self.filename(),
                offset,
                n,
                len        = self.length(),
                status_str = %status.to_string(),
                "PosixMmapReadableFile::read: range error"
            );
            return status;
        }

        // Safe now to cast offset to usize, since end <= length <= usize::MAX.
        let offset_usize = offset as usize;

        unsafe {
            let ptr = self.mmap_base().add(offset_usize);
            *result = Slice::from_ptr_len(ptr, n);
        }

        let status = Status::ok();
        debug!(
            file   = %self.filename(),
            offset,
            n,
            "PosixMmapReadableFile::read: success"
        );
        status
    }
}
