// ---------------- [ File: bitcoinleveldb-memenv/src/sequential_file.rs ]
crate::ix!();

pub struct SequentialFileImpl {
    file: *mut FileState,
    pos:  u64,
}

impl SequentialFile for SequentialFileImpl { }

impl SequentialFileRead for SequentialFileImpl {

    fn read(
        &mut self,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {
        trace!(
            "SequentialFileImpl::read: pos={}, n={}, file_ptr={:?}",
            self.pos,
            n,
            self.file
        );

        unsafe {
            if self.file.is_null() {
                error!(
                    "SequentialFileImpl::read: underlying FileState pointer is null"
                );
                let msg = Slice::from(
                    "SequentialFileImpl has no backing FileState".as_bytes(),
                );
                return crate::Status::io_error(&msg, None);
            }

            let file_ref: &FileState = &*self.file;
            let status = file_ref.read(self.pos, n, result, scratch);

            if status.is_ok() {
                if !result.is_null() {
                    let bytes_read: usize = *(*result).size();
                    self.pos = self.pos.saturating_add(bytes_read as u64);
                    trace!(
                        "SequentialFileImpl::read: advanced pos to {} (read {} bytes)",
                        self.pos,
                        bytes_read
                    );
                } else {
                    warn!(
                        "SequentialFileImpl::read: result pointer is null on successful read"
                    );
                }
            } else {
                debug!(
                    "SequentialFileImpl::read: underlying FileState::read returned error: {:?}",
                    status
                );
            }

            status
        }
    }
}

impl SequentialFileSkip for SequentialFileImpl {

    fn skip(&mut self, mut n: u64) -> crate::Status {
        trace!(
            "SequentialFileImpl::skip: current pos={}, requested skip={}",
            self.pos,
            n
        );

        unsafe {
            if self.file.is_null() {
                error!(
                    "SequentialFileImpl::skip: underlying FileState pointer is null"
                );
                let msg = Slice::from(
                    "SequentialFileImpl has no backing FileState".as_bytes(),
                );
                return crate::Status::io_error(&msg, None);
            }

            let file_ref: &FileState = &*self.file;
            let size = file_ref.size();

            if self.pos > size {
                error!(
                    "SequentialFileImpl::skip: pos {} > file size {}",
                    self.pos,
                    size
                );
                let msg =
                    Slice::from("pos_ > file_->Size()".as_bytes());
                return crate::Status::io_error(&msg, None);
            }

            let available = size - self.pos;
            if n > available {
                n = available;
            }

            self.pos = self.pos.saturating_add(n);
            trace!(
                "SequentialFileImpl::skip: advanced pos to {}",
                self.pos
            );

            crate::Status::ok()
        }
    }
}

impl Drop for SequentialFileImpl {

    fn drop(&mut self) {
        trace!(
            "SequentialFileImpl::drop: dropping sequential handle for file_ptr={:?}",
            self.file
        );

        unsafe {
            if !self.file.is_null() {
                FileState::unref_raw(self.file);
                self.file = std::ptr::null_mut();
            } else {
                debug!("SequentialFileImpl::drop: file_ptr already null");
            }
        }
    }
}

impl Named for SequentialFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}

impl SequentialFileImpl {

    pub fn new(file: *mut FileState) -> Self {
        trace!(
            "SequentialFileImpl::new: constructing for file_ptr={:?}, initial pos=0",
            file
        );

        unsafe {
            if !file.is_null() {
                FileState::ref_raw(file);
            } else {
                warn!(
                    "SequentialFileImpl::new: constructed with null FileState pointer"
                );
            }
        }

        SequentialFileImpl { file, pos: 0 }
    }
}

#[cfg(test)]
mod sequential_file_impl_tests {
    use super::*;

    #[traced_test]
    fn sequential_read_advances_position() {
        crate::ix!();

        unsafe {
            let file_box = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file_box);

            FileState::ref_raw(raw);

            {
                let file_mut: &mut FileState = &mut *raw;
                let payload = b"abcdefg";
                let slice = Slice::from(&payload[..]);
                let status = file_mut.append(&slice);
                assert!(status.is_ok());
            }

            {
                let mut seq = SequentialFileImpl::new(raw);

                // First read 3 bytes.
                let mut result1 = Slice::default();
                let mut scratch1 = vec![0_u8; 3];
                let status1 = seq.read(
                    3,
                    &mut result1 as *mut Slice,
                    scratch1.as_mut_ptr(),
                );
                assert!(status1.is_ok());
                assert_eq!(*result1.size(), 3);
                assert_eq!(&scratch1[..3], &b"abc"[..]);

                // Second read 4 bytes: should yield the remainder.
                let mut result2 = Slice::default();
                let mut scratch2 = vec![0_u8; 4];
                let status2 = seq.read(
                    4,
                    &mut result2 as *mut Slice,
                    scratch2.as_mut_ptr(),
                );
                assert!(status2.is_ok());
                assert_eq!(*result2.size(), 4);
                assert_eq!(&scratch2[..4], &b"defg"[..]);
            }

            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn sequential_skip_moves_forward_without_reading() {
        crate::ix!();

        unsafe {
            let file_box = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file_box);

            FileState::ref_raw(raw);

            {
                let file_mut: &mut FileState = &mut *raw;
                let payload = b"0123456789";
                let slice = Slice::from(&payload[..]);
                let status = file_mut.append(&slice);
                assert!(status.is_ok());
            }

            {
                let mut seq = SequentialFileImpl::new(raw);

                // Skip first 5 bytes.
                let status_skip = seq.skip(5);
                assert!(status_skip.is_ok());

                // Read the next 3 bytes; should be "567".
                let mut result = Slice::default();
                let mut scratch = vec![0_u8; 3];
                let status_read = seq.read(
                    3,
                    &mut result as *mut Slice,
                    scratch.as_mut_ptr(),
                );
                assert!(status_read.is_ok());
                assert_eq!(&scratch[..3], &b"567"[..]);
            }

            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn sequential_operations_on_null_file_return_io_error() {
        crate::ix!();

        let mut seq = SequentialFileImpl {
            file: core::ptr::null_mut(),
            pos:  0,
        };

        let mut result = Slice::default();
        let mut scratch = vec![0_u8; 4];

        let status_read = seq.read(
            4,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status_read.is_io_error());

        let status_skip = seq.skip(10);
        assert!(status_skip.is_io_error());
    }
}
