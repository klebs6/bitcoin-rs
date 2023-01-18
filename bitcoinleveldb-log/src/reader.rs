crate::ix!();

/**
  | Interface for reporting errors.
  |
  */
pub trait LogReaderReporter {

    /**
      | Some corruption was detected. "size"
      | is the approximate number of bytes dropped
      | due to the corruption.
      |
      */
    fn corruption(&mut self, 
        bytes:  usize,
        status: &Status);

}

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

impl Drop for LogReader {
    fn drop(&mut self) {
        todo!();
        /*
            delete[] backing_store_;
        */
    }
}

impl LogReader {

    /**
      | Create a reader that will return log records
      | from "*file".  "*file" must remain live while
      | this Reader is in use.
      |
      | If "reporter" is non-null, it is notified
      | whenever some data is dropped due to
      | a detected corruption.  "*reporter" must
      | remain live while this Reader is in use.
      |
      | If "checksum" is true, verify checksums if
      | available.
      |
      | The Reader will start reading at the first
      | record located at physical position >=
      | initial_offset within the file.
      */
    pub fn new(
        file:           Rc<RefCell<dyn SequentialFile>>,
        reporter:       Rc<RefCell<dyn LogReaderReporter>>,
        checksum:       bool,
        initial_offset: u64) -> Self {
    
        todo!();
        /*


            : file_(file),
          reporter_(reporter),
          checksum_(checksum),
          backing_store_(new char[kBlockSize]),
          buffer_(),
          eof_(false),
          last_record_offset_(0),
          end_of_buffer_offset_(0),
          initial_offset_(initial_offset),
          resyncing_(initial_offset > 0)
        */
    }
    
    /**
      | Skips all blocks that are completely before
      | "initial_offset_".
      |
      | Returns true on success. Handles reporting.
      */
    pub fn skip_to_initial_block(&mut self) -> bool {
        
        todo!();
        /*
            const size_t offset_in_block = initial_offset_ % kBlockSize;
      uint64_t block_start_location = initial_offset_ - offset_in_block;

      // Don't search a block if we'd be in the trailer
      if (offset_in_block > kBlockSize - 6) {
        block_start_location += kBlockSize;
      }

      end_of_buffer_offset_ = block_start_location;

      // Skip to start of first block that can contain the initial record
      if (block_start_location > 0) {
        Status skip_status = file_->Skip(block_start_location);
        if (!skip_status.ok()) {
          ReportDrop(block_start_location, skip_status);
          return false;
        }
      }

      return true;
        */
    }
    
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
    
    /**
      | Returns the physical offset of the last
      | record returned by ReadRecord.
      |
      | Undefined before the first call to
      | ReadRecord.
      */
    pub fn last_record_offset(&mut self) -> u64 {
        
        todo!();
        /*
            return last_record_offset_;
        */
    }
    
    /**
      | Reports dropped bytes to the reporter.
      | buffer_ must be updated to remove the
      | dropped bytes prior to invocation.
      |
      */
    pub fn report_corruption(&mut self, 
        bytes:  u64,
        reason: *const u8)  {
        
        todo!();
        /*
            ReportDrop(bytes, Status::Corruption(reason, file_->GetName()));
        */
    }
    
    pub fn report_drop(&mut self, 
        bytes:  u64,
        reason: &Status)  {
        
        todo!();
        /*
            if (reporter_ != nullptr &&
          end_of_buffer_offset_ - buffer_.size() - bytes >= initial_offset_) {
        reporter_->Corruption(static_cast<size_t>(bytes), reason);
      }
        */
    }
    
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
