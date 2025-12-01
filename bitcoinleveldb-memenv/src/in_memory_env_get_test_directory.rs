crate::ix!();

impl InMemoryEnv {
    
    pub fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        
        todo!();
        /*
            *path = "/test";
        return Status::OK();
        */
    }

    pub fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        trace!("InMemoryEnv::get_test_directory called");

        unsafe {
            if path.is_null() {
                warn!("InMemoryEnv::get_test_directory: path out parameter is null");
            } else {
                *path = "/test".to_string();
            }
        }

        crate::Status::ok()
    }
}
