crate::ix!();

#[derive(Clone, Default)]
pub struct MockCorruptComparator;

impl Compare for MockCorruptComparator {
    fn compare(&self, _a: &Slice, _b: &Slice) -> i32 {
        0
    }
}

impl Named for MockCorruptComparator {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("dummy-comparator".to_string())
    }
}

impl FindShortestSeparator for MockCorruptComparator {
    fn find_shortest_separator(&self, _start: &mut Vec<u8>, _limit: &[u8]) {}
}

impl FindShortSuccessor for MockCorruptComparator {
    fn find_short_successor(&self, _key: &mut Vec<u8>) {}
}

impl SliceComparator for MockCorruptComparator {

    fn bytewise_comparator(&self) -> *const (dyn SliceComparator + 'static) { todo!() }
}
