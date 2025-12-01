crate::ix!();

impl InMemoryEnv {

    pub fn get_file_size(&mut self, 
        fname:     &String,
        file_size: *mut u64) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        if (file_map_.find(fname) == file_map_.end()) {
          return Status::IOError(fname, "File not found");
        }

        *file_size = file_map_[fname]->Size();
        return Status::OK();
        */
    }

    pub fn get_file_size(
        &mut self,
        fname:     &String,
        file_size: *mut u64,
    ) -> crate::Status {
        trace!("InMemoryEnv::get_file_size: '{}'", fname);

        let guard = self.mutex.lock();
        let inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::get_file_size: mutex poisoned; recovering"
                );
                poisoned.into_inner()
            }
        };

        let file_ptr_opt = inner.file_map.get(fname).copied();

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
