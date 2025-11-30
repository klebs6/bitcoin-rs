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
    fn delete_file(&mut self, f: &String) -> Status {

        trace!(file = %f, "EnvWrapper::delete_file forwarding to target Env");
        let status = self.target.borrow_mut().delete_file(f);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::delete_file completed");
        status
    }
}

impl CreateDir for EnvWrapper {
    fn create_dir(&mut self, d: &String) -> Status {

        trace!(dir = %d, "EnvWrapper::create_dir forwarding to target Env");
        let status = self.target.borrow_mut().create_dir(d);
        debug!(dir = %d, ok = status.is_ok(), "EnvWrapper::create_dir completed");
        status
    }
}

impl DeleteDir for EnvWrapper {
    fn delete_dir(&mut self, d: &String) -> Status {

        trace!(dir = %d, "EnvWrapper::delete_dir forwarding to target Env");
        let status = self.target.borrow_mut().delete_dir(d);
        debug!(dir = %d, ok = status.is_ok(), "EnvWrapper::delete_dir completed");
        status
    }
}

impl NewSequentialFile for EnvWrapper {

    /// The following text is boilerplate that
    /// forwards all methods to target().
    /// 
    fn new_sequential_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        trace!(file = %f, "EnvWrapper::new_sequential_file forwarding to target Env");
        let status = self.target.borrow_mut().new_sequential_file(f, r);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_sequential_file completed");
        status
    }
}

impl NewRandomAccessFile for EnvWrapper {
    fn new_random_access_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {

        trace!(file = %f, "EnvWrapper::new_random_access_file forwarding to target Env");
        let status = self.target.borrow_mut().new_random_access_file(f, r);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_random_access_file completed");
        status
    }
}

impl NewWritableFile for EnvWrapper {
    fn new_writable_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn WritableFile>,
    ) -> Status {

        trace!(file = %f, "EnvWrapper::new_writable_file forwarding to target Env");
        let status = self.target.borrow_mut().new_writable_file(f, r);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_writable_file completed");
        status
    }
}

impl NewAppendableFile for EnvWrapper {

    fn new_appendable_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn WritableFile>,
    ) -> Status {

        trace!(file = %f, "EnvWrapper::new_appendable_file forwarding to target Env");

        // Disambiguate against the Env::new_appendable_file default helper.
        let status = bitcoinleveldb_file::NewAppendableFile::new_appendable_file(
            &mut *self.target.borrow_mut(),
            f,
            r,
        );

        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::new_appendable_file completed");
        status
    }
}

impl FileExists for EnvWrapper {
    fn file_exists(&mut self, f: &String) -> bool {

        trace!(file = %f, "EnvWrapper::file_exists forwarding to target Env");
        let exists = self.target.borrow_mut().file_exists(f);
        debug!(file = %f, exists, "EnvWrapper::file_exists completed");
        exists
    }
}

impl GetChildren for EnvWrapper {
    fn get_children(&mut self, dir: &String, r: *mut Vec<String>) -> Status {

        trace!(dir = %dir, "EnvWrapper::get_children forwarding to target Env");
        let status = self.target.borrow_mut().get_children(dir, r);
        debug!(dir = %dir, ok = status.is_ok(), "EnvWrapper::get_children completed");
        status
    }
}

impl GetFileSize for EnvWrapper {
    fn get_file_size(&mut self, f: &String, s: *mut u64) -> Status {

        trace!(file = %f, "EnvWrapper::get_file_size forwarding to target Env");
        let status = self.target.borrow_mut().get_file_size(f, s);
        let size_val = unsafe { s.as_ref().copied() };
        debug!(
            file = %f,
            ok = status.is_ok(),
            size = size_val,
            "EnvWrapper::get_file_size completed"
        );
        status
    }
}

impl RenameFile for EnvWrapper {
    fn rename_file(&mut self, src: &String, target: &String) -> Status {

        trace!(src = %src, target = %target, "EnvWrapper::rename_file forwarding");
        let status = self.target.borrow_mut().rename_file(src, target);
        debug!(
            src = %src,
            target = %target,
            ok = status.is_ok(),
            "EnvWrapper::rename_file completed"
        );
        status
    }
}

