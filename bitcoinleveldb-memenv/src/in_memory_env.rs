// ---------------- [ File: bitcoinleveldb-memenv/src/in_memory_env.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.cc]

pub struct InMemoryEnv {
    base:     EnvWrapper,
    mutex:    Mutex<in_memory_env::Inner>,
}

pub mod in_memory_env {
    use super::*;

    pub struct Inner {
        file_map: FileSystem,
    }

    /**
      | Map from filenames to FileState objects,
      | representing a simple file system.
      |
      */
    pub type FileSystem = HashMap<String,*mut FileState>;
}

impl Drop for InMemoryEnv {
    fn drop(&mut self) {
        todo!();
        /*
            for (const auto& kvp : file_map_) {
          kvp.second->Unref();
        }
        */
    }
}

impl InMemoryEnv {

    pub fn new(base_env: Rc<RefCell<dyn Env>>) -> Self {
    
        todo!();
        /*
        : env_wrapper(base_env),

        
        */
    }

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
    
    pub fn new_writable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        FileSystem::iterator it = file_map_.find(fname);

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
    
    pub fn file_exists(&mut self, fname: &String) -> bool {
        
        todo!();
        /*
            MutexLock lock(&mutex_);
        return file_map_.find(fname) != file_map_.end();
        */
    }
    
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
    
    pub fn create_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
    
    pub fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
    
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
    
    pub fn lock_file(&mut self, 
        fname: &String,
        lock:  *mut *mut dyn FileLock) -> crate::Status {
        
        todo!();
        /*
            *lock = new FileLock;
        return Status::OK();
        */
    }
    
    pub fn unlock_file(&mut self, lock: *mut dyn FileLock) -> crate::Status {
        
        todo!();
        /*
            delete lock;
        return Status::OK();
        */
    }
    
    pub fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        
        todo!();
        /*
            *path = "/test";
        return Status::OK();
        */
    }
    
    pub fn new_logger(&mut self, 
        fname:  &String,
        result: *mut *mut dyn Logger) -> crate::Status {
        
        todo!();
        /*
            *result = new NoOpLogger;
        return Status::OK();
        */
    }
}

/**
  | Returns a new environment that stores its data
  | in memory and delegates all non-file-storage
  | tasks to base_env. The caller must delete the
  | result when it is no longer needed.  *base_env
  | must remain live while the result is in use.
  */
pub fn new_mem_env(base_env: Rc<RefCell<dyn Env>>) -> Rc<RefCell<dyn Env>> {
    
    todo!();
        /*
            return new InMemoryEnv(base_env);
        */
}
