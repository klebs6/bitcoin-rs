// ---------------- [ File: bitcoinleveldb-versionsetutil/src/by_smallest_key.rs ]
crate::ix!();

/**
  | Helper to sort by
  | v->files_[file_number].smallest
  |
  */
pub struct BySmallestKeyComparator {
    internal_comparator: *const InternalKeyComparator,
}

impl BySmallestKeyComparator {

    pub fn invoke(&self, 
        f1: *mut FileMetaData,
        f2: *mut FileMetaData) -> bool {
        
        todo!();
        /*
            int r = internal_comparator->Compare(f1->smallest, f2->smallest);
          if (r != 0) {
            return (r < 0);
          } else {
            // Break ties by file number
            return (f1->number < f2->number);
          }
        */
    }
}
