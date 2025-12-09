// ---------------- [ File: bitcoinleveldb-merger/src/new_merging_iterator.rs ]
crate::ix!();

/// Internal empty iterator used when `new_merging_iterator` is called
/// with `n == 0`.
///
/// This mirrors LevelDB's `NewEmptyIterator()` semantics by providing
/// an always-invalid iterator that reports `Status::ok()` and panics
/// if `key()` or `value()` are ever called.
struct MergerEmptyInternalIterator;

impl MergerEmptyInternalIterator {
    fn new() -> Self {
        trace!(
            "MergerEmptyInternalIterator::new: constructing empty internal iterator"
        );
        MergerEmptyInternalIterator
    }
}

impl LevelDBIteratorInterface for MergerEmptyInternalIterator {}

impl LevelDBIteratorValid for MergerEmptyInternalIterator {
    fn valid(&self) -> bool {
        trace!("MergerEmptyInternalIterator::valid -> false");
        false
    }
}

impl LevelDBIteratorSeekToFirst for MergerEmptyInternalIterator {
    fn seek_to_first(&mut self) {
        trace!("MergerEmptyInternalIterator::seek_to_first: no-op");
    }
}

impl LevelDBIteratorSeekToLast for MergerEmptyInternalIterator {
    fn seek_to_last(&mut self) {
        trace!("MergerEmptyInternalIterator::seek_to_last: no-op");
    }
}

impl LevelDBIteratorSeek for MergerEmptyInternalIterator {
    fn seek(&mut self, _target: &Slice) {
        trace!("MergerEmptyInternalIterator::seek: no-op");
    }
}

impl LevelDBIteratorNext for MergerEmptyInternalIterator {
    fn next(&mut self) {
        trace!("MergerEmptyInternalIterator::next: no-op");
    }
}

impl LevelDBIteratorPrev for MergerEmptyInternalIterator {
    fn prev(&mut self) {
        trace!("MergerEmptyInternalIterator::prev: no-op");
    }
}

impl LevelDBIteratorStatus for MergerEmptyInternalIterator {
    fn status(&self) -> Status {
        trace!("MergerEmptyInternalIterator::status -> Status::ok()");
        Status::ok()
    }
}

impl LevelDBIteratorKey for MergerEmptyInternalIterator {
    fn key(&self) -> Slice {
        error!(
            "MergerEmptyInternalIterator::key called on invalid iterator; \
             this is a programming error"
        );
        panic!("MergerEmptyInternalIterator::key should not be called");
    }
}

impl LevelDBIteratorValue for MergerEmptyInternalIterator {
    fn value(&self) -> Slice {
        error!(
            "MergerEmptyInternalIterator::value called on invalid iterator; \
             this is a programming error"
        );
        panic!("MergerEmptyInternalIterator::value should not be called");
    }
}

/**
  | Return an iterator that provided the union of
  | the data in children[0,n-1].  Takes ownership
  | of the child iterators and will delete them
  | when the result iterator is deleted.
  |
  | The result does no duplicate suppression.
  | I.e., if a particular key is present in K child
  | iterators, it will be yielded K times.
  |
  | REQUIRES: n >= 0
  */
pub fn new_merging_iterator(
    comparator: Box<dyn SliceComparator>,
    children:   *mut *mut LevelDBIterator,
    n:          i32,
) -> *mut LevelDBIterator {
    trace!(
        "new_merging_iterator: invoked with n_children={}",
        n
    );

    if n < 0 {
        error!(
            "new_merging_iterator: invalid negative child count: {}",
            n
        );
        panic!("new_merging_iterator requires n >= 0");
    }

    if n == 0 {
        trace!(
            "new_merging_iterator: constructing internal empty iterator for n == 0"
        );

        let internal: Box<dyn LevelDBIteratorInterface> =
            Box::new(MergerEmptyInternalIterator::new());
        let wrapper = LevelDBIterator::new(Some(internal));
        let boxed = Box::new(wrapper);
        let raw = Box::into_raw(boxed);

        trace!(
            "new_merging_iterator: returning internal empty LevelDBIterator at {:p}",
            raw
        );

        return raw;
    }

    if n == 1 {
        assert!(
            !children.is_null(),
            "new_merging_iterator: children pointer is null for n == 1"
        );
        let child_ptr = unsafe { *children };
        trace!(
            "new_merging_iterator: passthrough for single child; child_ptr={:p}",
            child_ptr
        );
        // Ownership transfer matches the C++ semantics: the caller must
        // treat the returned pointer as the only owner of the child.
        return child_ptr;
    }

    // n >= 2: construct a full merging iterator that owns all children.
    assert!(
        !children.is_null(),
        "new_merging_iterator: children pointer is null for n >= 2"
    );
    trace!(
        "new_merging_iterator: building MergingIterator with {} children",
        n
    );

    let merging = MergingIterator::new(comparator, children, n);
    let internal: Box<dyn LevelDBIteratorInterface> = Box::new(merging);
    let wrapper = LevelDBIterator::new(Some(internal));
    let boxed = Box::new(wrapper);
    let raw = Box::into_raw(boxed);

    trace!(
        "new_merging_iterator: returning merging LevelDBIterator at {:p}",
        raw
    );

    raw
}

