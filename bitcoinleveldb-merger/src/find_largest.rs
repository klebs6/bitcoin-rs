// ---------------- [ File: bitcoinleveldb-merger/src/find_largest.rs ]
crate::ix!();

impl MergingIterator {

    pub fn find_largest(&mut self) {
        trace!(
            "MergingIterator::find_largest: scanning {} children (reverse)",
            self.children().len()
        );

        let mut largest_idx: Option<usize> = None;

        if self.children().is_empty() {
            self.set_current_index(None);
            trace!(
                "MergingIterator::find_largest: no children; \
                 current_index=None"
            );
            return;
        }

        for (idx, child) in self.children().iter().enumerate().rev() {
            if !child.valid() {
                trace!(
                    "MergingIterator::find_largest: child_index={} invalid; \
                     skipping",
                    idx
                );
                continue;
            }

            match largest_idx {
                None => {
                    trace!(
                        "MergingIterator::find_largest: child_index={} is \
                         first valid (largest so far)",
                        idx
                    );
                    largest_idx = Some(idx);
                }
                Some(cur) => {
                    let child_key = child.key();
                    let largest_key = self.children()[cur].key();
                    let cmp =
                        self.comparator().compare(&child_key, &largest_key);

                    trace!(
                        "MergingIterator::find_largest: compare child_index={} \
                         vs current_index={} -> {}",
                        idx,
                        cur,
                        cmp
                    );

                    if cmp > 0 {
                        largest_idx = Some(idx);
                    }
                }
            }
        }

        self.set_current_index(largest_idx);

        trace!(
            "MergingIterator::find_largest: selected current_index={:?}",
            self.current_index()
        );
    }
}

#[cfg(test)]
mod merging_iterator_find_largest_tests {
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
    fn find_largest_via_seek_to_last_picks_global_maximum() {
        trace!("TEST(find_largest): find_largest_via_seek_to_last_picks_global_maximum");

        let c0 = make_stub_child(&[(b"b", b"v0b"), (b"g", b"v0g")]);
        let c1 = make_stub_child(&[(b"a", b"v1a"), (b"h", b"v1h")]);
        let c2 = make_stub_child(&[(b"c", b"v2c"), (b"f", b"v2f")]);

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
            "Merged iterator must be valid after seek_to_last when there are entries"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "h",
            "seek_to_last (backed by find_largest) must position at global maximum key"
        );
    }

    #[traced_test]
    fn find_largest_on_all_empty_children_results_in_invalid_iterator() {
        trace!("TEST(find_largest): find_largest_on_all_empty_children_results_in_invalid_iterator");

        let c0 = make_stub_child(&[]);
        let c1 = make_stub_child(&[]);

        let mut children = [c0, c1];
        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_last();

        assert!(
            !wrapper.valid(),
            "seek_to_last (backed by find_largest) on all-empty children must produce an invalid iterator"
        );
    }
}
