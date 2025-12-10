// ---------------- [ File: bitcoinleveldb-versionsetutil/src/some_file_overlaps_range.rs ]
crate::ix!();

/// Returns true iff some file in "files" overlaps the user key range [*smallest,*largest].
/// 
/// smallest==nullptr represents a key smaller than all keys in the DB.
/// 
/// largest==nullptr represents a key largest than all keys in the DB.
/// 
/// REQUIRES: If disjoint_sorted_files, files[] contains disjoint ranges in sorted order.
///
pub fn some_file_overlaps_range(
    icmp:                  &InternalKeyComparator,
    disjoint_sorted_files: bool,
    files:                 &Vec<*mut FileMetaData>,
    smallest_user_key_:    *const Slice,
    largest_user_key_:     *const Slice,
) -> bool {
    let user_cmp_ptr = icmp.user_comparator();

    unsafe {
        if user_cmp_ptr.is_null() {
            debug!(
                "some_file_overlaps_range: user_comparator is null; returning false"
            );
            return false;
        }

        let ucmp: &dyn SliceComparator = &*user_cmp_ptr;

        if !disjoint_sorted_files {
            // Need to check against all files
            for (i, &f_ptr) in files.iter().enumerate() {
                let f = &*f_ptr;

                let mut after_start = false;
                if !smallest_user_key_.is_null() {
                    let smallest = &*smallest_user_key_;
                    let f_largest_user = f.largest().user_key();
                    let cmp = ucmp.compare(smallest, &f_largest_user);
                    after_start = cmp > 0;
                }

                let mut before_limit = false;
                if !largest_user_key_.is_null() {
                    let largest = &*largest_user_key_;
                    let f_smallest_user = f.smallest().user_key();
                    let cmp = ucmp.compare(largest, &f_smallest_user);
                    before_limit = cmp < 0;
                }

                debug!(
                    index        = i,
                    file_number  = *f.number(),
                    after_start,
                    before_limit,
                    "some_file_overlaps_range: full-scan inspection"
                );

                if !(after_start || before_limit) {
                    trace!(
                        index       = i,
                        file_number = *f.number(),
                        "some_file_overlaps_range: overlap detected (non-disjoint)"
                    );
                    return true;
                }
            }

            trace!(
                file_count = files.len(),
                "some_file_overlaps_range: no overlap found (non-disjoint)"
            );
            return false;
        }

        // Binary search over file list (disjoint, sorted files)
        let mut index: usize = 0;
        if !smallest_user_key_.is_null() {
            let smallest = &*smallest_user_key_;
            let small_key =
                InternalKey::new(smallest, MAX_SEQUENCE_NUMBER, VALUE_TYPE_FOR_SEEK);
            let encoded = small_key.encode();
            let idx = find_file(icmp, files, &encoded);
            index = if idx > 0 { idx as usize } else { 0 };
        }

        if index >= files.len() {
            // beginning of range is after all files, so no overlap.
            trace!(
                index,
                file_count = files.len(),
                "some_file_overlaps_range: beginning of range after all files"
            );
            return false;
        }

        let target_f_ptr = files[index];
        let f = &*target_f_ptr;

        let mut before_limit = false;
        if !largest_user_key_.is_null() {
            let largest = &*largest_user_key_;
            let f_smallest_user = f.smallest().user_key();
            let cmp = ucmp.compare(largest, &f_smallest_user);
            before_limit = cmp < 0;
        }

        let overlap = !before_limit;

        trace!(
            index,
            file_number = *f.number(),
            before_limit,
            overlap,
            "some_file_overlaps_range: disjoint case result"
        );

        overlap
    }
}

#[cfg(test)]
mod overlaps_range_spec {
    use super::*;

    struct OverlapHarness {
        icmp: InternalKeyComparator,
        files_owned: Vec<Box<FileMetaData>>,
        file_ptrs: Vec<*mut FileMetaData>,
        disjoint_sorted_files: bool,
    }

    impl OverlapHarness {
        fn new(disjoint: bool) -> Self {
            OverlapHarness {
                icmp: InternalKeyComparator::new(bytewise_comparator()),
                files_owned: Vec::new(),
                file_ptrs: Vec::new(),
                disjoint_sorted_files: disjoint,
            }
        }

        fn add_with_seq(
            &mut self,
            smallest: &str,
            largest: &str,
            smallest_seq: u64,
            largest_seq: u64,
        ) {
            let smallest_slice = Slice::from(smallest);
            let largest_slice = Slice::from(largest);

            let smallest_ik =
                InternalKey::new(&smallest_slice, smallest_seq, ValueType::TypeValue);
            let largest_ik =
                InternalKey::new(&largest_slice, largest_seq, ValueType::TypeValue);

            let mut f = FileMetaData::default();
            let number = (self.file_ptrs.len() + 1) as u64;
            f.set_number(number);
            f.set_smallest(smallest_ik);
            f.set_largest(largest_ik);

            let mut boxed = Box::new(f);
            let ptr: *mut FileMetaData = &mut *boxed;

            trace!(
                file_number = number,
                smallest = smallest,
                largest = largest,
                "OverlapHarness::add_with_seq: created file"
            );

            self.files_owned.push(boxed);
            self.file_ptrs.push(ptr);
        }

