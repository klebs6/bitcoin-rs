// ---------------- [ File: bitcoinleveldb-iterator/src/set.rs ]
crate::ix!();

impl LevelDBIterator {

    /**
      | Takes ownership of `iter` and will delete
      | it when destroyed, or when `set` is invoked
      | again.
      |
      | This is the direct analogue of the C++
      | `LevelDBIterator::Set(Iterator*)`.
      */
    pub fn set(&mut self, iter: Option<Box<dyn LevelDBIteratorInterface>>) {
        trace!(
            "LevelDBIterator::set: delegating to reset_iterator; new_has_iter={}",
            iter.is_some()
        );
        self.reset_iterator(iter);
    }
}

#[cfg(test)]
mod iterator_wrapper_set_tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[traced_test]
    fn set_attaches_iterator_and_updates_state() {
        trace!("set_attaches_iterator_and_updates_state: start");

        let stub = crate::MockStubIterator::new_with_entries(&[
            (b"a".as_ref(), b"1".as_ref()),
            (b"b".as_ref(), b"2".as_ref()),
        ]);

        let mut wrapper = LevelDBIterator::default();

        assert!(
            !wrapper.has_iterator(),
            "wrapper must start with no iterator"
        );

        wrapper.set(Some(Box::new(stub)));

        assert!(
            wrapper.has_iterator(),
            "set(Some(..)) must attach an iterator"
        );

        wrapper.seek_to_first();
        assert!(wrapper.valid());
        assert_eq!(wrapper.key().to_string(), "a");
    }

    #[traced_test]
    fn set_replaces_existing_iterator_and_drops_old_one() {
        trace!("set_replaces_existing_iterator_and_drops_old_one: start");

        let c1 = Arc::new(AtomicUsize::new(0));
        let c2 = Arc::new(AtomicUsize::new(0));

        let iter1 = MockTrackedIterator::new(c1.clone());
        let iter2 = MockTrackedIterator::new(c2.clone());

        let mut wrapper = LevelDBIterator::new(Some(Box::new(iter1)));

        assert_eq!(
            c1.load(Ordering::SeqCst),
            0,
            "first iterator must not be dropped while attached"
        );

        wrapper.set(Some(Box::new(iter2)));

        assert_eq!(
            c1.load(Ordering::SeqCst),
            1,
            "replacing iterator via set() must drop the previous iterator immediately"
        );
        assert_eq!(
            c2.load(Ordering::SeqCst),
            0,
            "new iterator must remain alive while wrapper is alive"
        );

        drop(wrapper);

        assert_eq!(
            c2.load(Ordering::SeqCst),
            1,
            "second iterator must be dropped when wrapper is dropped"
        );
    }

    #[traced_test]
    fn set_to_none_detaches_and_drops_iterator() {
        trace!("set_to_none_detaches_and_drops_iterator: start");

        let counter = Arc::new(AtomicUsize::new(0));

        let iter = MockTrackedIterator::new(counter.clone());
        let mut wrapper = LevelDBIterator::new(Some(Box::new(iter)));

        assert!(wrapper.has_iterator());

        wrapper.set(None);

        assert!(
            !wrapper.has_iterator(),
            "set(None) must detach the underlying iterator"
        );
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "set(None) must drop the previously attached iterator"
        );
        assert!(
            !wrapper.valid(),
            "wrapper must become invalid when iterator is detached"
        );
    }
}
