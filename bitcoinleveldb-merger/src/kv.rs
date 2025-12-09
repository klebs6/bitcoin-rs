// ---------------- [ File: bitcoinleveldb-merger/src/kv.rs ]
crate::ix!();

impl LevelDBIteratorKey for MergingIterator {
    
    fn key(&self) -> Slice {
        assert!(
            self.valid(),
            "MergingIterator::key requires the iterator to be valid"
        );

        let idx = self
            .current_index()
            .expect("MergingIterator::key: current_index must be set");

        let k = self.children()[idx].key();

        trace!(
            "MergingIterator::key: current_index={}, key={:?}",
            idx,
            k
        );

        k
    }
}

impl LevelDBIteratorValue for MergingIterator {

    fn value(&self) -> Slice {
        assert!(
            self.valid(),
            "MergingIterator::value requires the iterator to be valid"
        );

        let idx = self
            .current_index()
            .expect("MergingIterator::value: current_index must be set");

        let v = self.children()[idx].value();

        trace!(
            "MergingIterator::value: current_index={}, value={:?}",
            idx,
            v
        );

        v
    }
}

#[cfg(test)]
mod merging_iterator_kv_tests {
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
    fn key_and_value_follow_current_child_selection() {
        trace!("TEST(kv): key_and_value_follow_current_child_selection");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c")]);
        let c1 = make_stub_child(&[(b"b", b"v1b")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let mut merging =
            MergingIterator::new(cmp, children.as_mut_ptr(), children.len() as i32);

        // Position children and choose the smallest using the normal API.
        merging.seek_to_first();

        assert!(
            merging.valid(),
            "MergingIterator must be valid after seek_to_first"
        );
        assert_eq!(
            merging.key().to_string(),
            "a",
            "key() must reflect the smallest child's key"
        );
        assert_eq!(
            merging.value().to_string(),
            "v0a",
            "value() must reflect the corresponding child's value"
        );

        // Advance once and check that key/value move to the next key.
        merging.next();

        assert!(
            merging.valid(),
            "Iterator must remain valid after moving to next key"
        );
        assert_eq!(merging.key().to_string(), "b");
        assert_eq!(merging.value().to_string(), "v1b");
    }

    #[test]
    #[should_panic(expected = "MergingIterator::key requires the iterator to be valid")]
    fn key_panics_when_iterator_is_invalid() {
        trace!("TEST(kv): key_panics_when_iterator_is_invalid");

        let c0 = make_stub_child(&[(b"a", b"v0a")]);
        let mut children = [c0];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let mut merging =
            MergingIterator::new(cmp, children.as_mut_ptr(), children.len() as i32);

        // Force invalid state.
        merging.set_current_index(None);
        assert!(
            !merging.valid(),
            "Iterator must report invalid when current_index is None"
        );

        // This must panic.
        let _ = merging.key();
    }

    #[test]
    #[should_panic(expected = "MergingIterator::value requires the iterator to be valid")]
    fn value_panics_when_iterator_is_invalid() {
        trace!("TEST(kv): value_panics_when_iterator_is_invalid");

        let c0 = make_stub_child(&[(b"a", b"v0a")]);
        let mut children = [c0];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let mut merging =
            MergingIterator::new(cmp, children.as_mut_ptr(), children.len() as i32);

        // Force invalid state.
        merging.set_current_index(None);
        assert!(
            !merging.valid(),
            "Iterator must report invalid when current_index is None"
        );

        // This must panic.
        let _ = merging.value();
    }
}
