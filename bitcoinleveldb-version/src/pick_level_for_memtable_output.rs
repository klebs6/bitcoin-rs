// ---------------- [ File: bitcoinleveldb-version/src/pick_level_for_memtable_output.rs ]
crate::ix!();

impl Version {

    /**
      | Return the level at which we should place
      | a new memtable compaction result that covers
      | the range
      | [smallest_user_key,largest_user_key].
      */
    pub fn pick_level_for_mem_table_output(
        &mut self,
        smallest_user_key_: &Slice,
        largest_user_key_:  &Slice,
    ) -> i32 {
        trace!(
            "Version::pick_level_for_mem_table_output: enter; smallest_len={}, largest_len={}",
            *smallest_user_key_.size(),
            *largest_user_key_.size()
        );

        const MAX_MEM_COMPACT_LEVEL: i32 = 2;

        let mut level: i32 = 0;

        unsafe {
            let smallest_ptr: *const Slice = smallest_user_key_ as *const Slice;
            let largest_ptr:  *const Slice = largest_user_key_  as *const Slice;

            if !self.overlap_in_level(0, smallest_ptr, largest_ptr) {
                // Push to next level if there is no overlap in next level,
                // and the #bytes overlapping in the level after that are limited.
                let start = InternalKey::new(
                    smallest_user_key_,
                    MAX_SEQUENCE_NUMBER,
                    VALUE_TYPE_FOR_SEEK,
                );
                let limit = InternalKey::new(
                    largest_user_key_,
                    0,
                    ValueType::TypeDeletion,
                );

                let mut overlaps: Vec<*mut FileMetaData> = Vec::new();

                while level < MAX_MEM_COMPACT_LEVEL {
                    if self.overlap_in_level(
                        level + 1,
                        smallest_ptr,
                        largest_ptr,
                    ) {
                        trace!(
                            "Version::pick_level_for_mem_table_output: overlap found at level {}; stopping",
                            level + 1
                        );
                        break;
                    }

                    if level + 2 < NUM_LEVELS as i32 {

                        self.get_overlapping_inputs(
                            level + 2,
                            &start as *const InternalKey,
                            &limit as *const InternalKey,
                            &mut overlaps as *mut Vec<*mut FileMetaData>,
                        );

                        let sum       = total_file_size(&overlaps);
                        let vset_ptr  = self.vset();
                        assert!(
                            !vset_ptr.is_null(),
                            "Version::pick_level_for_mem_table_output: vset pointer must not be null when computing max_grand_parent_overlap_bytes"
                        );
                        let opts_ptr  = (*vset_ptr).options();
                        let max_bytes = max_grand_parent_overlap_bytes(opts_ptr);

                        trace!(
                            "Version::pick_level_for_mem_table_output: level={} grandparent_bytes={}; max_bytes={}",
                            level,
                            sum,
                            max_bytes
                        );

                        if sum > max_bytes {
                            trace!(
                                "Version::pick_level_for_mem_table_output: grandparent overlap exceeds limit; stopping at level={}",
                                level
                            );
                            break;
                        }
                    }

                    level += 1;
                }
            }
        }

        trace!(
            "Version::pick_level_for_mem_table_output: chosen level={}",
            level
        );
        level
    }
}

#[cfg(test)]
mod version_pick_level_for_memtable_output_signature_tests {
    use super::*;

    #[traced_test]
    fn pick_level_for_mem_table_output_signature_is_stable() {
        let _fn_ptr: fn(
            &mut Version,
            &Slice,
            &Slice,
        ) -> i32 = Version::pick_level_for_mem_table_output;
        let _ = _fn_ptr;
    }
}
