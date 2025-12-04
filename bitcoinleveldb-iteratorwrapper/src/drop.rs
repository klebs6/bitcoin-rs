// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/drop.rs ]
crate::ix!();

impl Drop for LevelDBIteratorWrapper {
    fn drop(&mut self) {
        trace!(
            "LevelDBIteratorWrapper::drop: dropping wrapper; iter={:?}, valid={}",
            self.iter(),
            self.valid()
        );

        unsafe {
            if !self.iter().is_null() {
                trace!(
                    "LevelDBIteratorWrapper::drop: deallocating owned iterator at {:p}",
                    self.iter()
                );

                // Safety:
                // IteratorWrapper takes ownership of `iter` and must match the
                // C++ `delete iter_` semantics. `self.iter` must have come
                // from `Box::into_raw(Box<dyn LevelDBIteratorInterface>)`.
                let boxed: Box<dyn LevelDBIteratorInterface> =
                    Box::from_raw(self.iter());
                drop(boxed);

                self.set_iter(core::ptr::null_mut());
                self.set_valid(false);

                trace!(
                    "LevelDBIteratorWrapper::drop: iterator deallocated and state cleared"
                );
            } else {
                trace!(
                    "LevelDBIteratorWrapper::drop: iter pointer already null; nothing to deallocate"
                );
            }
        }
    }
}
