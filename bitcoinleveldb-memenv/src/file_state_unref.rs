// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_unref.rs ]
crate::ix!();

impl FileState {

    /// Decrease the reference count. Delete
    /// if this is the last reference.
    ///
    pub fn unref(&mut self) {
        trace!("FileState::unref: decreasing reference count via &mut self");
        let ptr: *mut FileState = self;
        unsafe { FileState::unref_raw(ptr) };
    }

    /// Raw-pointer variant used by the in-memory env and file wrappers.
    pub(crate) unsafe fn unref_raw(file_ptr: *mut FileState) {
        if file_ptr.is_null() {
            warn!("FileState::unref_raw called with null pointer");
            return;
        }

        let mut do_delete = false;

        {
            let file_ref: &FileState = &*file_ptr;
            let mut guard = file_ref
                .refs_mutex()
                .lock();

            *guard.refs_mut() -= 1;
            let current = *guard.refs();
            trace!(
                "FileState::unref_raw: decremented refcount to {} for {:?}",
                current,
                file_ptr
            );

            if current < 0 {
                error!(
                    "FileState::unref_raw: refcount went negative ({}); this indicates a bug",
                    current
                );
                debug_assert!(current >= 0, "FileState refcount must not be negative");
            }

            if current <= 0 {
                do_delete = true;
            }
        }

        if do_delete {
            trace!(
                "FileState::unref_raw: refcount <= 0 for {:?}; dropping FileState",
                file_ptr
            );
            // Reconstruct the Box and let it drop, which will invoke FileState::drop.
            let _boxed: Box<FileState> = Box::from_raw(file_ptr);
        }
    }
}

#[cfg(test)]
mod file_state_refcount_decrement_tests {
    use super::*;

    #[traced_test]
    fn unref_raw_decrements_reference_count_without_deletion_when_positive() {
        crate::ix!();

        unsafe {
            let file = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file);

            // refs = 0 initially
            FileState::ref_raw(raw);
            FileState::ref_raw(raw);

            {
                let file_ref: &FileState = &*raw;
                let guard = file_ref.refs_mutex().lock();
                assert_eq!(*guard.refs(), 2);
            }

            // First unref: should decrement but not delete.
            FileState::unref_raw(raw);

            {
                let file_ref: &FileState = &*raw;
                let guard = file_ref.refs_mutex().lock();
                assert_eq!(*guard.refs(), 1);
            }

            // Final unref: should drop the Box<FileState>.
            FileState::unref_raw(raw);
        }
    }

    #[traced_test]
    fn safe_unref_method_invokes_unref_raw() {
        crate::ix!();

        unsafe {
            let file = Box::new(FileState::default());
            let raw: *mut FileState = Box::into_raw(file);

            FileState::ref_raw(raw);
            {
                let file_ref: &FileState = &*raw;
                let mut guard = file_ref.refs_mutex().lock();
                assert_eq!(*guard.refs(), 1);
            }

            // Construct a temporary wrapper to exercise the safe unref() path.
            {
                let file_ref: &mut FileState = &mut *raw;
                file_ref.unref();
            }
            // At this point refcount reached zero and raw has been dropped; do not touch `raw` again.
        }
    }
}
