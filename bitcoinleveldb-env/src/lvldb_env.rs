// ---------------- [ File: bitcoinleveldb-env/src/lvldb_env.rs ]
/*!
  | An Env is an interface used by the leveldb
  | implementation to access operating system
  | functionality like the filesystem etc.  Callers
  | may wish to provide a custom Env object when
  | opening a database to get fine gain control;
  | e.g., to rate limit file system operations.
  |
  | All Env implementations are safe for concurrent
  | access from multiple threads without any
  | external synchronization.
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env.cc]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/env.h]

/**
  | The leveldb::Env class below contains
  | a DeleteFile method.  At the same time,
  | <windows.h>, a fairly popular header file for
  | Windows applications, defines a DeleteFile
  | macro.
  |
  | Without any intervention on our part, the
  | result of this unfortunate coincidence is that
  | the name of the leveldb::Env::DeleteFile method
  | seen by the compiler depends on whether
  | <windows.h> was included before or after the
  | LevelDB headers.
  |
  | To avoid headaches, we undefined DeleteFile (if
  | defined) and redefine it at the bottom of this
  | file. This way <windows.h> can be included
  | before this file (or not at all) and the
  | exported method will always be
  | leveldb::Env::DeleteFile.
  */
#[cfg(_WIN32)]
#[cfg(DeleteFile)]
pub const DeleteFile: bool = false;
pub const LEVELDB_DELETEFILE_UNDEFINED: bool = true;

/**
  | in the c++, we had the following annotated
  | empty Default impl:
  |
  | Return a default environment suitable for the
  | current operating system.  Sophisticated
  | users may wish to provide their own Env
  | implementation instead of relying on this
  | default environment.
  |
  | The result of Default() belongs to leveldb
  | and must never be deleted.
  */
pub trait Env:
NewSequentialFile
+ NewRandomAccessFile
+ NewWritableFile
+ NewAppendableFile
+ FileExists
+ GetChildren
+ DeleteFile
+ CreateDir
+ DeleteDir
+ GetFileSize
+ RenameFile
+ LockFile
+ UnlockFile
+ Schedule
+ StartThread
+ GetTestDirectory
+ NewLogger
+ NowMicros
+ SleepForMicroseconds {

    fn new_appendable_file(&mut self, 
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            return crate::Status::NotSupported("NewAppendableFile", fname);
        */
    }

    fn posix_default(&mut self) -> Rc<RefCell<dyn Env>> {
        
        todo!();
        /*
            static PosixDefaultEnv env_container;
      return env_container.env();
        */
    }
}

pub trait Schedule {

    /**
      | Arrange to run "(*function)(arg)" once in
      | a background thread.
      |
      | "function" may run in an unspecified thread.
      | Multiple functions added to the same Env may
      | run concurrently in different threads.
      |
      | I.e., the caller may not assume that
      | background work items are serialized.
      */
    fn schedule(&mut self, 
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void);
}

pub trait StartThread {

    /**
      | Start a new thread, invoking "function(arg)"
      | within the new thread.
      |
      | When "function(arg)" returns, the thread will
      | be destroyed.
      */
    fn start_thread(&mut self, 
            function: fn(arg: *mut c_void) -> c_void,
            arg:      *mut c_void);
}

pub trait GetTestDirectory {

    /**
      | *path is set to a temporary directory that
      | can be used for testing. It may or may not
      | have just been created. The directory may or
      | may not differ between runs of the same
      | process, but subsequent calls will return the
      | same directory.
      */
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status;
}

pub trait NewLogger {

    /**
       Create and return a log file for storing
       informational messages.
      */
    fn new_logger(&mut self, 
            fname:  &String,
            result: *mut *mut Box<dyn Logger>) -> crate::Status;
}

pub trait NowMicros {

    /**
      | Returns the number of micro-seconds
      | since some fixed point in time. Only
      | useful for computing deltas of time.
      |
      */
    fn now_micros(&mut self) -> u64;
}

pub trait SleepForMicroseconds {

    /**
      | Sleep/delay the thread for the prescribed
      | number of micro-seconds.
      |
      */
    fn sleep_for_microseconds(&mut self, micros: i32);
}

/**
  | An implementation of Env that forwards all
  | calls to another Env.
  |
  | May be useful to clients who wish to override
  | just part of the functionality of another Env.
  */
pub struct EnvWrapper {
    target: Rc<RefCell<dyn Env>>,
}

impl Env for EnvWrapper {

}

impl DeleteFile for EnvWrapper {

    fn delete_file(&mut self, f: &String) -> crate::Status {
        
        todo!();
        /*
            return target_->DeleteFile(f);
        */
    }
}

impl CreateDir for EnvWrapper {

