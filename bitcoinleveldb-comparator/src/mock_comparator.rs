// ---------------- [ File: bitcoinleveldb-comparator/src/mock_comparator.rs ]
crate::ix!();

#[derive(Clone, Default)]
pub struct MockComparator;

impl Compare for MockComparator {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        let a_bytes = unsafe { core::slice::from_raw_parts(*a.data(), *a.size()) };
        let b_bytes = unsafe { core::slice::from_raw_parts(*b.data(), *b.size()) };
        for (aa, bb) in a_bytes.iter().zip(b_bytes.iter()) {
            if aa < bb {
                return -1;
            }
            if aa > bb {
                return 1;
            }
        }
        a_bytes.len().cmp(&b_bytes.len()) as i32
    }
}

impl Named for MockComparator {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("dummy-comparator".to_string())
    }
}

impl FindShortestSeparator for MockComparator {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {}
}

impl FindShortSuccessor for MockComparator {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {}
}

impl SliceComparator for MockComparator {
    fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { todo!() }
}
