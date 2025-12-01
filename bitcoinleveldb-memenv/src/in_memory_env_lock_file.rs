crate::ix!();

impl InMemoryEnv {
    
    pub fn lock_file(&mut self, 
        fname: &String,
        lock:  *mut *mut dyn FileLock) -> crate::Status {
        
        todo!();
        /*
            *lock = new FileLock;
        return Status::OK();
        */
    }

    pub fn lock_file(
        &mut self,
        fname: &String,
        lock:  *mut *mut dyn FileLock,
    ) -> crate::Status {
        trace!("InMemoryEnv::lock_file: '{}'", fname);

        unsafe {
            if lock.is_null() {
                warn!(
                    "InMemoryEnv::lock_file: lock out parameter is null for '{}'",
                    fname
                );
            } else {
                // In-memory env does not need a real FileLock object.
                // Set a null pointer and rely on status for correctness.
                *lock = std::ptr::null_mut();
            }
        }

        crate::Status::ok()
    }
}
