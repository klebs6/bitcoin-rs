crate::ix!();

impl InMemoryEnv {
    
    pub fn create_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }

    pub fn create_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            "InMemoryEnv::create_dir: dirname='{}' (no-op for in-memory env)",
            dirname
        );
        crate::Status::ok()
    }
}
