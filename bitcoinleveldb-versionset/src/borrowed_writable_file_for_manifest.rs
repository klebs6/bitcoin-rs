// ---------------- [ File: bitcoinleveldb-versionset/src/borrowed_writable_file_for_manifest.rs ]
crate::ix!();

// Create a LogWriter that borrows descriptor_file_ (does not own it).
pub struct BorrowedWritableFileForManifest {
    inner: *mut dyn WritableFile,
}

impl BorrowedWritableFileForManifest {
    pub(crate) fn new(inner: *mut dyn WritableFile) -> Self {
        trace!(
            inner_ptr = %format!("{:p}", inner),
            "BorrowedWritableFileForManifest::new"
        );
        Self { inner }
    }

    pub(crate) fn inner_ptr(&self) -> *mut dyn WritableFile {
        self.inner
    }
}

impl WritableFile for BorrowedWritableFileForManifest {}

impl WritableFileAppend for BorrowedWritableFileForManifest {
    fn append(&mut self, data: &Slice) -> Status {
        unsafe {
            (*self.inner).append(data)
        }
    }
}

impl WritableFileClose for BorrowedWritableFileForManifest {
    fn close(&mut self) -> Status {
        unsafe {
            (*self.inner).close()
        }
    }
}

impl WritableFileFlush for BorrowedWritableFileForManifest {
    fn flush(&mut self) -> Status {
        unsafe {
            (*self.inner).flush()
        }
    }
}

impl WritableFileSync for BorrowedWritableFileForManifest {
    fn sync(&mut self) -> Status {
        unsafe {
            (*self.inner).sync()
        }
    }
}

impl Named for BorrowedWritableFileForManifest {
    fn name(&self) -> Cow<'_, str> {
        Cow::Owned("[borrowed-manifest-writablefile]".to_string())
    }
}

#[cfg(test)]
mod borrowed_writable_file_for_manifest_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn borrowed_writable_file_for_manifest_forwards_all_operations_and_does_not_take_ownership() {
        let state = Rc::new(RefCell::new(WritableFileRecordingState::default()));

        let inner: Box<dyn WritableFile> =
            Box::new(WritableFileRecorder::bind_shared_state_with_label(state.clone(), "inner"));
        let inner_ptr: *mut dyn WritableFile = Box::into_raw(inner);

        trace!(inner_ptr = %format!("{:p}", inner_ptr), "constructed inner writable file");

        let mut borrowed = BorrowedWritableFileForManifest::new(inner_ptr);

        let d1 = Slice::from("hello");
        let d2 = Slice::from(" world");

        let s1 = borrowed.append(&d1);
        assert!(
            s1.is_ok(),
            "expected OK from append('hello'); got {:?}",
            s1
        );

        let s2 = borrowed.append(&d2);
        assert!(
            s2.is_ok(),
            "expected OK from append(' world'); got {:?}",
            s2
        );

        let sf = borrowed.flush();
        assert!(sf.is_ok(), "expected OK from flush; got {:?}", sf);

        let ss = borrowed.sync();
        assert!(ss.is_ok(), "expected OK from sync; got {:?}", ss);

        let sc = borrowed.close();
        assert!(sc.is_ok(), "expected OK from close; got {:?}", sc);

        {
            let st = state.borrow();
            debug!(
                append_call_count = st.append_call_count(),
                flush_call_count = st.flush_call_count(),
                sync_call_count = st.sync_call_count(),
                close_call_count = st.close_call_count(),
                appended_bytes = ?st.appended_bytes(),
                "state after forwarding through BorrowedWritableFileForManifest"
            );
            assert_eq!(st.append_call_count(), 2, "append should be forwarded exactly twice");
            assert_eq!(st.flush_call_count(), 1, "flush should be forwarded exactly once");
            assert_eq!(st.sync_call_count(), 1, "sync should be forwarded exactly once");
            assert_eq!(st.close_call_count(), 1, "close should be forwarded exactly once");
            assert_eq!(
                st.appended_bytes(),
                b"hello world".as_slice(),
                "appended bytes must be forwarded and concatenated"
            );
        }

        drop(borrowed);
        info!("dropped BorrowedWritableFileForManifest; inner must still be alive");

        // Prove the inner was not dropped by exercising it again.
        unsafe {
            let s3 = (*inner_ptr).append(&Slice::from("!"));
            assert!(s3.is_ok(), "inner append after borrowed drop must still work");
        }

        {
            let st = state.borrow();
            debug!(
                append_call_count = st.append_call_count(),
                appended_bytes = ?st.appended_bytes(),
                "state after using inner post-drop"
            );
            assert_eq!(
                st.appended_bytes(),
                b"hello world!".as_slice(),
                "inner still functions after borrowed wrapper drop"
            );
            assert_eq!(st.append_call_count(), 3, "inner append should have been called again");
        }

        unsafe {
            drop(Box::<dyn WritableFile>::from_raw(inner_ptr));
        }
    }

    #[traced_test]
    fn borrowed_writable_file_for_manifest_reports_expected_name_and_exposes_inner_pointer() {
        let state = Rc::new(RefCell::new(WritableFileRecordingState::default()));
        let inner: Box<dyn WritableFile> =
            Box::new(WritableFileRecorder::bind_shared_state_with_label(state.clone(), "inner2"));
        let inner_ptr: *mut dyn WritableFile = Box::into_raw(inner);

        let borrowed = BorrowedWritableFileForManifest::new(inner_ptr);

        let name = borrowed.name().to_string();
        debug!(name = %name, "borrowed name");
        assert_eq!(
            name.as_str(),
            "[borrowed-manifest-writablefile]",
            "BorrowedWritableFileForManifest::name must be stable and well-known"
        );

        let exposed = borrowed.inner_ptr();
        debug!(
            exposed_ptr = %format!("{:p}", exposed),
            expected_ptr = %format!("{:p}", inner_ptr),
            "inner pointer exposure"
        );
        assert_eq!(
            exposed as *mut (),
            inner_ptr as *mut (),
            "BorrowedWritableFileForManifest::inner_ptr must return the exact inner pointer"
        );

        unsafe {
            drop(Box::<dyn WritableFile>::from_raw(inner_ptr));
        }
        drop(borrowed);
    }
}
