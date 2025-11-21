// ---------------- [ File: bitcoinleveldb-logwriter/src/emit_physical_record.rs ]
crate::ix!();

impl LogWriter {

    pub fn emit_physical_record(
        &mut self,
        t: LogRecordType,
        ptr: *const u8,
        length: usize,
    ) -> Status {
        debug!(
            "LogWriter::emit_physical_record: type={:?} length={}",
            t,
            length
        );

        self.validate_record_length(length);
        self.validate_record_fits_in_block(length);

        let crc    = self.crc32c_for_record(t, ptr, length);
        let header = self.build_record_header(t, length, crc);

        let status = self.append_header_and_payload(&header, ptr, length);

        self.advance_block_offset(LOG_HEADER_SIZE + (length as i32));

        status
    }
}

#[cfg(test)]
mod log_writer_emit_physical_record_tests {
    use super::*;

    #[traced_test]
    fn emit_physical_record_writes_valid_header_and_payload() {
        let file  = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7];
        let status = w.emit_physical_record(
            LogRecordType::Full,
            payload.as_ptr(),
            payload.len(),
        );

        assert!(status.is_ok());

        let buffer      = file.borrow().recorded_bytes().to_vec();
        let header_size = LOG_HEADER_SIZE as usize;

        assert_eq!(buffer.len(), header_size + payload.len());

        let header = &buffer[..header_size];
        let data   = &buffer[header_size..];

        let length_field =
            (header[4] as usize) | ((header[5] as usize) << 8);
        assert_eq!(length_field, payload.len());

        assert_eq!(header[6], LogRecordType::Full as u8);
        assert_eq!(data, &payload[..]);

        let stored_crc =
            (header[0] as u32)
            | ((header[1] as u32) << 8)
            | ((header[2] as u32) << 16)
            | ((header[3] as u32) << 24);

        let initial_crc = w.type_crc_for(LogRecordType::Full);

        let expected_crc = unsafe {
            let extended = crc32c_extend(initial_crc, payload.as_ptr(), payload.len());
            crc32c_mask(extended)
        };

        assert_eq!(stored_crc, expected_crc);
    }

    #[traced_test]
    fn emit_physical_record_advances_block_offset() {
        let file  = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload: Vec<u8> = vec![0xaa; 16];
        let initial_offset    = w.block_offset_value();

        let status = w.emit_physical_record(
            LogRecordType::Full,
            payload.as_ptr(),
            payload.len(),
        );

        assert!(status.is_ok());

        let expected_delta =
            LOG_HEADER_SIZE + (payload.len() as i32);
        let final_offset = w.block_offset_value();

        assert_eq!(final_offset, initial_offset.saturating_add(expected_delta));
    }

    #[traced_test]
    fn emit_physical_record_panics_on_too_large_length() {
        let file  = Rc::new(RefCell::new(MockWritableFileEmit::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload = vec![0u8; 0xffff + 1];

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = w.emit_physical_record(
                LogRecordType::Full,
                payload.as_ptr(),
                payload.len(),
            );
        }));

        assert!(result.is_err());
    }

    #[traced_test]
    fn emit_physical_record_propagates_io_errors() {
        let file = Rc::new(RefCell::new(
            MockWritableFileEmit::with_fail_append_after(1),
        ));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload: Vec<u8> = vec![0x11, 0x22, 0x33];

        let status = w.emit_physical_record(
            LogRecordType::Full,
            payload.as_ptr(),
            payload.len(),
        );

        assert!(!status.is_ok());
        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 1);
    }
}
