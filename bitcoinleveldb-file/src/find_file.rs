// ---------------- [ File: bitcoinleveldb-file/src/find_file.rs ]
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
    key_:  &Slice,
) -> i32 {
    use tracing::{trace, debug};

    trace!("find_file: begin binary search over {} files", files.len());

    let mut left:  u32 = 0;
    let mut right: u32 = files.len() as u32;

    while left < right {
        let mid = (left + right) / 2;

        // SAFETY:
        // The original C++ used raw pointers. We preserve this exactly.
        // The caller provides a Vec<*mut FileMetaData> and the semantics
        // require dereferencing these pointers just as in C++.
        let f_ptr = files[mid as usize];
        let f: &FileMetaData = unsafe {
            &*f_ptr
        };

        let largest_encoded = f.largest().encode();
        let cmp = icmp.compare(&largest_encoded, key_);

        debug!(
            mid       = mid,
            left      = left,
            right     = right,
            cmp       = cmp,
            file_num  = f.number(),
            file_size = f.file_size(),
            "find_file: inspection step"
        );

        if cmp < 0 {
            // f->largest < key  → all files ≤ mid are uninteresting
            left = mid + 1;
        } else {
            // f->largest ≥ key → mid might contain answer
            right = mid;
        }
    }

    trace!("find_file: completed with index={}", right);

    right as i32
}


#[cfg(test)]
mod find_file_leveldb_semantics_tests {
    use super::*;

    /// Helper: build a valid InternalKey for a given user key and sequence number.
    fn build_internal_key_for_tests(user_key: &str, seq: SequenceNumber) -> InternalKey {
        trace!(
            "build_internal_key_for_tests: user_key={}, seq={}",
            user_key,
            seq
        );

        let user_bytes = user_key.as_bytes();
        let user_slice = Slice::from(user_bytes);

        InternalKey::new(&user_slice, seq, ValueType::TypeValue)
    }

    /// Helper: build a FileMetaData with the given smallest/largest user-key range.
    ///
    /// The range is defined purely in terms of user keys; sequence and value type
    /// are fixed for test purposes and are consistent across all files in a test.
    fn build_file_meta_for_tests(
        file_number: u64,
        smallest_user_key: &str,
        largest_user_key: &str,
        seq: SequenceNumber,
    ) -> *mut FileMetaData {
        trace!(
            "build_file_meta_for_tests: file_number={}, smallest_user_key={}, largest_user_key={}, seq={}",
            file_number,
            smallest_user_key,
            largest_user_key,
            seq
        );

        let smallest = build_internal_key_for_tests(smallest_user_key, seq);
        let largest  = build_internal_key_for_tests(largest_user_key,  seq);

        let mut meta = FileMetaData::default();

        // Keep ref-counting and seek budget consistent with default semantics.
        meta.set_refs(0);
        meta.set_allowed_seeks(1 << 30);
        meta.set_number(file_number);
        meta.set_file_size(0);
        meta.set_smallest(smallest);
        meta.set_largest(largest);

        let boxed = Box::new(meta);
        let ptr   = Box::into_raw(boxed);

        debug!(
            file_number = file_number,
            smallest_user_key = smallest_user_key,
            largest_user_key  = largest_user_key,
            ptr = ?ptr,
            "build_file_meta_for_tests: constructed FileMetaData"
        );

        ptr
    }

    /// Helper: construct an InternalKeyComparator using the null (bytewise) comparator.
    fn make_test_comparator() -> InternalKeyComparator {
        trace!("make_test_comparator: constructing InternalKeyComparator with null_slice_comparator");
        let cmp = InternalKeyComparator::new(null_slice_comparator());
        debug!(
            user_comparator_ptr = ?cmp.user_comparator(),
            "make_test_comparator: constructed comparator"
        );
        cmp
    }

