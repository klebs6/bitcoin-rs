crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/iterator.cc]

impl Drop for LevelDBIteratorInner {
    fn drop(&mut self) {
        todo!();
        /*
            if (!cleanup_head_.IsEmpty()) {
        cleanup_head_.Run();
        for (CleanupNode* node = cleanup_head_.next; node != nullptr;) {
          node->Run();
          CleanupNode* next_node = node->next;
          delete node;
          node = next_node;
        }
      }
        */
    }
}

impl LevelDBIteratorInner {
    
    pub fn new() -> Self {
    
        todo!();
        /*

            cleanup_head_.function = nullptr;
      cleanup_head_.next = nullptr;
        */
    }
    
    /**
      | Clients are allowed to register
      | function/arg1/arg2 triples that will be
      | invoked when this iterator is destroyed.
      |
      | Note that unlike all of the preceding
      | methods, this method is not abstract and
      | therefore clients should not override it.
      */
    pub fn register_cleanup(&mut self, 
        func: LevelDBIteratorCleanupFunction,
        arg1: *mut c_void,
        arg2: *mut c_void)  {
        
        todo!();
        /*
            assert(func != nullptr);
      CleanupNode* node;
      if (cleanup_head_.IsEmpty()) {
        node = &cleanup_head_;
      } else {
        node = new CleanupNode();
        node->next = cleanup_head_.next;
        cleanup_head_.next = node;
      }
      node->function = func;
      node->arg1 = arg1;
      node->arg2 = arg2;
        */
    }
}

///------------------------
pub struct EmptyIterator {
    base:   LevelDBIterator,
    status: Status,
}

impl EmptyIterator {
    
    pub fn new(s: &Status) -> Self {
    
        todo!();
        /*
        : status(s),

        
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return false;
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
        
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(false);
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(false);
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(false);
        return Slice();
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(false);
        return Slice();
        */
    }
    
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            return status_;
        */
    }
}

pub fn new_empty_iterator() -> *mut LevelDBIterator {
    
    todo!();
        /*
            return new EmptyIterator(Status::OK());
        */
}

pub fn new_error_iterator(status: &Status) -> *mut LevelDBIterator {
    
    todo!();
        /*
            return new EmptyIterator(status);
        */
}
