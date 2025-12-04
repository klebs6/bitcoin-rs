// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_new_writable_file.rs ]
crate::ix!();

impl NewWritableFile for InMemoryEnv {

    fn new_writable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            "InMemoryEnv::new_writable_file: opening '{}' for write (truncate)",
            fname
        );

        use std::collections::hash_map::Entry;

        let mut guard = self.inner_mutex().lock();

        // Mirror C++ logic:
        //   FileSystem::iterator it = file_map_.find(fname);
        //   if not found -> new FileState (Ref + insert)
        //   else -> reuse and Truncate()
        let file_ptr: *mut FileState = match guard.file_map_mut().entry(fname.clone()) {
            Entry::Occupied(mut o) => {
                let ptr: *mut FileState = *o.get();
                if !ptr.is_null() {
                    debug!(
                        "InMemoryEnv::new_writable_file: reusing existing FileState for '{}'; truncating",
                        fname
                    );
                    unsafe {
                        let file_state: &mut FileState = &mut *ptr;
                        file_state.truncate();
                    }
                    ptr
                } else {
                    debug!(
                        "InMemoryEnv::new_writable_file: occupied entry with null pointer for '{}'; creating new FileState",
                        fname
                    );
                    let mut boxed = Box::new(FileState::default());
                    let raw_ptr: *mut FileState = &mut *boxed;
                    unsafe {
                        FileState::ref_raw(raw_ptr);
                    }
                    let raw = Box::into_raw(boxed);
                    o.insert(raw);
                    raw
                }
            }
            Entry::Vacant(v) => {
                debug!(
                    "InMemoryEnv::new_writable_file: file '{}' not open; creating new FileState",
                    fname
                );
                let mut boxed = Box::new(FileState::default());
                let raw_ptr: *mut FileState = &mut *boxed;
                unsafe {
                    FileState::ref_raw(raw_ptr);
                }
                let raw = Box::into_raw(boxed);
                v.insert(raw);
                raw
            }
        };

        unsafe {
            if result.is_null() {
                warn!(
                    "InMemoryEnv::new_writable_file: result pointer is null for '{}'",
                    fname
                );
            } else {
                let wf = WritableFileImpl::new(file_ptr);
                let inner: Box<dyn WritableFile> = Box::new(wf);
                let outer: Box<Box<dyn WritableFile>> = Box::new(inner);
                *result = Box::into_raw(outer);
            }
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_new_writable_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        FileExists,
        NewWritableFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn new_writable_file_creates_and_overwrites_files() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "writable.dat".to_string();
        assert!(!env.file_exists(&fname));

        // First creation
        let mut wf_ptr1: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status1 = env.new_writable_file(
            &fname,
            &mut wf_ptr1 as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status1.is_ok());
        assert!(env.file_exists(&fname));
        unsafe {
            if !wf_ptr1.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr1);
            }
        }

        // Second creation should truncate/overwrite the same entry and still be OK.
        let mut wf_ptr2: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status2 = env.new_writable_file(
            &fname,
            &mut wf_ptr2 as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status2.is_ok());
        unsafe {
            if !wf_ptr2.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr2);
            }
        }
    }
}
