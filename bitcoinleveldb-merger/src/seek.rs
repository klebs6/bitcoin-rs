// ---------------- [ File: bitcoinleveldb-merger/src/seek.rs ]
crate::ix!();

impl LevelDBIteratorSeek for MergingIterator {
    
    fn seek(&mut self, target: &Slice) {
        trace!(
            "MergingIterator::seek: seeking all children to target={:?}",
            target
        );

        for (idx, child) in self.children_mut().iter_mut().enumerate() {
            trace!(
                "MergingIterator::seek: child_index={} before_valid={}",
                idx,
                child.valid()
            );
            child.seek(target);
            trace!(
                "MergingIterator::seek: child_index={} after_valid={}",
                idx,
                child.valid()
            );
        }

        self.find_smallest();
        self.set_direction(MergingIteratorDirection::Forward);

        trace!(
            "MergingIterator::seek: new_current_index={:?}, \
             direction=Forward",
            self.current_index()
        );
    }
}

#[cfg(test)]
mod merging_iterator_seek_tests {
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
    fn seek_positions_on_first_key_greater_or_equal_across_children() {
        trace!("TEST(seek): seek_positions_on_first_key_greater_or_equal_across_children");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"e", b"v0e")]);
        let c1 = make_stub_child(&[(b"c", b"v1c"), (b"f", b"v1f")]);

        let mut children = [c0, c1];
        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        let target = Slice::from("d");
        wrapper.seek(&target);

        assert!(
            wrapper.valid(),
            "seek must position at the first key >= target if any exists"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "e",
            "seek(d) must land at 'e' given children {{{{a,e}},{{c,f}}}}"
        );
    }

    #[traced_test]
    fn seek_beyond_all_keys_results_in_invalid_iterator() {
        trace!("TEST(seek): seek_beyond_all_keys_results_in_invalid_iterator");

        let c0 = make_stub_child(&[(b"a", b"v0a")]);
        let c1 = make_stub_child(&[(b"b", b"v1b")]);

        let mut children = [c0, c1];
        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        let target = Slice::from("z");
        wrapper.seek(&target);

        assert!(
            !wrapper.valid(),
            "seek past the maximum key must produce an invalid iterator"
        );
    }
}
