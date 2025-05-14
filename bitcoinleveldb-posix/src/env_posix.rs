// ---------------- [ File: bitcoinleveldb-posix/src/env_posix.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_posix.cc]

/**
  | Set by
  | 
  | EnvPosixTestHelper::SetReadOnlyMMapLimit()
  | and
  | 
  | MaxOpenFiles().
  |
  */
lazy_static!{
    /*
    int g_open_read_only_file_limit = -1;
    */
}

/**
  | Up to 4096 mmap regions for 64-bit binaries;
  | none for 32-bit.
  |
  */
pub const DEFAULT_MMAP_LIMIT: i32 = ternary!{size_of::<*mut c_void>() >= 8,  4096,  0};

/**
  | Can be set using
  | 
  | EnvPosixTestHelper::SetReadOnlyMMapLimit().
  |
  */
lazy_static!{
    /*
    int g_mmap_limit = kDefaultMmapLimit;
    */
}

/**
  | Common flags defined for all posix open
  | operations
  |
  */
#[cfg(HAVE_O_CLOEXEC)]
pub const OPEN_BASE_FLAGS: i32 = O_CLOEXEC;

#[cfg(not(HAVE_O_CLOEXEC))]
pub const OPEN_BASE_FLAGS: i32 = 0;

pub const WRITABLE_FILE_BUFFER_SIZE: usize = 65536;

pub fn posix_error(
        context:      &String,
        error_number: i32) -> crate::Status {
    
    todo!();
        /*
            if (error_number == ENOENT) {
        return crate::Status::NotFound(context, std::strerror(error_number));
      } else {
        return crate::Status::IOError(context, std::strerror(error_number));
      }
        */
}

/**
  | Implements sequential read access in a file
  | using read().
  |
  | Instances of this class are thread-friendly but
  | not thread-safe, as required by the
  | SequentialFile API.
  */
pub struct PosixSequentialFile {
    fd:       i32,
    filename: String,
}

impl SequentialFile for PosixSequentialFile { }

impl SequentialFileRead for PosixSequentialFile {

    fn read(&mut self, 
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            crate::Status status;
        while (true) {
          ::ssize_t read_size = ::read(fd_, scratch, n);
          if (read_size < 0) {  // Read error.
            if (errno == EINTR) {
              continue;  // Retry
            }
            status = PosixError(filename_, errno);
            break;
          }
          *result = Slice(scratch, read_size);
          break;
        }
        return status;
        */
    }
}

impl SequentialFileSkip for PosixSequentialFile {

    fn skip(&mut self, n: u64) -> crate::Status {
        
        todo!();
        /*
            if (::lseek(fd_, n, SEEK_CUR) == static_cast<off_t>(-1)) {
          return PosixError(filename_, errno);
        }
        return crate::Status::OK();
        */
    }
}

impl GetName for PosixSequentialFile {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return filename_;
        */
    }
}

impl Drop for PosixSequentialFile {
    fn drop(&mut self) {
        todo!();
        /*
            close(fd_);
        */
    }
}

impl PosixSequentialFile {
    
    pub fn new(
        filename: String,
        fd:       i32) -> Self {
    
        todo!();
        /*
        : fd(fd),
        : filename(filename),

        
        */
    }
}
 
/**
  | Implements random read access in a file using
  | pread().
  |
  | Instances of this class are thread-safe, as
  | required by the RandomAccessFile API. Instances
  | are immutable and Read() only calls thread-safe
  | library functions.
  */
pub struct PosixRandomAccessFile {

    /**
      | If false, the file is opened on every
      | read.
      |
      */
    has_permanent_fd: bool,

    /**
      | -1 if has_permanent_fd_ is false.
      |
      */
    fd:               i32,

    fd_limiter:       *const Limiter,
    filename:         String,
}

impl RandomAccessFile for PosixRandomAccessFile { }

impl Drop for PosixRandomAccessFile {
    fn drop(&mut self) {
        todo!();
        /*
            if (has_permanent_fd_) {
          assert(fd_ != -1);
          ::close(fd_);
          fd_limiter_->Release();
        }
        */
    }
}

