// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_skip_to_initial_block.rs ]
crate::ix!();

impl LogReader {

    /// Skips all blocks that are completely before "initial_offset_".
    ///
    /// Returns true on success. Handles reporting.
    ///
    pub fn skip_to_initial_block(&mut self) -> bool {
        let block_size        = LOG_BLOCK_SIZE as u64;
        let offset_in_block   = self.initial_log_offset() % block_size;
        let mut block_start_location =
            self.initial_log_offset() - offset_in_block;

        // Don't search a block if we'd be in the trailer
        if offset_in_block > block_size - 6 {
            block_start_location = block_start_location.saturating_add(block_size);
        }

        self.set_end_of_buffer_offset_value(block_start_location);

        trace!(
            "LogReader::skip_to_initial_block: initial_offset={} offset_in_block={} block_start_location={}",
            self.initial_log_offset(),
            offset_in_block,
            block_start_location
        );

        // Skip to start of first block that can contain the initial record
        if block_start_location > 0 {
            let status = self.skip_file_bytes(block_start_location);
            if !status.is_ok() {
                warn!(
                    "LogReader::skip_to_initial_block: skip({}) failed with status={:?}",
                    block_start_location,
                    status
                );

                // We attempted to position the underlying file at the first block
                // that can contain the initial record and failed. Treat the bytes
                // we tried to skip as dropped and report the corruption directly
                // to the reporter, bypassing `report_drop`'s initial_offset gating.
                self.emit_corruption_to_reporter(
                    block_start_location as usize,
                    &status,
                );

                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod log_reader_skip_to_initial_block_tests {
    use super::*;

    struct SkipSequentialFile {
        skip_calls: Arc<std::sync::Mutex<Vec<u64>>>,
        fail_skip: bool,
    }

    impl SkipSequentialFile {
        fn new(
            skip_calls: Arc<std::sync::Mutex<Vec<u64>>>,
            fail_skip: bool,
        ) -> Self {
            SkipSequentialFile { skip_calls, fail_skip }
        }
    }

    impl bitcoin_support::GetName for SkipSequentialFile {
        fn get_name(&self) -> &'static str {
            "skip_sequential_file"
        }
    }

    impl SequentialFileRead for SkipSequentialFile {
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

    impl SequentialFileSkip for SkipSequentialFile {
        fn skip(&mut self, n: u64) -> Status {
            if let Ok(mut guard) = self.skip_calls.lock() {
                guard.push(n);
            }
            if self.fail_skip {
                let reason_slice = Slice::default();
                Status::corruption(&reason_slice, None)
            } else {
                Status::ok()
            }
        }
    }

    impl SequentialFile for SkipSequentialFile {}

    struct SkipReporter {
        events: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl SkipReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            SkipReporter { events }
        }
    }

    impl LogReaderReporter for SkipReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(bytes);
            }
        }
    }

    fn make_reader_with_initial_offset(
        initial_offset: u64,
        fail_skip: bool,
    ) -> (
        LogReader,
        Arc<std::sync::Mutex<Vec<u64>>>,
        Arc<std::sync::Mutex<Vec<usize>>>,
    ) {
        let skip_calls: Arc<std::sync::Mutex<Vec<u64>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let reporter_events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));

        let file = SkipSequentialFile::new(Arc::clone(&skip_calls), fail_skip);
        let reporter =
            SkipReporter::new(Arc::clone(&reporter_events));

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            false,
            initial_offset,
        );

        (reader, skip_calls, reporter_events)
    }

    #[traced_test]
    fn skip_to_initial_block_with_zero_offset_does_not_call_skip() {
        let (mut reader, skip_calls, events) =
            make_reader_with_initial_offset(0, false);

        let ok = reader.skip_to_initial_block();
        assert!(ok);

        let skip_guard = skip_calls.lock().unwrap();
        assert!(skip_guard.is_empty());

        let events_guard = events.lock().unwrap();
        assert!(events_guard.is_empty());
        assert_eq!(reader.end_of_buffer_offset_value(), 0);
    }

    #[traced_test]
    fn skip_to_initial_block_skips_full_blocks_and_sets_offset() {
        let block_size = LOG_BLOCK_SIZE as u64;
        let initial_offset = block_size + 10;

        let (mut reader, skip_calls, events) =
            make_reader_with_initial_offset(initial_offset, false);

        let ok = reader.skip_to_initial_block();
        assert!(ok);

        let calls_guard = skip_calls.lock().unwrap();
        assert_eq!(calls_guard.as_slice(), &[block_size]);

        let events_guard = events.lock().unwrap();
        assert!(events_guard.is_empty());
        assert_eq!(reader.end_of_buffer_offset_value(), block_size);
    }

    #[traced_test]
    fn skip_to_initial_block_reports_drop_on_skip_error() {
        let block_size = LOG_BLOCK_SIZE as u64;
        let initial_offset = block_size;

        let (mut reader, skip_calls, events) =
            make_reader_with_initial_offset(initial_offset, true);

        let ok = reader.skip_to_initial_block();
        assert!(!ok);

        let calls_guard = skip_calls.lock().unwrap();
        assert_eq!(calls_guard.as_slice(), &[block_size]);

        let events_guard = events.lock().unwrap();
        assert_eq!(events_guard.as_slice(), &[block_size as usize]);
    }
}
