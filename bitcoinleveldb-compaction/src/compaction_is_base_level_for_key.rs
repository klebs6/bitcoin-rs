// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_is_base_level_for_key.rs ]
crate::ix!();
    
impl Compaction {

    /// Returns true if the information we have available guarantees that the
    /// compaction is producing data in "level+1" for which no data exists in
    /// levels greater than "level+1".
    ///
    pub fn is_base_level_for_key(&mut self, user_key_: &Slice) -> bool {
        trace!(
            "Compaction::is_base_level_for_key: enter; level={}, user_key_len={}",
            self.level(),
            *user_key_.size()
        );

        let input_version_ptr: *mut Version = *self.input_version();
        if input_version_ptr.is_null() {
            trace!(
                "Compaction::is_base_level_for_key: input_version_ is null; treating as base level"
            );
            return true;
        }

        unsafe {
            let input_version: &mut Version = &mut *input_version_ptr;

            let vset_ptr: *mut dyn VersionSetInterface = input_version.vset();
            assert!(
                !vset_ptr.is_null(),
                "Compaction::is_base_level_for_key: vset pointer is null"
            );

            let vset: &mut dyn VersionSetInterface = &mut *vset_ptr;
            let icmp: &InternalKeyComparator = vset.icmp();

            // Maybe use binary search to find right entry instead of linear search?
            let user_cmp_ptr: *const dyn SliceComparator = icmp.user_comparator();

            assert!(
                !user_cmp_ptr.is_null(),
                "Compaction::is_base_level_for_key: user comparator pointer is null"
            );

            let user_cmp: &dyn SliceComparator = &*user_cmp_ptr;

            let mut lvl: i32 = self.level() + 2;

            while lvl < NUM_LEVELS as i32 {
                let files_for_level: &Vec<*mut FileMetaData> =
                    &input_version.files()[lvl as usize];

                while self.level_ptrs()[lvl as usize] < files_for_level.len() {
                    let index = self.level_ptrs()[lvl as usize];
                    let fptr = files_for_level[index];
                    assert!(
                        !fptr.is_null(),
                        "Compaction::is_base_level_for_key: null FileMetaData pointer at level {} index {}",
                        lvl,
                        index
                    );

                    let f: &FileMetaData = &*fptr;
                    let largest_user: Slice = f.largest().user_key();
                    let cmp_largest = user_cmp.compare(user_key_, &largest_user);

                    trace!(
                        "Compaction::is_base_level_for_key: lvl={} idx={} cmp_largest={}",
                        lvl,
                        index,
                        cmp_largest
                    );

                    if cmp_largest <= 0 {
                        // We've advanced far enough
                        let smallest_user: Slice = f.smallest().user_key();
                        let cmp_smallest =
                            user_cmp.compare(user_key_, &smallest_user);

                        trace!(
                            "Compaction::is_base_level_for_key: lvl={} idx={} cmp_smallest={}",
                            lvl,
                            index,
                            cmp_smallest
                        );

                        if cmp_smallest >= 0 {
                            // Key falls in this file's range, so definitely not base level
                            debug!(
                                "Compaction::is_base_level_for_key: key overlaps level {} file {} – not base level",
                                lvl,
                                *f.number()
                            );
                            return false;
                        }
                        break;
                    }

                    self.level_ptrs_mut()[lvl as usize] += 1;
                }

                lvl += 1;
            }

            trace!(
                "Compaction::is_base_level_for_key: no overlapping files in levels above {}; base level",
                self.level() + 1
            );
            true
        }
    }
}

#[cfg(test)]
mod compaction_base_level_for_key_null_version_tests {
    use super::*;

    #[traced_test]
    fn base_level_for_key_with_null_input_version_defaults_to_true() {
        let opts = Options::default();
        let mut compaction = Compaction::new(&opts as *const Options, 0);

        let key = Slice::from("user-key");
        assert!(compaction.is_base_level_for_key(&key));
    }

    #[traced_test]
    fn base_level_for_key_with_null_input_version_is_stable_across_calls() {
        let opts = Options::default();
        let mut compaction = Compaction::new(&opts as *const Options, 0);

        let key = Slice::from("user-key");

        assert!(compaction.is_base_level_for_key(&key));
        assert!(compaction.is_base_level_for_key(&key));
    }
}
