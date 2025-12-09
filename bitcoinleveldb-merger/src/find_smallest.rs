// ---------------- [ File: bitcoinleveldb-merger/src/find_smallest.rs ]
crate::ix!();

impl MergingIterator {
    
    pub fn find_smallest(&mut self) {
        trace!(
            "MergingIterator::find_smallest: scanning {} children",
            self.children().len()
        );

        let mut smallest_idx: Option<usize> = None;

        for (idx, child) in self.children().iter().enumerate() {
            if !child.valid() {
                trace!(
                    "MergingIterator::find_smallest: child_index={} invalid; \
                     skipping",
                    idx
                );
                continue;
            }

            match smallest_idx {
                None => {
                    trace!(
                        "MergingIterator::find_smallest: child_index={} is \
                         first valid",
                        idx
                    );
                    smallest_idx = Some(idx);
                }
                Some(cur) => {
                    let child_key = child.key();
                    let smallest_key = self.children()[cur].key();
                    let cmp =
                        self.comparator().compare(&child_key, &smallest_key);

                    trace!(
                        "MergingIterator::find_smallest: compare child_index={} \
                         vs current_index={} -> {}",
                        idx,
                        cur,
                        cmp
                    );

                    if cmp < 0 {
                        smallest_idx = Some(idx);
                    }
                }
            }
        }

        self.set_current_index(smallest_idx);

        trace!(
            "MergingIterator::find_smallest: selected current_index={:?}",
            self.current_index()
        );
    }
}

#[cfg(test)]
mod merging_iterator_find_smallest_tests {
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
    fn find_smallest_via_seek_to_first_picks_global_minimum() {
        trace!("TEST(find_smallest): find_smallest_via_seek_to_first_picks_global_minimum");

        let c0 = make_stub_child(&[(b"b", b"v0b"), (b"d", b"v0d")]);
        let c1 = make_stub_child(&[(b"a", b"v1a"), (b"e", b"v1e")]);
        let c2 = make_stub_child(&[(b"c", b"v2c")]);

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
            "Merged iterator must be valid after seek_to_first when there are entries"
        );
        assert_eq!(
            wrapper.key().to_string(),
            "a",
            "seek_to_first (backed by find_smallest) must position at global minimum key"
        );
    }

    #[traced_test]
    fn find_smallest_on_all_empty_children_results_in_invalid_iterator() {
        trace!("TEST(find_smallest): find_smallest_on_all_empty_children_results_in_invalid_iterator");

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
            "seek_to_first (backed by find_smallest) on all-empty children must produce an invalid iterator"
        );
    }
}
