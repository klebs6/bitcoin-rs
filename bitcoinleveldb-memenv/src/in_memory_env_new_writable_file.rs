crate::ix!();

impl InMemoryEnv {

    pub fn new_writable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        InMemoryEnvFileSystem::iterator it = file_map_.find(fname);

        FileState* file;
        if (it == file_map_.end()) {
          // File is not currently open.
          file = new FileState();
          file->Ref();
          file_map_[fname] = file;
        } else {
          file = it->second;
          file->Truncate();
        }

        *result = new WritableFileImpl(file);
        return Status::OK();
        */
    }

    pub fn new_writable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut dyn WritableFile,
    ) -> crate::Status {
        trace!(
            "InMemoryEnv::new_writable_file: opening '{}' for write (truncate)",
            fname
        );

        use std::collections::hash_map::Entry;

        let guard = self.mutex.lock();
        let mut inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::new_writable_file: mutex poisoned; recovering"
                );
                poisoned.into_inner()
            }
        };

        // Mirror C++ logic:
        //   FileSystem::iterator it = file_map_.find(fname);
        //   if not found -> new FileState (Ref + insert)
        //   else -> reuse and Truncate()
        let file_ptr = match inner.file_map.entry(fname.clone()) {
            Entry::Occupied(mut o) => {
                let ptr = *o.get();
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
                    let raw_ptr = Box::into_raw(boxed);
                    o.insert(raw_ptr);
                    raw_ptr
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
                let raw_ptr = Box::into_raw(boxed);
                v.insert(raw_ptr);
                raw_ptr
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
                let boxed: Box<dyn WritableFile> = Box::new(wf);
                *result = Box::into_raw(boxed);
            }
        }

        crate::Status::ok()
    }
}