    #[traced_test]
    fn find_file_returns_zero_for_empty_file_vector() {
        trace!("find_file_returns_zero_for_empty_file_vector: start");

        let icmp: InternalKeyComparator = make_test_comparator();
        let files: Vec<*mut FileMetaData> = Vec::new();

        let seq: SequenceNumber = 100;
        let key_internal        = build_internal_key_for_tests("k", seq);
        let key_slice           = key_internal.encode();

        let index = find_file(&icmp, &files, &key_slice);

        debug!(
            index,
            num_files = files.len(),
            key_user = "k",
            "find_file on empty file list"
        );

        assert_eq!(
            index, 0,
            "Expected index 0 (files.len()) when searching an empty file list"
        );

        trace!("find_file_returns_zero_for_empty_file_vector: end");
    }

    #[traced_test]
    fn find_file_binary_search_over_three_disjoint_ranges() {
        trace!("find_file_binary_search_over_three_disjoint_ranges: start");

        let icmp: InternalKeyComparator = make_test_comparator();
        let seq: SequenceNumber         = 50;

        // Three disjoint, sorted ranges in internal-key order:
        //   file 0: ["a", "c"]
        //   file 1: ["f", "h"]
        //   file 2: ["k", "n"]
        let f0 = build_file_meta_for_tests(0, "a", "c", seq);
        let f1 = build_file_meta_for_tests(1, "f", "h", seq);
        let f2 = build_file_meta_for_tests(2, "k", "n", seq);

        let files: Vec<*mut FileMetaData> = vec![f0, f1, f2];

        // Each case is (user_key, expected_index).
        let cases: [(&str, i32); 13] = [
            // Before the first file's smallest user key
            ("0", 0),
            // Inside the first file's range
            ("a", 0),
            ("b", 0),
            ("c", 0),
            // Between file 0 and file 1 (no range contains "d"),
            // but the first file with largest >= key is file 1.
            ("d", 1),
            // Inside the second file's range
            ("f", 1),
            ("g", 1),
            ("h", 1),
            // Between file 1 and file 2
            ("i", 2),
            // Inside the third file's range
            ("k", 2),
            ("m", 2),
            ("n", 2),
            // After all ranges: index == files.len()
            ("o", 3),
        ];

        for (user_key, expected_index) in cases.iter() {
            let key_internal = build_internal_key_for_tests(*user_key, seq);
            let key_slice    = key_internal.encode();

            let actual_index = find_file(&icmp, &files, &key_slice);

            debug!(
                user_key = *user_key,
                expected_index = *expected_index,
                actual_index,
                "find_file case evaluation for three-range configuration"
            );

            assert_eq!(
                actual_index,
                *expected_index,
                "Unexpected index for key {:?} in three-range test",
                user_key
            );
        }

        unsafe {
            drop(Box::from_raw(f0));
            drop(Box::from_raw(f1));
            drop(Box::from_raw(f2));
        }

        trace!("find_file_binary_search_over_three_disjoint_ranges: end");
    }

    #[traced_test]
    fn find_file_single_file_edge_cases_across_key_space() {
        trace!("find_file_single_file_edge_cases_across_key_space: start");

        let icmp: InternalKeyComparator = make_test_comparator();
        let seq: SequenceNumber         = 75;

        // Single file covering ["d", "m"] in user-key space.
        let f = build_file_meta_for_tests(7, "d", "m", seq);
        let files: Vec<*mut FileMetaData> = vec![f];

        // Keys spanning before, inside, and after the single range.
        let cases: [(&str, i32); 5] = [
            // Before smallest_user_key: still index 0 because largest >= key.
            ("b", 0),
            // At the boundaries and inside the file range.
            ("d", 0),
            ("h", 0),
            ("m", 0),
            // After largest_user_key: index == files.len() == 1.
            ("z", 1),
        ];

        for (user_key, expected_index) in cases.iter() {
            let key_internal = build_internal_key_for_tests(*user_key, seq);
            let key_slice    = key_internal.encode();

            let actual_index = find_file(&icmp, &files, &key_slice);

            debug!(
                user_key = *user_key,
                expected_index = *expected_index,
                actual_index,
                "find_file single-file edge-case evaluation"
            );

            assert_eq!(
                actual_index,
                *expected_index,
                "Unexpected index for key {:?} in single-file test",
                user_key
            );
        }

        unsafe {
            drop(Box::from_raw(f));
        }

        trace!("find_file_single_file_edge_cases_across_key_space: end");
    }
}

