crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/merger.h]

/**
  | Return an iterator that provided the union of
  | the data in children[0,n-1].  Takes ownership
  | of the child iterators and will delete them
  | when the result iterator is deleted.
  |
  | The result does no duplicate suppression.
  | I.e., if a particular key is present in K child
  | iterators, it will be yielded K times.
  |
  | REQUIRES: n >= 0
  */
pub fn new_merging_iterator(
        comparator: Box<dyn SliceComparator>,
        children:   *mut *mut LevelDBIterator,
        n:          i32) -> *mut LevelDBIterator {
    
    todo!();
        /*
            assert(n >= 0);
      if (n == 0) {
        return NewEmptyIterator();
      } else if (n == 1) {
        return children[0];
      } else {
        return new MergingIterator(comparator, children, n);
      }
        */
}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/merger.cc]

pub struct MergingIterator {
    base: LevelDBIterator,

    /**
      | We might want to use a heap in case there are
      |  lots of children.
      |
      | For now we use a simple array since we expect
      | a very small number of children in leveldb.
      */
    comparator: Box<dyn SliceComparator>,
    children:   *mut LevelDBIteratorWrapper,
    n:          i32,
    current:    *mut LevelDBIteratorWrapper,
    direction:  merging_iterator::Direction,
}

pub mod merging_iterator {

    /**
      | Which direction is the iterator moving?
      |
      */
    pub enum Direction { 
        Forward, 
        Reverse 
    }
}

impl Drop for MergingIterator {
    fn drop(&mut self) {
        todo!();
        /*
            delete[] children_;
        */
    }
}

impl MergingIterator {

    pub fn new(
        comparator: Box<dyn SliceComparator>,
        children:   *mut *mut LevelDBIterator,
        n:          i32) -> Self {
    
        todo!();
        /*


            : comparator_(comparator),
            children_(new LevelDBIteratorWrapper[n]),
            n_(n),
            current_(nullptr),
            direction_(kForward) 
        for (int i = 0; i < n; i++) {
          children_[i].Set(children[i]);
        }
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return (current_ != nullptr);
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < n_; i++) {
          children_[i].SeekToFirst();
        }
        FindSmallest();
        direction_ = kForward;
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < n_; i++) {
          children_[i].SeekToLast();
        }
        FindLargest();
        direction_ = kReverse;
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
            for (int i = 0; i < n_; i++) {
          children_[i].Seek(target);
        }
        FindSmallest();
        direction_ = kForward;
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(Valid());

        // Ensure that all children are positioned after key().
        // If we are moving in the forward direction, it is already
        // true for all of the non-current_ children since current_ is
        // the smallest child and key() == current_->key().  Otherwise,
        // we explicitly position the non-current_ children.
        if (direction_ != kForward) {
          for (int i = 0; i < n_; i++) {
            LevelDBIteratorWrapper* child = &children_[i];
            if (child != current_) {
              child->Seek(key());
              if (child->Valid() &&
                  comparator_->Compare(key(), child->key()) == 0) {
                child->Next();
              }
            }
          }
          direction_ = kForward;
        }

        current_->Next();
        FindSmallest();
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(Valid());

        // Ensure that all children are positioned before key().
        // If we are moving in the reverse direction, it is already
        // true for all of the non-current_ children since current_ is
        // the largest child and key() == current_->key().  Otherwise,
        // we explicitly position the non-current_ children.
        if (direction_ != kReverse) {
          for (int i = 0; i < n_; i++) {
            IteratorWrapper* child = &children_[i];
            if (child != current_) {
              child->Seek(key());
              if (child->Valid()) {
                // Child is at first entry >= key().  Step back one to be < key()
                child->Prev();
              } else {
                // Child has no entries >= key().  Position at last entry.
                child->SeekToLast();
              }
            }
          }
          direction_ = kReverse;
        }

        current_->Prev();
        FindLargest();
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return current_->key();
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return current_->value();
        */
    }
    
    pub fn status(&self) -> Status {
        
        todo!();
        /*
            Status status;
        for (int i = 0; i < n_; i++) {
          status = children_[i].status();
          if (!status.ok()) {
            break;
          }
        }
        return status;
        */
    }
    
    pub fn find_smallest(&mut self)  {
        
        todo!();
        /*
            IteratorWrapper* smallest = nullptr;
      for (int i = 0; i < n_; i++) {
        IteratorWrapper* child = &children_[i];
        if (child->Valid()) {
          if (smallest == nullptr) {
            smallest = child;
          } else if (comparator_->Compare(child->key(), smallest->key()) < 0) {
            smallest = child;
          }
        }
      }
      current_ = smallest;
        */
    }
    
    pub fn find_largest(&mut self)  {
        
        todo!();
        /*
            IteratorWrapper* largest = nullptr;
      for (int i = n_ - 1; i >= 0; i--) {
        IteratorWrapper* child = &children_[i];
        if (child->Valid()) {
          if (largest == nullptr) {
            largest = child;
          } else if (comparator_->Compare(child->key(), largest->key()) > 0) {
            largest = child;
          }
        }
      }
      current_ = largest;
        */
    }
}
