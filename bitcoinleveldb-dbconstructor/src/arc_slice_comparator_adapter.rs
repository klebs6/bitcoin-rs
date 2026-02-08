// ---------------- [ File: bitcoinleveldb-dbconstructor/src/arc_slice_comparator_adapter.rs ]
crate::ix!();

pub struct ArcSliceComparatorAdapter {
    inner: std::sync::Arc<dyn SliceComparator>,
}

impl From<Arc<dyn SliceComparator>> for ArcSliceComparatorAdapter {
    fn from(x: Arc<dyn SliceComparator>) -> Self {
        Self { inner: x }
    }
}

impl bitcoinleveldb_comparator::Compare for ArcSliceComparatorAdapter {
    #[inline]
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        bitcoinleveldb_comparator::Compare::compare(self.inner.as_ref(), a, b)
    }
}

impl bitcoin_imports::Named for ArcSliceComparatorAdapter {
    #[inline]
    fn name(&self) -> std::borrow::Cow<'_, str> {
        bitcoin_imports::Named::name(self.inner.as_ref())
    }
}

impl bitcoinleveldb_comparator::FindShortestSeparator for ArcSliceComparatorAdapter {
    #[inline]
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        bitcoinleveldb_comparator::FindShortestSeparator::find_shortest_separator(
            self.inner.as_ref(),
            start,
            limit,
        )
    }
}

impl bitcoinleveldb_comparator::FindShortSuccessor for ArcSliceComparatorAdapter {
    #[inline]
    fn find_short_successor(&self, key: &mut Vec<u8>) {
        bitcoinleveldb_comparator::FindShortSuccessor::find_short_successor(self.inner.as_ref(), key)
    }
}

impl bitcoinleveldb_comparator::SliceComparator for ArcSliceComparatorAdapter {
    #[inline]
    fn bytewise_comparator(
        &self,
    ) -> *const (dyn bitcoinleveldb_comparator::SliceComparator + 'static) {
        bitcoinleveldb_comparator::SliceComparator::bytewise_comparator(self.inner.as_ref())
    }
}
