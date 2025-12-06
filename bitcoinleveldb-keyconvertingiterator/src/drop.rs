// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/drop.rs ]
crate::ix!();

impl Drop for KeyConvertingIterator {
    fn drop(&mut self) {
        trace!(
            "KeyConvertingIterator::drop: begin; iter_ptr={:?}",
            self.iter_raw(),
        );

        unsafe {
            let raw_ptr: crate::RawInternalLevelDBIterator = self.iter_raw();

            if raw_ptr.is_null() {
                trace!(
                    "KeyConvertingIterator::drop: underlying iterator pointer is null; no owned iterator to drop"
                );
            } else {
                let boxed: Box<dyn LevelDBIteratorInterface> = Box::from_raw(raw_ptr);

                trace!(
                    "KeyConvertingIterator::drop: reconstructed Box<dyn LevelDBIteratorInterface> {:p}; dropping now",
                    &*boxed as *const dyn LevelDBIteratorInterface
                );
                // `boxed` is dropped here, invoking the underlying iterator's
                // destructor and any associated cleanup.
            }
        }

        trace!("KeyConvertingIterator::drop: end");
    }
}

#[cfg(test)]
mod key_converting_iterator_drop_tests {
    use super::*;
    use crate::{
        RecordingInternalIterator,
        RecordingInternalIteratorState,
    };
    use std::cell::RefCell;
    use std::rc::Rc;

    fn boxed_internal_iterator<T>(iter: T) -> crate::RawInternalLevelDBIterator
    where
        T: LevelDBIteratorInterface + 'static,
    {
        let boxed: Box<dyn LevelDBIteratorInterface> = Box::new(iter);
        Box::into_raw(boxed)
    }

    #[traced_test]
    fn drop_releases_underlying_iterator_and_sets_flag() {
        let shared    = Rc::new(RefCell::new(RecordingInternalIteratorState::default()));
        let drop_flag = Rc::new(RefCell::new(false));

        let raw = boxed_internal_iterator(RecordingInternalIterator::new(
            Vec::new(),
            Status::ok(),
            shared.clone(),
            drop_flag.clone(),
        ));

        {
            let _kc = KeyConvertingIterator::new(raw);
            assert!(
                !*drop_flag.borrow(),
                "underlying iterator must not be dropped while KeyConvertingIterator is alive"
            );
        }

        assert!(
            *drop_flag.borrow(),
            "dropping KeyConvertingIterator must drop the underlying iterator exactly once"
        );
    }
}
