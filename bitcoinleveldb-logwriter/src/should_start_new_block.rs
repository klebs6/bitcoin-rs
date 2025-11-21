// ---------------- [ File: bitcoinleveldb-logwriter/src/should_start_new_block.rs ]
crate::ix!();

impl LogWriter {

    /// Whether we need to start a new block because the remaining bytes are too
    /// small to hold a header.
    pub fn should_start_new_block(&self) -> bool {
        let leftover = self.block_trailer_bytes_remaining();
        let result   = leftover < LOG_HEADER_SIZE;
        trace!(
            "LogWriter::should_start_new_block: leftover={} header_size={} => {}",
            leftover,
            LOG_HEADER_SIZE,
            result
        );
        result
    }
}

#[cfg(test)]
mod log_writer_should_start_new_block_tests {
    use super::*;

    #[traced_test]
    fn should_start_new_block_threshold_is_header_size() {
        let file = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(0);
        assert!(!writer.should_start_new_block());

        writer.set_block_offset_value(LOG_BLOCK_SIZE - LOG_HEADER_SIZE);
        assert!(!writer.should_start_new_block());

        writer.set_block_offset_value(LOG_BLOCK_SIZE - LOG_HEADER_SIZE + 1);
        assert!(writer.should_start_new_block());
    }
}
