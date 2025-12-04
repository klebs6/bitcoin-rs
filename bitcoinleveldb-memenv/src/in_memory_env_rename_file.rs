// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_rename_file.rs ]
crate::ix!();

impl RenameFile for InMemoryEnv {
    
    fn rename_file(
        &mut self,
        src:    &String,
        target: &String,
    ) -> crate::Status {
        trace!(
            "InMemoryEnv::rename_file: '{}' -> '{}'",
            src,
            target
        );

        // Check src existence first.
        let src_exists = {
            let guard = self.inner_mutex().lock();
            guard.file_map().contains_key(src)
        };

        if !src_exists {
            debug!(
                "InMemoryEnv::rename_file: source '{}' not found; returning IO error",
                src
            );
            let src_slice = Slice::from(src.as_bytes());
            let msg_slice = Slice::from("File not found".as_bytes());
            return crate::Status::io_error(&src_slice, Some(&msg_slice));
        }

        // Delete target (if present).
        self.delete_file_internal(target);

        // Move src entry to target.
        let mut guard = self.inner_mutex().lock();

        if let Some(file_ptr) = guard.file_map_mut().remove(src) {
            debug!(
                "InMemoryEnv::rename_file: moved FileState {:?} from '{}' to '{}'",
                file_ptr, src, target
            );
            guard.file_map_mut().insert(target.clone(), file_ptr);
            crate::Status::ok()
        } else {
            error!(
                "InMemoryEnv::rename_file: src '{}' disappeared during rename",
                src
            );
            let src_slice = Slice::from(src.as_bytes());
            let msg_slice = Slice::from("File not found".as_bytes());
            crate::Status::io_error(&src_slice, Some(&msg_slice))
        }
    }
}

#[cfg(test)]
mod in_memory_env_rename_file_tests {
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
    fn rename_file_moves_entry_and_overwrites_target_if_present() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let src = "src.dat".to_string();
        let dst = "dst.dat".to_string();

        // Create src and dst.
        for name in [&src, &dst].iter() {
            let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
            let status = env.new_writable_file(
                name,
                &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
            );
            assert!(status.is_ok());
            unsafe {
                if !wf_ptr.is_null() {
                    let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
                }
            }
        }

        assert!(env.file_exists(&src));
        assert!(env.file_exists(&dst));

        let status = env.rename_file(&src, &dst);
        assert!(status.is_ok());
        assert!(!env.file_exists(&src));
        assert!(env.file_exists(&dst));
    }

    #[traced_test]
    fn rename_file_missing_source_returns_io_error() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        let src = "missing_src.dat".to_string();
        let dst = "dst.dat".to_string();

        let status = env.rename_file(&src, &dst);
        assert!(status.is_io_error());
    }
}
