crate::ix!();

impl InMemoryEnv {

    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn delete_file_internal(&mut self, fname: &String)  {
        
        todo!();
        /*
            if (file_map_.find(fname) == file_map_.end()) {
          return;
        }

        file_map_[fname]->Unref();
        file_map_.erase(fname);
        */
    }
    
    pub fn delete_file(&mut self, fname: &String) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        if (file_map_.find(fname) == file_map_.end()) {
          return Status::IOError(fname, "File not found");
        }

        DeleteFileInternal(fname);
        return Status::OK();
        */
    }

    #[allow(non_snake_case)]
    pub fn delete_file_internal(&mut self, fname: &String) {
        trace!(
            "InMemoryEnv::delete_file_internal: deleting file '{}' (internal helper)",
            fname
        );

        let guard = self.mutex.lock();
        let mut inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::delete_file_internal: mutex poisoned; recovering"
                );
                poisoned.into_inner()
            }
        };

        if let Some(file_ptr) = inner.file_map.remove(fname) {
            debug!(
                "InMemoryEnv::delete_file_internal: removed '{}' and unref'ing FileState {:?}",
                fname, file_ptr
            );
            unsafe {
                FileState::unref_raw(file_ptr);
            }
        } else {
            debug!(
                "InMemoryEnv::delete_file_internal: '{}' not present; nothing to do",
                fname
            );
        }
    }

    pub fn delete_file(&mut self, fname: &String) -> crate::Status {
        trace!("InMemoryEnv::delete_file: '{}'", fname);

        // First check existence under lock.
        let exists = {
            let guard = self.mutex.lock();
            let inner = match guard {
                Ok(inner) => inner,
                Err(poisoned) => {
                    warn!(
                        "InMemoryEnv::delete_file: mutex poisoned during existence check; recovering"
                    );
                    poisoned.into_inner()
                }
            };
            inner.file_map.contains_key(fname)
        };

        if !exists {
            debug!(
                "InMemoryEnv::delete_file: '{}' not found; returning IO error",
                fname
            );
            let fname_slice = Slice::from(fname.as_bytes());
            let msg_slice = Slice::from("File not found".as_bytes());
            return crate::Status::io_error(&fname_slice, Some(&msg_slice));
        }

        // Use internal helper (which acquires its own lock) to actually remove.
        self.delete_file_internal(fname);
        crate::Status::ok()
    }
}
