// ---------------- [ File: bitcoinleveldb-log/src/log_reader_report.rs ]
crate::ix!();

impl LogReader {
    
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
}