impl RandomAccessFileRead for PosixRandomAccessFile {

    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            int fd = fd_;
        if (!has_permanent_fd_) {
          fd = ::open(filename_.c_str(), O_RDONLY | kOpenBaseFlags);
          if (fd < 0) {
            return PosixError(filename_, errno);
          }
        }

        assert(fd != -1);

        crate::Status status;
        ssize_t read_size = ::pread(fd, scratch, n, static_cast<off_t>(offset));
        *result = Slice(scratch, (read_size < 0) ? 0 : read_size);
        if (read_size < 0) {
          // An error: return a non-ok status.
          status = PosixError(filename_, errno);
        }
        if (!has_permanent_fd_) {
          // Close the temporary file descriptor opened earlier.
          assert(fd != fd_);
          ::close(fd);
        }
        return status;
        */
    }
}

impl GetName for PosixRandomAccessFile {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return filename_;
        */
    }
}

impl PosixRandomAccessFile {

    /**
      | The new instance takes ownership of
      | fd|. |fd_limiter| must outlive this
      | instance, and will be used to determine
      | if .
      |
      */
    pub fn new(
        filename:   String,
        fd:         i32,
        fd_limiter: *mut Limiter) -> Self {
    
        todo!();
        /*


            : has_permanent_fd_(fd_limiter->Acquire()),
            fd_(has_permanent_fd_ ? fd : -1),
            fd_limiter_(fd_limiter),
            filename_(std::move(filename)) 
        if (!has_permanent_fd_) {
          assert(fd_ == -1);
          ::close(fd);  // The file will be opened on every read.
        }
        */
    }
}

/**
  | Implements random read access in a file using
  | mmap().
  |
  | Instances of this class are thread-safe, as
  | required by the RandomAccessFile API. Instances
  | are immutable and Read() only calls thread-safe
  | library functions.
  */
pub struct PosixMmapReadableFile {
    mmap_base:    *const u8,
    length:       usize,
    mmap_limiter: *const Limiter,
    filename:     String,
}

impl Drop for PosixMmapReadableFile {
    fn drop(&mut self) {
        todo!();
        /*
            ::munmap(static_cast<c_void*>(mmap_base_), length_);
        mmap_limiter_->Release();
        */
    }
}

impl RandomAccessFile for PosixMmapReadableFile { }

impl RandomAccessFileRead for PosixMmapReadableFile {

    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            if (offset + n > length_) {
          *result = Slice();
          return PosixError(filename_, EINVAL);
        }

        *result = Slice(mmap_base_ + offset, n);
        return crate::Status::OK();
        */
    }
}

impl GetName for PosixMmapReadableFile {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return filename_;
        */
    }
}

impl PosixMmapReadableFile {

    /**
      | mmap_base[0, length-1] points to the
      | memory-mapped contents of the file. It must
      | be the result of a successful call to
      | mmap(). This instances takes over the
      | ownership of the region.
      |
      | |mmap_limiter| must outlive this
      | instance. The caller must have already
      | aquired the right to use one mmap region,
      | which will be released when this instance is
      | destroyed.
      */
    pub fn new(
        filename:     String,
        mmap_base:    *mut u8,
        length:       usize,
        mmap_limiter: *mut Limiter) -> Self {
    
        todo!();
        /*
            : mmap_base_(mmap_base),
            length_(length),
            mmap_limiter_(mmap_limiter),
            filename_(std::move(filename))
        */
    }
}

///-------------------------
pub struct PosixWritableFile {

    /**
      | buf_[0, pos_ - 1] contains data to be
      | written to fd_.
      |
      */
    buf:         [u8; WRITABLE_FILE_BUFFER_SIZE],

    pos:         usize,
    fd:          i32,

    /**
      | True if the file's name starts with
      | MANIFEST.
      |
      */
    is_manifest: bool,

    filename:    String,

    /**
      | The directory of filename_.
      |
      */
    dirname:     String,
}

