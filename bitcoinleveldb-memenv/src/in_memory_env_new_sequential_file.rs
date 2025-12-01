crate::ix!();

impl InMemoryEnv {

    /**
      | Partial implementation of the Env interface.
      |
      */
    pub fn new_sequential_file(&mut self, 
        fname:  &String,
        result: *mut Rc<RefCell<dyn SequentialFile>>) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        if (file_map_.find(fname) == file_map_.end()) {
          *result = nullptr;
          return Status::IOError(fname, "File not found");
        }

        *result = new SequentialFileImpl(file_map_[fname]);
        return Status::OK();
        */
    }

    /**
      | Partial implementation of the Env interface.
      |
      */
    pub fn new_sequential_file(
        &mut self,
        fname:  &String,
        result: *mut Rc<RefCell<dyn SequentialFile>>,
    ) -> crate::Status {
        trace!("InMemoryEnv::new_sequential_file: '{}'", fname);

        let guard = self.mutex.lock();
        let inner = match guard {
            Ok(inner) => inner,
            Err(poisoned) => {
                warn!(
                    "InMemoryEnv::new_sequential_file: mutex poisoned; recovering"
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
                            "InMemoryEnv::new_sequential_file: result pointer is null for '{}'",
                            fname
                        );
                    } else {
                        let seq_impl = SequentialFileImpl::new(file_ptr);
                        let rc: Rc<RefCell<dyn SequentialFile>> =
                            Rc::new(RefCell::new(seq_impl));
                        *result = rc;
                    }
                }
                crate::Status::ok()
            }
            _ => {
                debug!(
                    "InMemoryEnv::new_sequential_file: file '{}' not found",
                    fname
                );
                let fname_slice = Slice::from(fname.as_bytes());
                let msg_slice = Slice::from("File not found".as_bytes());
                crate::Status::io_error(&fname_slice, Some(&msg_slice))
            }
        }
    }
}
