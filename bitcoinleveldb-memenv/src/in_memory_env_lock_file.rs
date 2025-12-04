// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_lock_file.rs ]
crate::ix!();

#[derive(Debug)]
pub struct InMemoryFileLock;

impl FileLock for InMemoryFileLock {}

impl LockFile for InMemoryEnv {
    
    fn lock_file(
        &mut self,
        fname: &String,
        lock:  *mut *mut Box<dyn FileLock>,
    ) -> crate::Status {
        trace!("InMemoryEnv::lock_file: '{}'", fname);

        unsafe {
            if lock.is_null() {
                warn!(
                    "InMemoryEnv::lock_file: lock out parameter is null for '{}'",
                    fname
                );
            } else {
                let lock_impl = InMemoryFileLock;
                let inner: Box<dyn FileLock> = Box::new(lock_impl);
                let outer: Box<Box<dyn FileLock>> = Box::new(inner);
                *lock = Box::into_raw(outer);
            }
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_lock_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Env, FileLock, LockFile, UnlockFile};
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn lock_and_unlock_file_round_trip() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "lockfile".to_string();
        let mut lock_ptr: *mut Box<dyn FileLock> = core::ptr::null_mut();

        let status = env.lock_file(&fname, &mut lock_ptr as *mut *mut Box<dyn FileLock>);
        assert!(status.is_ok());

        // We should receive some lock object (implementation-specific).
        assert!(!lock_ptr.is_null());

        // Unlock must succeed and free the outer box.
        let status = env.unlock_file(lock_ptr);
        assert!(status.is_ok());
    }
}
