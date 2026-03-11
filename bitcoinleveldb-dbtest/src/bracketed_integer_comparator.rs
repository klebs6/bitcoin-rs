// ---------------- [ File: bitcoinleveldb-dbtest/src/bracketed_integer_comparator.rs ]
crate::ix!();

/// Invariant: this comparator orders keys by the integer encoded inside bracket syntax,
/// matching the C++ custom-comparator test semantics.
///
/// Precondition: all compared keys must be of the form `"[<integer>]"`.
/// Postcondition: `compare(a,b)` returns the signed difference of the decoded integers.
#[derive(Clone, Copy, Debug, Default)]
pub struct DBTestBracketedIntegerComparator;

impl Named for DBTestBracketedIntegerComparator {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("test.NumberComparator")
    }
}

impl Compare for DBTestBracketedIntegerComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        dbtest_custom_number_comparator_parse_bracketed_integer(a)
            - dbtest_custom_number_comparator_parse_bracketed_integer(b)
    }
}

impl FindShortestSeparator for DBTestBracketedIntegerComparator {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        let start_slice = Slice::from(start.as_slice());
        let limit_slice = Slice::from(limit);
        let _ = dbtest_custom_number_comparator_parse_bracketed_integer(&start_slice);
        let _ = dbtest_custom_number_comparator_parse_bracketed_integer(&limit_slice);
    }
}

impl FindShortSuccessor for DBTestBracketedIntegerComparator {
    fn find_short_successor(&self, k: &mut Vec<u8>) {
        let key_slice = Slice::from(k.as_slice());
        let _ = dbtest_custom_number_comparator_parse_bracketed_integer(&key_slice);
    }
}

impl SliceComparator for DBTestBracketedIntegerComparator {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        self as *const Self as *const dyn SliceComparator
    }
}
