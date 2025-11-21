// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_reader.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_reader.cc]

#[derive(Builder,Setters,Getters,MutGetters)]
#[getset(set = "pub", get = "pub", get_mut = "pub")]
#[builder(pattern = "owned", setter(into))]
pub struct LogReader {

    file:                 Box<dyn SequentialFile>,
    reporter:             Box<dyn LogReaderReporter>,
    checksum:             bool,
    backing_store:        *const u8,
    buffer:               Slice,

    /// Last Read() indicated EOF by returning < kBlockSize
    ///
    eof:                  bool,

    /// Offset of the last record returned by ReadRecord.
    ///
    last_record_offset:   u64,

    /// Offset of the first location past the end of buffer_.
    ///
    end_of_buffer_offset: u64,

    /// Offset at which to start looking for the first record to return
    /// 
    initial_offset:       u64,

    /// True if we are resynchronizing after a seek (initial_offset_ > 0). 
    ///
    /// In particular, a run of kMiddleType and kLastType records can be
    /// silently skipped in this mode
    /// 
    resyncing:            bool,
}

impl LogReader {
    
    pub fn buffer_slice(&self) -> &Slice {
        &self.buffer
    }

    pub fn buffer_slice_mut(&mut self) -> &mut Slice {
        &mut self.buffer
    }

    pub fn eof_flag(&self) -> bool {
        self.eof
    }

    pub fn set_eof_flag(&mut self, eof: bool) {
        debug!("LogReader::set_eof_flag: eof={}", eof);
        self.eof = eof;
    }

    pub fn end_of_buffer_offset_value(&self) -> u64 {
        self.end_of_buffer_offset
    }

    pub fn set_end_of_buffer_offset_value(&mut self, offset: u64) {
        debug!(
            "LogReader::set_end_of_buffer_offset_value: from {} to {}",
            self.end_of_buffer_offset,
            offset
        );
        self.end_of_buffer_offset = offset;
    }

    pub fn initial_log_offset(&self) -> u64 {
        self.initial_offset
    }

    pub fn is_resyncing(&self) -> bool {
        self.resyncing
    }

    pub fn set_resyncing_flag(&mut self, resyncing: bool) {
        debug!(
            "LogReader::set_resyncing_flag: resyncing={}",
            resyncing
        );
        self.resyncing = resyncing;
    }

    pub fn checksum_enabled(&self) -> bool {
        self.checksum
    }

    pub fn backing_store_pointer(&self) -> *const u8 {
        self.backing_store
    }

    pub fn skip_file_bytes(&mut self, n: u64) -> Status {
        trace!("LogReader::skip_file_bytes: skipping {} bytes", n);
        self.file.skip(n)
    }    

    pub fn emit_corruption_to_reporter(&mut self, bytes: usize, reason: &Status) {
        trace!(
            "LogReader::emit_corruption_to_reporter: bytes={} reason={:?}",
            bytes,
            reason
        );
        self.reporter.corruption(bytes, reason);
    }
}

impl Drop for LogReader {

    fn drop(&mut self) {
        trace!("LogReader::drop: releasing backing_store buffer");
        if !self.backing_store.is_null() {
            unsafe {
                let _ =
                    Box::from_raw(self.backing_store as *mut [u8; LOG_BLOCK_SIZE as usize]);
            }
            self.backing_store = std::ptr::null();
        }
    }
}

#[cfg(test)]
mod log_reader_core_tests {
    use super::*;

    struct CoreTestSequentialFile {
        data: Vec<u8>,
        position: u64,
        skip_calls: Arc<std::sync::Mutex<Vec<u64>>>,
    }

    impl CoreTestSequentialFile {
        fn new(
            data: Vec<u8>,
            skip_calls: Arc<std::sync::Mutex<Vec<u64>>>,
        ) -> Self {
            CoreTestSequentialFile {
                data,
                position: 0,
                skip_calls,
            }
        }
    }

