// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_unlock_file.rs ]
crate::ix!();

impl UnlockFile for InMemoryEnv {
    
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        trace!(
            "InMemoryEnv::unlock_file: unlocking in-memory lock pointer={:?}",
            lock
        );

        unsafe {
            if !lock.is_null() {
                // Reconstruct the outer Box<Box<dyn FileLock>> and drop it.
                let outer: Box<Box<dyn FileLock>> = Box::from_raw(lock);
                drop(outer);
            } else {
                debug!(
                    "InMemoryEnv::unlock_file: lock pointer is null; nothing to do"
                );
            }
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_unlock_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Env, FileLock, LockFile, UnlockFile};
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn unlock_file_is_noop_for_null_pointer_and_ok_for_real_lock() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        // Null pointer path.
        let status_null = env.unlock_file(core::ptr::null_mut::<Box<dyn FileLock>>());
        assert!(status_null.is_ok());

        // Real lock path.
        let fname = "lockfile_unlock".to_string();
        let mut lock_ptr: *mut Box<dyn FileLock> = core::ptr::null_mut();
        let status_lock = env.lock_file(&fname, &mut lock_ptr as *mut *mut Box<dyn FileLock>);
        assert!(status_lock.is_ok());
        assert!(!lock_ptr.is_null());

        let status_unlock = env.unlock_file(lock_ptr);
        assert!(status_unlock.is_ok());
    }
}
