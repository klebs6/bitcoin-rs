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
                    let bytes_read = (*result).size();
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
