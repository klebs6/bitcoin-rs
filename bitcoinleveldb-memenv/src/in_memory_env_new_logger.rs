crate::ix!();

impl InMemoryEnv {
    
    pub fn new_logger(&mut self, 
        fname:  &String,
        result: *mut *mut dyn Logger) -> crate::Status {
        
        todo!();
        /*
            *result = new NoOpLogger;
        return Status::OK();
        */
    }

    pub fn new_logger(
        &mut self,
        fname:  &String,
        result: *mut *mut dyn Logger,
    ) -> crate::Status {
        trace!("InMemoryEnv::new_logger: creating NoOpLogger for '{}'", fname);

        unsafe {
            if result.is_null() {
                warn!(
                    "InMemoryEnv::new_logger: result out parameter is null for '{}'",
                    fname
                );
            } else {
                let logger = NoOpLogger {};
                let boxed: Box<dyn Logger> = Box::new(logger);
                *result = Box::into_raw(boxed);
            }
        }

        crate::Status::ok()
    }
}
