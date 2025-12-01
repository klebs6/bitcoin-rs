crate::ix!();

impl InMemoryEnv {
    
    pub fn new_appendable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        FileState** sptr = &file_map_[fname];
        FileState* file = *sptr;
        if (file == nullptr) {
          file = new FileState();
          file->Ref();
        }
        *result = new WritableFileImpl(file);
        return Status::OK();
        */
    }

    pub fn new_appendable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut dyn WritableFile,
    ) -> crate::Status {
        trace!(
            "InMemoryEnv::new_appendable_file: opening '{}' for append",
            fname
        );

        use std::collections::hash_map::Entry;

        let guard = self.mutex.lock();
        let mut inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::new_appendable_file: mutex poisoned; recovering"
                );
                poisoned.into_inner()
            }
        };

        // C++:
        // FileState** sptr = &file_map_[fname];
        // FileState* file = *sptr;
        // if (file == nullptr) { file = new FileState(); file->Ref(); *sptr = file; }
        let file_ptr = match inner.file_map.entry(fname.clone()) {
            Entry::Occupied(mut o) => {
                let mut ptr = *o.get();
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
                    let ptr = Box::into_raw(boxed);
                    o.insert(ptr);
                    ptr
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
                let mut_ptr: *mut FileState = &mut *boxed;
                unsafe {
                    FileState::ref_raw(mut_ptr);
                }
                let raw_ptr = Box::into_raw(boxed);
                v.insert(raw_ptr);
                raw_ptr
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
                let boxed: Box<dyn WritableFile> = Box::new(wf);
                *result = Box::into_raw(boxed);
            }
        }

        crate::Status::ok()
    }
}
