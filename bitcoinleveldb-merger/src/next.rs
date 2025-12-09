// ---------------- [ File: bitcoinleveldb-merger/src/next.rs ]
crate::ix!();

impl LevelDBIteratorNext for MergingIterator {
    
    fn next(&mut self) {
        trace!("MergingIterator::next: invoked");

        assert!(
            self.valid(),
            "MergingIterator::next requires the iterator to be valid"
        );

        // Snapshot current direction so we do not repeatedly borrow `self`.
        let current_direction = *self.direction();

        // Ensure that all children are positioned after key().
        //
        // If we are moving in the forward direction, it is already true for all
        // of the non-current children since current is the smallest child and
        // key() == current->key(). Otherwise, we explicitly position the
        // non-current children.
        if current_direction != MergingIteratorDirection::Forward {
            trace!(
                "MergingIterator::next: switching direction from {:?} to Forward; realigning children",
                current_direction
            );

            let current_key = self.key();
            let cur_idx = self
                .current_index()
                .expect("MergingIterator::next: current_index must be set");

            let child_count = self.children().len();

            for idx in 0..child_count {
                if idx == cur_idx {
                    continue;
                }

                trace!(
                    "MergingIterator::next: aligning child_index={} using current_key={:?}",
                    idx,
                    current_key
                );

                // Stage 1: reposition the child using a short-lived mutable borrow.
                {
                    let child = &mut self.children_mut()[idx];
                    child.seek(&current_key);
                }

                // Stage 2: compute whether this child should be advanced using only
                // immutable borrows (no mutable alias of `self` is active here).
                let mut advance = false;

                {
                    let child_ref = &self.children()[idx];

                    if child_ref.valid() {
                        let child_key = child_ref.key();
                        let comparator_ref: &dyn SliceComparator =
                            &**self.comparator();
                        let cmp =
                            comparator_ref.compare(&current_key, &child_key);

                        trace!(
                            "MergingIterator::next: child_index={} after seek; \
                             child_key={:?}, cmp(current,child)={}",
                            idx,
                            child_key,
                            cmp
                        );

                        if cmp == 0 {
                            advance = true;
                        }
                    } else {
                        trace!(
                            "MergingIterator::next: child_index={} invalid after \
                             seek; leaving as-is",
                            idx
                        );
                    }
                }

                // Stage 3: if needed, advance the child using a fresh mutable
                // borrow that does not overlap with the immutable borrows above.
                if advance {
                    trace!(
                        "MergingIterator::next: child_index={} had duplicate key; advancing",
                        idx
                    );

                    let child = &mut self.children_mut()[idx];
                    child.next();
                }
            }

            self.set_direction(MergingIteratorDirection::Forward);
        }

        let cur_idx = self
            .current_index()
            .expect("MergingIterator::next: current_index must be set");

        trace!(
            "MergingIterator::next: advancing current child at index {}",
            cur_idx
        );

        self.children_mut()[cur_idx].next();

        self.find_smallest();
    }
}

#[cfg(test)]
mod merging_iterator_next_tests {
    use super::*;

    fn make_stub_child(pairs: &[(&[u8], &[u8])]) -> *mut LevelDBIterator {
        let internal = if pairs.is_empty() {
            MockStubIterator::new_empty()
        } else {
            MockStubIterator::new_with_entries(pairs)
        };

        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        let boxed = Box::new(wrapper);
        let raw = Box::into_raw(boxed);

        trace!(
            "make_stub_child(next-tests): created child wrapper at {:p} with {} entries",
            raw,
            pairs.len()
        );

        raw
    }

    #[traced_test]
    fn next_traverses_all_keys_in_forward_merged_order() {
        trace!("TEST(next): next_traverses_all_keys_in_forward_merged_order");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"d", b"v0d")]);
        let c1 = make_stub_child(&[(b"b", b"v1b"), (b"e", b"v1e")]);
        let c2 = make_stub_child(&[(b"c", b"v2c"), (b"f", b"v2f")]);

        let mut children = [c0, c1, c2];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        let mut keys = Vec::new();

        while wrapper.valid() {
            keys.push(wrapper.key().to_string());
            wrapper.next();
        }

        assert_eq!(
            keys,
            vec!["a", "b", "c", "d", "e", "f"],
            "MergingIterator::next must yield merged keys in sorted order"
        );
    }

    #[traced_test]
    fn next_realigns_after_reverse_iteration() {
        trace!("TEST(next): next_realigns_after_reverse_iteration");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c"), (b"e", b"v0e")]);
        let c1 = make_stub_child(&[(b"b", b"v1b"), (b"d", b"v1d"), (b"f", b"v1f")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let mut wrapper: Box<LevelDBIterator> =
            unsafe { Box::from_raw(result_ptr) };

        // Move forward a bit.
        wrapper.seek_to_first();
        assert_eq!(wrapper.key().to_string(), "a");
        wrapper.next();
        assert_eq!(wrapper.key().to_string(), "b");
        wrapper.next();
        assert_eq!(wrapper.key().to_string(), "c");

        // Now move backwards, switching direction inside MergingIterator.
        wrapper.prev();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "b");

        // Switch direction again by calling next and ensure we did not skip.
        wrapper.next();
        assert!(wrapper.valid());
        assert_eq!(
            wrapper.key().to_string(),
            "c",
            "After switching direction back to forward, next() must yield the key we came from"
        );
    }
}
