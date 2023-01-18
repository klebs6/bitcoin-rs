crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/helpers/memenv/memenv.cc]

#[no_copy]
pub struct FileState {
    refs_mutex:   Mutex<FileStateRefs>,
    blocks_mutex: RefCell<Mutex<FileStateBlocks>>,
}

pub const FILE_STATE_BLOCK_SIZE: usize = 8 * 1024;

#[no_copy]
pub struct FileStateRefs {
    refs: i32,
}

#[no_copy]
pub struct FileStateBlocks {
    blocks: Vec<*mut u8>,
    size:   u64,
}

impl Default for FileState {
    
    /**
      | FileStates are reference counted.
      | The initial reference count is zero
      | and the caller must call Ref() at least
      | once.
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : refs(0),
        : size(0),

        
        */
    }
}

impl FileState {

    /**
       Increase the reference count.
      */
    pub fn ref_(&mut self)  {
        
        todo!();
        /*
            MutexLock lock(&refs_mutex_);
        ++refs_;
        */
    }

    /**
      | Decrease the reference count. Delete
      | if this is the last reference.
      |
      */
    pub fn unref(&mut self)  {
        
        todo!();
        /*
            bool do_delete = false;

        {
          MutexLock lock(&refs_mutex_);
          --refs_;
          assert(refs_ >= 0);
          if (refs_ <= 0) {
            do_delete = true;
          }
        }

        if (do_delete) {
          delete this;
        }
        */
    }
    
    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        return size_;
        */
    }
    
    pub fn truncate(&mut self)  {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        for (char*& block : blocks_) {
          delete[] block;
        }
        blocks_.clear();
        size_ = 0;
        */
    }
    
    pub fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            MutexLock lock(&blocks_mutex_);
        if (offset > size_) {
          return Status::IOError("Offset greater than file size.");
        }
        const uint64_t available = size_ - offset;
        if (n > available) {
          n = static_cast<size_t>(available);
        }
        if (n == 0) {
          *result = Slice();
          return Status::OK();
        }

        assert(offset / kBlockSize <= std::numeric_limits<size_t>::max());
        size_t block = static_cast<size_t>(offset / kBlockSize);
        size_t block_offset = offset % kBlockSize;
        size_t bytes_to_copy = n;
        char* dst = scratch;

        while (bytes_to_copy > 0) {
          size_t avail = kBlockSize - block_offset;
          if (avail > bytes_to_copy) {
            avail = bytes_to_copy;
          }
          memcpy(dst, blocks_[block] + block_offset, avail);

          bytes_to_copy -= avail;
          dst += avail;
          block++;
          block_offset = 0;
        }

        *result = Slice(scratch, n);
        return Status::OK();
        */
    }
    
    pub fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            const char* src = data.data();
        size_t src_len = data.size();

        MutexLock lock(&blocks_mutex_);
        while (src_len > 0) {
          size_t avail;
          size_t offset = size_ % kBlockSize;

          if (offset != 0) {
            // There is some room in the last block.
            avail = kBlockSize - offset;
          } else {
            // No room in the last block; push new one.
            blocks_.push_back(new char[kBlockSize]);
            avail = kBlockSize;
          }

          if (avail > src_len) {
            avail = src_len;
          }
          memcpy(blocks_.back() + offset, src, avail);
          src_len -= avail;
          src += avail;
          size_ += avail;
        }

        return Status::OK();
        */
    }
}

impl Drop for FileState {

    /**
      | Private since only Unref() should be
      | used to delete it.
      |
      */
    fn drop(&mut self) {
        todo!();
        /*
            Truncate();
        */
    }
}

///-------------------
pub struct SequentialFileImpl {
    file: *mut FileState,
    pos:  u64,
}

impl SequentialFile for SequentialFileImpl { }

impl SequentialFileRead for SequentialFileImpl {

    fn read(&mut self, 
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            Status s = file_->Read(pos_, n, result, scratch);
        if (s.ok()) {
          pos_ += result->size();
        }
        return s;
        */
    }
}

impl SequentialFileSkip for SequentialFileImpl {

    fn skip(&mut self, n: u64) -> crate::Status {
        
        todo!();
        /*
            if (pos_ > file_->Size()) {
          return Status::IOError("pos_ > file_->Size()");
        }
        const uint64_t available = file_->Size() - pos_;
        if (n > available) {
          n = available;
        }
        pos_ += n;
        return Status::OK();
        */
    }
}

impl Drop for SequentialFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl GetName for SequentialFileImpl {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "[memenv]";
        */
    }
}

impl SequentialFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),
        : pos(0),

            file_->Ref();
        */
    }
}

///-------------------
pub struct RandomAccessFileImpl {
    file: *mut FileState,
}

impl RandomAccessFile for RandomAccessFileImpl { }

impl RandomAccessFileRead for RandomAccessFileImpl { 

    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            return file_->Read(offset, n, result, scratch);
        */
    }
}

impl Drop for RandomAccessFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl GetName for RandomAccessFileImpl {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "[memenv]";
        */
    }
}
 
impl RandomAccessFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),

            file_->Ref();
        */
    }
}

///------------------
pub struct WritableFileImpl {
    file: *mut FileState,
}

impl WritableFile for WritableFileImpl {}

impl Drop for WritableFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl GetName for WritableFileImpl {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return "[memenv]";
        */
    }
}

impl WritableFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),

            file_->Ref();
        */
    }
}

impl WritableFileAppend for WritableFileImpl {

    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            return file_->Append(data);
        */
    }
}
    
impl WritableFileClose for WritableFileImpl {
    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileFlush for WritableFileImpl {
    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileSync for WritableFileImpl {
    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}

///------------
pub struct NoOpLogger {

}

impl Logger for NoOpLogger { }

impl Logv for NoOpLogger {

    fn logv(&mut self, 
        format: *const u8,
        ap:     &[&str])  { }
}

///------------
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
