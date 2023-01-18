/*!
  | Log format information shared by reader and
  | writer.
  |
  | See ../doc/log_format.md for more detail.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_format.h]

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
