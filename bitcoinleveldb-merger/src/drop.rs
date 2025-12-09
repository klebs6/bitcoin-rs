// ---------------- [ File: bitcoinleveldb-merger/src/drop.rs ]
crate::ix!();

impl Drop for MergingIterator {
    fn drop(&mut self) {
        trace!(
            "MergingIterator::drop: dropping with {} children",
            self.children().len()
        );
        // `children` owns all child iterators via `Box<LevelDBIterator>`.
        // Dropping `self.children` will drop each wrapper, which in turn
        // drops the underlying LevelDBIteratorInterface implementations.
    }
}

#[cfg(test)]
mod merging_iterator_drop_tests {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    fn make_tracked_child(counter: Arc<AtomicUsize>) -> *mut LevelDBIterator {
        let internal = MockTrackedIterator::new(counter);
        let internal_box: Box<dyn LevelDBIteratorInterface> = Box::new(internal);
        let wrapper = LevelDBIterator::new(Some(internal_box));
        Box::into_raw(Box::new(wrapper))
    }

    #[traced_test]
    fn dropping_merging_iterator_drops_all_children_exactly_once() {
        trace!("TEST(drop): dropping_merging_iterator_drops_all_children_exactly_once");

        let drops = Arc::new(AtomicUsize::new(0));

        let c0 = make_tracked_child(drops.clone());
        let c1 = make_tracked_child(drops.clone());
        let c2 = make_tracked_child(drops.clone());

        let mut children = [c0, c1, c2];

        {
            let cmp: Box<dyn SliceComparator> =
                Box::new(BytewiseComparatorImpl::default());

            let merging = MergingIterator::new(
                cmp,
                children.as_mut_ptr(),
                children.len() as i32,
            );

            assert!(
                children.iter().all(|p| p.is_null()),
                "All raw child slots must be nulled after adoption"
            );

            trace!(
                "drop-test: constructed MergingIterator with {} children",
                merging.children().len()
            );

            // Dropping `merging` should cascade drops into all tracked iterators.
            drop(merging);
        }

        let observed = drops.load(Ordering::SeqCst);
        trace!("drop-test: observed {} tracked child drops", observed);

        assert_eq!(
            observed,
            3,
            "Dropping MergingIterator must drop each tracked child exactly once"
        );
    }
}
