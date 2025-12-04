// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_new_sequential_file.rs ]
crate::ix!();

impl NewSequentialFile for InMemoryEnv {

    /**
      | Partial implementation of the Env interface.
      |
      */
    fn new_sequential_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn SequentialFile>,
    ) -> crate::Status {
        trace!("InMemoryEnv::new_sequential_file: '{}'", fname);

        let guard = self.inner_mutex().lock();

        let file_ptr_opt = guard.file_map().get(fname).copied();

        match file_ptr_opt {
            Some(file_ptr) if !file_ptr.is_null() => {
                unsafe {
                    if result.is_null() {
                        warn!(
                            "InMemoryEnv::new_sequential_file: result pointer is null for '{}'",
                            fname
                        );
                    } else {
                        let seq_impl = SequentialFileImpl::new(file_ptr);
                        let inner: Box<dyn SequentialFile> = Box::new(seq_impl);
                        let outer: Box<Box<dyn SequentialFile>> = Box::new(inner);
                        *result = Box::into_raw(outer);
                    }
                }
                crate::Status::ok()
            }
            _ => {
                debug!(
                    "InMemoryEnv::new_sequential_file: file '{}' not found",
                    fname
                );
                let fname_slice = Slice::from(fname.as_bytes());
                let msg_slice = Slice::from("File not found".as_bytes());
                crate::Status::io_error(&fname_slice, Some(&msg_slice))
            }
        }
    }
}

#[cfg(test)]
mod in_memory_env_new_sequential_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewSequentialFile,
        NewWritableFile,
        SequentialFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn new_sequential_file_behaves_like_new_random_access_file_for_existence() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "seq.dat".to_string();

        // Missing file should yield an IOError.
        let mut seq_ptr_missing: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        let status_missing = env.new_sequential_file(
            &fname,
            &mut seq_ptr_missing as *mut *mut Box<dyn SequentialFile>,
        );
        assert!(status_missing.is_io_error());

        // Create the file.
        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status_wf = env.new_writable_file(
            &fname,
            &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status_wf.is_ok());
        unsafe {
            if !wf_ptr.is_null() {
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
            }
        }

        // Now sequential file creation should succeed.
        let mut seq_ptr: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        let status = env.new_sequential_file(
            &fname,
            &mut seq_ptr as *mut *mut Box<dyn SequentialFile>,
        );
        assert!(status.is_ok());
        unsafe {
            if !seq_ptr.is_null() {
                let _outer: Box<Box<dyn SequentialFile>> = Box::from_raw(seq_ptr);
            }
        }
    }
}
