// ---------------- [ File: bitcoinleveldb-version/src/create.rs ]
crate::ix!();

impl Version {

    pub fn new(vset: *mut VersionSet) -> Self {
    
        todo!();
        /*
        : vset(vset),
        : next(this),
        : prev(this),
        : refs(0),
        : file_to_compact(nullptr),
        : file_to_compact_level(-1),
        : compaction_score(-1),
        : compaction_level(-1),

        
        */
    }
    
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