    fn create_dir(&mut self, d: &String) -> crate::Status {
        
        todo!();
        /*
            return target_->CreateDir(d);
        */
    }
}

impl DeleteDir for EnvWrapper {

    fn delete_dir(&mut self, d: &String) -> crate::Status {
        
        todo!();
        /*
            return target_->DeleteDir(d);
        */
    }
}

impl NewSequentialFile for EnvWrapper {

    /**
      | The following text is boilerplate that
      | forwards all methods to target().
      |
      */
    fn new_sequential_file(&mut self, 
        f: &String,
        r: *mut *mut Box<dyn SequentialFile>) -> crate::Status {
        
        todo!();
        /*
            return target_->NewSequentialFile(f, r);
        */
    }
}
    
impl NewRandomAccessFile for EnvWrapper {

    fn new_random_access_file(&mut self, 
        f: &String,
        r: *mut *mut Box<dyn RandomAccessFile>) -> crate::Status {
        
        todo!();
        /*
            return target_->NewRandomAccessFile(f, r);
        */
    }
}
    
impl NewWritableFile for EnvWrapper {

    fn new_writable_file(&mut self, 
        f: &String,
        r: *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            return target_->NewWritableFile(f, r);
        */
    }
}
    
impl NewAppendableFile for EnvWrapper {

    fn new_appendable_file(&mut self, 
        f: &String,
        r: *mut *mut Box<dyn WritableFile>) -> crate::Status {
        
        todo!();
        /*
            return target_->NewAppendableFile(f, r);
        */
    }
}
    
impl FileExists for EnvWrapper {

    fn file_exists(&mut self, f: &String) -> bool {
        
        todo!();
        /*
            return target_->FileExists(f);
        */
    }
}
    
impl GetChildren for EnvWrapper {

    fn get_children(&mut self, 
        dir: &String,
        r:   *mut Vec<String>) -> crate::Status {
        
        todo!();
        /*
            return target_->GetChildren(dir, r);
        */
    }
}
    
impl GetFileSize for EnvWrapper {
    fn get_file_size(&mut self, 
        f: &String,
        s: *mut u64) -> crate::Status {
        
        todo!();
        /*
            return target_->GetFileSize(f, s);
        */
    }
}
    
impl RenameFile for EnvWrapper {
    fn rename_file(&mut self, 
        s: &String,
        t: &String) -> crate::Status {
        
        todo!();
        /*
            return target_->RenameFile(s, t);
        */
    }
}
    
impl LockFile for EnvWrapper {
    fn lock_file(&mut self, 
        f: &String,
        l: *mut *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
            return target_->LockFile(f, l);
        */
    }
}
    
impl UnlockFile for EnvWrapper {
    fn unlock_file(&mut self, l: *mut Box<dyn FileLock>) -> crate::Status {
        
        todo!();
        /*
            return target_->UnlockFile(l);
        */
    }
}
    
impl Schedule for EnvWrapper {
    fn schedule(&mut self, 
        f: fn(_0: *mut c_void) -> c_void,
        a: *mut c_void)  {
        
        todo!();
        /*
            return target_->Schedule(f, a);
        */
    }
}
    
impl StartThread for EnvWrapper {
    fn start_thread(&mut self, 
        f: fn(_0: *mut c_void) -> c_void,
        a: *mut c_void)  {
        
        todo!();
        /*
            return target_->StartThread(f, a);
        */
    }
}
    
impl GetTestDirectory for EnvWrapper {
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        
        todo!();
        /*
            return target_->GetTestDirectory(path);
        */
    }
}
    
impl NewLogger for EnvWrapper {
    fn new_logger(&mut self, 
        fname:  &String,
        result: *mut *mut Box<dyn Logger>) -> crate::Status {
        
        todo!();
        /*
            return target_->NewLogger(fname, result);
        */
    }
}
    
impl NowMicros for EnvWrapper {
    fn now_micros(&mut self) -> u64 {
        
        todo!();
        /*
            return target_->NowMicros();
        */
    }
}

impl SleepForMicroseconds for EnvWrapper {
    fn sleep_for_microseconds(&mut self, micros: i32)  {
        
        todo!();
        /*
            target_->SleepForMicroseconds(micros);
        */
    }
}
    
impl EnvWrapper {

    /**
      | Initialize an EnvWrapper that delegates
      | all calls to *t.
      |
      */
    pub fn new(t: Rc<RefCell<dyn Env>>) -> Self {
    
        todo!();
        /*
        : target(t),

        
        */
    }

    /**
      | Return the target to which this Env forwards
      | all calls.
      |
      */
    pub fn target(&self) -> Rc<RefCell<dyn Env>> {
        
        todo!();
        /*
            return target_;
        */
    }
}

/**
  | Redefine DeleteFile if necessary.
  |
  */
