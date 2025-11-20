// ---------------- [ File: bitcoinleveldb-log/src/log_reader.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_reader.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_reader.cc]

pub struct LogReader {

    file:                 Box<dyn SequentialFile>,
    reporter:             Box<dyn LogReaderReporter>,
    checksum:             bool,
    backing_store:        *const u8,
    buffer:               Slice,

    /**
      | Last Read() indicated EOF by returning
      | < kBlockSize
      |
      */
    eof:                  bool,

    /**
      | Offset of the last record returned by
      | 
      | ReadRecord.
      |
      */
    last_record_offset:   u64,

    /**
      | Offset of the first location past the
      | end of buffer_.
      |
      */
    end_of_buffer_offset: u64,

    /**
      | Offset at which to start looking for
      | the first record to return
      |
      */
    initial_offset:       u64,

    /**
      | True if we are resynchronizing after
      | a seek (initial_offset_ > 0). In particular,
      | a run of kMiddleType and kLastType records
      | can be silently skipped in this mode
      |
      */
    resyncing:            bool,
}

impl LogReader {
    
    /**
      | Returns the physical offset of the last
      | record returned by ReadRecord.
      |
      | Undefined before the first call to
      | ReadRecord.
      */
    pub fn last_record_offset(&mut self) -> u64 {
        debug!(
            "LogReader::last_record_offset: returning {}",
            self.last_record_offset
        );
        self.last_record_offset
    }
}

impl Drop for LogReader {
    fn drop(&mut self) {
        todo!();
        /*
            delete[] backing_store_;
        */
    }
}
