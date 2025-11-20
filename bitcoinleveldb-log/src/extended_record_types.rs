// ---------------- [ File: bitcoinleveldb-log/src/extended_record_types.rs ]
crate::ix!();

/**
  | Extend record types with the following
  | special values
  |
  */
bitflags!{ 
    pub struct ExtendedRecordTypes: i32 {
        const Eof = LOG_MAX_RECORD_TYPE as i32 + 1;

        /*
          | Returned whenever we find an invalid
          | physical record.
          | 
          | Currently there are three situations
          | in which this happens:
          | 
          | - The record has an invalid CRC (ReadPhysicalRecord
          | reports a drop)
          | 
          | - The record is a 0-length record (No
          | drop is reported)
          | 
          | - The record is below constructor's
          | initial_offset (No drop is reported)
          |
          */
        const BadRecord = LOG_MAX_RECORD_TYPE as i32 + 2;
    }
}
