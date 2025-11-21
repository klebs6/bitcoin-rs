// ---------------- [ File: bitcoinleveldb-logwriter/src/initial_block_offset_from_length.rs ]
crate::ix!();

impl LogWriter {

    /// Compute the starting block offset from the current file length.
    pub fn initial_block_offset_from_length(dest_length: u64) -> i32 {
        let block_size   = LOG_BLOCK_SIZE as u64;
        let block_offset = (dest_length % block_size) as i32;

        debug!(
            "LogWriter::initial_block_offset_from_length: dest_length={} block_size={} -> offset={}",
            dest_length,
            block_size,
            block_offset
        );

        block_offset
    }
}

#[cfg(test)]
mod log_writer_initial_block_offset_tests {
    use super::*;

    #[traced_test]
    fn initial_block_offset_matches_modulo_block_size() {
        let block_size = LOG_BLOCK_SIZE as u64;
        let cases = [
            0u64,
            1u64,
            block_size.saturating_sub(1),
            block_size,
            block_size.saturating_add(1),
            (2 * block_size).saturating_add(17),
        ];

        for length in cases {
            let offset = LogWriter::initial_block_offset_from_length(length);
            let expected = (length % block_size) as i32;
            assert_eq!(
                offset, expected,
                "offset mismatch for dest_length={}",
                length
            );
        }
    }
}
