// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_new_random_access_file.rs ]
crate::ix!();

impl NewRandomAccessFile for InMemoryEnv {
    
    fn new_random_access_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> crate::Status {
        trace!("InMemoryEnv::new_random_access_file: '{}'", fname);

        let guard = self.inner_mutex().lock();

        let file_ptr_opt = guard.file_map().get(fname).copied();

        match file_ptr_opt {
            Some(file_ptr) if !file_ptr.is_null() => {
                unsafe {
                    if result.is_null() {
                        warn!(
                            "InMemoryEnv::new_random_access_file: result pointer is null for '{}'",
                            fname
                        );
                    } else {
                        let raf = RandomAccessFileImpl::new(file_ptr);
                        let inner: Box<dyn RandomAccessFile> = Box::new(raf);
                        let outer: Box<Box<dyn RandomAccessFile>> = Box::new(inner);
                        *result = Box::into_raw(outer);
                    }
                }
                crate::Status::ok()
            }
            _ => {
                debug!(
                    "InMemoryEnv::new_random_access_file: file '{}' not found",
                    fname
                );
                unsafe {
                    if !result.is_null() {
                        *result = std::ptr::null_mut();
                    }
                }
                let fname_slice = Slice::from(fname.as_bytes());
                let msg_slice = Slice::from("File not found".as_bytes());
                crate::Status::io_error(&fname_slice, Some(&msg_slice))
            }
        }
    }
}

#[cfg(test)]
mod in_memory_env_new_random_access_file_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewRandomAccessFile,
        NewWritableFile,
        RandomAccessFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn new_random_access_file_succeeds_for_existing_file_and_fails_for_missing() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "random.dat".to_string();

        // First, calling on a missing file should yield an IOError.
        let mut raf_ptr_missing: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
        let status_missing = env.new_random_access_file(
            &fname,
            &mut raf_ptr_missing as *mut *mut Box<dyn RandomAccessFile>,
        );
        assert!(status_missing.is_io_error());
        unsafe {
            assert!(raf_ptr_missing.is_null());
        }

        // Create the file via NewWritableFile.
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

        // Now NewRandomAccessFile should succeed.
        let mut raf_ptr: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
        let status = env.new_random_access_file(
            &fname,
            &mut raf_ptr as *mut *mut Box<dyn RandomAccessFile>,
        );
        assert!(status.is_ok());

        unsafe {
            if !raf_ptr.is_null() {
                let _outer: Box<Box<dyn RandomAccessFile>> = Box::from_raw(raf_ptr);
            }
        }
    }
}