impl WritableFile for PosixWritableFile {}

impl WritableFileAppend for PosixWritableFile {

    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            size_t write_size = data.size();
        const char* write_data = data.data();

        // Fit as much as possible into buffer.
        size_t copy_size = std::min(write_size, kWritableFileBufferSize - pos_);
        std::memcpy(buf_ + pos_, write_data, copy_size);
        write_data += copy_size;
        write_size -= copy_size;
        pos_ += copy_size;
        if (write_size == 0) {
          return crate::Status::OK();
        }

        // Can't fit in buffer, so need to do at least one write.
        crate::Status status = FlushBuffer();
        if (!status.ok()) {
          return status;
        }

        // Small writes go to buffer, large writes are written directly.
        if (write_size < kWritableFileBufferSize) {
          std::memcpy(buf_, write_data, write_size);
          pos_ = write_size;
          return crate::Status::OK();
        }
        return WriteUnbuffered(write_data, write_size);
        */
    }
}

impl WritableFileClose for PosixWritableFile {

    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            crate::Status status = FlushBuffer();
        const int close_result = ::close(fd_);
        if (close_result < 0 && status.ok()) {
          status = PosixError(filename_, errno);
        }
        fd_ = -1;
        return status;
        */
    }
}

impl WritableFileFlush for PosixWritableFile {

    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            return FlushBuffer();
        */
    }
}

impl Drop for PosixWritableFile {
    fn drop(&mut self) {
        todo!();
        /*
            if (fd_ >= 0) {
          // Ignoring any potential errors
          Close();
        }
        */
    }
}

impl WritableFileSync for PosixWritableFile {

    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            // Ensure new files referred to by the manifest are in the filesystem.
        //
        // This needs to happen before the manifest file is flushed to disk, to
        // avoid crashing in a state where the manifest refers to files that are not
        // yet on disk.
        crate::Status status = SyncDirIfManifest();
        if (!status.ok()) {
          return status;
        }

        status = FlushBuffer();
        if (!status.ok()) {
          return status;
        }

        return SyncFd(fd_, filename_, false);
        */
    }
}

impl GetName for PosixWritableFile {

    fn get_name(&self) -> &'static str {
        
        todo!();
        /*
            return filename_;
        */
    }
}

impl PosixWritableFile {
    pub fn flush_buffer(&mut self) -> crate::Status {
        
        todo!();
        /*
            crate::Status status = WriteUnbuffered(buf_, pos_);
        pos_ = 0;
        return status;
        */
    }

    pub fn write_unbuffered(&mut self, 
        data: *const u8,
        size: usize) -> crate::Status {
        
        todo!();
        /*
            while (size > 0) {
          ssize_t write_result = ::write(fd_, data, size);
          if (write_result < 0) {
            if (errno == EINTR) {
              continue;  // Retry
            }
            return PosixError(filename_, errno);
          }
          data += write_result;
          size -= write_result;
        }
        return crate::Status::OK();
        */
    }

    pub fn sync_dir_if_manifest(&mut self) -> crate::Status {
        
        todo!();
        /*
        crate::Status status;
        if (!is_manifest_) {
          return status;
        }

        int fd = ::open(dirname_.c_str(), O_RDONLY | kOpenBaseFlags);
        if (fd < 0) {
          status = PosixError(dirname_, errno);
        } else {
          status = SyncFd(fd, dirname_, true);
          ::close(fd);
        }
        return status;
        */
    }

