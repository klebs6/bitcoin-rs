// ---------------- [ File: bitcoinleveldb-memenv/src/writable_file.rs ]
crate::ix!();

pub struct WritableFileImpl {
    file: *mut FileState,
}

impl WritableFile for WritableFileImpl {}

impl Drop for WritableFileImpl {

    fn drop(&mut self) {
        trace!(
            "WritableFileImpl::drop: dropping writable handle for file_ptr={:?}",
            self.file
        );

        unsafe {
            if !self.file.is_null() {
                FileState::unref_raw(self.file);
                self.file = std::ptr::null_mut();
            } else {
                debug!("WritableFileImpl::drop: file_ptr already null");
            }
        }
    }
}

impl Named for WritableFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}

impl WritableFileImpl {

    pub fn new(file: *mut FileState) -> Self {
        trace!(
            "WritableFileImpl::new: constructing for file_ptr={:?}",
            file
        );

        unsafe {
            if !file.is_null() {
                FileState::ref_raw(file);
            } else {
                warn!(
                    "WritableFileImpl::new: constructed with null FileState pointer"
                );
            }
        }

        WritableFileImpl { file }
    }
}

impl WritableFileAppend for WritableFileImpl {
    fn append(&mut self, data: &Slice) -> crate::Status {
        trace!(
            "WritableFileImpl::append: appending {} bytes",
            data.size()
        );

        unsafe {
            if self.file.is_null() {
                error!(
                    "WritableFileImpl::append: underlying FileState pointer is null"
                );
                let msg = Slice::from(
                    "WritableFileImpl has no backing FileState".as_bytes(),
                );
                return crate::Status::io_error(&msg, None);
            }

            let file_ref: &mut FileState = &mut *self.file;
            file_ref.append(data)
        }
    }
}

impl WritableFileClose for WritableFileImpl {

    fn close(&mut self) -> crate::Status {
        trace!("WritableFileImpl::close: no-op close for in‑memory file");
        crate::Status::ok()
    }
}

impl WritableFileFlush for WritableFileImpl {

    fn flush(&mut self) -> crate::Status {
        trace!("WritableFileImpl::flush: no-op flush for in‑memory file");
        crate::Status::ok()
    }
}

impl WritableFileSync for WritableFileImpl {
    fn sync(&mut self) -> crate::Status {
        trace!("WritableFileImpl::sync: no-op sync for in‑memory file");
        crate::Status::ok()
    }
}
