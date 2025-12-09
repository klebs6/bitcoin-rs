// ---------------- [ File: bitcoinleveldb-versionsetutil/src/add_boundary_inputs.rs ]
crate::ix!();

/**
  | Extracts the largest file b1 from
  | |compaction_files| and then searches for a b2
  | in |level_files| for which user_key(u1)
  | = user_key(l2). If it finds such a file b2
  | (known as a boundary file) it adds it to
  | |compaction_files| and then searches again
  | using this new upper bound.
  |
  | If there are two blocks, b1=(l1, u1) and
  | b2=(l2, u2) and user_key(u1) = user_key(l2),
  | and if we compact b1 but not b2 then
  | a subsequent get operation will yield an
  | incorrect result because it will return the
  | record from b2 in level i rather than from b1
  | because it searches level by level for records
  | matching the supplied user key.
  |
  | parameters:
  |
  |   in     level_files:      List of files to
  |   search for boundary files.
  |
  |   in/out compaction_files: List of files to
  |   extend by adding boundary files.
  */
pub fn add_boundary_inputs(
        icmp:             &InternalKeyComparator,
        level_files:      &Vec<*mut FileMetaData>,
        compaction_files: *mut Vec<*mut FileMetaData>)  {
    
    todo!();
        /*
            InternalKey largest_key;

      // Quick return if compaction_files is empty.
      if (!FindLargestKey(icmp, *compaction_files, &largest_key)) {
        return;
      }

      bool continue_searching = true;
      while (continue_searching) {
        FileMetaData* smallest_boundary_file =
            FindSmallestBoundaryFile(icmp, level_files, largest_key);

        // If a boundary file was found advance largest_key, otherwise we're done.
        if (smallest_boundary_file != NULL) {
          compaction_files->push_back(smallest_boundary_file);
          largest_key = smallest_boundary_file->largest;
        } else {
          continue_searching = false;
        }
      }
        */
}
