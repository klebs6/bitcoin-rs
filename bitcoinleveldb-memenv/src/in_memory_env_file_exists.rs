crate::ix!();

impl InMemoryEnv {
    
    pub fn file_exists(&mut self, fname: &String) -> bool {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        return file_map_.find(fname) != file_map_.end();
        */
    }

    pub fn file_exists(&mut self, fname: &String) -> bool {
        trace!("InMemoryEnv::file_exists: '{}'", fname);

        let guard = self.mutex.lock();
        let inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!("InMemoryEnv::file_exists: mutex poisoned; recovering");
                poisoned.into_inner()
            }
        };

        let exists = inner.file_map.contains_key(fname);
        debug!(
            "InMemoryEnv::file_exists: '{}' exists? {}",
            fname, exists
        );
        exists
    }
}
