// ---------------- [ File: bitcoinleveldb-memenv/src/random_access_file.rs ]
crate::ix!();

pub struct RandomAccessFileImpl {
    file: *mut FileState,
}

impl RandomAccessFile for RandomAccessFileImpl { }

impl RandomAccessFileRead for RandomAccessFileImpl {

    fn read(
        &self,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> crate::Status {
        trace!(
            "RandomAccessFileImpl::read: offset={}, n={}, file_ptr={:?}",
            offset,
            n,
            self.file
        );

        unsafe {
            if self.file.is_null() {
                error!(
                    "RandomAccessFileImpl::read: underlying FileState pointer is null"
                );
                let msg = Slice::from(
                    "RandomAccessFileImpl has no backing FileState".as_bytes(),
                );
                return crate::Status::io_error(&msg, None);
            }

            let file_ref: &FileState = &*self.file;
            file_ref.read(offset, n, result, scratch)
        }
    }
}

impl Drop for RandomAccessFileImpl {
    fn drop(&mut self) {
        trace!(
            "RandomAccessFileImpl::drop: dropping random-access handle for file_ptr={:?}",
            self.file
        );

        unsafe {
            if !self.file.is_null() {
                FileState::unref_raw(self.file);
                self.file = std::ptr::null_mut();
            } else {
                debug!("RandomAccessFileImpl::drop: file_ptr already null");
            }
        }
    }
}

impl Named for RandomAccessFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}
 
impl RandomAccessFileImpl {

    pub fn new(file: *mut FileState) -> Self {
        trace!(
            "RandomAccessFileImpl::new: constructing for file_ptr={:?}",
            file
        );

        unsafe {
            if !file.is_null() {
                FileState::ref_raw(file);
            } else {
                warn!(
                    "RandomAccessFileImpl::new: constructed with null FileState pointer"
                );
            }
        }

        RandomAccessFileImpl { file }
    }
}
