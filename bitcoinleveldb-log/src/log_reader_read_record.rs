// ---------------- [ File: bitcoinleveldb-log/src/log_reader_read_record.rs ]
crate::ix!();

impl LogReader {
    
    /**
      | Read the next record into *record.  Returns
      | true if read successfully, false if we hit
      | end of the input.  May use "*scratch" as
      | temporary storage.  The contents filled in
      | *record will only be valid until the next
      | mutating operation on this reader or the next
      | mutation to *scratch.
      */
    pub fn read_record(&mut self, 
        record:  *mut Slice,
        scratch: *mut String) -> bool {
        
        todo!();
        /*
            if (last_record_offset_ < initial_offset_) {
        if (!SkipToInitialBlock()) {
          return false;
        }
      }

      scratch->clear();
      record->clear();
      bool in_fragmented_record = false;
      // Record offset of the logical record that we're reading
      // 0 is a dummy value to make compilers happy
      uint64_t prospective_record_offset = 0;

      Slice fragment;
      while (true) {
        const unsigned int record_type = ReadPhysicalRecord(&fragment);

        // ReadPhysicalRecord may have only had an empty trailer remaining in its
        // internal buffer. Calculate the offset of the next physical record now
        // that it has returned, properly accounting for its header size.
        uint64_t physical_record_offset =
            end_of_buffer_offset_ - buffer_.size() - kHeaderSize - fragment.size();

        if (resyncing_) {
          if (record_type == kMiddleType) {
            continue;
          } else if (record_type == kLastType) {
            resyncing_ = false;
            continue;
          } else {
            resyncing_ = false;
          }
        }

        switch (record_type) {
          case kFullType:
            if (in_fragmented_record) {
              // Handle bug in earlier versions of LogWriter where
              // it could emit an empty kFirstType record at the tail end
              // of a block followed by a kFullType or kFirstType record
              // at the beginning of the next block.
              if (!scratch->empty()) {
                ReportCorruption(scratch->size(), "partial record without end(1)");
              }
            }
            prospective_record_offset = physical_record_offset;
            scratch->clear();
            *record = fragment;
            last_record_offset_ = prospective_record_offset;
            return true;

          case kFirstType:
            if (in_fragmented_record) {
              // Handle bug in earlier versions of LogWriter where
              // it could emit an empty kFirstType record at the tail end
              // of a block followed by a kFullType or kFirstType record
              // at the beginning of the next block.
              if (!scratch->empty()) {
                ReportCorruption(scratch->size(), "partial record without end(2)");
              }
            }
            prospective_record_offset = physical_record_offset;
            scratch->assign(fragment.data(), fragment.size());
            in_fragmented_record = true;
            break;

          case kMiddleType:
            if (!in_fragmented_record) {
              ReportCorruption(fragment.size(),
                               "missing start of fragmented record(1)");
            } else {
              scratch->append(fragment.data(), fragment.size());
            }
            break;

          case kLastType:
            if (!in_fragmented_record) {
              ReportCorruption(fragment.size(),
                               "missing start of fragmented record(2)");
            } else {
              scratch->append(fragment.data(), fragment.size());
              *record = Slice(*scratch);
              last_record_offset_ = prospective_record_offset;
              return true;
            }
            break;

          case kEof:
            if (in_fragmented_record) {
              // This can be caused by the writer dying immediately after
              // writing a physical record but before completing the next; don't
              // treat it as a corruption, just ignore the entire logical record.
              scratch->clear();
            }
            return false;

          case kBadRecord:
            if (in_fragmented_record) {
              ReportCorruption(scratch->size(), "error in middle of record");
              in_fragmented_record = false;
              scratch->clear();
            }
            break;

          default: {
            char buf[40];
            snprintf(buf, sizeof(buf), "unknown record type %u", record_type);
            ReportCorruption(
                (fragment.size() + (in_fragmented_record ? scratch->size() : 0)),
                buf);
            in_fragmented_record = false;
            scratch->clear();
            break;
          }
        }
      }
      return false;
        */
    }
}
