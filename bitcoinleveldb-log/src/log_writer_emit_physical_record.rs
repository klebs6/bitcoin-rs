// ---------------- [ File: bitcoinleveldb-log/src/log_writer_emit_physical_record.rs ]
crate::ix!();

impl LogWriter {
    
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
