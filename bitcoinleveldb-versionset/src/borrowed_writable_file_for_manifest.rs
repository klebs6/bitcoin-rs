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
