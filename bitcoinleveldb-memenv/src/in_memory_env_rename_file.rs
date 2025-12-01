crate::ix!();

impl InMemoryEnv {
    
    pub fn rename_file(&mut self, 
        src:    &String,
        target: &String) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        if (file_map_.find(src) == file_map_.end()) {
          return Status::IOError(src, "File not found");
        }

        DeleteFileInternal(target);
        file_map_[target] = file_map_[src];
        file_map_.erase(src);
        return Status::OK();
        */
    }

    pub fn rename_file(
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
            let guard = self.mutex.lock();
            let inner = match guard {
                Ok(inner) => inner,
                Err(poisoned) => {
                    warn!(
                        "InMemoryEnv::rename_file: mutex poisoned during existence check; recovering"
                    );
                    poisoned.into_inner()
                }
            };
            inner.file_map.contains_key(src)
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
        let guard = self.mutex.lock();
        let mut inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::rename_file: mutex poisoned while moving entry; recovering"
                );
                poisoned.into_inner()
            }
        };

        if let Some(file_ptr) = inner.file_map.remove(src) {
            debug!(
                "InMemoryEnv::rename_file: moved FileState {:?} from '{}' to '{}'",
                file_ptr, src, target
            );
            inner.file_map.insert(target.clone(), file_ptr);
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
