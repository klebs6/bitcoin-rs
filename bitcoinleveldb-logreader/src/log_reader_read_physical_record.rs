// ---------------- [ File: bitcoinleveldb-logreader/src/log_reader_read_physical_record.rs ]
crate::ix!();

impl LogReader {

    /// Return type, or one of the preceding special values
    /// 
    pub fn read_physical_record(&mut self, result: *mut Slice) -> u32 {
        trace!("LogReader::read_physical_record: enter");

        const BAD_LENGTH_REASON: &[u8]        = b"bad record length\0";
        const CHECKSUM_MISMATCH_REASON: &[u8] = b"checksum mismatch\0";

        if result.is_null() {
            error!("LogReader::read_physical_record: null result pointer");
            return ExtendedRecordTypes::BadRecord.bits() as u32;
        }

        loop {
            let header_size = LOG_HEADER_SIZE as usize;
            let block_size  = LOG_BLOCK_SIZE as usize;

            // Ensure we have at least a full header in the buffer.
            if *self.buffer_slice().size() < header_size {
                if !self.eof_flag() {
                    debug!(
                        "LogReader::read_physical_record: need {} bytes for header, have {}; reading new block",
                        header_size,
                        self.buffer_slice().size()
                    );

                    let status = self.read_into_buffer_from_file(block_size);

                    if !status.is_ok() {
                        self.buffer_slice_mut().clear();
                        self.report_drop(block_size as u64, &status);
                        self.set_eof_flag(true);
                        debug!(
                            "LogReader::read_physical_record: read error while refilling; treating as EOF"
                        );
                        return ExtendedRecordTypes::Eof.bits() as u32;
                    } else if *self.buffer_slice().size() < block_size {
                        debug!(
                            "LogReader::read_physical_record: short read {} (< {}), setting EOF",
                            self.buffer_slice().size(),
                            block_size
                        );
                        self.set_eof_flag(true);
                    }

                    continue;
                } else {
                    debug!(
                        "LogReader::read_physical_record: buffer too small ({}) and EOF already set; returning EOF",
                        self.buffer_slice().size()
                    );
                    self.buffer_slice_mut().clear();
                    return ExtendedRecordTypes::Eof.bits() as u32;
                }
            }

            // Parse header from current buffer start.
            let header_ptr_ptr = self.buffer_slice().data();
            if header_ptr_ptr.is_null() {
                error!("LogReader::read_physical_record: buffer has null data pointer");
                self.buffer_slice_mut().clear();
                return ExtendedRecordTypes::BadRecord.bits() as u32;
            }

            let header_ptr = unsafe { *header_ptr_ptr };
            if header_ptr.is_null() {
                error!("LogReader::read_physical_record: buffer has null inner data pointer");
                self.buffer_slice_mut().clear();
                return ExtendedRecordTypes::BadRecord.bits() as u32;
            }

            let a                = unsafe { (*header_ptr.add(4)) as u32 & 0xff };
            let b                = unsafe { (*header_ptr.add(5)) as u32 & 0xff };
            let record_type_byte = unsafe { (*header_ptr.add(6)) as u8 };
            let length: u32      = a | (b << 8);

            let buffer_size = *self.buffer_slice().size();

            if header_size + (length as usize) > buffer_size {
                let drop_size = buffer_size;
                debug!(
                    "LogReader::read_physical_record: bad record length {} (buffer_size={})",
                    length,
                    drop_size
                );
                self.buffer_slice_mut().clear();
                if !self.eof_flag() {
                    self.report_corruption(
                        drop_size as u64,
                        BAD_LENGTH_REASON.as_ptr(),
                    );
                    return ExtendedRecordTypes::BadRecord.bits() as u32;
                } else {
                    // If the end of the file has been reached without reading |length| bytes
                    // of payload, assume the writer died in the middle of writing the record.
                    // Don't report a corruption.
                    return ExtendedRecordTypes::Eof.bits() as u32;
                }
            }

            if record_type_byte == LogRecordType::Zero as u8 && length == 0 {
                // Skip zero length record without reporting any drops since
                // such records are produced by the mmap based writing code in
                // env_posix.cc that preallocates file regions.
                debug!(
                    "LogReader::read_physical_record: skipping zero-length preallocated record"
                );
                self.buffer_slice_mut().clear();
                return ExtendedRecordTypes::BadRecord.bits() as u32;
            }

            if self.checksum_enabled() {
                let expected_crc = unsafe {
                    let encoded = decode_fixed32(header_ptr);
                    crc32c_unmask(encoded)
                };
                let actual_crc = unsafe {
                    crc32c_value(
                        header_ptr.add(6),
                        1 + length as usize,
                    )
                };
                if actual_crc != expected_crc {
                    let drop_size = *self.buffer_slice().size();
                    self.buffer_slice_mut().clear();
                    self.report_corruption(
                        drop_size as u64,
                        CHECKSUM_MISMATCH_REASON.as_ptr(),
                    );
                    return ExtendedRecordTypes::BadRecord.bits() as u32;
                }
            }

            // Compute the physical offset of this record's header BEFORE we mutate the buffer.
            let physical_record_offset = self
                .end_of_buffer_offset_value()
                .saturating_sub(buffer_size as u64);

            // If this record starts before the initial offset, skip it without returning payload.
            if physical_record_offset < self.initial_log_offset() {
                unsafe {
                    (*result).clear();
                }
                trace!(
                    "LogReader::read_physical_record: skipping record at {} before initial_offset {}",
                    physical_record_offset,
                    self.initial_log_offset()
                );
                self.buffer_slice_mut()
                    .remove_prefix(header_size + length as usize);
                return ExtendedRecordTypes::BadRecord.bits() as u32;
            }

            // Expose the payload for this record.
            unsafe {
                *result = Slice::from_ptr_len(
                    header_ptr.add(header_size),
                    length as usize,
                );
            }

            // Now advance the buffer past this physical record.
            self.buffer_slice_mut()
                .remove_prefix(header_size + length as usize);

            trace!(
                "LogReader::read_physical_record: read record type={} length={}",
                record_type_byte,
                length
            );

            return record_type_byte as u32;
        }
    }
}

