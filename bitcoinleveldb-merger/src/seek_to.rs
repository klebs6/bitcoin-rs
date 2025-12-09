// ---------------- [ File: bitcoinleveldb-merger/src/seek_to.rs ]
crate::ix!();

impl LevelDBIteratorSeekToFirst for MergingIterator {
    
    fn seek_to_first(&mut self) {
        trace!(
            "MergingIterator::seek_to_first: resetting {} children to first",
            self.children().len()
        );

        for (idx, child) in self.children_mut().iter_mut().enumerate() {
            trace!(
                "MergingIterator::seek_to_first: child_index={} before_valid={}",
                idx,
                child.valid()
            );
            child.seek_to_first();
            trace!(
                "MergingIterator::seek_to_first: child_index={} after_valid={}",
                idx,
                child.valid()
            );
        }

        self.find_smallest();
        self.set_direction(MergingIteratorDirection::Forward);

        trace!(
            "MergingIterator::seek_to_first: new_current_index={:?}, \
             direction=Forward",
            self.current_index()
        );
    }
}

impl LevelDBIteratorSeekToLast for MergingIterator {

    fn seek_to_last(&mut self) {
        trace!(
            "MergingIterator::seek_to_last: resetting {} children to last",
            self.children().len()
        );

        for (idx, child) in self.children_mut().iter_mut().enumerate() {
            trace!(
                "MergingIterator::seek_to_last: child_index={} before_valid={}",
                idx,
                child.valid()
            );
            child.seek_to_last();
            trace!(
                "MergingIterator::seek_to_last: child_index={} after_valid={}",
                idx,
                child.valid()
            );
        }

        self.find_largest();
        self.set_direction(MergingIteratorDirection::Reverse);

        trace!(
            "MergingIterator::seek_to_last: new_current_index={:?}, \
             direction=Reverse",
            self.current_index()
        );
    }
}

#[cfg(test)]
mod merging_iterator_seek_to_tests {
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
    fn seek_to_first_positions_at_global_minimum_or_invalid() {
        trace!("TEST(seek_to): seek_to_first_positions_at_global_minimum_or_invalid");

        let c0 = make_stub_child(&[(b"b", b"v0b")]);
        let c1 = make_stub_child(&[(b"a", b"v1a")]);
        let c2 = make_stub_child(&[]);

        let mut children = [c0, c1, c2];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();

        assert!(
            wrapper.valid(),
            "seek_to_first must be valid when at least one child has entries"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "a",
            "seek_to_first must position at the global minimum key"
        );
    }

    #[traced_test]
    fn seek_to_last_positions_at_global_maximum_or_invalid() {
        trace!("TEST(seek_to): seek_to_last_positions_at_global_maximum_or_invalid");

        let c0 = make_stub_child(&[]);
        let c1 = make_stub_child(&[(b"c", b"v1c")]);
        let c2 = make_stub_child(&[(b"d", b"v2d")]);

        let mut children = [c0, c1, c2];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_last();

        assert!(
            wrapper.valid(),
            "seek_to_last must be valid when at least one child has entries"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "d",
            "seek_to_last must position at the global maximum key"
        );
    }

    #[traced_test]
    fn seek_to_first_and_last_on_all_empty_children_yield_invalid() {
        trace!("TEST(seek_to): seek_to_first_and_last_on_all_empty_children_yield_invalid");

        let c0 = make_stub_child(&[]);
        let c1 = make_stub_child(&[]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        assert!(
            !wrapper.valid(),
            "seek_to_first on all-empty children must produce an invalid iterator"
        );

        wrapper.seek_to_last();
        assert!(
            !wrapper.valid(),
            "seek_to_last on all-empty children must produce an invalid iterator"
        );
    }
}