#[cfg(test)]
mod find_file_and_overlap_spec {
    use super::*;

    struct FindFileHarness {
        icmp: InternalKeyComparator,
        files_owned: Vec<Box<FileMetaData>>,
        file_ptrs: Vec<*mut FileMetaData>,
        disjoint_sorted_files: bool,
    }

    impl FindFileHarness {
        fn new() -> Self {
            FindFileHarness {
                icmp: InternalKeyComparator::new(bytewise_comparator()),
                files_owned: Vec::new(),
                file_ptrs: Vec::new(),
                disjoint_sorted_files: true,
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
                "FindFileHarness::add_with_seq: created file"
            );

            self.files_owned.push(boxed);
            self.file_ptrs.push(ptr);
        }

        fn add(&mut self, smallest: &str, largest: &str) {
            self.add_with_seq(smallest, largest, 100, 100);
        }

        fn find(&self, key: &str) -> i32 {
            let key_slice = Slice::from(key);
            let internal = InternalKey::new(&key_slice, 100, ValueType::TypeValue);
            let encoded = internal.encode();

            let idx = find_file(&self.icmp, &self.file_ptrs, &encoded);

            debug!(
                key = key,
                index = idx,
                "FindFileHarness::find: completed search"
            );

            idx
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
                "FindFileHarness::overlaps: overlap check"
            );

