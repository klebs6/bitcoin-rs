// ---------------- [ File: bitcoinleveldb-versionsetutil/src/find_smallest_boundary_file.rs ]
crate::ix!();

/**
  | Finds minimum file b2=(l2, u2) in level
  | file for which l2 > u1 and user_key(l2)
  | = user_key(u1)
  |
  */
pub fn find_smallest_boundary_file(
        icmp:        &InternalKeyComparator,
        level_files: &Vec<*mut FileMetaData>,
        largest_key_: &InternalKey) -> *mut FileMetaData {
    
    todo!();
        /*
            const Comparator* user_cmp = icmp.user_comparator();
      FileMetaData* smallest_boundary_file = nullptr;
      for (size_t i = 0; i < level_files.size(); ++i) {
        FileMetaData* f = level_files[i];
        if (icmp.Compare(f->smallest, largest_key) > 0 &&
            user_cmp->Compare(f->smallest.user_key(), largest_key.user_key()) ==
                0) {
          if (smallest_boundary_file == nullptr ||
              icmp.Compare(f->smallest, smallest_boundary_file->smallest) < 0) {
            smallest_boundary_file = f;
          }
        }
      }
      return smallest_boundary_file;
        */
}
