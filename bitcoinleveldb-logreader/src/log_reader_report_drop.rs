// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_report_drop.rs ]
crate::ix!();

impl LogReader {

    pub fn report_drop(&mut self, bytes: u64, reason: &Status) {
        let buffer_size = (*self.buffer_slice().size()) as u64;

        let current_offset = self
            .end_of_buffer_offset_value()
            .saturating_sub(buffer_size)
            .saturating_sub(bytes);

        trace!(
            "LogReader::report_drop: bytes={} end_of_buffer_offset={} buffer_size={} current_offset={} initial_offset={}",
            bytes,
            self.end_of_buffer_offset_value(),
            buffer_size,
            current_offset,
            self.initial_log_offset()
        );

        if current_offset >= self.initial_log_offset() {
            self.emit_corruption_to_reporter(bytes as usize, reason);
        }
    }
}

#[cfg(test)]
mod log_reader_report_drop_tests {
    use super::*;

    struct DropSequentialFile;

    impl bitcoin_support::GetName for DropSequentialFile {
        fn get_name(&self) -> &'static str {
            "drop_sequential_file"
        }
    }

    impl SequentialFileRead for DropSequentialFile {
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

    impl SequentialFileSkip for DropSequentialFile {
        fn skip(&mut self, _n: u64) -> Status {
            Status::ok()
        }
    }

    impl SequentialFile for DropSequentialFile {}

    struct DropReporter {
        events: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl DropReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            DropReporter { events }
        }
    }

    impl LogReaderReporter for DropReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(bytes);
            }
        }
    }

    fn make_reader_for_drop() -> (LogReader, Arc<std::sync::Mutex<Vec<usize>>>) {
        let events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let file = DropSequentialFile;
        let reporter = DropReporter::new(Arc::clone(&events));
        let mut reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            false,
            0,
        );

        reader.set_end_of_buffer_offset_value(100);
        reader.buffer_slice_mut().clear();

        (reader, events)
    }

    #[traced_test]
    fn report_drop_emits_corruption_when_offset_after_initial() {
        let (mut reader, events) = make_reader_for_drop();

        reader.report_drop(10, &Status::ok());

        let events_guard = events.lock().unwrap();
        assert_eq!(events_guard.as_slice(), &[10usize]);
    }

    #[traced_test]
    fn report_drop_suppresses_corruption_when_before_initial_offset() {
        let (mut reader, events) = make_reader_for_drop();

        reader.set_initial_offset(150);

        reader.report_drop(10, &Status::ok());

        let events_guard = events.lock().unwrap();
        assert!(events_guard.is_empty());
    }
}
