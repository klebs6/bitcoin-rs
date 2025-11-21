// ---------------- [ File: bitcoinleveldb-logwriter/src/validate_record_fits_in_block.rs ]
crate::ix!();

impl LogWriter {

    /// Validate that the record fits entirely within the current block.
    pub fn validate_record_fits_in_block(&self, length: usize) {
        let block_offset = self.block_offset_value() as usize;
        let header_size  = LOG_HEADER_SIZE as usize;
        let block_size   = LOG_BLOCK_SIZE as usize;

        debug!(
            "LogWriter::validate_record_fits_in_block: block_offset={} header_size={} length={} block_size={}",
            block_offset,
            header_size,
            length,
            block_size
        );

        assert!(
            block_offset + header_size + length <= block_size,
            "record does not fit into block"
        );
    }
}

#[cfg(test)]
mod log_writer_validate_record_fits_in_block_tests {
    use super::*;

    #[traced_test]
    fn validate_record_fits_in_block_accepts_exact_capacity() {
        let file  = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(0);

        let length = (LOG_BLOCK_SIZE - LOG_HEADER_SIZE) as usize;
        writer.validate_record_fits_in_block(length);
    }

    #[traced_test]
    fn validate_record_fits_in_block_panics_when_too_large() {
        let file  = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(0);

        let length = LOG_BLOCK_SIZE as usize;
        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                writer.validate_record_fits_in_block(length);
            }),
        );

        assert!(result.is_err());
    }
}
