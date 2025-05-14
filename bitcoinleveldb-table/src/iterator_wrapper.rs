// ---------------- [ File: bitcoinleveldb-table/src/iterator_wrapper.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/iterator_wrapper.h]

/**
  | A internal wrapper class with an interface
  | similar to Iterator that caches the valid() and
  | key() results for an underlying iterator.
  |
  | This can help avoid virtual function calls and
  | also gives better cache locality.
  */
pub struct LevelDBIteratorWrapper {
    iter:  *mut LevelDBIterator,
    valid: bool,
    key_:   Slice,
}

impl Default for LevelDBIteratorWrapper {
    
    fn default() -> Self {
        todo!();
        /*
        : iter(nullptr),
        : valid(false),

        
        */
    }
}

impl Drop for LevelDBIteratorWrapper {
    fn drop(&mut self) {
        todo!();
        /*
            delete iter_;
        */
    }
}

impl LevelDBIteratorWrapper {

    pub fn new(iter: *mut LevelDBIterator) -> Self {
    
        todo!();
        /*
        : iter(nullptr),

            Set(iter);
        */
    }
    
    pub fn iter(&self) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return iter_;
        */
    }

    /**
      | Takes ownership of "iter" and will delete
      | it when destroyed, or when Set() is invoked
      | again.
      |
      */
    pub fn set(&mut self, iter: *mut LevelDBIterator)  {
        
        todo!();
        /*
            delete iter_;
        iter_ = iter;
        if (iter_ == nullptr) {
          valid_ = false;
        } else {
          Update();
        }
        */
    }

    /**
      | Iterator interface methods
      |
      */
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return valid_;
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return key_;
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return iter_->value();
        */
    }

    /**
       Methods below require iter() != nullptr
      */
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            assert(iter_);
        return iter_->status();
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(iter_);
        iter_->Next();
        Update();
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(iter_);
        iter_->Prev();
        Update();
        */
    }
    
    pub fn seek(&mut self, k: &Slice)  {
        
        todo!();
        /*
            assert(iter_);
        iter_->Seek(k);
        Update();
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            assert(iter_);
        iter_->SeekToFirst();
        Update();
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            assert(iter_);
        iter_->SeekToLast();
        Update();
        */
    }
    
    pub fn update(&mut self)  {
        
        todo!();
        /*
            valid_ = iter_->Valid();
        if (valid_) {
          key_ = iter_->key();
        }
        */
    }
}