    impl bitcoin_support::GetName for CoreTestSequentialFile {
        fn get_name(&self) -> &'static str {
            "core_test_sequential_file"
        }
    }

    impl SequentialFileRead for CoreTestSequentialFile {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            let remaining = self
                .data
                .len()
                .saturating_sub(self.position as usize);
            let to_read = core::cmp::min(n, remaining);

            unsafe {
                if to_read == 0 {
                    if !result.is_null() {
                        (*result).clear();
                    }
                } else {
                    std::ptr::copy_nonoverlapping(
                        self.data.as_ptr().add(self.position as usize),
                        scratch,
                        to_read,
                    );
                    if !result.is_null() {
                        *result =
                            Slice::from_ptr_len(scratch as *const u8, to_read);
                    }
                }
            }

            self.position = self.position.saturating_add(to_read as u64);
            Status::ok()
        }
    }

    impl SequentialFileSkip for CoreTestSequentialFile {
        fn skip(&mut self, n: u64) -> Status {
            self.position = self
                .position
                .saturating_add(n)
                .min(self.data.len() as u64);

            if let Ok(mut guard) = self.skip_calls.lock() {
                guard.push(n);
            }

            Status::ok()
        }
    }

    impl SequentialFile for CoreTestSequentialFile {}

    struct CoreTestReporter {
        events: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl CoreTestReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            CoreTestReporter { events }
        }
    }

    impl LogReaderReporter for CoreTestReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(bytes);
            }
        }
    }

    fn make_core_reader(
    ) -> (
        LogReader,
        Arc<std::sync::Mutex<Vec<u64>>>,
        Arc<std::sync::Mutex<Vec<usize>>>,
    ) {
        let skip_calls: Arc<std::sync::Mutex<Vec<u64>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let reporter_events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        let file =
            CoreTestSequentialFile::new(Vec::new(), Arc::clone(&skip_calls));
        let reporter =
            CoreTestReporter::new(Arc::clone(&reporter_events));

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            true,
            0,
        );

        (reader, skip_calls, reporter_events)
    }

    #[traced_test]
    fn log_reader_flags_and_offsets_round_trip() {
        let (mut reader, _, _) = make_core_reader();

        assert!(!reader.eof_flag());
        reader.set_eof_flag(true);
        assert!(reader.eof_flag());

        assert_eq!(reader.end_of_buffer_offset_value(), 0);
        reader.set_end_of_buffer_offset_value(42);
        assert_eq!(reader.end_of_buffer_offset_value(), 42);

        assert_eq!(reader.initial_log_offset(), 0);
        assert!(reader.checksum_enabled());

        assert!(!reader.is_resyncing());
        reader.set_resyncing_flag(true);
        assert!(reader.is_resyncing());
        reader.set_resyncing_flag(false);
        assert!(!reader.is_resyncing());

        let backing_ptr = reader.backing_store_pointer();
        assert!(!backing_ptr.is_null());
    }

    #[traced_test]
    fn log_reader_skip_file_bytes_delegates_to_sequential_file() {
        let data = vec![1u8, 2, 3, 4, 5];
        let skip_calls: Arc<std::sync::Mutex<Vec<u64>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let reporter_events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        let file =
            CoreTestSequentialFile::new(data, Arc::clone(&skip_calls));
        let reporter =
            CoreTestReporter::new(Arc::clone(&reporter_events));

        let mut reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            false,
            0,
        );

        let status = reader.skip_file_bytes(3);
        assert!(status.is_ok());

        let recorded_guard = skip_calls.lock().unwrap();
        assert_eq!(recorded_guard.as_slice(), &[3u64]);

        let events_guard = reporter_events.lock().unwrap();
        assert!(events_guard.is_empty());
    }

    #[traced_test]
    fn log_reader_emit_corruption_calls_reporter() {
        let (mut reader, _skip_calls, reporter_events) = make_core_reader();

        let reason_slice = Slice::default();
        let status = Status::corruption(&reason_slice, None);

        reader.emit_corruption_to_reporter(17, &status);

        let events_guard = reporter_events.lock().unwrap();
        assert_eq!(events_guard.as_slice(), &[17usize]);
    }
}

