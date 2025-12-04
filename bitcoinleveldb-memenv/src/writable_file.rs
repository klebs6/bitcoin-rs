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
            *data.size()
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

#[cfg(test)]
mod writable_file_impl_tests {
    use super::*;

    #[traced_test]
    fn writable_append_writes_to_underlying_file_state() {
        crate::ix!();

        unsafe {
            let file_box = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file_box);

            // Simulate owner reference.
            FileState::ref_raw(raw);

            {
                let mut writable = WritableFileImpl::new(raw);

                let payload = b"hello writable";
                let slice = Slice::from(&payload[..]);
                let status = writable.append(&slice);
                assert!(status.is_ok());

                let status_flush = writable.flush();
                assert!(status_flush.is_ok());
                let status_sync = writable.sync();
                assert!(status_sync.is_ok());
                let status_close = writable.close();
                assert!(status_close.is_ok());
            }

            {
                let file_ref: &FileState = &*raw;
                assert_eq!(file_ref.size(), b"hello writable".len() as u64);

                let mut result = Slice::default();
                let mut scratch = vec![0_u8; b"hello writable".len()];
                let status = file_ref.read(
                    0,
                    scratch.len(),
                    &mut result as *mut Slice,
                    scratch.as_mut_ptr(),
                );
                assert!(status.is_ok());
                assert_eq!(&scratch[..], &b"hello writable"[..]);
            }

            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn writable_on_null_file_returns_io_error() {
        crate::ix!();

        let mut writable = WritableFileImpl {
            file: core::ptr::null_mut(),
        };

        let payload = b"data";
        let slice = Slice::from(&payload[..]);
        let status_append = writable.append(&slice);
        assert!(status_append.is_io_error());
    }
}
