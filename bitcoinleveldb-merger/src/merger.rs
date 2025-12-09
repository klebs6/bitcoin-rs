// ---------------- [ File: bitcoinleveldb-merger/src/merger.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/merger.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/merger.cc]
#[derive(Setters,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub",set="pub")]
pub struct MergingIterator {
    /// Embedded LevelDBIterator wrapper, mirroring the original C++
    /// inheritance from `Iterator`. We do not delegate through this
    /// field directly, but we keep it to preserve the translated
    /// structure.
    base: LevelDBIterator,

    /**
      | We might want to use a heap in case there are
      |  lots of children.
      |
      | For now we use a simple array since we expect
      | a very small number of children in leveldb.
      */
    comparator:    Box<dyn SliceComparator>,
    children:      Vec<Box<LevelDBIterator>>,
    current_index: Option<usize>,
    direction:     MergingIteratorDirection,
}

impl LevelDBIteratorInterface for MergingIterator {}

impl LevelDBIteratorValid for MergingIterator {
    
    fn valid(&self) -> bool {
        let is_valid = self.current_index.is_some();
        trace!(
            "MergingIterator::valid: current_index={:?} -> {}",
            self.current_index,
            is_valid
        );
        is_valid
    }
}

impl MergingIterator {

    pub fn new(
        comparator: Box<dyn SliceComparator>,
        children:   *mut *mut LevelDBIterator,
        n:          i32,
    ) -> Self {
        trace!(
            "MergingIterator::new: constructing with n_children={}",
            n
        );

        assert!(
            n >= 0,
            "MergingIterator::new requires non-negative child count"
        );

        let count = n as usize;
        let mut owned_children: Vec<Box<LevelDBIterator>> =
            Vec::with_capacity(count);

        for i in 0..count {
            let slot_ptr = unsafe { children.add(i) };
            let child_ptr = unsafe { *slot_ptr };

            if child_ptr.is_null() {
                warn!(
                    "MergingIterator::new: child pointer at index {} is null; \
                     inserting default invalid LevelDBIterator",
                    i
                );
                owned_children.push(Box::new(LevelDBIterator::default()));
            } else {
                trace!(
                    "MergingIterator::new: taking ownership of child index {} \
                     at {:p}",
                    i,
                    child_ptr
                );
                let child_box = unsafe { Box::from_raw(child_ptr) };
                owned_children.push(child_box);

                // Clear the caller's slot to make ownership transfer explicit
                // and reduce the chance of accidental double-free patterns.
                unsafe {
                    *slot_ptr = core::ptr::null_mut();
                }
            }
        }

        trace!(
            "MergingIterator::new: finished adopting {} children",
            owned_children.len()
        );

        MergingIterator {
            base:          LevelDBIterator::default(),
            comparator,
            children:      owned_children,
            current_index: None,
            direction:     MergingIteratorDirection::Forward,
        }
    }
}

#[cfg(test)]
mod merging_iterator_integration_tests {
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
    fn merging_iterator_new_adopts_children_and_nulls_slots() {
        trace!("TEST(create): merging_iterator_new_adopts_children_and_nulls_slots");

        let c0 = make_stub_child(&[(b"a", b"v0a")]);
        let c1 = make_stub_child(&[(b"b", b"v1b")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let merging = MergingIterator::new(
            cmp,
            children.as_mut_ptr(),
            children.len() as i32,
        );

        assert!(
            children.iter().all(|p| p.is_null()),
            "All child slots must be nulled out after ownership is transferred"
        );
        assert_eq!(
            merging.children().len(),
            2,
            "MergingIterator must adopt exactly all children"
        );

        // Freshly constructed children should be invalid until positioned.
        for (idx, child) in merging.children().iter().enumerate() {
            trace!(
                "create-test: child_index={} initial_valid={}",
                idx,
                child.valid()
            );
            assert!(
                !child.valid(),
                "Adopted LevelDBIterator wrappers should start invalid before any seek"
            );
        }
    }

    #[traced_test]
    fn merging_iterator_new_replaces_null_child_with_default_wrapper() {
        trace!("TEST(create): merging_iterator_new_replaces_null_child_with_default_wrapper");

        let c0: *mut LevelDBIterator = core::ptr::null_mut();
        let c1 = make_stub_child(&[(b"x", b"vx")]);

        let mut children = [c0, c1];

        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        let merging = MergingIterator::new(
            cmp,
            children.as_mut_ptr(),
            children.len() as i32,
        );

        assert!(
            children.iter().all(|p| p.is_null()),
            "All slots (including null ones) must be nulled out after construction"
        );
        assert_eq!(
            merging.children().len(),
            2,
            "MergingIterator must still contain the expected number of children"
        );

        assert!(
            !merging.children()[0].valid(),
            "Null child pointer must be replaced by an invalid default iterator"
        );
    }

    #[test]
    #[should_panic(expected = "MergingIterator::new requires non-negative child count")]
    fn merging_iterator_new_panics_for_negative_child_count() {
        trace!("TEST(create): merging_iterator_new_panics_for_negative_child_count");

        let mut children: [*mut LevelDBIterator; 0] = [];
        let cmp: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());

        // This must panic before attempting to traverse the children pointer.
        let _ = MergingIterator::new(cmp, children.as_mut_ptr(), -1);
    }

    #[traced_test]
    fn empty_merging_iterator_behaves_like_empty() {
        trace!("TEST: empty_merging_iterator_behaves_like_empty");

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let result_ptr = new_merging_iterator(cmp, core::ptr::null_mut(), 0);

        assert!(
            !result_ptr.is_null(),
            "empty merging iterator should return a non-null pointer"
        );

        let mut wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };

        assert!(
            !wrapper.valid(),
            "fresh empty merging iterator should be invalid"
        );

        wrapper.seek_to_first();
        assert!(
            !wrapper.valid(),
            "seek_to_first on empty iterator must remain invalid"
        );

        wrapper.seek_to_last();
        assert!(
            !wrapper.valid(),
            "seek_to_last on empty iterator must remain invalid"
        );

        let target = Slice::from("key");
        wrapper.seek(&target);
        assert!(
            !wrapper.valid(),
            "seek(target) on empty iterator must remain invalid"
        );

        wrapper.next();
        assert!(
            !wrapper.valid(),
            "next() on empty iterator must remain invalid"
        );

        wrapper.prev();
        assert!(
            !wrapper.valid(),
            "prev() on empty iterator must remain invalid"
        );

        let st = wrapper.status();
        assert!(st.is_ok(), "status() on empty iterator must be OK");
    }

