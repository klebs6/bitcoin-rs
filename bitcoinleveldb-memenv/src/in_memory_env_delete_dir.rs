// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_delete_dir.rs ]
crate::ix!();

impl DeleteDir for InMemoryEnv {
    
    fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        trace!(
            "InMemoryEnv::delete_dir: dirname='{}' (no-op for in-memory env)",
            dirname
        );
        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_delete_dir_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::Env;
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn delete_dir_is_ok_and_has_no_effect_on_files() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let dirname = "/tmp/in-mem-dir".to_string();
        let status = env.delete_dir(&dirname);
        assert!(status.is_ok());

        let guard = env.inner_mutex().lock();
        assert!(guard.file_map().is_empty());
    }
}
