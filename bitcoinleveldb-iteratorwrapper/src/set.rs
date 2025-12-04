// ---------------- [ File: bitcoinleveldb-iteratorwrapper/src/set.rs ]
crate::ix!();

impl LevelDBIteratorWrapper {
    /**
      | Takes ownership of "iter" and will delete
      | it when destroyed, or when set() is invoked
      | again.
      |
      */
    pub fn set(&mut self, iter: *mut dyn LevelDBIteratorInterface) {
        trace!(
            "LevelDBIteratorWrapper::set: replacing iterator; current_iter={:?}, new_iter={:?}",
            self.iter(),
            iter
        );

        unsafe {
            if !self.iter().is_null() {
                trace!(
                    "LevelDBIteratorWrapper::set: deallocating previously owned iterator at {:p}",
                    self.iter()
                );

                // Safety: matches C++ `delete iter_`. Calling `set` with a
                // pointer that was not allocated via Box<dyn LevelDBIteratorInterface>
                // is undefined, just like calling `delete` on a non‑owned pointer.
                let boxed: Box<dyn LevelDBIteratorInterface> =
                    Box::from_raw(self.iter());
                drop(boxed);
            } else {
                trace!(
                    "LevelDBIteratorWrapper::set: no existing iterator to deallocate (current_iter is null)"
                );
            }
        }

        self.set_iter(iter);

        if self.iter().is_null() {
            trace!(
                "LevelDBIteratorWrapper::set: new iterator is null; marking wrapper as invalid"
            );
            // Matches C++ IteratorWrapper::Set: clear valid_, leave key_ untouched.
            self.set_valid(false);
        } else {
            trace!(
                "LevelDBIteratorWrapper::set: new iterator is non-null; updating cached state"
            );
            self.update();
        }
    }
}