    #[traced_test]
    fn single_child_merging_iterator_is_passthrough() {
        trace!("TEST: single_child_merging_iterator_is_passthrough");

        let child_ptr = make_stub_child(&[(b"a", b"va"), (b"b", b"vb")]);
        let mut children: [*mut LevelDBIterator; 1] = [child_ptr];

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let result_ptr = new_merging_iterator(cmp, children.as_mut_ptr(), 1);

        assert_eq!(
            result_ptr, child_ptr,
            "n == 1 must return the original child pointer"
        );

        let mut wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        assert!(wrapper.valid(), "after seek_to_first, iterator must be valid");
        assert_eq!(wrapper.key().to_string(), "a");
        assert_eq!(wrapper.value().to_string(), "va");

        wrapper.next();
        assert!(wrapper.valid(), "after first next, still valid");
        assert_eq!(wrapper.key().to_string(), "b");
        assert_eq!(wrapper.value().to_string(), "vb");

        wrapper.next();
        assert!(!wrapper.valid(), "after consuming all, iterator must be invalid");

        wrapper.seek_to_last();
        assert!(wrapper.valid(), "seek_to_last must position at last key");
        assert_eq!(wrapper.key().to_string(), "b");
    }

    #[traced_test]
    fn merge_three_non_overlapping_children_forward() {
        trace!("TEST: merge_three_non_overlapping_children_forward");

        let c0 = make_stub_child(&[
            (b"a", b"v0a"),
            (b"d", b"v0d"),
            (b"g", b"v0g"),
        ]);
        let c1 = make_stub_child(&[
            (b"b", b"v1b"),
            (b"e", b"v1e"),
            (b"h", b"v1h"),
        ]);
        let c2 = make_stub_child(&[
            (b"c", b"v2c"),
            (b"f", b"v2f"),
            (b"i", b"v2i"),
        ]);

        let mut children = [c0, c1, c2];
        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);
        let mut wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        let mut seen = Vec::new();

        while wrapper.valid() {
            let k = wrapper.key().to_string();
            seen.push(k);
            wrapper.next();
        }

