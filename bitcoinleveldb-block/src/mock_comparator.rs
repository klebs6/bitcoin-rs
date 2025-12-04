// ---------------- [ File: bitcoinleveldb-block/src/mock_comparator.rs ]
crate::ix!();

#[derive(Clone, Default)]
pub struct MockComparator;

impl Compare for MockComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        trace!(
            "MockComparator::compare: a_len={}, b_len={}",
            *a.size(),
            *b.size()
        );
        unsafe {
            let ab = core::slice::from_raw_parts(*a.data(), *a.size());
            let bb = core::slice::from_raw_parts(*b.data(), *b.size());
            for (aa, bb) in ab.iter().zip(bb.iter()) {
                if aa < bb {
                    return -1;
                }
                if aa > bb {
                    return 1;
                }
            }
            ab.len().cmp(&bb.len()) as i32
        }
    }
}

impl Named for MockComparator {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        trace!("MockComparator::name called");
        std::borrow::Cow::Borrowed("mock-comparator")
    }
}

impl FindShortestSeparator for MockComparator {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {
        trace!("MockComparator::find_shortest_separator called");
        // no-op for testing
    }
}

impl FindShortSuccessor for MockComparator {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {
        trace!("MockComparator::find_short_successor called");
        // no-op for testing
    }
}

impl SliceComparator for MockComparator {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        trace!("MockComparator::bytewise_comparator called");
        bitcoinleveldb_comparator::bytewise_comparator()
    }
}
