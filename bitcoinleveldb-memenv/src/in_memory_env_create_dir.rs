// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_create_dir.rs ]
crate::ix!();

impl CreateDir for InMemoryEnv {
    
    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            "InMemoryEnv::create_dir: dirname='{}' (no-op for in-memory env)",
            dirname
        );
        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_create_dir_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Env;
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn create_dir_is_ok_and_does_not_create_files() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let dirname = "/tmp/in-mem-dir".to_string();
        let status = env.create_dir(&dirname);
        assert!(status.is_ok());

        // No files should have been created as a side-effect.
        let guard = env.inner_mutex().lock();
        assert!(guard.file_map().is_empty());
    }
}
