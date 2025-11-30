// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_create.rs ]
crate::ix!();

impl LogReader {

    /// Create a reader that will return log records from "*file".  "*file" must
    /// remain live while this Reader is in use.
    /// 
    /// If "reporter" is non-null, it is notified whenever some data is dropped
    /// due to a detected corruption.  "*reporter" must remain live while this
    /// Reader is in use.
    /// 
    /// If "checksum" is true, verify checksums if available.
    /// 
    /// The Reader will start reading at the first record located at physical
    /// position >= initial_offset within the file.
    ///
    pub fn new(
        file: Box<dyn SequentialFile>,
        reporter: Box<dyn LogReaderReporter>,
        checksum: bool,
        initial_offset: u64,
    ) -> Self {
        info!(
            "LogReader::new: checksum={} initial_offset={}",
            checksum,
            initial_offset
        );

        let backing_box: Box<[u8; LOG_BLOCK_SIZE as usize]> =
            Box::new([0u8; LOG_BLOCK_SIZE as usize]);
        let backing_store = Box::into_raw(backing_box) as *const u8;

        LogReaderBuilder::default()
            .file(file)
            .reporter(reporter)
            .checksum(checksum)
            .backing_store(backing_store)
            .buffer(Slice::default())
            .eof(false)
            .last_record_offset(0u64)
            .end_of_buffer_offset(0u64)
            .initial_offset(initial_offset)
            .resyncing(initial_offset > 0)
            .build()
            .unwrap()
    }
}

#[cfg(test)]
mod log_reader_create_tests {
    use super::*;

    struct CreateTestSequentialFile;

    impl Named for CreateTestSequentialFile {
        fn name(&self) -> Cow<'_,str> {
            Cow::Owned("create_test_sequential_file".to_string())
        }
    }

    impl SequentialFileRead for CreateTestSequentialFile {
        fn read(
            &mut self,
            _n: usize,
            result: *mut Slice,
            _scratch: *mut u8,
        ) -> Status {
            unsafe {
                if !result.is_null() {
                    (*result).clear();
                }
            }
            Status::ok()
        }
    }

    impl SequentialFileSkip for CreateTestSequentialFile {
        fn skip(&mut self, _n: u64) -> Status {
            Status::ok()
        }
    }

    impl SequentialFile for CreateTestSequentialFile {}

    struct CreateTestReporter {
        calls: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl CreateTestReporter {
        fn new(calls: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            CreateTestReporter { calls }
        }
    }

    impl LogReaderReporter for CreateTestReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.calls.lock() {
                guard.push(bytes);
            }
        }
    }

    #[traced_test]
    fn log_reader_new_initializes_fields_correctly() {
        let reporter_calls: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        let file = CreateTestSequentialFile;
        let reporter = CreateTestReporter::new(Arc::clone(&reporter_calls));

        let initial_offset = 128u64;
        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            true,
            initial_offset,
        );

        assert_eq!(reader.initial_log_offset(), initial_offset);
        assert!(reader.checksum_enabled());
        assert!(reader.is_resyncing());
        assert!(!reader.eof_flag());
        assert_eq!(reader.end_of_buffer_offset_value(), 0);

        assert!(!reader.backing_store_pointer().is_null());

        let guard = reporter_calls.lock().unwrap();
        assert!(guard.is_empty());
    }
}
