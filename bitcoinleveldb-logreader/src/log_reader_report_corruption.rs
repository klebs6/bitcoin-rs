// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_report_corruption.rs ]
crate::ix!();

impl LogReader {

    /// Reports dropped bytes to the reporter. buffer_ must be updated to remove
    /// the dropped bytes prior to invocation.
    /// 
    pub fn report_corruption(&mut self, bytes: u64, reason: *const u8) {
        unsafe {
            let reason_str = if reason.is_null() {
                "<unknown corruption>"
            } else {
                match std::ffi::CStr::from_ptr(reason as *const libc::c_char)
                    .to_str()
                {
                    Ok(s) => s,
                    Err(_) => "<non-utf8 corruption reason>",
                }
            };

            warn!(
                "LogReader::report_corruption: bytes={} reason={}",
                bytes, reason_str
            );

            let reason_bytes = reason_str.as_bytes();
            let reason_slice =
                Slice::from_ptr_len(reason_bytes.as_ptr(), reason_bytes.len());

            let status = Status::corruption(&reason_slice, None);
            self.report_drop(bytes, &status);
        }
    }
}

#[cfg(test)]
mod log_reader_report_corruption_tests {
    use super::*;

    struct CorruptionSequentialFile;

    impl bitcoin_support::GetName for CorruptionSequentialFile {
        fn get_name(&self) -> &'static str {
            "corruption_sequential_file"
        }
    }

    impl SequentialFileRead for CorruptionSequentialFile {
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

    impl SequentialFileSkip for CorruptionSequentialFile {
        fn skip(&mut self, _n: u64) -> Status {
            Status::ok()
        }
    }

    impl SequentialFile for CorruptionSequentialFile {}

    struct CorruptionReporter {
        events: Arc<std::sync::Mutex<Vec<(usize, bool)>>>,
    }

    impl CorruptionReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<(usize, bool)>>>) -> Self {
            CorruptionReporter { events }
        }
    }

    impl LogReaderReporter for CorruptionReporter {
        fn corruption(&mut self, bytes: usize, status: &Status) {
            let is_ok = status.is_ok();
            if let Ok(mut guard) = self.events.lock() {
                guard.push((bytes, is_ok));
            }
        }
    }

    fn make_reader_for_corruption(
    ) -> (LogReader, Arc<std::sync::Mutex<Vec<(usize, bool)>>>) {
        let events: Arc<std::sync::Mutex<Vec<(usize, bool)>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let file = CorruptionSequentialFile;
        let reporter = CorruptionReporter::new(Arc::clone(&events));

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            true,
            0,
        );

        (reader, events)
    }

    #[traced_test]
    fn report_corruption_builds_corruption_status_and_calls_report_drop() {
        let (mut reader, events) = make_reader_for_corruption();

        let reason = b"bad length\0";
        reader.report_corruption(32, reason.as_ptr());

        let events_guard = events.lock().unwrap();
        assert_eq!(events_guard.len(), 1);
        assert_eq!(events_guard[0].0, 32);
        assert!(!events_guard[0].1);
    }

    #[traced_test]
    fn report_corruption_handles_null_reason_pointer() {
        let (mut reader, events) = make_reader_for_corruption();

        reader.report_corruption(5, std::ptr::null());

        let events_guard = events.lock().unwrap();
        assert_eq!(events_guard.len(), 1);
        assert_eq!(events_guard[0].0, 5);
    }
}