#[cfg(test)]
mod log_reader_read_physical_record_tests {
    use super::*;

    struct PhysicalSequentialFile {
        data: Vec<u8>,
        position: usize,
    }

    impl PhysicalSequentialFile {
        fn new(data: Vec<u8>) -> Self {
            PhysicalSequentialFile { data, position: 0 }
        }
    }

    impl bitcoin_support::GetName for PhysicalSequentialFile {
        fn get_name(&self) -> &'static str {
            "physical_sequential_file"
        }
    }

    impl SequentialFileRead for PhysicalSequentialFile {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
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

    impl SequentialFileSkip for PhysicalSequentialFile {
        fn skip(&mut self, n: u64) -> Status {
            self.position = self
                .position
                .saturating_add(n as usize)
                .min(self.data.len());
            Status::ok()
        }
    }

    impl SequentialFile for PhysicalSequentialFile {}

    struct PhysicalReporter {
        events: Arc<std::sync::Mutex<Vec<usize>>>,
    }

    impl PhysicalReporter {
        fn new(events: Arc<std::sync::Mutex<Vec<usize>>>) -> Self {
            PhysicalReporter { events }
        }
    }

    impl LogReaderReporter for PhysicalReporter {
        fn corruption(&mut self, bytes: usize, _status: &Status) {
            if let Ok(mut guard) = self.events.lock() {
                guard.push(bytes);
            }
        }
    }

    fn build_physical_record(record_type: LogRecordType, payload: &[u8]) -> Vec<u8> {
        let header_size = LOG_HEADER_SIZE as usize;
        let mut buf = Vec::with_capacity(header_size + payload.len());
        buf.resize(header_size, 0u8);

        let len = payload.len();
        assert!(len <= u16::MAX as usize);
        buf[4] = (len & 0xff) as u8;
        buf[5] = ((len >> 8) & 0xff) as u8;
        buf[6] = record_type as u8;

        buf.extend_from_slice(payload);
        buf
    }

    fn make_reader_with_raw_bytes(
        bytes: Vec<u8>,
        checksum: bool,
    ) -> (LogReader, Arc<std::sync::Mutex<Vec<usize>>>) {
        let reporter_events: Arc<std::sync::Mutex<Vec<usize>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let file = PhysicalSequentialFile::new(bytes);
        let reporter =
            PhysicalReporter::new(Arc::clone(&reporter_events));

        let reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            checksum,
            0,
        );

        (reader, reporter_events)
    }

    #[traced_test]
    fn read_physical_record_parses_valid_record_without_checksum() {
        let payload = b"physical-record".to_vec();
        let bytes = build_physical_record(LogRecordType::Full, &payload);

        let (mut reader, reporter_events) =
            make_reader_with_raw_bytes(bytes, false);

        let mut out = Slice::default();
        let record_type = reader.read_physical_record(&mut out as *mut Slice);

        assert_eq!(record_type, LogRecordType::Full as u32);

        let size = unsafe { *out.size() };
        let data_ptr = unsafe { *out.data() };
        let out_bytes = unsafe { std::slice::from_raw_parts(data_ptr, size) };

        assert_eq!(out_bytes, payload.as_slice());

        let events_guard = reporter_events.lock().unwrap();
        assert!(events_guard.is_empty());
    }

    #[traced_test]
    fn read_physical_record_returns_eof_on_short_trailer_at_eof() {
        let bytes = Vec::<u8>::new();
        let (mut reader, _events) = make_reader_with_raw_bytes(bytes, false);

        let mut out = Slice::default();
        let record_type = reader.read_physical_record(&mut out as *mut Slice);

        assert_eq!(record_type, ExtendedRecordTypes::Eof.bits() as u32);
    }
}
