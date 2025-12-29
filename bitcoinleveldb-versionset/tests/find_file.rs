use bitcoinleveldb_versionset::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_versionsetutil::*;
use bitcoinleveldb_comparator::*;
use bitcoin_imports::*;

// ---------------- [ File: bitcoinleveldb-versionset/tests/find_file.rs ]
struct FindFileTest {
    disjoint_sorted_files: bool,
    files:                 Vec<*mut FileMetaData>,
}

impl Default for FindFileTest {
    fn default() -> Self {
        tracing::trace!("FindFileTest::default: constructing harness");
        Self {
            disjoint_sorted_files: true,
            files: Vec::new(),
        }
    }
}

impl Drop for FindFileTest {
    fn drop(&mut self) {
        tracing::trace!(
            files_len = self.files.len(),
            "FindFileTest::drop: freeing FileMetaData allocations"
        );

        for (i, fptr) in self.files.drain(..).enumerate() {
            if fptr.is_null() {
                tracing::warn!(index = i, "FindFileTest::drop: null FileMetaData pointer; skipping");
                continue;
            }

            unsafe {
                drop(Box::from_raw(fptr));
            }
        }

        tracing::trace!("FindFileTest::drop: complete");
    }
}

impl FindFileTest {
    pub fn add(
        &mut self,
        smallest: &str,
        largest: &str,
        smallest_seq: Option<SequenceNumber>,
        largest_seq: Option<SequenceNumber>,
    ) {
        let smallest_seq: SequenceNumber = smallest_seq.unwrap_or(100);
        let largest_seq: SequenceNumber = largest_seq.unwrap_or(100);

        let number: u64 = (self.files.len() as u64).saturating_add(1);

        let smallest_ikey =
            InternalKey::new(&Slice::from(smallest), smallest_seq, ValueType::TypeValue);
        let largest_ikey =
            InternalKey::new(&Slice::from(largest), largest_seq, ValueType::TypeValue);

        let mut f = Box::new(FileMetaData::default());
        f.set_number(number);
        f.set_smallest(smallest_ikey);
        f.set_largest(largest_ikey);

        let raw: *mut FileMetaData = Box::into_raw(f);
        self.files.push(raw);

        tracing::debug!(
            file_number = number,
            ptr = %format!("{:p}", raw),
            smallest,
            largest,
            smallest_seq,
            largest_seq,
            files_len = self.files.len(),
            "FindFileTest::add: added file"
        );
    }

    pub fn find(&mut self, key_: &str) -> i32 {
        let target = InternalKey::new(&Slice::from(key_), 100, ValueType::TypeValue);
        let cmp = InternalKeyComparator::new(bytewise_comparator());
        let encoded = target.encode();

        let idx = find_file(&cmp, self.files.as_slice(), &encoded);

        tracing::debug!(
            key = key_,
            idx,
            files_len = self.files.len(),
            "FindFileTest::find: computed index"
        );

        idx
    }

    pub fn overlaps(&mut self, smallest: Option<&str>, largest: Option<&str>) -> bool {
        let cmp = InternalKeyComparator::new(bytewise_comparator());

        let s_slice_opt: Option<Slice> = smallest.map(Slice::from);
        let l_slice_opt: Option<Slice> = largest.map(Slice::from);

        let s_ptr: *const Slice = match s_slice_opt.as_ref() {
            Some(s) => s as *const Slice,
            None => core::ptr::null(),
        };
        let l_ptr: *const Slice = match l_slice_opt.as_ref() {
            Some(l) => l as *const Slice,
            None => core::ptr::null(),
        };

        let ov = some_file_overlaps_range(
            &cmp,
            self.disjoint_sorted_files,
            &self.files,
            s_ptr,
            l_ptr,
        );

        tracing::debug!(
            smallest = smallest.unwrap_or("<null>"),
            largest = largest.unwrap_or("<null>"),
            disjoint_sorted_files = self.disjoint_sorted_files,
            overlaps = ov,
            files_len = self.files.len(),
            "FindFileTest::overlaps"
        );

        ov
    }
}

mod find_file_search_and_overlap_suite {

    use super::*;

