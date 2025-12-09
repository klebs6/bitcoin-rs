// ---------------- [ File: bitcoinleveldb-version/src/overlap_in_level.rs ]
crate::ix!();

impl Version {
    
    /**
      | Returns true iff some file in the specified
      | level overlaps some part of
      | [*smallest_user_key,*largest_user_key].
      |
      | smallest_user_key==nullptr represents a key
      | smaller than all the DB's keys.
      |
      | largest_user_key==nullptr represents a key
      | largest than all the DB's keys.
      */
    pub fn overlap_in_level(&mut self, 
        level:             i32,
        smallest_user_key_: *const Slice,
        largest_user_key_:  *const Slice) -> bool {
        
        todo!();
        /*
            return SomeFileOverlapsRange(vset_->icmp_, (level > 0), files_[level],
                                   smallest_user_key, largest_user_key);
        */
    }
}
