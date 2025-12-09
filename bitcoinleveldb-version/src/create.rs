// ---------------- [ File: bitcoinleveldb-version/src/create.rs ]
crate::ix!();

impl Version {
    
    pub fn new_concatenating_iterator(&self, 
        options: &ReadOptions,
        level:   i32) -> *mut LevelDBIterator {
        
        todo!();
        /*
            return NewTwoLevelIterator(
          new LevelFileNumIterator(vset_->icmp_, &files_[level]), &GetFileIterator,
          vset_->table_cache_, options);
        */
    }
}
