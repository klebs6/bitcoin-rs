// ---------------- [ File: bitcoinleveldb-logwriter/src/add_record.rs ]
crate::ix!();

impl LogWriter {

    /// Add a logical record, fragmenting it into one or more physical records.
    pub fn add_record(&mut self, slice: &Slice) -> Status {
        info!(
            "LogWriter::add_record: writing logical record of size {}",
            slice.size()
        );

        unsafe { self.add_record_internal(slice) }
    }
}

#[cfg(test)]
mod log_writer_add_record_tests {
    use super::*;

    #[traced_test]
    fn add_record_single_fragment_produces_full_record() {
        let file  = Rc::new(RefCell::new(MockWritableFileAddRecord::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload_str    = "add_record_single_fragment";
        let payload_string = payload_str.to_string();
        let slice          = Slice::from(&payload_string);

        let status = w.add_record(&slice);
        assert!(status.is_ok());

        let buffer = file.borrow().recorded_bytes().to_vec();
        let header_size = LOG_HEADER_SIZE as usize;
        assert_eq!(buffer.len(), header_size + payload_string.as_bytes().len());

        let header = &buffer[..header_size];
        let data   = &buffer[header_size..];

        let length_field =
            (header[4] as usize) | ((header[5] as usize) << 8);
        assert_eq!(length_field, payload_string.as_bytes().len());

        assert_eq!(header[6], LogRecordType::Full as u8);
        assert_eq!(data, payload_string.as_bytes());

        let stored_crc =
            (header[0] as u32)
            | ((header[1] as u32) << 8)
            | ((header[2] as u32) << 16)
            | ((header[3] as u32) << 24);

        let initial_crc = w.type_crc_for(LogRecordType::Full);

        let expected_crc = unsafe {
            let ptr      = payload_string.as_bytes().as_ptr();
            let len      = payload_string.as_bytes().len();
            let extended = crc32c_extend(initial_crc, ptr, len);
            crc32c_mask(extended)
        };

        assert_eq!(stored_crc, expected_crc);
    }

    #[traced_test]
    fn add_record_large_payload_is_fragmented_and_round_trips() {
        let file  = Rc::new(RefCell::new(MockWritableFileAddRecord::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let first_fragment_capacity =
            (LOG_BLOCK_SIZE - LOG_HEADER_SIZE) as usize;
        let total_len = first_fragment_capacity + 50;

        let mut payload = Vec::with_capacity(total_len);
        for i in 0..total_len {
            payload.push((i % 251) as u8);
        }

        let slice = Slice::from_ptr_len(payload.as_ptr(), payload.len());

        let status = w.add_record(&slice);
        assert!(status.is_ok());

        let buffer      = file.borrow().recorded_bytes().to_vec();
        let header_size = LOG_HEADER_SIZE as usize;

        let mut cursor          = 0usize;
        let mut fragments_types = Vec::<u8>::new();
        let mut fragments_data  = Vec::<Vec<u8>>::new();

        while cursor < buffer.len() {
            assert!(cursor + header_size <= buffer.len());
            let header = &buffer[cursor..cursor + header_size];
            cursor += header_size;

            let length =
                (header[4] as usize) | ((header[5] as usize) << 8);
            let record_type = header[6];

            assert!(cursor + length <= buffer.len());
            let frag_payload =
                buffer[cursor..cursor + length].to_vec();
            cursor += length;

            fragments_types.push(record_type);
            fragments_data.push(frag_payload);
        }

        assert_eq!(fragments_types.len(), 2);
        assert_eq!(fragments_types[0], LogRecordType::First as u8);
        assert_eq!(fragments_types[1], LogRecordType::Last as u8);

        let mut combined = Vec::new();
        for frag in &fragments_data {
            combined.extend_from_slice(&frag[..]);
        }

        assert_eq!(combined, payload);
    }

    #[traced_test]
    fn add_record_handles_empty_slice_with_single_zero_length_record() {
        let file  = Rc::new(RefCell::new(MockWritableFileAddRecord::new()));
        let mut w = LogWriter::new(file.clone(), 0);

        let empty_string = String::new();
        let slice        = Slice::from(&empty_string);

        let status = w.add_record(&slice);
        assert!(status.is_ok());

        let buffer = file.borrow().recorded_bytes().to_vec();
        let header_size = LOG_HEADER_SIZE as usize;

        assert_eq!(buffer.len(), header_size);

        let header = &buffer[..header_size];
        let length_field =
            (header[4] as usize) | ((header[5] as usize) << 8);
        assert_eq!(length_field, 0usize);

        assert_eq!(header[6], LogRecordType::Full as u8);
    }

    #[traced_test]
    fn add_record_stops_on_io_error_from_destination() {
        let file = Rc::new(RefCell::new(
            MockWritableFileAddRecord::with_fail_append_after(1),
        ));
        let mut w = LogWriter::new(file.clone(), 0);

        let payload_string = "io_error_path".to_string();
        let slice          = Slice::from(&payload_string);

        let status = w.add_record(&slice);
        assert!(!status.is_ok());

        // We should have attempted at least one append and no panic occurred.
        let inner = file.borrow();
        assert_eq!(*inner.append_call_count(), 1);
    }
}
