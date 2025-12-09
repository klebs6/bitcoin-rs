// ---------------- [ File: bitcoinleveldb-dbimpl/src/build_batch_group.rs ]
crate::ix!();

impl DBImpl {
    
    /**
      | REQUIRES: Writer list must be non-empty
      |
      | REQUIRES: First writer must have a non-null
      | batch
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn build_batch_group(&mut self, last_writer: *mut *mut DBImplWriter) -> *mut WriteBatch {
        
        todo!();
        /*
            mutex_.AssertHeld();
      assert(!writers_.empty());
      Writer* first = writers_.front();
      WriteBatch* result = first->batch;
      assert(result != nullptr);

      size_t size = WriteBatchInternal::ByteSize(first->batch);

      // Allow the group to grow up to a maximum size, but if the
      // original write is small, limit the growth so we do not slow
      // down the small write too much.
      size_t max_size = 1 << 20;
      if (size <= (128 << 10)) {
        max_size = size + (128 << 10);
      }

      *last_writer = first;
      std::deque<Writer*>::iterator iter = writers_.begin();
      ++iter;  // Advance past "first"
      for (; iter != writers_.end(); ++iter) {
        Writer* w = *iter;
        if (w->sync && !first->sync) {
          // Do not include a sync write into a batch handled by a non-sync write.
          break;
        }

        if (w->batch != nullptr) {
          size += WriteBatchInternal::ByteSize(w->batch);
          if (size > max_size) {
            // Do not make batch too big
            break;
          }

          // Append to *result
          if (result == first->batch) {
            // Switch to temporary batch instead of disturbing caller's batch
            result = tmp_batch_;
            assert(WriteBatchInternal::Count(result) == 0);
            WriteBatchInternal::Append(result, first->batch);
          }
          WriteBatchInternal::Append(result, w->batch);
        }
        *last_writer = w;
      }
      return result;
        */
    }
}
