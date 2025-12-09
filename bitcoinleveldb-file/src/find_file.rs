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
