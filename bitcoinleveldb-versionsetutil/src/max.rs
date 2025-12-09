// ---------------- [ File: bitcoinleveldb-versionsetutil/src/max.rs ]
crate::ix!();

/**
  | Maximum bytes of overlaps in grandparent
  | (i.e., level+2) before we stop building
  | a single file in a level->level+1 compaction.
  |
  */
pub fn max_grand_parent_overlap_bytes(options: *const Options) -> i64 {
    
    todo!();
        /*
            return 10 * TargetFileSize(options);
        */
}

pub fn max_bytes_for_level(
        options: *const Options,
        level:   i32) -> f64 {
    
    todo!();
        /*
            // Note: the result for level zero is not really used since we set
      // the level-0 compaction threshold based on number of files.

      // Result for both level-0 and level-1
      double result = 10. * 1048576.0;
      while (level > 1) {
        result *= 10;
        level--;
      }
      return result;
        */
}

pub fn max_file_size_for_level(
        options: *const Options,
        level:   i32) -> u64 {
    
    todo!();
        /*
            // We could vary per level to reduce number of files?
      return TargetFileSize(options);
        */
}
