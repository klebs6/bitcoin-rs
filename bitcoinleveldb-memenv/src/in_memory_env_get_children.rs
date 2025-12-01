crate::ix!();

impl InMemoryEnv {
    
    pub fn get_children(&mut self, 
        dir:    &String,
        result: *mut Vec<String>) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        result->clear();

        for (const auto& kvp : file_map_) {
          const std::string& filename = kvp.first;

          if (filename.size() >= dir.size() + 1 && filename[dir.size()] == '/' &&
              Slice(filename).starts_with(Slice(dir))) {
            result->push_back(filename.substr(dir.size() + 1));
          }
        }

        return Status::OK();
        */
    }

    pub fn get_children(
        &mut self,
        dir:    &String,
        result: *mut Vec<String>,
    ) -> crate::Status {
        trace!("InMemoryEnv::get_children: dir='{}'", dir);

        let guard = self.mutex.lock();
        let inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!("InMemoryEnv::get_children: mutex poisoned; recovering");
                poisoned.into_inner()
            }
        };

        unsafe {
            if result.is_null() {
                warn!("InMemoryEnv::get_children: result pointer is null");
            } else {
                (*result).clear();

                for (filename, _) in inner.file_map.iter() {
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
