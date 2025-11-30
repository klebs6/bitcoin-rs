// ---------------- [ File: bitcoinleveldb-posixwfile/src/posix_writable_file_append.rs ]
crate::ix!();

impl WritableFileAppend for PosixWritableFile {

    fn append(&mut self, data: &Slice) -> Status {
        use std::cmp;

        let mut write_size = *data.size();

        trace!(
            file      = %self.filename(),
            write_len = write_size,
            buf_pos   = *self.pos(),
            "PosixWritableFile::append"
        );

        if write_size == 0 {
            return Status::ok();
        }

        unsafe {
            let mut write_ptr = *data.data();

            // Fit as much as possible into the buffer.
            let avail     = WRITABLE_FILE_BUFFER_SIZE.saturating_sub(*self.pos());
            let copy_size = cmp::min(write_size, avail);

            if copy_size > 0 {
                let dst = self.buf_mut().as_mut_ptr().add(*self.pos());
                std::ptr::copy_nonoverlapping(write_ptr, dst, copy_size);
                *self.pos_mut() += copy_size;
                write_ptr = write_ptr.add(copy_size);
                write_size -= copy_size;
            }

            if write_size == 0 {
                // Everything fit in the buffer.
                debug!(
                    file    = %self.filename(),
                    buf_pos = *self.pos(),
                    "PosixWritableFile::append: fully buffered"
                );
                return Status::ok();
            }

            // Can't fit in buffer, so need to do at least one write.
            let mut status = self.flush_buffer();
            if !status.is_ok() {
                debug!(
                    file       = %self.filename(),
                    status_str = %status.to_string(),
                    "PosixWritableFile::append: FlushBuffer failed"
                );
                return status;
            }

            // Small writes go to buffer, large writes are written directly.
            if write_size < WRITABLE_FILE_BUFFER_SIZE {
                let dst = self.buf_mut().as_mut_ptr();
                std::ptr::copy_nonoverlapping(write_ptr, dst, write_size);
                *self.pos_mut() = write_size;
                debug!(
                    file    = %self.filename(),
                    buf_pos = *self.pos(),
                    "PosixWritableFile::append: remaining data buffered"
                );
                return Status::ok();
            }

            // Large remaining write: bypass the buffer.
            self.write_unbuffered(write_ptr, write_size)
        }
    }
}
