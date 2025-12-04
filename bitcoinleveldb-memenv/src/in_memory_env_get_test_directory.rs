// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_get_test_directory.rs ]
crate::ix!();

impl GetTestDirectory for InMemoryEnv {
    
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
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

#[cfg(test)]
mod in_memory_env_get_test_directory_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Env;
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn get_test_directory_returns_constant_path() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let mut path = String::new();
        let status = env.get_test_directory(&mut path as *mut String);
        assert!(status.is_ok());
        assert_eq!(path, "/test".to_string());
    }
}
