// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_internal_iterator.rs ]
crate::ix!();

impl DBImpl {
    
    pub fn new_internal_iterator(&mut self, 
        options:         &ReadOptions,
        latest_snapshot: *mut SequenceNumber,
        seed:            *mut u32) -> *mut LevelDBIterator {
        
        todo!();
        /*
            mutex_.Lock();
      *latest_snapshot = versions_->LastSequence();

      // Collect together all needed child iterators
      std::vector<Iterator*> list;
      list.push_back(mem_->NewIterator());
      mem_->Ref();
      if (imm_ != nullptr) {
        list.push_back(imm_->NewIterator());
        imm_->Ref();
      }
      versions_->current()->AddIterators(options, &list);
      Iterator* internal_iter =
          NewMergingIterator(&internal_comparator_, &list[0], list.size());
      versions_->current()->Ref();

      IterState* cleanup = new IterState(&mutex_, mem_, imm_, versions_->current());
      internal_iter->RegisterCleanup(CleanupIteratorState, cleanup, nullptr);

      *seed = ++seed_;
      mutex_.Unlock();
      return internal_iter;
        */
    }
    
    /**
      | Return an internal iterator over the current
      | state of the database.
      |
      | The keys of this iterator are internal keys
      | (see format.h).
      |
      | The returned iterator should be deleted when
      | no longer needed.
      */
    pub fn test_new_internal_iterator(&mut self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            SequenceNumber ignored;
      uint32_t ignored_seed;
      return NewInternalIterator(ReadOptions(), &ignored, &ignored_seed);
        */
    }

}
