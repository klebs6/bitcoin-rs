// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_get_file_size.rs ]
crate::ix!();

impl GetFileSize for InMemoryEnv {

    fn get_file_size(
        &mut self,
        fname:     &String,
        file_size: *mut u64,
    ) -> crate::Status {
        trace!("InMemoryEnv::get_file_size: '{}'", fname);

        let guard = self.inner_mutex().lock();

        let file_ptr_opt = guard.file_map().get(fname).copied();

        match file_ptr_opt {
            Some(file_ptr) if !file_ptr.is_null() => unsafe {
                let file_ref: &FileState = &*file_ptr;
                let size = file_ref.size();
                if file_size.is_null() {
                    warn!(
                        "InMemoryEnv::get_file_size: file_size out parameter is null for '{}'",
                        fname
                    );
                } else {
                    *file_size = size;
                }
                debug!(
                    "InMemoryEnv::get_file_size: '{}' size={}",
                    fname, size
                );
                crate::Status::ok()
            },
            _ => {
                debug!(
                    "InMemoryEnv::get_file_size: '{}' not found; returning IO error",
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
mod in_memory_env_get_file_size_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewWritableFile,
        WritableFile,
        WritableFileAppend,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    #[traced_test]
    fn get_file_size_returns_size_for_existing_file() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let fname = "size.dat".to_string();
        let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let status = env.new_writable_file(
            &fname,
            &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());

        // Append some data via the concrete WritableFileImpl behind the trait.
        unsafe {
            if !wf_ptr.is_null() {
                let writable_ref: &mut Box<dyn WritableFile> = &mut *wf_ptr;
                let payload = b"1234567890";
                let slice = Slice::from(&payload[..]);
                let append_status =
                    WritableFileAppend::append(writable_ref.as_mut(), &slice);
                assert!(append_status.is_ok());

                // Drop the wrapper to release one reference.
                let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
            }
        }

        let mut size: u64 = 0;
        let status = env.get_file_size(&fname, &mut size as *mut u64);
        assert!(status.is_ok());
        assert_eq!(size, 10);
    }

    #[traced_test]
    fn get_file_size_for_missing_file_returns_io_error() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let missing = "missing_size.dat".to_string();
        let mut size: u64 = 123;

        let status = env.get_file_size(&missing, &mut size as *mut u64);
        assert!(status.is_io_error());
    }
}
