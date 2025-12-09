// ---------------- [ File: bitcoinleveldb-merger/src/prev.rs ]
crate::ix!();

impl LevelDBIteratorPrev for MergingIterator {

    fn prev(&mut self) {
        trace!("MergingIterator::prev: invoked");

        assert!(
            self.valid(),
            "MergingIterator::prev requires the iterator to be valid"
        );

        // Ensure that all children are positioned before key().
        // If we are moving in the reverse direction, it is already
        // true for all of the non-current children since current is
        // the largest child and key() == current->key(). Otherwise,
        // we explicitly position the non-current children.
        if *self.direction() != MergingIteratorDirection::Reverse {
            trace!(
                "MergingIterator::prev: switching direction from {:?} to \
                 Reverse; realigning children",
                self.direction()
            );

            let current_key = self.key();
            let cur_idx = self
                .current_index()
                .expect("MergingIterator::prev: current_index must be set");

            for (idx, child) in self.children_mut().iter_mut().enumerate() {
                if idx == cur_idx {
                    continue;
                }

                trace!(
                    "MergingIterator::prev: aligning child_index={} using \
                     current_key={:?}",
                    idx,
                    current_key
                );

                child.seek(&current_key);

                if child.valid() {
                    trace!(
                        "MergingIterator::prev: child_index={} at first \
                         entry >= key; stepping back",
                        idx
                    );
                    child.prev();
                } else {
                    trace!(
                        "MergingIterator::prev: child_index={} has no entries \
                         >= key; seeking to last",
                        idx
                    );
                    child.seek_to_last();
                }
            }

            self.set_direction(MergingIteratorDirection::Reverse);
        }

        let cur_idx = self
            .current_index()
            .expect("MergingIterator::prev: current_index must be set");
        trace!(
            "MergingIterator::prev: moving current child at index {} backward",
            cur_idx
        );
        self.children_mut()[cur_idx].prev();

        self.find_largest();
    }
}

#[cfg(test)]
mod merging_iterator_prev_tests {
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
    fn prev_traverses_keys_in_reverse_order_from_end() {
        trace!("TEST(prev): prev_traverses_keys_in_reverse_order_from_end");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c")]);
        let c1 = make_stub_child(&[(b"b", b"v1b"), (b"d", b"v1d")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_last();

        let mut keys = Vec::new();
        while wrapper.valid() {
            keys.push(wrapper.key().to_string());
            wrapper.prev();
        }

        assert_eq!(
            keys,
            vec!["d", "c", "b", "a"],
            "Prev from the end must iterate keys in descending order"
        );
    }
}
