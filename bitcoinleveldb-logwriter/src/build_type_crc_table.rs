// ---------------- [ File: bitcoinleveldb-logwriter/src/build_type_crc_table.rs ]
crate::ix!();

impl LogWriter {

    /// Build and initialize the CRC table for all supported record types.
    pub fn build_type_crc_table() -> [u32; LOG_MAX_RECORD_TYPE as usize + 1] {
        trace!(
            "LogWriter::build_type_crc_table: initializing crc table for {} record types",
            LOG_MAX_RECORD_TYPE as usize + 1
        );

        let mut table = [0u32; LOG_MAX_RECORD_TYPE as usize + 1];

        unsafe {
            init_type_crc(table.as_mut_ptr());
        }

        trace!("LogWriter::build_type_crc_table: initialization complete");
        table
    }
}

#[cfg(test)]
mod log_writer_build_type_crc_table_tests {
    use super::*;

    #[traced_test]
    fn crc_table_is_deterministic_and_non_trivial() {
        let table1 = LogWriter::build_type_crc_table();
        let table2 = LogWriter::build_type_crc_table();

        assert_eq!(
            table1.len(),
            LOG_MAX_RECORD_TYPE as usize + 1
        );
        assert_eq!(table1, table2);

        let any_non_zero = table1.iter().any(|v| *v != 0);
        assert!(
            any_non_zero,
            "expected at least one non-zero CRC entry"
        );
    }
}
