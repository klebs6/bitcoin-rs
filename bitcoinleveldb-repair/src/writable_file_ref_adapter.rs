// ---------------- [ File: bitcoinleveldb-repair/src/writable_file_ref_adapter.rs ]
crate::ix!();

#[derive(Debug)]
pub struct WritableFileRefAdapter {
    inner: *mut dyn WritableFile,
}

impl WritableFileRefAdapter {
    fn new(inner: *mut dyn WritableFile) -> Self {
        assert!(
            !inner.is_null(),
            "WritableFileRefAdapter::new: inner WritableFile pointer is null"
        );
        Self { inner }
    }
}

impl WritableFile for WritableFileRefAdapter {
    fn append(&mut self, data: &Slice) -> crate::Status {
        unsafe { (&mut *self.inner).append(data) }
    }

    fn close(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).close() }
    }

    fn flush(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).flush() }
    }

    fn sync(&mut self) -> crate::Status {
        unsafe { (&mut *self.inner).sync() }
    }
}
