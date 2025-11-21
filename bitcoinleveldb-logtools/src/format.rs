// ---------------- [ File: bitcoinleveldb-logtools/src/format.rs ]
/*!
  | Log format information shared by reader and
  | writer.
  |
  | See ../doc/log_format.md for more detail.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_format.h]

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum LogRecordType {

    /**
      | Zero is reserved for preallocated files
      |
      */
    Zero   = 0,

    Full   = 1,

    /**
      | For fragments
      |
      */
    First  = 2,
    Middle = 3,
    Last   = 4
}

pub const LOG_MAX_RECORD_TYPE: LogRecordType = LogRecordType::Last;

pub const LOG_BLOCK_SIZE: i32 = 32768;

/**
   Header is checksum (4 bytes), length (2 bytes),
   type (1 byte).
  */
pub const LOG_HEADER_SIZE: i32 = 4 + 2 + 1;

#[cfg(test)]
mod log_format_constants_spec {
    use super::*;

    #[traced_test]
    fn log_record_type_discriminants_are_stable() {
        let zero = LogRecordType::Zero as i32;
        let full = LogRecordType::Full as i32;
        let first = LogRecordType::First as i32;
        let middle = LogRecordType::Middle as i32;
        let last = LogRecordType::Last as i32;

        info!(
            "log_record_type_discriminants_are_stable: zero={} full={} first={} middle={} last={}",
            zero, full, first, middle, last
        );

        assert_eq!(zero, 0);
        assert_eq!(full, 1);
        assert_eq!(first, 2);
        assert_eq!(middle, 3);
        assert_eq!(last, 4);
    }

    #[traced_test]
    fn log_format_block_and_header_sizes_match_expected_values() {
        info!(
            "log_format_block_and_header_sizes_match_expected_values: LOG_BLOCK_SIZE={} LOG_HEADER_SIZE={}",
            LOG_BLOCK_SIZE,
            LOG_HEADER_SIZE
        );

        assert_eq!(LOG_BLOCK_SIZE, 32768);
        assert_eq!(LOG_HEADER_SIZE, 7);
        assert!(LOG_HEADER_SIZE < LOG_BLOCK_SIZE);
    }

    #[traced_test]
    fn log_max_record_type_matches_last_variant() {
        let max_type = LOG_MAX_RECORD_TYPE as i32;
        let last_type = LogRecordType::Last as i32;

        info!(
            "log_max_record_type_matches_last_variant: max_type={} last_type={}",
            max_type, last_type
        );

        assert_eq!(max_type, last_type);
    }
}
