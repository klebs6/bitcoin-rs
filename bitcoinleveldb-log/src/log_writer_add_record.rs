// ---------------- [ File: bitcoinleveldb-log/src/log_writer_add_record.rs ]
crate::ix!();

impl LogWriter {

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
}