#[cfg(all(_WIN32,LEVELDB_DELETEFILE_UNDEFINED))]
lazy_static!{
    /*
    #if UNICODE
    #define DeleteFile DeleteFileW
    #else
    #define DeleteFile DeleteFileA
    #endif  // defined(UNICODE)
    */
}
/**
  | Log the specified data to *info_log
  | if info_log is non-null.
  |
  */
pub fn log(
        info_log: Rc<RefCell<dyn Logger>>,
        format:   *const u8,
        args:     &[&str])  {
    
    todo!();
        /*
            if (info_log != nullptr) {
        va_list ap;
        va_start(ap, format);
        info_log->Logv(format, ap);
        va_end(ap);
      }
        */
}

pub fn do_write_string_to_file(
        env:         Rc<RefCell<dyn Env>>,
        data:        &Slice,
        fname:       &String,
        should_sync: bool) -> crate::Status {
    
    todo!();
        /*
            WritableFile* file;
      crate::Status s = env->NewWritableFile(fname, &file);
      if (!s.ok()) {
        return s;
      }
      s = file->Append(data);
      if (s.ok() && should_sync) {
        s = file->Sync();
      }
      if (s.ok()) {
        s = file->Close();
      }
      delete file;  // Will auto-close if we did not close above
      if (!s.ok()) {
        env->DeleteFile(fname);
      }
      return s;
        */
}

/**
  | A utility routine: write "data" to the
  | named file.
  |
  */
pub fn write_string_to_file(
        env:   Rc<RefCell<dyn Env>>,
        data:  &Slice,
        fname: &String) -> crate::Status {
    
    todo!();
        /*
            return DoWriteStringToFile(env, data, fname, false);
        */
}

/**
  | A utility routine: write "data" to the
  | named file and Sync() it.
  |
  */
pub fn write_string_to_file_sync(
        env:   Rc<RefCell<dyn Env>>,
        data:  &Slice,
        fname: &String) -> crate::Status {
    
    todo!();
        /*
            return DoWriteStringToFile(env, data, fname, true);
        */
}

/**
  | A utility routine: read contents of
  | named file into *data
  |
  */
pub fn read_file_to_string(
        env:   Rc<RefCell<dyn Env>>,
        fname: &String,
        data:  *mut String) -> crate::Status {
    
    todo!();
        /*
            data->clear();
      SequentialFile* file;
      crate::Status s = env->NewSequentialFile(fname, &file);
      if (!s.ok()) {
        return s;
      }
      static const int kBufferSize = 8192;
      char* space = new char[kBufferSize];
      while (true) {
        Slice fragment;
        s = file->Read(kBufferSize, &fragment, space);
        if (!s.ok()) {
          break;
        }
        data->append(fragment.data(), fragment.size());
        if (fragment.empty()) {
          break;
        }
      }
      delete[] space;
      delete file;
      return s;
        */
}

pub fn handle_dump_command(
        env:   Rc<RefCell<dyn Env>>,
        files: *mut *mut u8,
        num:   i32) -> bool {
    
    todo!();
        /*
            StdoutPrinter printer;
      bool ok = true;
      for (int i = 0; i < num; i++) {
        Status s = DumpFile(env, files[i], &printer);
        if (!s.ok()) {
          fprintf(stderr, "%s\n", s.ToString().c_str());
          ok = false;
        }
      }
      return ok;
        */
}

pub fn dbleveldbutil_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            leveldb::Env* env = leveldb::Env::Default();
      bool ok = true;
      if (argc < 2) {
        Usage();
        ok = false;
      } else {
        std::string command = argv[1];
        if (command == "dump") {
          ok = leveldb::HandleDumpCommand(env, argv + 2, argc - 2);
        } else {
          Usage();
          ok = false;
        }
      }
      return (ok ? 0 : 1);
        */
}

/**
  | Make the CURRENT file point to the descriptor
  | file with the specified number.
  |
  */
pub fn set_current_file(
        env:               Rc<RefCell<dyn Env>>,
        dbname:            &String,
        descriptor_number: u64) -> crate::Status {
    
    todo!();
        /*
            // Remove leading "dbname/" and add newline to manifest file name
      std::string manifest = DescriptorFileName(dbname, descriptor_number);
      Slice contents = manifest;
      assert(contents.starts_with(dbname + "/"));
      contents.remove_prefix(dbname.size() + 1);
      std::string tmp = TempFileName(dbname, descriptor_number);
      Status s = WriteStringToFileSync(env, contents.ToString() + "\n", tmp);
      if (s.ok()) {
        s = env->RenameFile(tmp, CurrentFileName(dbname));
      }
      if (!s.ok()) {
        env->DeleteFile(tmp);
      }
      return s;
        */
}
