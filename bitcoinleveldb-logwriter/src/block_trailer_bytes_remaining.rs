// ---------------- [ File: bitcoinleveldb-logwriter/src/block_trailer_bytes_remaining.rs ]
crate::ix!();

impl LogWriter {

    /// Remaining bytes in the current block (including space that may be too small
    /// for a header).
    pub fn block_trailer_bytes_remaining(&self) -> i32 {
        let remaining = LOG_BLOCK_SIZE - self.block_offset_value();
        trace!(
            "LogWriter::block_trailer_bytes_remaining: block_offset={} remaining={}",
            self.block_offset_value(),
            remaining
        );
        remaining
    }
}

#[cfg(test)]
mod log_writer_block_trailer_bytes_remaining_tests {
    use super::*;

    #[traced_test]
    fn block_trailer_bytes_remaining_matches_block_offset() {
        let file = Rc::new(RefCell::new(MockWritableFileCore::new()));
        let mut writer = LogWriter::new(file.clone(), 0);

        writer.set_block_offset_value(0);
        assert_eq!(
            writer.block_trailer_bytes_remaining(),
            LOG_BLOCK_SIZE
        );

        writer.set_block_offset_value(LOG_BLOCK_SIZE / 2);
        assert_eq!(
            writer.block_trailer_bytes_remaining(),
            LOG_BLOCK_SIZE / 2
        );

        writer.set_block_offset_value(LOG_BLOCK_SIZE);
        assert_eq!(writer.block_trailer_bytes_remaining(), 0);
    }
}
