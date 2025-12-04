// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_new_appendable_file.rs ]
crate::ix!();

impl NewAppendableFile for InMemoryEnv {

    fn new_appendable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            "InMemoryEnv::new_appendable_file: opening '{}' for append",
            fname
        );

        use std::collections::hash_map::Entry;

        let mut guard = self.inner_mutex().lock();

        // C++:
        // FileState** sptr = &file_map_[fname];
        // FileState* file = *sptr;
        // if (file == nullptr) { file = new FileState(); file->Ref(); *sptr = file; }
        let file_ptr: *mut FileState = match guard.file_map_mut().entry(fname.clone()) {
            Entry::Occupied(mut o) => {
                let mut ptr: *mut FileState = *o.get();
                if ptr.is_null() {
                    debug!(
                        "InMemoryEnv::new_appendable_file: existing entry with null FileState for '{}'; creating new",
                        fname
                    );
                    let mut boxed = Box::new(FileState::default());
                    ptr = &mut *boxed;
                    unsafe {
                        FileState::ref_raw(ptr);
                    }
                    let raw = Box::into_raw(boxed);
                    o.insert(raw);
                    raw
                } else {
                    debug!(
                        "InMemoryEnv::new_appendable_file: using existing FileState for '{}'",
                        fname
                    );
                    ptr
                }
            }
            Entry::Vacant(v) => {
                debug!(
                    "InMemoryEnv::new_appendable_file: file '{}' not present; creating new FileState",
                    fname
                );
                let mut boxed = Box::new(FileState::default());
                let ptr: *mut FileState = &mut *boxed;
                unsafe {
                    FileState::ref_raw(ptr);
                }
                let raw = Box::into_raw(boxed);
                v.insert(raw);
                raw
            }
        };

        unsafe {
            if result.is_null() {
                warn!(
                    "InMemoryEnv::new_appendable_file: result pointer is null for '{}'",
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
mod in_memory_env_new_appendable_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        FileExists,
        NewAppendableFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn new_appendable_file_creates_file_if_missing() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "appendable.dat".to_string();
        assert!(!env.file_exists(&fname));

        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status = env.new_appendable_file(
            &fname,
            &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());
        assert!(env.file_exists(&fname));

        unsafe {
            if !wf_ptr.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
            }
        }
    }
}