    /**
      | Ensures that all the caches associated with
      | the given file descriptor's data are flushed
      | all the way to durable media, and can
      | withstand power failures.
      |
      | The path argument is only used to populate
      | the description string in the returned crate::Status
      | if an error occurs.
      */
    pub fn sync_fd(
        fd:          i32,
        fd_path:     &String,
        syncing_dir: bool) -> crate::Status {
        
        todo!();
        /*
            #if HAVE_FULLFSYNC
        // On macOS and iOS, fsync() doesn't guarantee durability past power
        // failures. fcntl(F_FULLFSYNC) is required for that purpose. Some
        // filesystems don't support fcntl(F_FULLFSYNC), and require a fallback to
        // fsync().
        if (::fcntl(fd, F_FULLFSYNC) == 0) {
          return crate::Status::OK();
        }
    #endif  // HAVE_FULLFSYNC

    #if HAVE_FDATASYNC
        bool sync_success = ::fdatasync(fd) == 0;
    #else
        bool sync_success = ::fsync(fd) == 0;
    #endif  // HAVE_FDATASYNC

        if (sync_success) {
          return crate::Status::OK();
        }
        // Do not crash if filesystem can't fsync directories
        // (see https://github.com/bitcoin/bitcoin/pull/10000)
        if (syncing_dir && errno == EINVAL) {
          return crate::Status::OK();
        }
        return PosixError(fd_path, errno);
        */
    }

    /**
      | Returns the directory name in a path pointing
      | to a file.
      |
      | Returns "." if the path does not contain any
      | directory separator.
      */
    pub fn dirname(filename: &String) -> String {
        
        todo!();
        /*
            std::string::size_type separator_pos = filename.rfind('/');
        if (separator_pos == std::string::npos) {
          return std::string(".");
        }
        // The filename component should not contain a path separator. If it does,
        // the splitting was done incorrectly.
        assert(filename.find('/', separator_pos + 1) == std::string::npos);

        return filename.substr(0, separator_pos);
        */
    }

    /**
      | Extracts the file name from a path pointing
      | to a file.
      |
      | The returned Slice points to |filename|'s
      | data buffer, so it is only valid while
      | |filename| is alive and unchanged.
      */
    pub fn basename(filename: &String) -> Slice {
        
        todo!();
        /*
            std::string::size_type separator_pos = filename.rfind('/');
        if (separator_pos == std::string::npos) {
          return Slice(filename);
        }
        // The filename component should not contain a path separator. If it does,
        // the splitting was done incorrectly.
        assert(filename.find('/', separator_pos + 1) == std::string::npos);

        return Slice(filename.data() + separator_pos + 1,
                     filename.length() - separator_pos - 1);
        */
    }

    /**
      | True if the given file is a manifest file.
      |
      */
    pub fn is_manifest(filename: &String) -> bool {
        
        todo!();
        /*
            return Basename(filename).starts_with("MANIFEST");
        */
    }

    pub fn new(
        filename: String,
        fd:       i32) -> Self {
    
        todo!();
        /*
        : pos(0),
        : fd(fd),
        : is_manifest(IsManifest(filename)),
        : filename(std::move(filename)),
        : dirname(Dirname(filename_)),
        */
    }
}

pub fn lock_or_unlock(
        fd:   i32,
        lock: bool) -> i32 {
    
    todo!();
        /*
            errno = 0;
      struct ::flock file_lock_info;
      std::memset(&file_lock_info, 0, sizeof(file_lock_info));
      file_lock_info.l_type = (lock ? F_WRLCK : F_UNLCK);
      file_lock_info.l_whence = SEEK_SET;
      file_lock_info.l_start = 0;
      file_lock_info.l_len = 0;  // Lock/unlock entire file.
      return ::fcntl(fd, F_SETLK, &file_lock_info);
        */
}

/**
  | Instances are thread-safe because
  | they are immutable.
  |
  */
pub struct PosixFileLock {
    fd:       i32,
    filename: String,
}

impl FileLock for PosixFileLock {

}

impl PosixFileLock {

    pub fn new(
        fd:       i32,
        filename: String) -> Self {
    
        todo!();
        /*
        : fd(fd),
        : filename(std::move(filename)),
        */
    }
    
    pub fn fd(&self) -> i32 {
        
        todo!();
        /*
            return fd_;
        */
    }
    
    pub fn filename(&self) -> &String {
        
        todo!();
        /*
            return filename_;
        */
    }
}

/**
  | Tracks the files locked by
  | PosixEnv::LockFile().
  |
  | We maintain a separate set instead of relying
  | on fcntl(F_SETLK) because fcntl(F_SETLK) does
  | not provide any protection against multiple
  | uses from the same process.
  |
  | Instances are thread-safe because all member
  | data is guarded by a mutex.
  */
