// ---------------- [ File: bitcoinleveldb-versionsetutil/src/some_file_overlaps_range.rs ]
crate::ix!();

/**
  | Returns true iff some file in "files" overlaps
  | the user key range [*smallest,*largest].
  |
  | smallest==nullptr represents a key smaller than
  | all keys in the DB.
  |
  | largest==nullptr represents a key largest than
  | all keys in the DB.
  |
  | REQUIRES: If disjoint_sorted_files, files[]
  |           contains disjoint ranges in sorted
  |           order.
  */
pub fn some_file_overlaps_range(
        icmp:                  &InternalKeyComparator,
        disjoint_sorted_files: bool,
        files:                 &Vec<*mut FileMetaData>,
        smallest_user_key_:     *const Slice,
        largest_user_key_:      *const Slice) -> bool {
    
    todo!();
        /*
            const Comparator* ucmp = icmp.user_comparator();
      if (!disjoint_sorted_files) {
        // Need to check against all files
        for (size_t i = 0; i < files.size(); i++) {
          const FileMetaData* f = files[i];
          if (AfterFile(ucmp, smallest_user_key, f) ||
              BeforeFile(ucmp, largest_user_key, f)) {
            // No overlap
          } else {
            return true;  // Overlap
          }
        }
        return false;
      }

      // Binary search over file list
      uint32_t index = 0;
      if (smallest_user_key != nullptr) {
        // Find the earliest possible internal key for smallest_user_key
        InternalKey small_key(*smallest_user_key, kMaxSequenceNumber,
                              kValueTypeForSeek);
        index = FindFile(icmp, files, small_key.Encode());
      }

      if (index >= files.size()) {
        // beginning of range is after all files, so no overlap.
        return false;
      }

      return !BeforeFile(ucmp, largest_user_key, files[index]);
        */
}
