// ---------------- [ File: bitcoinleveldb-logwriter/src/mock_writable_file_emit.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get = "pub")]
pub struct MockWritableFileEmit {
    buffer:            Vec<u8>,
    fail_append_after: Option<usize>,
    append_call_count: usize,
    flush_call_count:  usize,
    close_call_count:  usize,
    sync_call_count:   usize,
}

impl MockWritableFileEmit {
    pub fn new() -> Self {
        Self {
            buffer:            Vec::new(),
            fail_append_after: None,
            append_call_count: 0,
            flush_call_count:  0,
            close_call_count:  0,
            sync_call_count:   0,
        }
    }

    pub fn with_fail_append_after(call_index: usize) -> Self {
        Self {
            fail_append_after: Some(call_index),
            ..Self::new()
        }
    }

    pub fn recorded_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl Named for MockWritableFileEmit {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("mock_writable_file_emit".to_string())
    }
}

impl WritableFileAppend for MockWritableFileEmit {
    fn append(&mut self, data: &Slice) -> Status {
        self.append_call_count = self.append_call_count.saturating_add(1);

        if let Some(limit) = self.fail_append_after {
            if self.append_call_count == limit {
                debug!(
                    "MockWritableFileEmit::append: injecting IO error at call {}",
                    self.append_call_count
                );
                let msg = "mock append error emit".to_string();
                let msg_slice = Slice::from(&msg);
                return Status::io_error(&msg_slice, None);
            }
        }

        unsafe {
            let ptr = *data.data();
            let len = *data.size();
            trace!(
                "MockWritableFileEmit::append: ptr={:?} len={}",
                ptr,
                len
            );
            let src = std::slice::from_raw_parts(ptr, len);
            self.buffer.extend_from_slice(src);
        }

        Status::ok()
    }
}

impl WritableFileFlush for MockWritableFileEmit {
    fn flush(&mut self) -> Status {
        self.flush_call_count = self.flush_call_count.saturating_add(1);
        trace!(
            "MockWritableFileEmit::flush: flush_call_count={}",
            self.flush_call_count
        );
        Status::ok()
    }
}

impl WritableFileClose for MockWritableFileEmit {
    fn close(&mut self) -> Status {
        self.close_call_count = self.close_call_count.saturating_add(1);
        trace!(
            "MockWritableFileEmit::close: close_call_count={}",
            self.close_call_count
        );
        Status::ok()
    }
}

impl WritableFileSync for MockWritableFileEmit {
    fn sync(&mut self) -> Status {
        self.sync_call_count = self.sync_call_count.saturating_add(1);
        trace!(
            "MockWritableFileEmit::sync: sync_call_count={}",
            self.sync_call_count
        );
        Status::ok()
    }
}

impl WritableFile for MockWritableFileEmit {}

#[cfg(test)]
mod mock_writable_file_emit_tests {
    use super::*;

    #[traced_test]
    fn append_and_flush_are_tracked() {
        let mut file = MockWritableFileEmit::new();
        let payload = "emit_payload".to_string();
        let slice = Slice::from(&payload);

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(*file.append_call_count(), 1);
        assert_eq!(file.recorded_bytes(), payload.as_bytes());

        let flush_status = file.flush();
        assert!(flush_status.is_ok());
        assert_eq!(*file.flush_call_count(), 1);
    }

    #[traced_test]
    fn append_error_path_is_triggered() {
        let mut file = MockWritableFileEmit::with_fail_append_after(1);
        let payload = "emit_fail".to_string();
        let slice = Slice::from(&payload);

        let status = file.append(&slice);
        assert!(!status.is_ok());
        assert_eq!(*file.append_call_count(), 1);
        assert!(file.recorded_bytes().is_empty());
    }
}
