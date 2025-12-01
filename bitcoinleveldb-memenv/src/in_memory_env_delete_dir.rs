crate::ix!();

impl InMemoryEnv {
    
    pub fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }

    pub fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            "InMemoryEnv::delete_dir: dirname='{}' (no-op for in-memory env)",
            dirname
        );
        crate::Status::ok()
    }
}
