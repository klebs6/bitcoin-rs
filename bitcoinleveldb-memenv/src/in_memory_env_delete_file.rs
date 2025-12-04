// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_delete_file.rs ]
crate::ix!();

impl InMemoryEnv {

    #[allow(non_snake_case)]
    pub(crate) fn delete_file_internal(&mut self, fname: &String) {
        trace!(
            "InMemoryEnv::delete_file_internal: deleting file '{}' (internal helper)",
            fname
        );

        let mut guard = self.inner_mutex().lock();

        if let Some(file_ptr) = guard.file_map_mut().remove(fname) {
            debug!(
                "InMemoryEnv::delete_file_internal: removed '{}' and unref'ing FileState {:?}",
                fname, file_ptr
            );
            unsafe {
                FileState::unref_raw(file_ptr);
            }
        } else {
            debug!(
                "InMemoryEnv::delete_file_internal: '{}' not present; nothing to do",
                fname
            );
        }
    }
}

impl DeleteFile for InMemoryEnv {

    fn delete_file(&mut self, fname: &String) -> crate::Status {
        trace!("InMemoryEnv::delete_file: '{}'", fname);

        // First check existence under lock.
        let exists = {
            let guard = self.inner_mutex().lock();
            guard.file_map().contains_key(fname)
        };

        if !exists {
            debug!(
                "InMemoryEnv::delete_file: '{}' not found; returning IO error",
                fname
            );
            let fname_slice = Slice::from(fname.as_bytes());
            let msg_slice = Slice::from("File not found".as_bytes());
            return crate::Status::io_error(&fname_slice, Some(&msg_slice));
        }

        // Use internal helper to actually remove.
        self.delete_file_internal(fname);
        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_delete_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewWritableFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    fn create_empty_file(env: &mut InMemoryEnv, name: &str) {
        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let fname = name.to_string();
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
    }

    #[traced_test]
    fn delete_file_removes_existing_entry() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);
        let fname = "delete_me".to_string();

        create_empty_file(&mut env, &fname);
        assert!(env.file_exists(&fname));

        let status = env.delete_file(&fname);
        assert!(status.is_ok());
        assert!(!env.file_exists(&fname));
    }

    #[traced_test]
    fn delete_file_on_missing_entry_returns_io_error() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);
        let missing = "missing_file".to_string();

        assert!(!env.file_exists(&missing));

        let status = env.delete_file(&missing);
        assert!(status.is_io_error());
    }
}
