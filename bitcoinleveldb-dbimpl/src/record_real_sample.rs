crate::ix!();

impl DBImpl {
    
    /**
      | Record a sample of bytes read at the
      | specified internal key.
      |
      | Samples are taken approximately once every
      | config::kReadBytesPeriod bytes.
      */
    pub fn record_read_sample(&mut self, key_: Slice)  {
        
        todo!();
        /*
            MutexLock l(&mutex_);
      if (versions_->current()->RecordReadSample(key)) {
        MaybeScheduleCompaction();
      }
        */
    }
}
