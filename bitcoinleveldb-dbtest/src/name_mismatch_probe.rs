// ---------------- [ File: bitcoinleveldb-dbtest/src/name_mismatch_probe.rs ]
crate::ix!();

/// Invariant: this comparator preserves bytewise ordering while intentionally advertising
/// a distinct comparator name, which is sufficient to trigger the persisted-comparator
/// mismatch path during reopen.
///
/// Precondition: none.
/// Postcondition: comparisons are bytewise and the reported comparator name is stable.
#[derive(Clone, Copy, Debug, Default)]
pub struct DBTestComparatorNameMismatchProbe;

impl Named for DBTestComparatorNameMismatchProbe {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("leveldb.NewComparator")
    }
}

impl Compare for DBTestComparatorNameMismatchProbe {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        a.compare(b)
    }
}

impl FindShortestSeparator for DBTestComparatorNameMismatchProbe {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {}
}

impl FindShortSuccessor for DBTestComparatorNameMismatchProbe {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {}
}

impl SliceComparator for DBTestComparatorNameMismatchProbe {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        self as *const Self as *const dyn SliceComparator
    }
}