#[cfg(test)]
mod new_merging_iterator_tests {
    use super::*;

    fn make_stub_child(pairs: &[(&[u8], &[u8])]) -> *mut LevelDBIterator {
        let internal = if pairs.is_empty() {
            MockStubIterator::new_empty()
        } else {
            MockStubIterator::new_with_entries(pairs)
        };

        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        Box::into_raw(Box::new(wrapper))
    }

    #[traced_test]
    fn new_merging_iterator_zero_children_returns_empty_invalid_iterator() {
        trace!("TEST(new_merging_iterator): zero_children_returns_empty_invalid_iterator");

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr = new_merging_iterator(cmp, core::ptr::null_mut(), 0);

        assert!(
            !result_ptr.is_null(),
            "new_merging_iterator must never return a null pointer for n == 0"
        );

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        assert!(
            !wrapper.valid(),
            "Empty iterator produced for n == 0 must start invalid"
        );

        wrapper.seek_to_first();
        assert!(
            !wrapper.valid(),
            "seek_to_first on empty iterator must keep it invalid"
        );

        wrapper.seek_to_last();
        assert!(
            !wrapper.valid(),
            "seek_to_last on empty iterator must keep it invalid"
        );

        let st = wrapper.status();
        assert!(
            st.is_ok(),
            "status() for empty iterator must be OK"
        );
    }

    #[traced_test]
    fn new_merging_iterator_single_child_is_passthrough() {
        trace!("TEST(new_merging_iterator): single_child_is_passthrough");

        let child_ptr =
            make_stub_child(&[(b"a", b"va"), (b"b", b"vb"), (b"c", b"vc")]);
        let mut children: [*mut LevelDBIterator; 1] = [child_ptr];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), 1);

        assert_eq!(
            result_ptr, child_ptr,
            "For n == 1, new_merging_iterator must return the original child pointer"
        );

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "a");
        assert_eq!(wrapper.value().to_string(), "va");

        wrapper.next();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "b");
        assert_eq!(wrapper.value().to_string(), "vb");

        wrapper.next();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "c");
        assert_eq!(wrapper.value().to_string(), "vc");

        wrapper.next();
        assert!(
            !wrapper.valid(),
            "After exhausting entries, iterator must become invalid"
        );
    }

    #[traced_test]
    fn new_merging_iterator_multiple_children_wraps_in_merging_iterator() {
        trace!("TEST(new_merging_iterator): multiple_children_wraps_in_merging_iterator");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c")]);
        let c1 = make_stub_child(&[(b"b", b"v1b"), (b"d", b"v1d")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        assert_ne!(
            result_ptr, c0,
            "For n >= 2, new_merging_iterator must allocate a fresh wrapper"
        );
        assert_ne!(
            result_ptr, c1,
            "For n >= 2, new_merging_iterator must allocate a fresh wrapper"
        );

        assert!(
            children.iter().all(|p| p.is_null()),
            "All child slots must be nulled when wrapped in MergingIterator"
        );

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();

        let mut seen = Vec::new();
        while wrapper.valid() {
            seen.push(wrapper.key().to_string());
            wrapper.next();
        }

        assert_eq!(
            seen,
            vec!["a", "b", "c", "d"],
            "Merged iterator must yield sorted keys from all children"
        );
    }
}