pub struct PosixLockTable {
    mu: Mutex<posix_lock_table::Inner>,
}

pub mod posix_lock_table {
    use super::*;

    pub struct Inner {
        locked_files: HashSet<String>,
    }
}

impl PosixLockTable {

    #[LOCKS_EXCLUDED(mu_)]
    pub fn insert(&mut self, fname: &String) -> bool {
        
        todo!();
        /*
            mu_.Lock();
        bool succeeded = locked_files_.insert(fname).second;
        mu_.Unlock();
        return succeeded;
        */
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn remove(&mut self, fname: &String)  {
        
        todo!();
        /*
            mu_.Lock();
        locked_files_.erase(fname);
        mu_.Unlock();
        */
    }
}

///--------------------
pub struct PosixEnv {

    background_work_mutex:     Mutex<posix_env::BackgroundWork>,

    /**
      | Thread-safe.
      |
      */
    locks:                     PosixLockTable,

    /**
      | Thread-safe.
      |
      */
    mmap_limiter:              Limiter,

    /**
      | Thread-safe.
      |
      */
    fd_limiter:                Limiter,
}

impl Env for PosixEnv {

}

impl DeleteFile for PosixEnv {

    fn delete_file(&mut self, filename: &String) -> crate::Status {
        
        todo!();
        /*
            if (::unlink(filename.c_str()) != 0) {
          return PosixError(filename, errno);
        }
        return crate::Status::OK();
        */
    }
}

impl CreateDir for PosixEnv {

    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            if (::mkdir(dirname.c_str(), 0755) != 0) {
          return PosixError(dirname, errno);
        }
        return crate::Status::OK();
        */
    }
}

impl DeleteDir for PosixEnv {

    fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        
        todo!();
        /*
            if (::rmdir(dirname.c_str()) != 0) {
          return PosixError(dirname, errno);
        }
        return crate::Status::OK();
        */
    }
}

impl GetFileSize for PosixEnv {

    fn get_file_size(&mut self, 
        filename: &String,
        size:     *mut u64) -> crate::Status {
        
        todo!();
        /*
            struct ::stat file_stat;
        if (::stat(filename.c_str(), &file_stat) != 0) {
          *size = 0;
          return PosixError(filename, errno);
        }
        *size = file_stat.st_size;
        return crate::Status::OK();
        */
    }
}

impl RenameFile for PosixEnv {
    fn rename_file(&mut self, 
        from: &String,
        to:   &String) -> crate::Status {
        
        todo!();
        /*
            if (std::rename(from.c_str(), to.c_str()) != 0) {
          return PosixError(from, errno);
        }
        return crate::Status::OK();
        */
    }
}

impl LockFile for PosixEnv {

    fn lock_file(&mut self, 
        filename: &String,
        lock:     *mut *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
            *lock = nullptr;

        int fd = ::open(filename.c_str(), O_RDWR | O_CREAT | kOpenBaseFlags, 0644);
        if (fd < 0) {
          return PosixError(filename, errno);
        }

        if (!locks_.Insert(filename)) {
          ::close(fd);
          return crate::Status::IOError("lock " + filename, "already held by process");
        }

        if (LockOrUnlock(fd, true) == -1) {
          int lock_errno = errno;
          ::close(fd);
          locks_.Remove(filename);
          return PosixError("lock " + filename, lock_errno);
        }

        *lock = new PosixFileLock(fd, filename);
        return crate::Status::OK();
        */
    }
}

impl StartThread for PosixEnv {

    fn start_thread(&mut self, 
        thread_main:     fn(thread_main_arg: *mut c_void) -> c_void,
        thread_main_arg: *mut c_void)  {
        
        todo!();
        /*
            std::thread new_thread(thread_main, thread_main_arg);
        new_thread.detach();
        */
    }
}

impl GetTestDirectory for PosixEnv {

