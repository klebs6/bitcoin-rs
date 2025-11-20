// ---------------- [ File: bitcoinleveldb-env/src/env_wrapper.rs ]
crate::ix!();

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
