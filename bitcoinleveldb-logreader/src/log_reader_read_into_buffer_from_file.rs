// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_read_into_buffer_from_file.rs ]
crate::ix!();

impl LogReader {

    pub fn read_into_buffer_from_file(&mut self, block_size: usize) -> Status {
        let current_size = *self.buffer_slice().size();

        trace!(
            "LogReader::read_into_buffer_from_file: block_size={} current_buffer_size={}",
            block_size,
            current_size
        );

        // Clear the buffer and capture a raw pointer we can hand to the file implementation.
        let result_ptr: *mut Slice = {
            let buf_ref: &mut Slice = self.buffer_slice_mut();
            buf_ref.clear();
            buf_ref as *mut Slice
        };

        let scratch_ptr: *mut u8 = self.backing_store_pointer() as *mut u8;

        let status = {
            let file_ref: &mut Box<dyn SequentialFile> = self.file_mut();
            file_ref.read(block_size, result_ptr, scratch_ptr)
        };

        let new_size = *self.buffer_slice().size() as u64;
        let new_end = self
            .end_of_buffer_offset_value()
            .saturating_add(new_size);

        self.set_end_of_buffer_offset_value(new_end);

        trace!(
            "LogReader::read_into_buffer_from_file: read {} bytes, end_of_buffer_offset={}",
            new_size,
            self.end_of_buffer_offset_value()
        );

        status
    }
}

#[cfg(test)]
mod log_reader_read_into_buffer_from_file_tests {
    use super::*;

    struct BufferTestSequentialFile {
        data: Vec<u8>,
        position: usize,
        read_calls: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl BufferTestSequentialFile {
        fn new(
            data: Vec<u8>,
            read_calls: Arc<std::sync::Mutex<Vec<usize>>>,
        ) -> Self {
            BufferTestSequentialFile {
                data,
                position: 0,
                read_calls,
            }
        }
    }

    impl Named for BufferTestSequentialFile {
        fn name(&self) -> Cow<'_,str> {
            Cow::Owned("buffer_test_sequential_file".to_string())
        }
    }

    impl SequentialFileRead for BufferTestSequentialFile {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            if let Ok(mut calls) = self.read_calls.lock() {
                calls.push(n);
            }

            let remaining = self.data.len().saturating_sub(self.position);
            let to_read = core::cmp::min(n, remaining);

            unsafe {
                if to_read == 0 {
                    if !result.is_null() {
                        (*result).clear();
                    }
                } else {
                    std::ptr::copy_nonoverlapping(
                        self.data.as_ptr().add(self.position),
                        scratch,
                        to_read,
                    );
                    if !result.is_null() {
                        *result =
                            Slice::from_ptr_len(scratch as *const u8, to_read);
                    }
                }
            }

            self.position = self.position.saturating_add(to_read);

            Status::ok()
        }
    }

    impl SequentialFileSkip for BufferTestSequentialFile {
        fn skip(&mut self, _n: u64) -> Status {
            Status::ok()
        }
    }

    impl SequentialFile for BufferTestSequentialFile {}

    struct BufferTestReporter;

    impl LogReaderReporter for BufferTestReporter {
        fn corruption(&mut self, _bytes: usize, _status: &Status) {}
    }

    fn make_reader_with_bytes(
        bytes: Vec<u8>,
    ) -> (LogReader, Arc<std::sync::Mutex<Vec<usize>>>) {
        let read_calls: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let file =
            BufferTestSequentialFile::new(bytes, Arc::clone(&read_calls));
        let reporter = BufferTestReporter;

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            false,
            0,
        );

        (reader, read_calls)
    }

    #[traced_test]
    fn read_into_buffer_from_file_reads_data_and_updates_offset() {
        let payload = b"hello world".to_vec();
        let (mut reader, read_calls) = make_reader_with_bytes(payload.clone());

        assert_eq!(reader.end_of_buffer_offset_value(), 0);

        let status =
            reader.read_into_buffer_from_file(LOG_BLOCK_SIZE as usize);
        assert!(status.is_ok());

        let size_after = *reader.buffer_slice().size();
        assert_eq!(size_after as usize, payload.len());

        let end_offset = reader.end_of_buffer_offset_value();
        assert_eq!(end_offset, payload.len() as u64);

        let calls_guard = read_calls.lock().unwrap();
        assert_eq!(calls_guard.len(), 1);
        assert_eq!(calls_guard[0], LOG_BLOCK_SIZE as usize);
    }

    #[traced_test]
    fn read_into_buffer_from_file_accumulates_end_of_buffer_offset() {
        let first = b"abcd".to_vec();
        let second = b"efghijkl".to_vec();
        let mut data = first.clone();
        data.extend_from_slice(&second);

        let (mut reader, _read_calls) = make_reader_with_bytes(data);

        let status1 = reader.read_into_buffer_from_file(4);
        assert!(status1.is_ok());
        assert_eq!(reader.end_of_buffer_offset_value(), 4);

        let status2 = reader.read_into_buffer_from_file(8);
        assert!(status2.is_ok());
        assert_eq!(reader.end_of_buffer_offset_value(), 12);
    }
}