    fn get_test_directory(&mut self, result: *mut String) -> crate::Status {
        
        todo!();
        /*
            const char* env = std::getenv("TEST_TMPDIR");
        if (env && env[0] != '\0') {
          *result = env;
        } else {
          char buf[100];
          std::snprintf(buf, sizeof(buf), "/tmp/leveldbtest-%d",
                        static_cast<int>(::geteuid()));
          *result = buf;
        }

        // The CreateDir status is ignored because the directory may already exist.
        CreateDir(*result);

        return crate::Status::OK();
        */
    }
}

impl NewLogger for PosixEnv {

    fn new_logger(&mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn Logger>) -> crate::Status {
        
        todo!();
        /*
            int fd = ::open(filename.c_str(),
                        O_APPEND | O_WRONLY | O_CREAT | kOpenBaseFlags, 0644);
        if (fd < 0) {
          *result = nullptr;
          return PosixError(filename, errno);
        }

        std::FILE* fp = ::fdopen(fd, "w");
        if (fp == nullptr) {
          ::close(fd);
          *result = nullptr;
          return PosixError(filename, errno);
        } else {
          *result = new PosixLogger(fp);
          return crate::Status::OK();
        }
        */
    }
}

impl NowMicros for PosixEnv {

    fn now_micros(&mut self) -> u64 {
        
        todo!();
        /*
            static constexpr uint64_t kUsecondsPerSecond = 1000000;
        struct ::timeval tv;
        ::gettimeofday(&tv, nullptr);
        return static_cast<uint64_t>(tv.tv_sec) * kUsecondsPerSecond + tv.tv_usec;
        */
    }
}

impl SleepForMicroseconds for PosixEnv {

    fn sleep_for_microseconds(&mut self, micros: i32)  {
        
        todo!();
        /*
            std::this_thread::sleep_for(std::chrono::microseconds(micros));
        */
    }
}

impl UnlockFile for PosixEnv {
    
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
            PosixFileLock* posix_file_lock = static_cast<PosixFileLock*>(lock);
        if (LockOrUnlock(posix_file_lock->fd(), false) == -1) {
          return PosixError("unlock " + posix_file_lock->filename(), errno);
        }
        locks_.Remove(posix_file_lock->filename());
        ::close(posix_file_lock->fd());
        delete posix_file_lock;
        return crate::Status::OK();
        */
    }
}

impl GetChildren for PosixEnv {

    fn get_children(&mut self, 
        directory_path: &String,
        result:         *mut Vec<String>) -> crate::Status {
        
        todo!();
        /*
            result->clear();
        ::DIR* dir = ::opendir(directory_path.c_str());
        if (dir == nullptr) {
          return PosixError(directory_path, errno);
        }
        struct ::dirent* entry;
        while ((entry = ::readdir(dir)) != nullptr) {
          result->emplace_back(entry->d_name);
        }
        ::closedir(dir);
        return crate::Status::OK();
        */
    }
}

impl FileExists for PosixEnv {

    fn file_exists(&mut self, filename: &String) -> bool {
        
        todo!();
        /*
            return ::access(filename.c_str(), F_OK) == 0;
        */
    }
}

impl NewAppendableFile for PosixEnv {

    fn new_appendable_file(&mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            int fd = ::open(filename.c_str(),
                        O_APPEND | O_WRONLY | O_CREAT | kOpenBaseFlags, 0644);
        if (fd < 0) {
          *result = nullptr;
          return PosixError(filename, errno);
        }

        *result = new PosixWritableFile(filename, fd);
        return crate::Status::OK();
        */
    }
}

impl NewWritableFile for PosixEnv {

    fn new_writable_file(&mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            int fd = ::open(filename.c_str(),
                        O_TRUNC | O_WRONLY | O_CREAT | kOpenBaseFlags, 0644);
        if (fd < 0) {
          *result = nullptr;
          return PosixError(filename, errno);
        }

        *result = new PosixWritableFile(filename, fd);
        return crate::Status::OK();
        */
    }
}

impl NewRandomAccessFile for PosixEnv {
 
