// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_read_record.rs ]
crate::ix!();

impl LogReader {

    /// Read the next logical record into `record`.
    ///
    /// Returns `true` if a record was read successfully and `false` on EOF.
    ///
    /// The contents in `record` remain valid only until the next
    /// mutating operation on this reader or the next mutation to `scratch`.
    pub fn read_record(
        &mut self,
        record: &mut Slice,
        scratch: &mut Vec<u8>,
    ) -> bool {
        trace!(
            last_record_offset = *self.last_record_offset(),
            initial_offset     = self.initial_log_offset(),
            "LogReader::read_record: enter"
        );

        if *self.last_record_offset() < self.initial_log_offset() {
            if !self.skip_to_initial_block() {
                info!(
                    last_record_offset = *self.last_record_offset(),
                    initial_offset     = self.initial_log_offset(),
                    "LogReader::read_record: skip_to_initial_block returned false (EOF or error)"
                );
                return false;
            }
        }

        scratch.clear();
        record.clear();

        let mut in_fragmented_record          = false;
        let mut prospective_record_offset: u64 = 0;
        let mut fragment                      = Slice::default();

        const PARTIAL_WITHOUT_END_1: &[u8] =
            b"partial record without end(1)\0";
        const PARTIAL_WITHOUT_END_2: &[u8] =
            b"partial record without end(2)\0";
        const MISSING_START_1: &[u8] =
            b"missing start of fragmented record(1)\0";
        const MISSING_START_2: &[u8] =
            b"missing start of fragmented record(2)\0";
        const ERROR_MIDDLE: &[u8] = b"error in middle of record\0";
        const UNKNOWN_RECORD_TYPE: &[u8] =
            b"unknown record type\0";

        loop {
            let record_type =
                self.read_physical_record(&mut fragment as *mut Slice);

            let fragment_len: u64 =
                *fragment.size() as u64;
            let buffer_remaining: u64 =
                *self.buffer_slice().size() as u64;
            let header_len: u64 = LOG_HEADER_SIZE as u64;

            let physical_record_offset = self
                .end_of_buffer_offset_value()
                .saturating_sub(buffer_remaining)
                .saturating_sub(header_len)
                .saturating_sub(fragment_len);

            trace!(
                record_type,
                physical_record_offset,
                fragment_len,
                buffer_remaining,
                end_of_buffer_offset = self.end_of_buffer_offset_value(),
                resyncing            = self.is_resyncing(),
                in_fragmented_record,
                "LogReader::read_record: state after read_physical_record"
            );

            // Resync logic when starting from a non-zero initial offset.
            if self.is_resyncing() {
                if record_type == LogRecordType::Middle as u32 {
                    trace!(
                        "LogReader::read_record: resyncing, skipping kMiddleType fragment"
                    );
                    continue;
                } else if record_type == LogRecordType::Last as u32 {
                    trace!(
                        "LogReader::read_record: resyncing, skipping kLastType fragment and leaving resync mode"
                    );
                    self.set_resyncing_flag(false);
                    continue;
                } else {
                    trace!(
                        "LogReader::read_record: leaving resync mode at record_type={}",
                        record_type
                    );
                    self.set_resyncing_flag(false);
                }
            }

            if record_type == LogRecordType::Full as u32 {
                // A complete record in a single physical entry.
                if in_fragmented_record && !scratch.is_empty() {
                    warn!(
                        bytes = scratch.len(),
                        "LogReader::read_record: partial fragmented record without end (case 1)"
                    );
                    self.report_corruption(
                        scratch.len() as u64,
                        PARTIAL_WITHOUT_END_1.as_ptr(),
                    );
                }

                prospective_record_offset = physical_record_offset;
                scratch.clear();

                let data_ptr = *fragment.data();
                let data_len = *fragment.size();

                if data_len == 0 || data_ptr.is_null() {
                    record.clear();
                } else {
                    unsafe {
                        *record = Slice::from_ptr_len(
                            data_ptr,
                            data_len,
                        );
                    }
                }

                self.set_last_record_offset(prospective_record_offset);

                info!(
                    last_record_offset = *self.last_record_offset(),
                    record_len         = *record.size(),
                    "LogReader::read_record: returning Full record"
                );

                return true;
            } else if record_type == LogRecordType::First as u32 {
                // First fragment of a multi-fragment record.
                if in_fragmented_record && !scratch.is_empty() {
                    warn!(
                        bytes = scratch.len(),
                        "LogReader::read_record: partial fragmented record without end (case 2)"
                    );
                    self.report_corruption(
                        scratch.len() as u64,
                        PARTIAL_WITHOUT_END_2.as_ptr(),
                    );
                }

                prospective_record_offset = physical_record_offset;
                scratch.clear();
                append_slice_bytes(&fragment, scratch);
                in_fragmented_record = true;

                trace!(
                    prospective_record_offset,
                    first_fragment_len = fragment_len,
                    accumulated_bytes  = scratch.len(),
                    "LogReader::read_record: starting fragmented record (First fragment)"
                );
            } else if record_type == LogRecordType::Middle as u32 {
                // Middle fragment of a multi-fragment record.
                if !in_fragmented_record {
                    let dropped_bytes = fragment_len;
                    warn!(
                        fragment_len = dropped_bytes,
                        "LogReader::read_record: missing start of fragmented record (case 1)"
                    );
                    self.report_corruption(
                        dropped_bytes,
                        MISSING_START_1.as_ptr(),
                    );
                } else {
                    append_slice_bytes(&fragment, scratch);
                    trace!(
                        accumulated_bytes = scratch.len(),
                        "LogReader::read_record: appended Middle fragment"
                    );
                }
            } else if record_type == LogRecordType::Last as u32 {
                // Final fragment of a multi-fragment record.
                if !in_fragmented_record {
                    let dropped_bytes = fragment_len;
                    warn!(
                        fragment_len = dropped_bytes,
                        "LogReader::read_record: missing start of fragmented record (case 2)"
                    );
                    self.report_corruption(
                        dropped_bytes,
                        MISSING_START_2.as_ptr(),
                    );
                } else {
                    append_slice_bytes(&fragment, scratch);

                    let total_len = scratch.len();
                    if total_len == 0 {
                        record.clear();
                    } else {
                        let record_ptr = scratch.as_ptr();
                        unsafe {
                            *record =
                                Slice::from_ptr_len(record_ptr, total_len);
                        }
                    }

                    self.set_last_record_offset(prospective_record_offset);
                    in_fragmented_record = false;

                    info!(
                        last_record_offset = *self.last_record_offset(),
                        record_len         = *record.size(),
                        "LogReader::read_record: returning assembled fragmented record (Last fragment)"
                    );

                    return true;
                }
            } else if record_type
                == ExtendedRecordTypes::Eof.bits() as u32
            {
                // End of file.
                if in_fragmented_record && !scratch.is_empty() {
                    debug!(
                        accumulated_bytes = scratch.len(),
                        "LogReader::read_record: EOF while in fragmented record; dropping partial record"
                    );
                    scratch.clear();
                }

                info!("LogReader::read_record: reached EOF");
                return false;
            } else if record_type
                == ExtendedRecordTypes::BadRecord.bits() as u32
            {
                // Corrupted physical record.
                if in_fragmented_record && !scratch.is_empty() {
                    warn!(
                        accumulated_bytes = scratch.len(),
                        "LogReader::read_record: bad record while in fragmented record"
                    );
                    self.report_corruption(
                        scratch.len() as u64,
                        ERROR_MIDDLE.as_ptr(),
                    );
                    in_fragmented_record = false;
                    scratch.clear();
                } else {
                    debug!(
                        "LogReader::read_record: bad physical record outside fragmented record; continuing"
                    );
                }
            } else {
                // Unknown record type.
                let dropped_bytes =
                    fragment_len
                        + if in_fragmented_record {
                            scratch.len() as u64
                        } else {
                            0
                        };

                warn!(
                    record_type,
                    dropped_bytes,
                    "LogReader::read_record: unknown record type; treating as corruption"
                );

                self.report_corruption(
                    dropped_bytes,
                    UNKNOWN_RECORD_TYPE.as_ptr(),
                );
                in_fragmented_record = false;
                scratch.clear();
            }
        }
    }
}

