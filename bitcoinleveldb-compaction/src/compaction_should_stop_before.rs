// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_should_stop_before.rs ]
crate::ix!();
    
impl Compaction {

    /// Returns true iff we should stop building the current output before
    /// processing "internal_key".
    /// 
    pub fn should_stop_before(&mut self, internal_key_: &Slice) -> bool {
        trace!(
            "Compaction::should_stop_before: enter; internal_key_len={}",
            *internal_key_.size()
        );

        let input_version_ptr: *mut Version = *self.input_version();
        if input_version_ptr.is_null() {
            debug!(
                "Compaction::should_stop_before: input_version_ is null; returning false"
            );
            return false;
        }

        unsafe {
            let input_version: &mut Version = &mut *input_version_ptr;
            let vset_ptr: *mut dyn VersionSetInterface = input_version.vset();
            assert!(
                !vset_ptr.is_null(),
                "Compaction::should_stop_before: vset pointer is null"
            );

            let vset: &mut dyn VersionSetInterface = &mut *vset_ptr;
            let icmp: &InternalKeyComparator = vset.icmp();
            let options_ptr: *const Options = vset.options();

            // Scan to find earliest grandparent file that contains key.
            while *self.grandparent_index() < self.grandparents().len() {
                let gpi: usize = *self.grandparent_index();
                let fptr: *mut FileMetaData = self.grandparents()[gpi];
                assert!(
                    !fptr.is_null(),
                    "Compaction::should_stop_before: null grandparent FileMetaData pointer at index {}",
                    gpi
                );

                let f: &FileMetaData = &*fptr;
                let largest_encoded: Slice = f.largest().encode();
                let cmp_result =
                    icmp.compare(internal_key_, &largest_encoded);

                trace!(
                    "Compaction::should_stop_before: grandparent_index={} file_number={} cmp_result={}",
                    gpi,
                    *f.number(),
                    cmp_result
                );

                if cmp_result <= 0 {
                    break;
                }

                if *self.seen_key_() {
                    let file_sz: u64 = *f.file_size();
                    self.set_overlapped_bytes(
                        self.overlapped_bytes().saturating_add(file_sz as i64)
                    );
                    trace!(
                        "Compaction::should_stop_before: accumulated overlapped_bytes={}",
                        self.overlapped_bytes()
                    );
                }

                *self.grandparent_index_mut() += 1;
            }

            self.set_seen_key_(true);

            let max_overlap: i64 = max_grand_parent_overlap_bytes(options_ptr);

            let should_stop = *self.overlapped_bytes() > max_overlap;

            debug!(
                "Compaction::should_stop_before: overlapped_bytes={} max_overlap={} -> should_stop={}",
                self.overlapped_bytes(),
                max_overlap,
                should_stop
            );

            if should_stop {
                // Too much overlap for current output; start new output
                self.set_overlapped_bytes(0);
                true
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod compaction_stop_before_threshold_tests {
    use super::*;

    #[traced_test]
    fn should_stop_before_with_null_input_version_is_false() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 0);
        let internal = Slice::from("internal-key");
        assert!(!c.should_stop_before(&internal));
    }

    #[traced_test]
    fn should_stop_before_with_null_input_version_is_stable_across_calls() {
        let opts = Options::default();
        let mut c = Compaction::new(&opts as *const Options, 0);
        let internal = Slice::from("internal-key");

        assert!(!c.should_stop_before(&internal));
        assert!(!c.should_stop_before(&internal));
    }
}
