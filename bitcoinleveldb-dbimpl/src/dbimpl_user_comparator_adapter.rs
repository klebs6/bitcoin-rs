// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl_user_comparator_adapter.rs ]
crate::ix!();

#[derive(Clone, Copy)]
pub struct DbImplUserComparatorAdapter {
    ptr: *const dyn SliceComparator,
}

impl DbImplUserComparatorAdapter {
    #[inline]
    pub fn new(ptr: *const dyn SliceComparator) -> Self {
        Self { ptr }
    }

    #[inline]
    pub fn as_ref(&self) -> &dyn SliceComparator {
        unsafe {
            match self.ptr.as_ref() {
                Some(cmp) => cmp,
                None => {
                    tracing::error!(
                        "DBImpl user comparator pointer was null; falling back to null_slice_comparator()"
                    );
                    &*null_slice_comparator()
                }
            }
        }
    }
}

impl Named for DbImplUserComparatorAdapter {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        self.as_ref().name()
    }
}

impl Compare for DbImplUserComparatorAdapter {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.as_ref().compare(a, b)
    }
}

impl FindShortestSeparator for DbImplUserComparatorAdapter {
    fn find_shortest_separator(&self, start: &mut Vec<u8>, limit: &[u8]) {
        self.as_ref().find_shortest_separator(start, limit)
    }
}

impl FindShortSuccessor for DbImplUserComparatorAdapter {
    fn find_short_successor(&self, key: &mut Vec<u8>) {
        self.as_ref().find_short_successor(key)
    }
}

impl SliceComparator for DbImplUserComparatorAdapter {
    fn bytewise_comparator(&self) -> *const dyn SliceComparator {
        self.as_ref().bytewise_comparator()
    }
}
