// ---------------- [ File: bitcoinleveldb-version/src/ref_unref.rs ]
crate::ix!();

impl Version {

    /// Reference count management (so Versions do not disappear out from under
    /// live iterators)
    /// 
    pub fn ref_(&mut self) {
        *self.refs_mut() += 1;
        trace!(
            "Version::ref_: incremented refs; new_refs={}",
            self.refs()
        );
    }

    pub fn unref(&mut self) {
        trace!(
            "Version::unref: current_refs={}",
            self.refs()
        );

        assert!(
            *self.refs() >= 1,
            "Version::unref: refs_ must be >= 1, got {}",
            self.refs()
        );

        *self.refs_mut() -= 1;

        if *self.refs() == 0 {
            let self_ptr: *mut Version = self;
            trace!(
                "Version::unref: refs reached zero; dropping Version at {:p}",
                self_ptr
            );
            unsafe {
                // This mirrors C++ `delete this;` and will invoke
                // `Drop for Version`, which handles unlinking and
                // file refcount updates.
                drop(Box::from_raw(self_ptr));
            }
        }
    }
}
