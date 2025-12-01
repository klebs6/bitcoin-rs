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
        let mut guard = match file_ref.refs_mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("FileState::ref_raw: refs_mutex poisoned; recovering");
                poisoned.into_inner()
            }
        };
        guard.refs += 1;
        trace!(
            "FileState::ref_raw: incremented refcount to {} for {:?}",
            guard.refs,
            file_ptr
        );
    }
}