    #[traced_test]
    fn find_file_returns_zero_and_overlaps_false_for_empty_file_list() {
        let mut h = super::FindFileTest::default();

        assert_eq!(h.find("foo"), 0, "empty file list must return index 0");
        assert!(!h.overlaps(Some("a"), Some("z")), "empty file list must not overlap any range");
        assert!(!h.overlaps(None, Some("z")), "empty file list must not overlap with null smallest");
        assert!(!h.overlaps(Some("a"), None), "empty file list must not overlap with null largest");
        assert!(!h.overlaps(None, None), "empty file list must not overlap with null bounds");
    }

    #[traced_test]
    fn find_file_and_overlaps_for_single_file() {
        let mut h = super::FindFileTest::default();

        h.add("p", "q", None, None);

        assert_eq!(h.find("a"), 0);
        assert_eq!(h.find("p"), 0);
        assert_eq!(h.find("p1"), 0);
        assert_eq!(h.find("q"), 0);
        assert_eq!(h.find("q1"), 1);
        assert_eq!(h.find("z"), 1);

        assert!(!h.overlaps(Some("a"), Some("b")));
        assert!(!h.overlaps(Some("z1"), Some("z2")));
        assert!(h.overlaps(Some("a"), Some("p")));
        assert!(h.overlaps(Some("a"), Some("q")));
        assert!(h.overlaps(Some("a"), Some("z")));
        assert!(h.overlaps(Some("p"), Some("p1")));
        assert!(h.overlaps(Some("p"), Some("q")));
        assert!(h.overlaps(Some("p"), Some("z")));
        assert!(h.overlaps(Some("p1"), Some("p2")));
        assert!(h.overlaps(Some("p1"), Some("z")));
        assert!(h.overlaps(Some("q"), Some("q")));
        assert!(h.overlaps(Some("q"), Some("q1")));

        assert!(!h.overlaps(None, Some("j")));
        assert!(!h.overlaps(Some("r"), None));
        assert!(h.overlaps(None, Some("p")));
        assert!(h.overlaps(None, Some("p1")));
        assert!(h.overlaps(Some("q"), None));
        assert!(h.overlaps(None, None));
    }

    #[traced_test]
    fn find_file_indices_for_multiple_sorted_files() {
        let mut h = super::FindFileTest::default();

        h.add("150", "200", None, None);
        h.add("200", "250", None, None);
        h.add("300", "350", None, None);
        h.add("400", "450", None, None);

        assert_eq!(h.find("100"), 0);
        assert_eq!(h.find("150"), 0);
        assert_eq!(h.find("151"), 0);
        assert_eq!(h.find("199"), 0);
        assert_eq!(h.find("200"), 0);
        assert_eq!(h.find("201"), 1);
        assert_eq!(h.find("249"), 1);
        assert_eq!(h.find("250"), 1);
        assert_eq!(h.find("251"), 2);
        assert_eq!(h.find("299"), 2);
        assert_eq!(h.find("300"), 2);
        assert_eq!(h.find("349"), 2);
        assert_eq!(h.find("350"), 2);
        assert_eq!(h.find("351"), 3);
        assert_eq!(h.find("400"), 3);
        assert_eq!(h.find("450"), 3);
        assert_eq!(h.find("451"), 4);

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
    fn overlaps_with_null_boundaries_for_multiple_sorted_files() {
        let mut h = super::FindFileTest::default();

        h.add("150", "200", None, None);
        h.add("200", "250", None, None);
        h.add("300", "350", None, None);
        h.add("400", "450", None, None);

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
    fn overlaps_respects_sequence_number_ordering_for_point_files() {
        let mut h = super::FindFileTest::default();

        h.add("200", "200", Some(5000), Some(3000));

        assert!(!h.overlaps(Some("199"), Some("199")));
        assert!(!h.overlaps(Some("201"), Some("300")));
        assert!(h.overlaps(Some("200"), Some("200")));
        assert!(h.overlaps(Some("190"), Some("200")));
        assert!(h.overlaps(Some("200"), Some("210")));
    }

    #[traced_test]
    fn overlaps_works_when_files_are_not_disjoint_sorted() {
        let mut h = super::FindFileTest::default();

        h.add("150", "600", None, None);
        h.add("400", "500", None, None);

        h.disjoint_sorted_files = false;

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
