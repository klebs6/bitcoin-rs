// ---------------- [ File: bitcoinleveldb-memenv/src/test_util.rs ]
crate::ix!();

#[derive(Default)]
pub struct TestBaseEnv;

impl Env for TestBaseEnv {}

impl CreateDir for TestBaseEnv {
    fn create_dir(&mut self, _dirname: &String) -> Status {
        Status::ok()
    }
}

impl DeleteDir for TestBaseEnv {
    fn delete_dir(&mut self, _dirname: &String) -> Status {
        Status::ok()
    }
}

impl DeleteFile for TestBaseEnv {
    fn delete_file(&mut self, _fname: &String) -> Status {
        Status::ok()
    }
}

impl NewSequentialFile for TestBaseEnv {
    fn new_sequential_file(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                *result = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl NewRandomAccessFile for TestBaseEnv {
    fn new_random_access_file(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                *result = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl NewWritableFile for TestBaseEnv {
    fn new_writable_file(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                *result = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl NewAppendableFile for TestBaseEnv {
    fn new_appendable_file(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                *result = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl FileExists for TestBaseEnv {
    fn file_exists(&mut self, _fname: &String) -> bool {
        false
    }
}

impl GetChildren for TestBaseEnv {
    fn get_children(
        &mut self,
        _dir: &String,
        result: *mut Vec<String>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                (*result).clear();
            }
        }
        Status::ok()
    }
}

impl GetFileSize for TestBaseEnv {
    fn get_file_size(
        &mut self,
        _fname: &String,
        file_size: *mut u64,
    ) -> Status {
        unsafe {
            if !file_size.is_null() {
                *file_size = 0;
            }
        }
        Status::ok()
    }
}

impl RenameFile for TestBaseEnv {
    fn rename_file(
        &mut self,
        _src: &String,
        _target: &String,
    ) -> Status {
        Status::ok()
    }
}

impl LockFile for TestBaseEnv {
    fn lock_file(
        &mut self,
        _fname: &String,
        lock: *mut *mut Box<dyn FileLock>,
    ) -> Status {
        unsafe {
            if !lock.is_null() {
                *lock = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl UnlockFile for TestBaseEnv {
    fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> Status {
        Status::ok()
    }
}

impl Schedule for TestBaseEnv {
    fn schedule(
        &mut self,
        _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        _arg: *mut std::ffi::c_void,
    ) {
    }
}

impl StartThread for TestBaseEnv {
    fn start_thread(
        &mut self,
        _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        _arg: *mut std::ffi::c_void,
    ) {
    }
}

impl GetTestDirectory for TestBaseEnv {
    fn get_test_directory(&mut self, path: *mut String) -> Status {
        unsafe {
            if !path.is_null() {
                *path = "/tmp/test-base-env".to_string();
            }
        }
        Status::ok()
    }
}

impl NewLogger for TestBaseEnv {
    fn new_logger(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn Logger>,
    ) -> Status {
        unsafe {
            if !result.is_null() {
                *result = core::ptr::null_mut();
            }
        }
        Status::ok()
    }
}

impl NowMicros for TestBaseEnv {
    fn now_micros(&mut self) -> u64 {
        0
    }
}

impl SleepForMicroseconds for TestBaseEnv {
    fn sleep_for_microseconds(&mut self, _micros: i32) {}
}

pub fn make_mem_env() -> InMemoryEnv {
    let base: std::rc::Rc<std::cell::RefCell<dyn Env>> =
        std::rc::Rc::new(std::cell::RefCell::new(TestBaseEnv::default()));
    InMemoryEnv::new(base)
}
