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

#[cfg(test)]
mod version_ref_and_unref_tests {
    use super::*;
    use super::version_test_helpers as helpers;

    #[traced_test]
    fn ref_increments_reference_count() {
        let version_box = helpers::build_boxed_empty_version();
        let raw: *mut Version = Box::into_raw(version_box);

        unsafe {
            assert_eq!(
                *(*raw).refs(),
                0,
                "Fresh Version must start with refs == 0"
            );
            (*raw).ref_();
            assert_eq!(
                *(*raw).refs(),
                1,
                "ref_ must increment refs from 0 to 1"
            );
        }

        // Leak the Version here to avoid interacting with unref / Drop.
    }

    #[traced_test]
    fn unref_deletes_version_when_reference_count_reaches_zero() {
        let version_box = helpers::build_boxed_empty_version();
        let raw: *mut Version = Box::into_raw(version_box);

        unsafe {
            (*raw).ref_();
            (*raw).unref();
            // After unref, the Version has been deallocated via Box::from_raw;
            // we must not touch `raw` again.
        }
    }
}
