// ---------------- [ File: bitcoinleveldb-versionsetutil/src/find_file.rs ]
crate::ix!();

/**
  | Return the smallest index i such that
  | files[i]->largest >= key.
  |
  | Return files.size() if there is no such file.
  |
  | REQUIRES: "files" contains a sorted list of
  | non-overlapping files.
  */
pub fn find_file(
        icmp:  &InternalKeyComparator,
        files: &Vec<*mut FileMetaData>,
        key_:   &Slice) -> i32 {
    
    todo!();
        /*
            uint32_t left = 0;
      uint32_t right = files.size();
      while (left < right) {
        uint32_t mid = (left + right) / 2;
        const FileMetaData* f = files[mid];
        if (icmp.InternalKeyComparator::Compare(f->largest.Encode(), key) < 0) {
          // Key at "mid.largest" is < "target".  Therefore all
          // files at or before "mid" are uninteresting.
          left = mid + 1;
        } else {
          // Key at "mid.largest" is >= "target".  Therefore all files
          // after "mid" are uninteresting.
          right = mid;
        }
      }
      return right;
        */
}
