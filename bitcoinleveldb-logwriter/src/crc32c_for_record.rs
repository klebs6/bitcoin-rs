// ---------------- [ File: bitcoinleveldb-logwriter/src/crc32c_for_record.rs ]
crate::ix!();

impl LogWriter {

    /// Compute the CRC32C for the given record type and payload, including masking.
    pub fn crc32c_for_record(
        &self,
        t: LogRecordType,
        ptr: *const u8,
        length: usize,
    ) -> u32 {
        debug!(
            "LogWriter::crc32c_for_record: type={:?} length={}",
            t,
            length
        );

        unsafe {
            let initial_crc = self.type_crc_for(t);
            let extended    = crc32c_extend(initial_crc, ptr, length);
            let masked      = crc32c_mask(extended);

            trace!(
                "LogWriter::crc32c_for_record: type={:?} initial_crc={:#010x} masked_crc={:#010x}",
                t,
                initial_crc,
                masked
            );

            masked
        }
    }
}

#[cfg(test)]
mod log_writer_crc32c_for_record_tests {
    use super::*;

    #[traced_test]
    fn crc32c_for_record_matches_crc32c_primitives() {
        let file = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let writer = LogWriter::new(file.clone(), 0);

        let payload = b"crc32c_for_record_payload".to_vec();
        let record_type = LogRecordType::Full;

        let initial_crc = writer.type_crc_for(record_type);

        let expected = unsafe {
            let extended = crc32c_extend(
                initial_crc,
                payload.as_ptr(),
                payload.len(),
            );
            crc32c_mask(extended)
        };

        let actual = writer.crc32c_for_record(
            record_type,
            payload.as_ptr(),
            payload.len(),
        );

        assert_eq!(actual, expected);
    }
}
