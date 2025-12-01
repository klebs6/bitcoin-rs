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
            let mut guard = match file_ref.refs_mutex.lock() {
                Ok(guard) => guard,
                Err(poisoned) => {
                    warn!("FileState::unref_raw: refs_mutex poisoned; recovering");
                    poisoned.into_inner()
                }
            };

            guard.refs -= 1;
            let current = guard.refs;
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
