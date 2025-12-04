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

#[cfg(test)]
mod random_access_file_impl_tests {
    use super::*;

    #[traced_test]
    fn random_access_read_round_trips_data() {
        crate::ix!();

        unsafe {
            // Own the FileState via Box/into_raw and use refcounting as in InMemoryEnv.
            let file_box = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file_box);

            // Simulate "map owner" reference.
            FileState::ref_raw(raw);

            // Write some data through FileState.
            {
                let file_mut: &mut FileState = &mut *raw;
                let payload = b"random access payload";
                let slice = Slice::from(&payload[..]);
                let status = file_mut.append(&slice);
                assert!(status.is_ok());
                assert_eq!(file_mut.size(), payload.len() as u64);
            }

            // Create a RandomAccessFileImpl (adds one reference).
            {
                let raf = RandomAccessFileImpl::new(raw);

                let mut result = Slice::default();
                let mut scratch = vec![0_u8; 6];

                let status = raf.read(
                    7, // offset into "random "
                    scratch.len(),
                    &mut result as *mut Slice,
                    scratch.as_mut_ptr(),
                );
                assert!(status.is_ok());
                assert_eq!(*result.size(), 6);

                let expected = &b"random access payload"[7..13];
                assert_eq!(&scratch[..6], expected);
                // raf dropped here, removing one reference.
            }

            // Last unref for the "map owner".
            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn random_access_read_on_null_file_returns_io_error() {
        crate::ix!();

        let raf = RandomAccessFileImpl { file: core::ptr::null_mut() };
        let mut result = Slice::default();
        let mut scratch = vec![0_u8; 4];

        let status = raf.read(
            0,
            scratch.len(),
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );

        assert!(status.is_io_error());
    }
}
