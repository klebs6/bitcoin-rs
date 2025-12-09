// ---------------- [ File: bitcoinleveldb-versionsetutil/src/after_before.rs ]
crate::ix!();

pub fn after_file(
        ucmp:     Box<dyn SliceComparator>,
        user_key_: *const Slice,
        f:        *const FileMetaData) -> bool {
    
    todo!();
        /*
            // null user_key occurs before all keys and is therefore never after *f
      return (user_key != nullptr &&
              ucmp->Compare(*user_key, f->largest.user_key()) > 0);
        */
}

pub fn before_file(
        ucmp:     Box<dyn SliceComparator>,
        user_key_: *const Slice,
        f:        *const FileMetaData) -> bool {
    
    todo!();
        /*
            // null user_key occurs after all keys and is therefore never before *f
      return (user_key != nullptr &&
              ucmp->Compare(*user_key, f->smallest.user_key()) < 0);
        */
}
