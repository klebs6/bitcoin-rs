// ---------------- [ File: bitcoinleveldb-versionsetutil/src/find_largest_key.rs ]
crate::ix!();

/**
  | Finds the largest key in a vector of files.
  | Returns true if files it not empty.
  |
  */
pub fn find_largest_key(
        icmp:        &InternalKeyComparator,
        files:       &Vec<*mut FileMetaData>,
        largest_key_: *mut InternalKey) -> bool {
    
    todo!();
        /*
            if (files.empty()) {
        return false;
      }
      *largest_key = files[0]->largest;
      for (size_t i = 1; i < files.size(); ++i) {
        FileMetaData* f = files[i];
        if (icmp.Compare(f->largest, *largest_key) > 0) {
          *largest_key = f->largest;
        }
      }
      return true;
        */
}
