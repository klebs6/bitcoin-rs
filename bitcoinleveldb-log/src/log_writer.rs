// ---------------- [ File: bitcoinleveldb-log/src/log_writer.rs ]
crate::ix!();
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.cc]

pub struct LogWriter {

    dest:         Rc<RefCell<dyn WritableFile>>,

    /**
      | Current offset in block
      |
      */
    block_offset: i32,

    /**
      | crc32c values for all supported record
      | types. These are pre-computed to reduce
      | the overhead of computing the crc of
      | the record type stored in the header.
      |
      */
    type_crc:     [u32; LOG_MAX_RECORD_TYPE as usize + 1],
}

impl LogWriter {

    /**
      | Create a writer that will append data to
      | "*dest".
      |
      | "*dest" must have initial length
      | "dest_length".
      |
      | "*dest" must remain live while this LogWriter is
      | in use.
      */
    pub fn new(
        dest:        Rc<RefCell<dyn WritableFile>>,
        dest_length: u64) -> Self {
    
        todo!();
        /*
          : dest_(dest), block_offset_(dest_length % kBlockSize) 
          InitTypeCrc(type_crc_);
        */
    }
}