            result
        }
    }

    #[traced_test]
    fn verify_find_file_on_empty_set() {
        let harness = FindFileHarness::new();

        assert_eq!(0, harness.find("foo"));
        assert!(!harness.overlaps(Some("a"), Some("z")));
        assert!(!harness.overlaps(None, Some("z")));
        assert!(!harness.overlaps(Some("a"), None));
        assert!(!harness.overlaps(None, None));
    }

    #[traced_test]
    fn verify_find_file_with_single_file() {
        let mut harness = FindFileHarness::new();
        harness.add("p", "q");

        assert_eq!(0, harness.find("a"));
        assert_eq!(0, harness.find("p"));
        assert_eq!(0, harness.find("p1"));
        assert_eq!(0, harness.find("q"));
        assert_eq!(1, harness.find("q1"));
        assert_eq!(1, harness.find("z"));

        assert!(!harness.overlaps(Some("a"), Some("b")));
        assert!(!harness.overlaps(Some("z1"), Some("z2")));
        assert!(harness.overlaps(Some("a"), Some("p")));
        assert!(harness.overlaps(Some("a"), Some("q")));
        assert!(harness.overlaps(Some("a"), Some("z")));
        assert!(harness.overlaps(Some("p"), Some("p1")));
        assert!(harness.overlaps(Some("p"), Some("q")));
        assert!(harness.overlaps(Some("p"), Some("z")));
        assert!(harness.overlaps(Some("p1"), Some("p2")));
        assert!(harness.overlaps(Some("p1"), Some("z")));
        assert!(harness.overlaps(Some("q"), Some("q")));
        assert!(harness.overlaps(Some("q"), Some("q1")));
        assert!(!harness.overlaps(None, Some("j")));
        assert!(!harness.overlaps(Some("r"), None));
        assert!(harness.overlaps(None, Some("p")));
        assert!(harness.overlaps(None, Some("p1")));
        assert!(harness.overlaps(Some("q"), None));
        assert!(harness.overlaps(None, None));
    }

    #[traced_test]
    fn verify_find_file_with_multiple_non_overlapping_files() {
        let mut harness = FindFileHarness::new();
        harness.add("150", "200");
        harness.add("200", "250");
        harness.add("300", "350");
        harness.add("400", "450");

        assert_eq!(0, harness.find("100"));
        assert_eq!(0, harness.find("150"));
        assert_eq!(0, harness.find("151"));
        assert_eq!(0, harness.find("199"));
        assert_eq!(0, harness.find("200"));
        assert_eq!(1, harness.find("201"));
        assert_eq!(1, harness.find("249"));
        assert_eq!(1, harness.find("250"));
        assert_eq!(2, harness.find("251"));
        assert_eq!(2, harness.find("299"));
        assert_eq!(2, harness.find("300"));
        assert_eq!(2, harness.find("349"));
        assert_eq!(2, harness.find("350"));
        assert_eq!(3, harness.find("351"));
        assert_eq!(3, harness.find("400"));
        assert_eq!(3, harness.find("450"));
        assert_eq!(4, harness.find("451"));

        assert!(!harness.overlaps(Some("100"), Some("149")));
        assert!(!harness.overlaps(Some("251"), Some("299")));
        assert!(!harness.overlaps(Some("451"), Some("500")));
        assert!(!harness.overlaps(Some("351"), Some("399")));

        assert!(harness.overlaps(Some("100"), Some("150")));
        assert!(harness.overlaps(Some("100"), Some("200")));
        assert!(harness.overlaps(Some("100"), Some("300")));
        assert!(harness.overlaps(Some("100"), Some("400")));
        assert!(harness.overlaps(Some("100"), Some("500")));
        assert!(harness.overlaps(Some("375"), Some("400")));
        assert!(harness.overlaps(Some("450"), Some("450")));
        assert!(harness.overlaps(Some("450"), Some("500")));
    }

    #[traced_test]
    fn verify_some_file_overlaps_range_with_null_boundaries_disjoint() {
        let mut harness = FindFileHarness::new();
        harness.add("150", "200");
        harness.add("200", "250");
        harness.add("300", "350");
        harness.add("400", "450");

        assert!(!harness.overlaps(None, Some("149")));
        assert!(!harness.overlaps(Some("451"), None));
        assert!(harness.overlaps(None, None));
        assert!(harness.overlaps(None, Some("150")));
        assert!(harness.overlaps(None, Some("199")));
        assert!(harness.overlaps(None, Some("200")));
        assert!(harness.overlaps(None, Some("201")));
        assert!(harness.overlaps(None, Some("400")));
        assert!(harness.overlaps(None, Some("800")));
        assert!(harness.overlaps(Some("100"), None));
        assert!(harness.overlaps(Some("200"), None));
        assert!(harness.overlaps(Some("449"), None));
        assert!(harness.overlaps(Some("450"), None));
    }

    #[traced_test]
    fn verify_overlap_sequence_checks_in_find_file_harness() {
        let mut harness = FindFileHarness::new();
        harness.add_with_seq("200", "200", 5000, 3000);

        assert!(!harness.overlaps(Some("199"), Some("199")));
        assert!(!harness.overlaps(Some("201"), Some("300")));
        assert!(harness.overlaps(Some("200"), Some("200")));
        assert!(harness.overlaps(Some("190"), Some("200")));
        assert!(harness.overlaps(Some("200"), Some("210")));
    }

    #[traced_test]
    fn verify_overlapping_files_with_non_disjoint_flag() {
        let mut harness = FindFileHarness::new();
        harness.add("150", "600");
        harness.add("400", "500");
        harness.disjoint_sorted_files = false;

        assert!(!harness.overlaps(Some("100"), Some("149")));
        assert!(!harness.overlaps(Some("601"), Some("700")));
        assert!(harness.overlaps(Some("100"), Some("150")));
        assert!(harness.overlaps(Some("100"), Some("200")));
        assert!(harness.overlaps(Some("100"), Some("300")));
        assert!(harness.overlaps(Some("100"), Some("400")));
        assert!(harness.overlaps(Some("100"), Some("500")));
        assert!(harness.overlaps(Some("375"), Some("400")));
        assert!(harness.overlaps(Some("450"), Some("450")));
        assert!(harness.overlaps(Some("450"), Some("500")));
        assert!(harness.overlaps(Some("450"), Some("700")));
        assert!(harness.overlaps(Some("600"), Some("700")));
    }
}