impl LockFile for EnvWrapper {
    fn lock_file(
        &mut self,
        f: &String,
        l: *mut *mut Box<dyn FileLock>,
    ) -> Status {

        trace!(file = %f, "EnvWrapper::lock_file forwarding to target Env");
        let status = self.target.borrow_mut().lock_file(f, l);
        debug!(file = %f, ok = status.is_ok(), "EnvWrapper::lock_file completed");
        status
    }
}

impl UnlockFile for EnvWrapper {
    fn unlock_file(&mut self, l: *mut Box<dyn FileLock>) -> Status {

        trace!("EnvWrapper::unlock_file forwarding to target Env");
        let status = self.target.borrow_mut().unlock_file(l);
        debug!(ok = status.is_ok(), "EnvWrapper::unlock_file completed");
        status
    }
}

impl Schedule for EnvWrapper {
    fn schedule(&mut self, f: fn(_0: *mut std::ffi::c_void) -> std::ffi::c_void, a: *mut std::ffi::c_void) {

        trace!("EnvWrapper::schedule forwarding to target Env");
        self.target.borrow_mut().schedule(f, a);
        debug!("EnvWrapper::schedule completed");
    }
}

impl StartThread for EnvWrapper {
    fn start_thread(&mut self, f: fn(_0: *mut std::ffi::c_void) -> std::ffi::c_void, a: *mut std::ffi::c_void) {

        trace!("EnvWrapper::start_thread forwarding to target Env");
        self.target.borrow_mut().start_thread(f, a);
        debug!("EnvWrapper::start_thread completed");
    }
}

impl GetTestDirectory for EnvWrapper {
    fn get_test_directory(&mut self, path: *mut String) -> Status {

        trace!("EnvWrapper::get_test_directory forwarding to target Env");
        let status = self.target.borrow_mut().get_test_directory(path);
        let path_preview = unsafe { path.as_ref().map(|s| s.as_str()).unwrap_or("") };
        debug!(
            path = %path_preview,
            ok = status.is_ok(),
            "EnvWrapper::get_test_directory completed"
        );
        status
    }
}

impl NewLogger for EnvWrapper {
    fn new_logger(
        &mut self,
        fname: &String,
        result: *mut *mut Box<dyn Logger>,
    ) -> Status {

        trace!(file = %fname, "EnvWrapper::new_logger forwarding to target Env");
        let status = self.target.borrow_mut().new_logger(fname, result);
        debug!(file = %fname, ok = status.is_ok(), "EnvWrapper::new_logger completed");
        status
    }
}

impl NowMicros for EnvWrapper {
    fn now_micros(&mut self) -> u64 {

        trace!("EnvWrapper::now_micros forwarding to target Env");
        let micros = self.target.borrow_mut().now_micros();
        debug!(micros, "EnvWrapper::now_micros completed");
        micros
    }
}

impl SleepForMicroseconds for EnvWrapper {
    fn sleep_for_microseconds(&mut self, micros: i32) {

        trace!(micros, "EnvWrapper::sleep_for_microseconds forwarding to target Env");
        self.target.borrow_mut().sleep_for_microseconds(micros);
        debug!(micros, "EnvWrapper::sleep_for_microseconds completed");
    }
}

impl EnvWrapper {

    /// Initialize an EnvWrapper that delegates all calls to *t.
    ///
    pub fn new(t: Rc<RefCell<dyn Env>>) -> Self {

        trace!("EnvWrapper::new constructing wrapper");
        let wrapper = Self { target: t };
        debug!("EnvWrapper::new constructed");
        wrapper
    }

    /// Return the target to which this Env forwards all calls.
    ///
    pub fn target(&self) -> Rc<RefCell<dyn Env>> {

        trace!("EnvWrapper::target cloning Rc to inner Env");
        self.target.clone()
    }
}