    fn new_random_access_file(&mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn RandomAccessFile>) -> crate::Status {
        
        todo!();
        /*
            *result = nullptr;
        int fd = ::open(filename.c_str(), O_RDONLY | kOpenBaseFlags);
        if (fd < 0) {
          return PosixError(filename, errno);
        }

        if (!mmap_limiter_.Acquire()) {
          *result = new PosixRandomAccessFile(filename, fd, &fd_limiter_);
          return crate::Status::OK();
        }

        uint64_t file_size;
        crate::Status status = GetFileSize(filename, &file_size);
        if (status.ok()) {
          c_void* mmap_base =
              ::mmap(/*addr=*/nullptr, file_size, PROT_READ, MAP_SHARED, fd, 0);
          if (mmap_base != MAP_FAILED) {
            *result = new PosixMmapReadableFile(filename,
                                                reinterpret_cast<char*>(mmap_base),
                                                file_size, &mmap_limiter_);
          } else {
            status = PosixError(filename, errno);
          }
        }
        ::close(fd);
        if (!status.ok()) {
          mmap_limiter_.Release();
        }
        return status;
        */
    }
}
    
impl NewSequentialFile for PosixEnv {

    fn new_sequential_file(&mut self, 
        filename: &String,
        result:   *mut *mut Box<dyn SequentialFile>) -> crate::Status {
        
        todo!();
        /*
            int fd = ::open(filename.c_str(), O_RDONLY | kOpenBaseFlags);
        if (fd < 0) {
          *result = nullptr;
          return PosixError(filename, errno);
        }

        *result = new PosixSequentialFile(filename, fd);
        return crate::Status::OK();
        */
    }
}

pub mod posix_env {

    use super::*;

    pub struct BackgroundWork {
        background_work_cv:        Condvar,
        started_background_thread: bool,
        background_work_queue:     SegQueue<BackgroundWorkItem>,
    }

    /**
      | Stores the work item data in a Schedule()
      | call.
      |
      | Instances are constructed on the thread
      | calling Schedule() and used on the background
      | thread.
      |
      | This structure is thread-safe beacuse it is
      | immutable.
      */
    pub struct BackgroundWorkItem {
        function: fn(_0: *mut c_void) -> c_void,
        arg:      *const c_void,
    }

    impl BackgroundWorkItem {

        pub fn new(
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void) -> Self {
        
            todo!();
            /*
            : function(function),
            : arg(arg),

            
            */
        }
    }
}

impl Drop for PosixEnv {
    fn drop(&mut self) {
        todo!();
        /*
            static const char msg[] =
            "PosixEnv singleton destroyed. Unsupported behavior!\n";
        std::fwrite(msg, 1, sizeof(msg), stderr);
        std::abort();
        */
    }
}

impl PosixEnv {
       
    pub fn background_thread_entry_point(env: *mut PosixEnv)  {
        
        todo!();
        /*
            env->BackgroundThreadMain();
        */
    }
}

/**
  | Return the maximum number of concurrent
  | mmaps.
  |
  */
pub fn max_mmaps() -> i32 {
    
    todo!();
        /*
            return g_mmap_limit;
        */
}

/**
  | Return the maximum number of read-only
  | files to keep open.
  |
  */
pub fn max_open_files() -> i32 {
    
    todo!();
        /*
            if (g_open_read_only_file_limit >= 0) {
        return g_open_read_only_file_limit;
      }
      struct ::rlimit rlim;
      if (::getrlimit(RLIMIT_NOFILE, &rlim)) {
        // getrlimit failed, fallback to hard-coded default.
        g_open_read_only_file_limit = 50;
      } else if (rlim.rlim_cur == RLIM_INFINITY) {
        g_open_read_only_file_limit = std::numeric_limits<int>::max();
      } else {
        // Allow use of 20% of available file descriptors for read-only files.
        g_open_read_only_file_limit = rlim.rlim_cur / 5;
      }
      return g_open_read_only_file_limit;
        */
}

impl Schedule for PosixEnv {

