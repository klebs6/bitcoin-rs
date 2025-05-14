// ---------------- [ File: bitcoinleveldb-log/src/writer.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_writer.cc]

pub fn init_type_crc(type_crc: *mut u32)  {
    
    todo!();
        /*
            for (int i = 0; i <= kMaxRecordType; i++) {
        char t = static_cast<char>(i);
        type_crc[i] = crc32c::Value(&t, 1);
      }
        */
}

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
    
    pub fn add_record(&mut self, slice: &Slice) -> crate::Status {
        
        todo!();
        /*
            const char* ptr = slice.data();
      size_t left = slice.size();

      // Fragment the record if necessary and emit it.  Note that if slice
      // is empty, we still want to iterate once to emit a single
      // zero-length record
      Status s;
      bool begin = true;
      do {
        const int leftover = kBlockSize - block_offset_;
        assert(leftover >= 0);
        if (leftover < kHeaderSize) {
          // Switch to a new block
          if (leftover > 0) {
            // Fill the trailer (literal below relies on kHeaderSize being 7)
            const_assert(kHeaderSize == 7, "");
            dest_->Append(Slice("\x00\x00\x00\x00\x00\x00", leftover));
          }
          block_offset_ = 0;
        }

        // Invariant: we never leave < kHeaderSize bytes in a block.
        assert(kBlockSize - block_offset_ - kHeaderSize >= 0);

        const size_t avail = kBlockSize - block_offset_ - kHeaderSize;
        const size_t fragment_length = (left < avail) ? left : avail;

        RecordType type;
        const bool end = (left == fragment_length);
        if (begin && end) {
          type = kFullType;
        } else if (begin) {
          type = kFirstType;
        } else if (end) {
          type = kLastType;
        } else {
          type = kMiddleType;
        }

        s = EmitPhysicalRecord(type, ptr, fragment_length);
        ptr += fragment_length;
        left -= fragment_length;
        begin = false;
      } while (s.ok() && left > 0);
      return s;
        */
    }
    
    pub fn emit_physical_record(&mut self, 
        t:      LogRecordType,
        ptr:    *const u8,
        length: usize) -> Status {
        
        todo!();
        /*
            assert(length <= 0xffff);  // Must fit in two bytes
      assert(block_offset_ + kHeaderSize + length <= kBlockSize);

      // Format the header
      char buf[kHeaderSize];
      buf[4] = static_cast<char>(length & 0xff);
      buf[5] = static_cast<char>(length >> 8);
      buf[6] = static_cast<char>(t);

      // Compute the crc of the record type and the payload.
      uint32_t crc = crc32c::Extend(type_crc_[t], ptr, length);
      crc = crc32c::Mask(crc);  // Adjust for storage
      EncodeFixed32(buf, crc);

      // Write the header and the payload
      Status s = dest_->Append(Slice(buf, kHeaderSize));
      if (s.ok()) {
        s = dest_->Append(Slice(ptr, length));
        if (s.ok()) {
          s = dest_->Flush();
        }
      }
      block_offset_ += kHeaderSize + length;
      return s;
        */
    }
}
