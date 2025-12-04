// ---------------- [ File: bitcoinleveldb-keyconvertingiterator/src/drop.rs ]
crate::ix!();

impl Drop for KeyConvertingIterator {
    fn drop(&mut self) {
        trace!(
            "KeyConvertingIterator::drop: begin; base_cleanup_rc_strong_count={}, iter_ptr={:?}",
            Rc::strong_count(&self.base().inner()),
            self.iter(),
        );

        unsafe {
            if self.iter().is_null() {
                trace!(
                    "KeyConvertingIterator::drop: underlying iterator pointer is null; no owned iterator to drop"
                );
            } else {
                // Reconstruct the boxed trait object so that the
                // underlying iterator implementation is properly
                // dropped and any destructors/cleanup it owns are
                // executed exactly once.
                let boxed: Box<dyn LevelDBIteratorInterface> = Box::from_raw(self.iter_mut());

                trace!(
                    "KeyConvertingIterator::drop: reconstructed Box<dyn LevelDBIteratorInterface> {:p}; dropping now",
                    &*boxed as *const dyn LevelDBIteratorInterface
                );

                // `boxed` is dropped here, invoking the underlying
                // iterator's destructor and, transitively, any
                // cleanup handlers registered on its own iterator
                // base state.
            }
        }

        trace!("KeyConvertingIterator::drop: end");
    }
}
