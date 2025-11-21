// ---------------- [ File: bitcoinleveldb-logwriter/src/choose_record_fragment_type.rs ]
crate::ix!();

impl LogWriter {

    /// Choose the physical record type for the current fragment.
    pub fn choose_record_fragment_type(begin: bool, end: bool) -> LogRecordType {
        let record_type = match (begin, end) {
            (true,  true)  => LogRecordType::Full,
            (true,  false) => LogRecordType::First,
            (false, true)  => LogRecordType::Last,
            (false, false) => LogRecordType::Middle,
        };

        trace!(
            "LogWriter::choose_record_fragment_type: begin={} end={} -> {:?}",
            begin,
            end,
            record_type
        );

        record_type
    }
}

#[cfg(test)]
mod log_writer_choose_record_fragment_type_tests {
    use super::*;

    #[traced_test]
    fn choose_record_fragment_type_covers_all_variants() {
        assert_eq!(
            LogWriter::choose_record_fragment_type(true, true),
            LogRecordType::Full
        );
        assert_eq!(
            LogWriter::choose_record_fragment_type(true, false),
            LogRecordType::First
        );
        assert_eq!(
            LogWriter::choose_record_fragment_type(false, true),
            LogRecordType::Last
        );
        assert_eq!(
            LogWriter::choose_record_fragment_type(false, false),
            LogRecordType::Middle
        );
    }
}
