// ---------------- [ File: bitcoinleveldb-table/src/version_iterator.rs ]
crate::ix!();

/**
  | An internal iterator.  For a given
  | version/level pair, yields information about
  | the files in the level.  For a given entry,
  | key() is the largest key that occurs in the
  | file, and value() is an 16-byte value
  | containing the file number and file size, both
  | encoded using EncodeFixed64.
  */
pub struct VersionLevelFileNumIterator {
    base:      LevelDBIterator,

    icmp:      InternalKeyComparator,
    flist:     *const Vec<*mut FileMetaData>,
    index:     u32,

    /**
      | Backing store for value(). Holds the
      | file number and size.
      |
      */
    value_buf: [RefCell<u8>; 16],
}

impl VersionLevelFileNumIterator {
    
    pub fn new(
        icmp:  &InternalKeyComparator,
        flist: *const Vec<*mut FileMetaData>) -> Self {
    
        todo!();
        /*


            : icmp_(icmp), flist_(flist), index_(flist->size()) 
              // Marks as invalid
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return index_ < flist_->size();
        */
    }
    
    pub fn seek(&mut self, target: &Slice)  {
        
        todo!();
        /*
            index_ = FindFile(icmp_, *flist_, target);
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            index_ = 0;
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            index_ = flist_->empty() ? 0 : flist_->size() - 1;
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(Valid());
        index_++;
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            assert(Valid());
        if (index_ == 0) {
          index_ = flist_->size();  // Marks as invalid
        } else {
          index_--;
        }
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        return (*flist_)[index_]->largest.Encode();
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            assert(Valid());
        EncodeFixed64(value_buf_, (*flist_)[index_]->number);
        EncodeFixed64(value_buf_ + 8, (*flist_)[index_]->file_size);
        return Slice(value_buf_, sizeof(value_buf_));
        */
    }
    
    pub fn status(&self) -> Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
