// ---------------- [ File: bitcoinleveldb-iterator/src/drop.rs ]
crate::ix!();

impl Drop for LevelDBIterator {

    fn drop(&mut self) {
        trace!(
            "LevelDBIterator::drop: dropping wrapper; has_iter={}, valid={}",
            self.has_iterator(),
            self.valid()
        );

        if let Some(iter_box) = self.take_iter() {
            let raw: *const dyn LevelDBIteratorInterface = &*iter_box;

            trace!(
                "LevelDBIterator::drop: deallocating owned iterator at {:p}",
                raw
            );

            drop(iter_box);

            self.set_valid_flag(false);

            trace!(
                "LevelDBIterator::drop: iterator deallocated and state cleared"
            );
        } else {
            trace!(
                "LevelDBIterator::drop: no iterator attached; nothing to deallocate"
            );
        }
    }
}

#[cfg(test)]
mod leveldb_iterator_wrapper_drop_tests {
    use super::*;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    #[traced_test]
    fn drop_destroys_owned_iterator_once() {
        trace!("drop_destroys_owned_iterator_once: start");

        let counter = Arc::new(AtomicUsize::new(0));

        {
            let iter = MockTrackedIterator::new(counter.clone());
            let _wrapper = LevelDBIterator::new(Some(Box::new(iter)));

            assert_eq!(
                counter.load(Ordering::SeqCst),
                0,
                "iterator must not be dropped while wrapper is alive"
            );
        }

        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "iterator must be dropped exactly once when wrapper is dropped"
        );
    }

    #[traced_test]
    fn drop_without_iterator_is_noop() {
        trace!("drop_without_iterator_is_noop: start");

        let wrapper = LevelDBIterator::default();
        drop(wrapper);

        // Reaching this point without panic is sufficient: there was no
        // iterator to deallocate and no observable side effects.
    }
}
