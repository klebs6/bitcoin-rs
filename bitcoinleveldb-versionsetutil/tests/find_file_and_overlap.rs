//! Search and overlap specifications retained at the overlap helper owner.
//! The original six-case module below is unchanged.

use bitcoin_imports::*;
use bitcoinleveldb_comparator::bytewise_comparator;
use bitcoinleveldb_file::{find_file, FileMetaData};
use bitcoinleveldb_key::{InternalKey, InternalKeyComparator, ValueType};
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_versionsetutil::some_file_overlaps_range;
use tracing::{debug, trace};

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
