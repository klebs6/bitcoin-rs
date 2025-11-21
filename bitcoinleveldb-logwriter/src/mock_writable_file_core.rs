// ---------------- [ File: bitcoinleveldb-logwriter/src/mock_writable_file_core.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get = "pub")]
pub struct MockWritableFileCore {
    buffer: Vec<u8>,
}

impl MockWritableFileCore {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn recorded_bytes(&self) -> &[u8] {
        &self.buffer
    }
}

impl GetName for MockWritableFileCore {
    fn get_name(&self) -> &'static str {
        "mock_writable_file_core"
    }
}

impl WritableFileAppend for MockWritableFileCore {
    fn append(&mut self, data: &Slice) -> Status {
        unsafe {
            let ptr = *data.data();
            let len = *data.size();
            trace!(
                "MockWritableFileCore::append: ptr={:?} len={}",
                ptr,
                len
            );
            let src = std::slice::from_raw_parts(ptr, len);
            self.buffer.extend_from_slice(src);
        }
        Status::ok()
    }
}

impl WritableFileFlush for MockWritableFileCore {
    fn flush(&mut self) -> Status {
        trace!("MockWritableFileCore::flush");
        Status::ok()
    }
}

impl WritableFileClose for MockWritableFileCore {
    fn close(&mut self) -> Status {
        trace!("MockWritableFileCore::close");
        Status::ok()
    }
}

impl WritableFileSync for MockWritableFileCore {
    fn sync(&mut self) -> Status {
        trace!("MockWritableFileCore::sync");
        Status::ok()
    }
}

impl WritableFile for MockWritableFileCore {}

#[cfg(test)]
mod mock_writable_file_core_tests {
    use super::*;

    #[traced_test]
    fn append_appends_bytes_to_buffer() {
        let mut file = MockWritableFileCore::new();
        let payload = "core_payload".to_string();
        let slice = Slice::from(&payload);

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(file.recorded_bytes(), payload.as_bytes());
    }
}
