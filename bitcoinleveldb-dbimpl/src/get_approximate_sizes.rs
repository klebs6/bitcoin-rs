// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_approximate_sizes.rs ]
crate::ix!();

impl GetApproximateSizes for DBImpl {
    
    fn get_approximate_sizes(&mut self, 
        range: *const db::Range,
        n:     i32,
        sizes: *mut u64)  {
        
        todo!();
        /*
            // TODO(opt): better implementation
      MutexLock l(&mutex_);
      Version* v = versions_->current();
      v->Ref();

      for (int i = 0; i < n; i++) {
        // Convert user_key into a corresponding internal key.
        InternalKey k1(range[i].start, kMaxSequenceNumber, kValueTypeForSeek);
        InternalKey k2(range[i].limit, kMaxSequenceNumber, kValueTypeForSeek);
        uint64_t start = versions_->ApproximateOffsetOf(v, k1);
        uint64_t limit = versions_->ApproximateOffsetOf(v, k2);
        sizes[i] = (limit >= start ? limit - start : 0);
      }

      v->Unref();
        */
    }
}
