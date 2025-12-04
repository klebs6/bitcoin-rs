// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_file_exists.rs ]
crate::ix!();

impl FileExists for InMemoryEnv {
    
    fn file_exists(&mut self, fname: &String) -> bool {
        trace!("InMemoryEnv::file_exists: '{}'", fname);

        let guard = self.inner_mutex().lock();

        let exists = guard.file_map().contains_key(fname);
        debug!(
            "InMemoryEnv::file_exists: '{}' exists? {}",
            fname, exists
        );
        exists
    }
}

#[cfg(test)]
mod in_memory_env_file_exists_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewWritableFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn file_exists_reflects_presence_in_file_system() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "exists.dat".to_string();
        assert!(!env.file_exists(&fname));

        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status = env.new_writable_file(
            &fname,
            &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());

        unsafe {
            if !wf_ptr.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
            }
        }

        assert!(env.file_exists(&fname));
    }
}
