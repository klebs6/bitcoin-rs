// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env_get_children.rs ]
crate::ix!();

impl GetChildren for InMemoryEnv {
    
    fn get_children(
        &mut self,
        dir:    &String,
        result: *mut Vec<String>,
    ) -> crate::Status {
        trace!("InMemoryEnv::get_children: dir='{}'", dir);

        let guard = self.inner_mutex().lock();

        unsafe {
            if result.is_null() {
                warn!("InMemoryEnv::get_children: result pointer is null");
            } else {
                (*result).clear();

                for (filename, _) in guard.file_map().iter() {
                    if filename.len() >= dir.len() + 1
                        && filename.as_bytes().get(dir.len()) == Some(&b'/')
                    {
                        let filename_slice = Slice::from(filename);
                        let dir_slice = Slice::from(dir);
                        if filename_slice.starts_with(&dir_slice) {
                            let child = filename[dir.len() + 1..].to_string();
                            debug!(
                                "InMemoryEnv::get_children: adding child '{}' under '{}'",
                                child, dir
                            );
                            (*result).push(child);
                        }
                    }
                }
            }
        }

        crate::Status::ok()
    }
}

#[cfg(test)]
mod in_memory_env_get_children_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{
        Env,
        NewWritableFile,
        WritableFile,
    };
    use crate::in_memory_env::in_memory_env_behavior_tests::TestBaseEnv;

    fn touch_file(env: &mut InMemoryEnv, name: &str) {
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
    fn get_children_lists_files_within_directory() {
        crate::ix!();

        let base: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(TestBaseEnv::default()));
        let mut env = InMemoryEnv::new(base);

        touch_file(&mut env, "dir1/file_a");
        touch_file(&mut env, "dir1/file_b");
        touch_file(&mut env, "dir2/file_c");

        let dir = "dir1".to_string();
        let mut result: Vec<String> = Vec::new();

        let status = env.get_children(&dir, &mut result as *mut Vec<String>);
        assert!(status.is_ok());

        result.sort();
        assert_eq!(result, vec!["file_a".to_string(), "file_b".to_string()]);
    }
}
