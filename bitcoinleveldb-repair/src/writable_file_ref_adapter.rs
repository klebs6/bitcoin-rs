// ---------------- [ File: bitcoinleveldb-repair/src/writable_file_ref_adapter.rs ]
crate::ix!();

#[derive(Debug)]
pub struct WritableFileRefAdapter {
    inner: *mut dyn WritableFile,
}

impl WritableFileRefAdapter {
    pub(crate) fn new(inner: *mut dyn WritableFile) -> Self {
        assert!(
            !inner.is_null(),
            "WritableFileRefAdapter::new: inner WritableFile pointer is null"
        );
        Self { inner }
    }
}

impl Named for WritableFileRefAdapter {
    fn name(&self) -> &'static str {
        "WritableFileRefAdapter"
    }
}

impl WritableFileAppend for WritableFileRefAdapter {
    fn append(&mut self, data: &Slice) -> crate::Status {
        unsafe { (&mut *self.inner).append(data) }
    }
}

impl WritableFileClose for WritableFileRefAdapter {
    fn close(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).close() }
    }
}

impl WritableFileFlush for WritableFileRefAdapter {
    fn flush(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).flush() }
    }
}

impl WritableFileSync for WritableFileRefAdapter {
    fn sync(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).sync() }
    }
}

impl WritableFile for WritableFileRefAdapter {}

