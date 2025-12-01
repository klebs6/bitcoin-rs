crate::ix!();

impl InMemoryEnv {
    
    pub fn unlock_file(&mut self, lock: *mut dyn FileLock) -> crate::Status {
        
        todo!();
        /*
            delete lock;
        return Status::OK();
        */
    }

    pub fn unlock_file(&mut self, lock: *mut dyn FileLock) -> crate::Status {
        trace!(
            "InMemoryEnv::unlock_file: unlocking in-memory lock pointer={:?}",
            lock
        );
        // No actual lock object is created for the in-memory env, so there is
        // nothing to free here.
        crate::Status::ok()
    }
}
