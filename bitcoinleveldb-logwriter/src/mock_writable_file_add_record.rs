// ---------------- [ File: bitcoinleveldb-logwriter/src/mock_writable_file_add_record.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get = "pub")]
pub struct MockWritableFileAddRecord {
    buffer:            Vec<u8>,
    fail_append_after: Option<usize>,
    append_call_count: usize,
    flush_call_count:  usize,
    close_call_count:  usize,
    sync_call_count:   usize,
}

impl MockWritableFileAddRecord {
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

impl Named for MockWritableFileAddRecord {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("mock_writable_file_add_record".to_string())
    }
}

impl WritableFileAppend for MockWritableFileAddRecord {
    fn append(&mut self, data: &Slice) -> Status {
        self.append_call_count = self.append_call_count.saturating_add(1);

        if let Some(limit) = self.fail_append_after {
            if self.append_call_count == limit {
                debug!(
                    "MockWritableFileAddRecord::append: injecting IO error at call {}",
                    self.append_call_count
                );
                let msg = "mock append error".to_string();
                let msg_slice = Slice::from(&msg);
                return Status::io_error(&msg_slice, None);
            }
        }

        unsafe {
            // Slice exposes pointer-to-pointer semantics; we must dereference
            // once to obtain the underlying payload pointer and length.
            let ptr = *data.data();
            let len = *data.size();
            trace!(
                "MockWritableFileAddRecord::append: ptr={:?} len={}",
                ptr,
                len
            );
            let src = std::slice::from_raw_parts(ptr, len);
            self.buffer.extend_from_slice(src);
        }

        Status::ok()
    }
}

impl WritableFileFlush for MockWritableFileAddRecord {
    fn flush(&mut self) -> Status {
        self.flush_call_count = self.flush_call_count.saturating_add(1);
        trace!(
            "MockWritableFileAddRecord::flush: flush_call_count={}",
            self.flush_call_count
        );
        Status::ok()
    }
}

impl WritableFileClose for MockWritableFileAddRecord {
    fn close(&mut self) -> Status {
        self.close_call_count = self.close_call_count.saturating_add(1);
        trace!(
            "MockWritableFileAddRecord::close: close_call_count={}",
            self.close_call_count
        );
        Status::ok()
    }
}

impl WritableFileSync for MockWritableFileAddRecord {
    fn sync(&mut self) -> Status {
        self.sync_call_count = self.sync_call_count.saturating_add(1);
        trace!(
            "MockWritableFileAddRecord::sync: sync_call_count={}",
            self.sync_call_count
        );
        Status::ok()
    }
}

impl WritableFile for MockWritableFileAddRecord {}

#[cfg(test)]
mod mock_writable_file_add_record_tests {
    use super::*;

    #[traced_test]
    fn append_records_payload_bytes_in_order() {
        let mut file = MockWritableFileAddRecord::new();
        let payload = "abc123".to_string();
        let slice = Slice::from(&payload);

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(*file.append_call_count(), 1);
        assert_eq!(file.recorded_bytes(), payload.as_bytes());
    }

    #[traced_test]
    fn append_respects_fail_append_after_and_does_not_modify_buffer() {
        let mut file = MockWritableFileAddRecord::with_fail_append_after(1);
        let payload = "should_fail".to_string();
        let slice = Slice::from(&payload);

        let status = file.append(&slice);
        assert!(!status.is_ok());
        assert_eq!(*file.append_call_count(), 1);
        assert!(file.recorded_bytes().is_empty());
    }
}
