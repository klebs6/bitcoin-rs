// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_ref.rs ]
crate::ix!();

impl FileState {

    /// Increase the reference count.
    pub fn ref_(&mut self) {
        trace!("FileState::ref_: increasing reference count via &mut self");
        let ptr: *mut FileState = self;
        unsafe { FileState::ref_raw(ptr) };
    }

    /// Raw-pointer variant used by the in-memory env and file wrappers.
    pub(crate) unsafe fn ref_raw(file_ptr: *mut FileState) {
        if file_ptr.is_null() {
            warn!("FileState::ref_raw called with null pointer");
            return;
        }

        let file_ref: &FileState = &*file_ptr;
        let mut guard = file_ref
            .refs_mutex()
            .lock();

        *guard.refs_mut() += 1;
        trace!(
            "FileState::ref_raw: incremented refcount to {} for {:?}",
            guard.refs(),
            file_ptr
        );
    }
}

#[cfg(test)]
mod file_state_refcount_increment_tests {
    use super::*;

    #[traced_test]
    fn ref_raw_increments_reference_count() {
        crate::ix!();

        unsafe {
            let file = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file);

            // Initial refs should be zero.
            {
                let file_ref: &FileState = &*raw;
                let guard = file_ref.refs_mutex().lock();
                assert_eq!(*guard.refs(), 0);
            }

            // Increment twice.
            FileState::ref_raw(raw);
            FileState::ref_raw(raw);

            {
                let file_ref: &FileState = &*raw;
                let guard = file_ref.refs_mutex().lock();
                assert_eq!(*guard.refs(), 2);
            }

            // Clean up: two corresponding unrefs.
            FileState::unref_raw(raw);
            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn safe_ref_method_calls_ref_raw_for_mut_self() {
        crate::ix!();

        // Stack-allocated FileState: using ref_ should just bump the counter;
        // we do not call unref here because this instance is not managed via Box::from_raw.
        let mut file = FileState::default();

        {
            let guard = file.refs_mutex().lock();
            assert_eq!(*guard.refs(), 0);
        }

        file.ref_();

        {
            let guard = file.refs_mutex().lock();
            assert_eq!(*guard.refs(), 1);
        }
    }
}
