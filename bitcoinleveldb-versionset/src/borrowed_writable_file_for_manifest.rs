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
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tracing::{debug, error, info, trace, warn};

    #[derive(Default, Debug)]
    struct RecordingWritableFileState {
        appended: Vec<u8>,
        append_calls: usize,
        close_calls: usize,
        flush_calls: usize,
        sync_calls: usize,
    }

    #[derive(Debug)]
    struct RecordingWritableFile {
        state: Rc<RefCell<RecordingWritableFileState>>,
        label: String,
    }

    impl RecordingWritableFile {
        fn new(state: Rc<RefCell<RecordingWritableFileState>>, label: &str) -> Self {
            trace!(label, "RecordingWritableFile::new");
            Self {
                state,
                label: label.to_string(),
            }
        }

        fn slice_bytes(data: &Slice) -> Vec<u8> {
            unsafe {
                let ptr: *const u8 = *data.data();
                let len: usize = *data.size();
                std::slice::from_raw_parts(ptr, len).to_vec()
            }
        }

    }

    impl WritableFileAppend for RecordingWritableFile {
        fn append(&mut self, data: &Slice) -> Status {
            let bytes = Self::slice_bytes(data);
            let mut st = self.state.borrow_mut();
            st.append_calls += 1;
            st.appended.extend_from_slice(&bytes);

            trace!(
                label = %self.label,
                append_calls = st.append_calls,
                bytes_len = bytes.len(),
                total_len = st.appended.len(),
                "RecordingWritableFile::append"
            );

            Status::ok()
        }
    }

    impl WritableFileClose for RecordingWritableFile {
        fn close(&mut self) -> Status {
            let mut st = self.state.borrow_mut();
            st.close_calls += 1;

            trace!(
                label = %self.label,
                close_calls = st.close_calls,
                "RecordingWritableFile::close"
            );

            Status::ok()
        }
    }

    impl WritableFileFlush for RecordingWritableFile {
        fn flush(&mut self) -> Status {
            let mut st = self.state.borrow_mut();
            st.flush_calls += 1;

            trace!(
                label = %self.label,
                flush_calls = st.flush_calls,
                "RecordingWritableFile::flush"
            );

            Status::ok()
        }
    }

    impl WritableFileSync for RecordingWritableFile {
        fn sync(&mut self) -> Status {
            let mut st = self.state.borrow_mut();
            st.sync_calls += 1;

            trace!(
                label = %self.label,
                sync_calls = st.sync_calls,
                "RecordingWritableFile::sync"
            );

            Status::ok()
        }
    }

    impl Named for RecordingWritableFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned(format!("[recording-writablefile:{}]", self.label))
        }
    }

    impl WritableFile for RecordingWritableFile {}

    #[traced_test]
    fn borrowed_writable_file_for_manifest_forwards_all_operations_and_does_not_take_ownership() {
        let state = Rc::new(RefCell::new(RecordingWritableFileState::default()));

        let inner: Box<dyn WritableFile> =
            Box::new(RecordingWritableFile::new(state.clone(), "inner"));
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
                ?st,
                "state after forwarding through BorrowedWritableFileForManifest"
            );
            assert_eq!(st.append_calls, 2, "append should be forwarded exactly twice");
            assert_eq!(st.flush_calls, 1, "flush should be forwarded exactly once");
            assert_eq!(st.sync_calls, 1, "sync should be forwarded exactly once");
            assert_eq!(st.close_calls, 1, "close should be forwarded exactly once");
            assert_eq!(
                st.appended,
                b"hello world".to_vec(),
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
            debug!(?st, "state after using inner post-drop");
            assert_eq!(
                st.appended,
                b"hello world!".to_vec(),
                "inner still functions after borrowed wrapper drop"
            );
            assert_eq!(st.append_calls, 3, "inner append should have been called again");
        }

        unsafe {
            drop(Box::<dyn WritableFile>::from_raw(inner_ptr));
        }
    }

    #[traced_test]
    fn borrowed_writable_file_for_manifest_reports_expected_name_and_exposes_inner_pointer() {
        let state = Rc::new(RefCell::new(RecordingWritableFileState::default()));
        let inner: Box<dyn WritableFile> =
            Box::new(RecordingWritableFile::new(state.clone(), "inner2"));
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
