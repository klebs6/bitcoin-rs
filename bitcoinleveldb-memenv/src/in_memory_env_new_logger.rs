// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_new_logger.rs ]
crate::ix!();

impl NewLogger for InMemoryEnv {
    
    fn new_logger(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn Logger>,
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
                let inner: Box<dyn Logger> = Box::new(logger);
                let outer: Box<Box<dyn Logger>> = Box::new(inner);
                *result = Box::into_raw(outer);
            }
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_new_logger_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Env, Logger, NewLogger};
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn new_logger_returns_a_logger_instance() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "logfile".to_string();
        let mut logger_ptr: *mut Box<dyn Logger> = core::ptr::null_mut();

        let status = env.new_logger(&fname, &mut logger_ptr as *mut *mut Box<dyn Logger>);
        assert!(status.is_ok());
        unsafe {
            if !logger_ptr.is_null() {
                let _outer: Box<Box<dyn Logger>> = Box::from_raw(logger_ptr);
            }
        }
    }
}
