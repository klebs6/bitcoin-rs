// ---------------- [ File: bitcoinleveldb-log/src/log_reader_read_physical_record.rs ]
crate::ix!();

impl LogReader {

    /**
      | Return type, or one of the preceding
      | special values
      |
      */
    pub fn read_physical_record(&mut self, result: *mut Slice) -> u32 {
        
        todo!();
        /*
            while (true) {
        if (buffer_.size() < kHeaderSize) {
          if (!eof_) {
            // Last read was a full read, so this is a trailer to skip
            buffer_.clear();
            Status status = file_->Read(kBlockSize, &buffer_, backing_store_);
            end_of_buffer_offset_ += buffer_.size();
            if (!status.ok()) {
              buffer_.clear();
              ReportDrop(kBlockSize, status);
              eof_ = true;
              return kEof;
            } else if (buffer_.size() < kBlockSize) {
              eof_ = true;
            }
            continue;
          } else {
            // Note that if buffer_ is non-empty, we have a truncated header at the
            // end of the file, which can be caused by the writer crashing in the
            // middle of writing the header. Instead of considering this an error,
            // just report EOF.
            buffer_.clear();
            return kEof;
          }
        }

        // Parse the header
        const char* header = buffer_.data();
        const uint32_t a = static_cast<uint32_t>(header[4]) & 0xff;
        const uint32_t b = static_cast<uint32_t>(header[5]) & 0xff;
        const unsigned int type = header[6];
        const uint32_t length = a | (b << 8);
        if (kHeaderSize + length > buffer_.size()) {
          size_t drop_size = buffer_.size();
          buffer_.clear();
          if (!eof_) {
            ReportCorruption(drop_size, "bad record length");
            return kBadRecord;
          }
          // If the end of the file has been reached without reading |length| bytes
          // of payload, assume the writer died in the middle of writing the record.
          // Don't report a corruption.
          return kEof;
        }

        if (type == kZeroType && length == 0) {
          // Skip zero length record without reporting any drops since
          // such records are produced by the mmap based writing code in
          // env_posix.cc that preallocates file regions.
          buffer_.clear();
          return kBadRecord;
        }

        // Check crc
        if (checksum_) {
          uint32_t expected_crc = crc32c::Unmask(DecodeFixed32(header));
          uint32_t actual_crc = crc32c::Value(header + 6, 1 + length);
          if (actual_crc != expected_crc) {
            // Drop the rest of the buffer since "length" itself may have
            // been corrupted and if we trust it, we could find some
            // fragment of a real log record that just happens to look
            // like a valid log record.
            size_t drop_size = buffer_.size();
            buffer_.clear();
            ReportCorruption(drop_size, "checksum mismatch");
            return kBadRecord;
          }
        }

        buffer_.remove_prefix(kHeaderSize + length);

        // Skip physical record that started before initial_offset_
        if (end_of_buffer_offset_ - buffer_.size() - kHeaderSize - length <
            initial_offset_) {
          result->clear();
          return kBadRecord;
        }

        *result = Slice(header + kHeaderSize, length);
        return type;
      }
        */
    }
}