        assert_eq!(
            seen,
            vec!["a", "b", "c", "d", "e", "f", "g", "h", "i"],
            "merged iterator must yield keys in sorted order across all children"
        );
    }

    #[traced_test]
    fn merge_children_with_duplicate_keys_yields_all_entries() {
        trace!("TEST: merge_children_with_duplicate_keys_yields_all_entries");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c")]);
        let c1 = make_stub_child(&[(b"a", b"v1a"), (b"b", b"v1b")]);

        let mut children = [c0, c1];
        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);
        let mut wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();

        let mut seen = Vec::new();
        while wrapper.valid() {
            let k = wrapper.key().to_string();
            let v = wrapper.value().to_string();
            seen.push((k, v));
            wrapper.next();
        }

        assert_eq!(
            seen,
            vec![
                ("a".to_string(), "v0a".to_string()),
                ("a".to_string(), "v1a".to_string()),
                ("b".to_string(), "v1b".to_string()),
                ("c".to_string(), "v0c".to_string()),
            ],
            "duplicate keys must be yielded once per child without suppression"
        );
    }

    #[traced_test]
    fn direction_switch_from_forward_to_reverse_behaves_like_cplusplus() {
        trace!("TEST: direction_switch_from_forward_to_reverse_behaves_like_cplusplus");

        let c0 = make_stub_child(&[(b"a", b"v0a"), (b"c", b"v0c"), (b"e", b"v0e")]);
        let c1 = make_stub_child(&[(b"b", b"v1b"), (b"d", b"v1d"), (b"f", b"v1f")]);

        let mut children = [c0, c1];
        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());

        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);
        let mut wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };

        wrapper.seek_to_first();
        assert_eq!(wrapper.key().to_string(), "a");

        wrapper.next(); // b
        assert_eq!(wrapper.key().to_string(), "b");

        wrapper.next(); // c
        assert_eq!(wrapper.key().to_string(), "c");

        wrapper.prev(); // should go back to b
        assert!(wrapper.valid());
        assert_eq!(
            wrapper.key().to_string(),
            "b",
            "Prev from c must yield b per C++ semantics"
        );

        wrapper.prev(); // should go back to a
        assert!(wrapper.valid());
        assert_eq!(
            wrapper.key().to_string(),
            "a",
            "Prev from b must yield a"
        );

        wrapper.prev(); // now should become invalid
        assert!(
            !wrapper.valid(),
            "Prev from first key must invalidate the iterator"
        );
    }

    #[traced_test]
    fn status_aggregates_first_non_ok_child_status() {
        trace!("TEST: status_aggregates_first_non_ok_child_status");

        #[derive(Default)]
        struct ErrorIterator {
            status: Status,
        }

        impl LevelDBIteratorInterface for ErrorIterator {}

        impl LevelDBIteratorValid for ErrorIterator {
            fn valid(&self) -> bool {
                false
            }
        }

        impl LevelDBIteratorSeekToFirst for ErrorIterator {
            fn seek_to_first(&mut self) {}
        }

        impl LevelDBIteratorSeekToLast for ErrorIterator {
            fn seek_to_last(&mut self) {}
        }

        impl LevelDBIteratorSeek for ErrorIterator {
            fn seek(&mut self, _target: &Slice) {}
        }

        impl LevelDBIteratorNext for ErrorIterator {
            fn next(&mut self) {}
        }

        impl LevelDBIteratorPrev for ErrorIterator {
            fn prev(&mut self) {}
        }

        impl LevelDBIteratorStatus for ErrorIterator {
            fn status(&self) -> Status {
                Status::new_from_other_copy(&self.status)
            }
        }

        impl LevelDBIteratorKey for ErrorIterator {
            fn key(&self) -> Slice {
                panic!("ErrorIterator::key should not be called");
            }
        }

        impl LevelDBIteratorValue for ErrorIterator {
            fn value(&self) -> Slice {
                panic!("ErrorIterator::value should not be called");
            }
        }

        let ok_child = make_stub_child(&[(b"a", b"va")]);

        let error_status = Status::corruption(&Slice::from("corrupt"), None);
        let err_internal = ErrorIterator {
            status: Status::new_from_other_copy(&error_status),
        };
        let err_box: Box<dyn LevelDBIteratorInterface> = Box::new(err_internal);
        let err_wrapper = LevelDBIterator::new(Some(err_box));
        let err_child = Box::into_raw(Box::new(err_wrapper));

        let mut children = [ok_child, err_child];

        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        let result_ptr =
            new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

        let wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };
        let st = wrapper.status();

        assert!(
            st.is_corruption(),
            "MergingIterator::status must report the first non-OK child status"
        );
    }

    #[traced_test]
    fn single_child_merging_iterator_drops_child_on_drop() {
        trace!("TEST: single_child_merging_iterator_drops_child_on_drop");

        let drops = Arc::new(AtomicUsize::new(0));

        let internal = MockTrackedIterator::new(drops.clone());
        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        let child_ptr = Box::into_raw(Box::new(wrapper));

        let mut children = [child_ptr];

        {
            let cmp: Box<dyn SliceComparator> =
                Box::new(BytewiseComparatorImpl::default());
            let result_ptr =
                new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

            let wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };
            drop(wrapper);
        }

        assert_eq!(
            drops.load(atomic::Ordering::SeqCst),
            1,
            "single-child merging iterator must drop its underlying iterator once"
        );
    }

    #[traced_test]
    fn merging_iterator_drops_all_children_on_drop() {
        trace!("TEST: merging_iterator_drops_all_children_on_drop");

        let drops = Arc::new(AtomicUsize::new(0));

        let internal0 = MockTrackedIterator::new(drops.clone());
        let internal0_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal0);
        let wrapper0 = LevelDBIterator::new(Some(internal0_box));
        let ptr0 = Box::into_raw(Box::new(wrapper0));

        let internal1 = MockTrackedIterator::new(drops.clone());
        let internal1_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal1);
        let wrapper1 = LevelDBIterator::new(Some(internal1_box));
        let ptr1 = Box::into_raw(Box::new(wrapper1));

        let mut children = [ptr0, ptr1];

        {
            let cmp: Box<dyn SliceComparator> =
                Box::new(BytewiseComparatorImpl::default());
            let result_ptr =
                new_merging_iterator(cmp, children.as_mut_ptr(), children.len() as i32);

            let wrapper: Box<LevelDBIterator> = unsafe { Box::from_raw(result_ptr) };
            drop(wrapper);
        }

        assert_eq!(
            drops.load(atomic::Ordering::SeqCst),
            2,
            "merging iterator must drop all of its child iterators exactly once"
        );
    }
}
