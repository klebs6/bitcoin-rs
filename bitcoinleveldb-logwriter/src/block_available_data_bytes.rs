// ---------------- [ File: bitcoinleveldb-logwriter/src/block_available_data_bytes.rs ]
crate::ix!();

impl LogWriter {

    /// Bytes available for payload in the current block, after accounting for the header.
    pub fn block_available_data_bytes(&self) -> usize {
        let available =
            (LOG_BLOCK_SIZE - self.block_offset_value() - LOG_HEADER_SIZE) as usize;
        trace!(
            "LogWriter::block_available_data_bytes: block_offset={} available={}",
            self.block_offset_value(),
            available
        );
        available
    }
}

#[cfg(test)]
mod log_writer_block_available_data_bytes_tests {
    use super::*;

    #[traced_test]
    fn block_available_data_bytes_respects_header_size() {
        let file = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        let available_at_zero = writer.block_available_data_bytes();
        assert_eq!(
            available_at_zero as i32,
            LOG_BLOCK_SIZE - LOG_HEADER_SIZE
        );

        writer.set_block_offset_value(LOG_BLOCK_SIZE - LOG_HEADER_SIZE);
        let available_at_trailer = writer.block_available_data_bytes();
        assert_eq!(available_at_trailer, 0);
    }
}