#[cfg(test)]
mod log_reader_read_record_tests {
    use super::*;

    struct RecordSequentialFile {
        data:     Vec<u8>,
        position: usize,
    }

    impl RecordSequentialFile {
        fn new(data: Vec<u8>) -> Self {
            RecordSequentialFile { data, position: 0 }
        }
    }

    impl bitcoin_support::GetName for RecordSequentialFile {
        fn get_name(&self) -> &'static str {
            "record_sequential_file"
        }
    }

    impl SequentialFileRead for RecordSequentialFile {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            let remaining =
                self.data.len().saturating_sub(self.position);
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
                        *result = Slice::from_ptr_len(
                            scratch as *const u8,
                            to_read,
                        );
                    }
                }
            }

            self.position =
                self.position.saturating_add(to_read);

            Status::ok()
        }
    }

    impl SequentialFileSkip for RecordSequentialFile {
        fn skip(&mut self, n: u64) -> Status {
            self.position = self
                .position
                .saturating_add(n as usize)
                .min(self.data.len());
            Status::ok()
        }
    }

    impl SequentialFile for RecordSequentialFile {}

    struct RecordReporter {
        events: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl RecordReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            RecordReporter { events }
        }
    }

    impl LogReaderReporter for RecordReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(bytes);
            }
        }
    }

    fn build_physical_record(
        record_type: LogRecordType,
        payload: &[u8],
    ) -> Vec<u8> {
        let header_size = LOG_HEADER_SIZE as usize;
        let mut buf =
            Vec::with_capacity(header_size + payload.len());
        buf.resize(header_size, 0u8);

        let len = payload.len();
        assert!(len <= u16::MAX as usize);
        buf[4] = (len & 0xff) as u8;
        buf[5] = ((len >> 8) & 0xff) as u8;
        buf[6] = record_type as u8;

        buf.extend_from_slice(payload);
        buf
    }

    fn make_reader(
        bytes: Vec<u8>,
        initial_offset: u64,
    ) -> (
        LogReader,
        Arc<std::sync::Mutex<Vec<usize>>>,
    ) {
        let reporter_events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let file = RecordSequentialFile::new(bytes);
        let reporter =
            RecordReporter::new(Arc::clone(&reporter_events));

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            false,
            initial_offset,
        );

        (reader, reporter_events)
    }

    fn slice_to_vec(slice: &Slice) -> Vec<u8> {
        let size     = *slice.size();
        let data_ptr = *slice.data();

        if data_ptr.is_null() || size == 0 {
            Vec::new()
        } else {
            unsafe {
                std::slice::from_raw_parts(data_ptr, size).to_vec()
            }
        }
    }

    #[traced_test]
    fn read_record_reads_single_full_record() {
        let payload = b"single full record";
        let bytes =
            build_physical_record(LogRecordType::Full, payload);

        let (mut reader, reporter_events) =
            make_reader(bytes, 0);

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let ok = reader.read_record(&mut record, &mut scratch);
        assert!(ok, "expected first record read to succeed");
        assert!(
            scratch.is_empty(),
            "scratch should remain empty for a full record"
        );

        let out = slice_to_vec(&record);
        assert_eq!(out.as_slice(), payload);

        let guard = reporter_events.lock().unwrap();
        assert!(
            guard.is_empty(),
            "no corruption should be reported for a valid full record"
        );
    }

    #[traced_test]
    fn read_record_assembles_fragmented_record_from_first_and_last() {
        let first  = b"hello ".to_vec();
        let last   = b"world".to_vec();

        let mut bytes =
            build_physical_record(LogRecordType::First, &first);
        bytes.extend_from_slice(&build_physical_record(
            LogRecordType::Last,
            &last,
        ));

        let (mut reader, reporter_events) =
            make_reader(bytes, 0);

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let ok = reader.read_record(&mut record, &mut scratch);
        assert!(
            ok,
            "expected fragmented record (First + Last) to be reassembled"
        );

        let out = slice_to_vec(&record);
        assert_eq!(out.as_slice(), b"hello world");

        let guard = reporter_events.lock().unwrap();
        assert!(
            guard.is_empty(),
            "no corruption should be reported for a clean fragmented record"
        );
    }

    #[traced_test]
    fn read_record_returns_false_on_eof() {
        let payload = b"only record";
        let bytes =
            build_physical_record(LogRecordType::Full, payload);

        let (mut reader, _events) = make_reader(bytes, 0);

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let first =
            reader.read_record(&mut record, &mut scratch);
        assert!(first, "first record should be read successfully");

        scratch.clear();
        record.clear();

        let second =
            reader.read_record(&mut record, &mut scratch);
        assert!(
            !second,
            "second read should return false at EOF"
        );
    }

    #[traced_test]
    fn read_record_respects_initial_offset_and_skips_earlier_records() {
        let first_payload  = b"first";
        let second_payload = b"second";

        let first_bytes =
            build_physical_record(LogRecordType::Full, first_payload);
        let second_bytes =
            build_physical_record(LogRecordType::Full, second_payload);

        let mut all = first_bytes.clone();
        all.extend_from_slice(&second_bytes);

        let initial_offset =
            (LOG_HEADER_SIZE as usize + first_payload.len()) as u64;

        let (mut reader, _events) =
            make_reader(all, initial_offset);

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let ok =
            reader.read_record(&mut record, &mut scratch);
        assert!(ok, "expected to read a record at/after initial_offset");

        let out = slice_to_vec(&record);
        assert_eq!(
            out.as_slice(),
            second_payload,
            "reader should skip first record and return the second"
        );
    }
}

