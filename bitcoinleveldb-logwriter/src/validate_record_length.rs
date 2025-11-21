// ---------------- [ File: bitcoinleveldb-logwriter/src/validate_record_length.rs ]
crate::ix!();

impl LogWriter {
    /// Validate that the record length is within the supported bounds.
    pub fn validate_record_length(&self, length: usize) {
        debug!(
            "LogWriter::validate_record_length: length={}",
            length
        );
        assert!(length <= 0xffff, "record length too large");
    }
}

#[cfg(test)]
mod log_writer_validate_record_length_tests {
    use super::*;

    #[traced_test]
    fn validate_record_length_accepts_zero_and_max_u16() {
        let file   = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let writer = LogWriter::new(file.clone(), 0);

        writer.validate_record_length(0);
        writer.validate_record_length(0xffff);
    }

    #[traced_test]
    fn validate_record_length_panics_for_too_large_length() {
        let file   = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let writer = LogWriter::new(file.clone(), 0);

        let result = std::panic::catch_unwind(
            std::panic::AssertUnwindSafe(|| {
                writer.validate_record_length(0x1_0000usize);
            }),
        );

        assert!(result.is_err());
    }
}