    fn schedule(&mut self, 
        background_work_function: fn(background_work_arg: *mut c_void) -> c_void,
        background_work_arg:      *mut c_void)  {
        
        todo!();
        /*
            background_work_mutex_.Lock();

      // Start the background thread, if we haven't done so already.
      if (!started_background_thread_) {
        started_background_thread_ = true;
        std::thread background_thread(PosixEnv::BackgroundThreadEntryPoint, this);
        background_thread.detach();
      }

      // If the queue is empty, the background thread may be waiting for work.
      if (background_work_queue_.empty()) {
        background_work_cv_.Signal();
      }

      background_work_queue_.emplace(background_work_function, background_work_arg);
      background_work_mutex_.Unlock();
        */
    }
}

impl Default for PosixEnv {
    
    fn default() -> Self {
    
        todo!();
        /*
        : background_work_cv(&background_work_mutex_),
        : started_background_thread(false),
        : mmap_limiter(MaxMmaps()),
        : fd_limiter(MaxOpenFiles()),

        
        */
    }
}
    
impl PosixEnv {
    pub fn background_thread_main(&mut self)  {
        
        todo!();
        /*
            while (true) {
        background_work_mutex_.Lock();

        // Wait until there is work to be done.
        while (background_work_queue_.empty()) {
          background_work_cv_.Wait();
        }

        assert(!background_work_queue_.empty());
        auto background_work_function = background_work_queue_.front().function;
        c_void* background_work_arg = background_work_queue_.front().arg;
        background_work_queue_.pop();

        background_work_mutex_.Unlock();
        background_work_function(background_work_arg);
      }
        */
    }
}

/**
  | Wraps an Env instance whose destructor
  | is never created.
  | 
  | Intended usage:
  | 
  | -----------
  | @code
  | 
  | using PlatformSingletonEnv = SingletonEnv<PlatformEnv>;
  | c_void ConfigurePosixEnv(int param) {
  |   PlatformSingletonEnv::AssertEnvNotInitialized();
  |   // set global configuration flags.
  | }
  | Env* Env::Default() {
  |   static PlatformSingletonEnv default_env;
  |   return default_env.env();
  | }
  |
  */
pub struct SingletonEnv<EnvType> {
    __remove_me__: std::marker::PhantomData<EnvType>,

    /*
    lazy_static!{
        /*
        typename std::aligned_storage<sizeof(EnvType), alignof(EnvType)>::type
              env_storage_;
        */
    }
*/
}

pub mod singleton_env {

    use super::*;

    #[cfg(not(NDEBUG))]
    lazy_static!{
        /*
        static std::atomic<bool> env_initialized_;
        */
    }
}

impl<EnvType> Default for SingletonEnv<EnvType> {
    
    fn default() -> Self {
        todo!();
        /*


            #if !defined(NDEBUG)
        env_initialized_.store(true, std::memory_order::memory_order_relaxed);
    #endif  // !defined(NDEBUG)
        const_assert(sizeof(env_storage_) >= sizeof(EnvType),
                      "env_storage_ will not fit the Env");
        const_assert(alignof(decltype(env_storage_)) >= alignof(EnvType),
                      "env_storage_ does not meet the Env's alignment needs");
        new (&env_storage_) EnvType();
        */
    }
}

impl<EnvType> SingletonEnv<EnvType> {

    pub fn env(&mut self) -> Rc<RefCell<dyn Env>> {
        
        todo!();
        /*
            return reinterpret_cast<Env*>(&env_storage_);
        */
    }
    
    pub fn assert_env_not_initialized()  {
        
        todo!();
        /*
            #if !defined(NDEBUG)
        assert(!env_initialized_.load(std::memory_order::memory_order_relaxed));
    #endif  // !defined(NDEBUG)
        */
    }
}

#[cfg(not(NDEBUG))]
lazy_static!{
    /*
    template <typename EnvType>
    std::atomic<bool> SingletonEnv<EnvType>::env_initialized_;
    */
}

pub type PosixDefaultEnv = SingletonEnv<PosixEnv>;
