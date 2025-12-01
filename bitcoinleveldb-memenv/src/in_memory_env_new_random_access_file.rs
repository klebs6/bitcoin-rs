crate::ix!();

impl InMemoryEnv {
    
    pub fn new_random_access_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn RandomAccessFile) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        if (file_map_.find(fname) == file_map_.end()) {
          *result = nullptr;
          return Status::IOError(fname, "File not found");
        }

        *result = new RandomAccessFileImpl(file_map_[fname]);
        return Status::OK();
        */
    }

    pub fn new_random_access_file(
        &mut self,
        fname:  &String,
        result: *mut *mut dyn RandomAccessFile,
    ) -> crate::Status {
        trace!("InMemoryEnv::new_random_access_file: '{}'", fname);

        let guard = self.mutex.lock();
        let inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::new_random_access_file: mutex poisoned; recovering"
                );
                poisoned.into_inner()
            }
        };

        let file_ptr_opt = inner.file_map.get(fname).copied();

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
                        let boxed: Box<dyn RandomAccessFile> = Box::new(raf);
                        *result = Box::into_raw(boxed);
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