        fn add(&mut self, smallest: &str, largest: &str) {
            self.add_with_seq(smallest, largest, 100, 100);
        }

        fn overlaps(&self, smallest: Option<&str>, largest: Option<&str>) -> bool {
            let mut s_slice = Slice::default();
            let mut l_slice = Slice::default();
            let mut smallest_ptr: *const Slice = core::ptr::null();
            let mut largest_ptr: *const Slice = core::ptr::null();

            if let Some(s) = smallest {
                s_slice = Slice::from(s);
                smallest_ptr = &s_slice as *const Slice;
            }
            if let Some(l) = largest {
                l_slice = Slice::from(l);
                largest_ptr = &l_slice as *const Slice;
            }

            let result = some_file_overlaps_range(
                &self.icmp,
                self.disjoint_sorted_files,
                &self.file_ptrs,
                smallest_ptr,
                largest_ptr,
            );

            debug!(
                smallest = smallest.unwrap_or("<null>"),
                largest = largest.unwrap_or("<null>"),
                result = result,
                "OverlapHarness::overlaps"
            );

            result
        }
    }

    #[traced_test]
    fn verify_overlaps_range_in_disjoint_sorted_case() {
        let mut h = OverlapHarness::new(true);
        h.add("150", "200");
        h.add("200", "250");
        h.add("300", "350");
        h.add("400", "450");

        assert!(!h.overlaps(Some("100"), Some("149")));
        assert!(!h.overlaps(Some("251"), Some("299")));
        assert!(!h.overlaps(Some("451"), Some("500")));
        assert!(!h.overlaps(Some("351"), Some("399")));

        assert!(h.overlaps(Some("100"), Some("150")));
        assert!(h.overlaps(Some("100"), Some("200")));
        assert!(h.overlaps(Some("100"), Some("300")));
        assert!(h.overlaps(Some("100"), Some("400")));
        assert!(h.overlaps(Some("100"), Some("500")));
        assert!(h.overlaps(Some("375"), Some("400")));
        assert!(h.overlaps(Some("450"), Some("450")));
        assert!(h.overlaps(Some("450"), Some("500")));
    }

    #[traced_test]
    fn verify_overlaps_range_with_null_boundaries() {
        let mut h = OverlapHarness::new(true);
        h.add("150", "200");
        h.add("200", "250");
        h.add("300", "350");
        h.add("400", "450");

        assert!(!h.overlaps(None, Some("149")));
        assert!(!h.overlaps(Some("451"), None));
        assert!(h.overlaps(None, None));
        assert!(h.overlaps(None, Some("150")));
        assert!(h.overlaps(None, Some("199")));
        assert!(h.overlaps(None, Some("200")));
        assert!(h.overlaps(None, Some("201")));
        assert!(h.overlaps(None, Some("400")));
        assert!(h.overlaps(None, Some("800")));
        assert!(h.overlaps(Some("100"), None));
        assert!(h.overlaps(Some("200"), None));
        assert!(h.overlaps(Some("449"), None));
        assert!(h.overlaps(Some("450"), None));
    }

    #[traced_test]
    fn verify_overlaps_range_sequence_sensitive() {
        let mut h = OverlapHarness::new(true);
        // same user key "200" but with a non-trivial internal-key sequence range
        h.add_with_seq("200", "200", 5000, 3000);

        assert!(!h.overlaps(Some("199"), Some("199")));
        assert!(!h.overlaps(Some("201"), Some("300")));
        assert!(h.overlaps(Some("200"), Some("200")));
        assert!(h.overlaps(Some("190"), Some("200")));
        assert!(h.overlaps(Some("200"), Some("210")));
    }

    #[traced_test]
    fn verify_overlaps_range_with_overlapping_files_non_disjoint() {
        let mut h = OverlapHarness::new(false);
        h.add("150", "600");
        h.add("400", "500");

        assert!(!h.overlaps(Some("100"), Some("149")));
        assert!(!h.overlaps(Some("601"), Some("700")));
        assert!(h.overlaps(Some("100"), Some("150")));
        assert!(h.overlaps(Some("100"), Some("200")));
        assert!(h.overlaps(Some("100"), Some("300")));
        assert!(h.overlaps(Some("100"), Some("400")));
        assert!(h.overlaps(Some("100"), Some("500")));
        assert!(h.overlaps(Some("375"), Some("400")));
        assert!(h.overlaps(Some("450"), Some("450")));
        assert!(h.overlaps(Some("450"), Some("500")));
        assert!(h.overlaps(Some("450"), Some("700")));
        assert!(h.overlaps(Some("600"), Some("700")));
    }
}
