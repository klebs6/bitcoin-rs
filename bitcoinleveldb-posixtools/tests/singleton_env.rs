// ---------------- [ File: bitcoinleveldb-posixtools/tests/singleton_env.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;
use bitcoin_support::*;

/// Minimal Env implementation used only to exercise SingletonEnv behavior.
#[derive(Default)]
struct DummyEnvForSingleton {
    now_micros_counter: u64,
}

impl NewSequentialFile for DummyEnvForSingleton {
    fn new_sequential_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::new_sequential_file -> NotSupported"
        );
        let msg = "new_sequential_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl NewRandomAccessFile for DummyEnvForSingleton {
    fn new_random_access_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::new_random_access_file -> NotSupported"
        );
        let msg = "new_random_access_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl NewWritableFile for DummyEnvForSingleton {
    fn new_writable_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::new_writable_file -> NotSupported"
        );
        let msg = "new_writable_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl NewAppendableFile for DummyEnvForSingleton {
    fn new_appendable_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::new_appendable_file -> NotSupported"
        );
        let msg = "new_appendable_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl FileExists for DummyEnvForSingleton {
    fn file_exists(&mut self, _fname: &String) -> bool {
        trace!(
            "DummyEnvForSingleton::file_exists -> false"
        );
        false
    }
}

impl GetChildren for DummyEnvForSingleton {
    fn get_children(
        &mut self,
        _dir: &String,
        _result: *mut Vec<String>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::get_children -> NotSupported"
        );
        let msg = "get_children".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl DeleteFile for DummyEnvForSingleton {
    fn delete_file(&mut self, _fname: &String) -> Status {
        trace!(
            "DummyEnvForSingleton::delete_file -> OK"
        );
        Status::ok()
    }
}

impl CreateDir for DummyEnvForSingleton {
    fn create_dir(&mut self, _dirname: &String) -> Status {
        trace!(
            "DummyEnvForSingleton::create_dir -> OK"
        );
        Status::ok()
    }
}

impl DeleteDir for DummyEnvForSingleton {
    fn delete_dir(&mut self, _dirname: &String) -> Status {
        trace!(
            "DummyEnvForSingleton::delete_dir -> OK"
        );
        Status::ok()
    }
}

impl GetFileSize for DummyEnvForSingleton {
    fn get_file_size(
        &mut self,
        _fname: &String,
        _file_size: *mut u64,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::get_file_size -> NotSupported"
        );
        let msg = "get_file_size".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl RenameFile for DummyEnvForSingleton {
    fn rename_file(
        &mut self,
        _src: &String,
        _target: &String,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::rename_file -> OK"
        );
        Status::ok()
    }
}

impl LockFile for DummyEnvForSingleton {
    fn lock_file(
        &mut self,
        _fname: &String,
        _lock: *mut *mut Box<dyn FileLock>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::lock_file -> NotSupported"
        );
        let msg = "lock_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl UnlockFile for DummyEnvForSingleton {
    fn unlock_file(
        &mut self,
        _lock: *mut Box<dyn FileLock>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::unlock_file -> NotSupported"
        );
        let msg = "unlock_file".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl Schedule for DummyEnvForSingleton {
    fn schedule(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        trace!(
            "DummyEnvForSingleton::schedule (no-op)"
        );
    }
}

impl StartThread for DummyEnvForSingleton {
    fn start_thread(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        trace!(
            "DummyEnvForSingleton::start_thread (no-op)"
        );
    }
}

impl GetTestDirectory for DummyEnvForSingleton {
    fn get_test_directory(&mut self, path: *mut String) -> Status {
        trace!(
            "DummyEnvForSingleton::get_test_directory"
        );

        unsafe {
            if !path.is_null() {
                let out: &mut String = &mut *path;
                out.clear();
                out.push_str("/tmp/dummy-env-singleton");
            }
        }

        Status::ok()
    }
}

impl NewLogger for DummyEnvForSingleton {
    fn new_logger(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn Logger>,
    ) -> Status {
        trace!(
            "DummyEnvForSingleton::new_logger -> NotSupported"
        );
        let msg = "new_logger".to_string();
        let msg_slice = Slice::from(&msg);
        Status::not_supported(&msg_slice, None)
    }
}

impl NowMicros for DummyEnvForSingleton {
    fn now_micros(&mut self) -> u64 {
        self.now_micros_counter = self.now_micros_counter.wrapping_add(1);
        trace!(
            counter = self.now_micros_counter,
            "DummyEnvForSingleton::now_micros"
        );
        self.now_micros_counter
    }
}

impl SleepForMicroseconds for DummyEnvForSingleton {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        trace!(
            micros,
            "DummyEnvForSingleton::sleep_for_microseconds (no-op)"
        );
    }
}

impl Env for DummyEnvForSingleton {}

#[traced_test]
fn singleton_env_returns_consistent_env_rc_for_dummy_env() {
    use std::rc::Rc;

    trace!(
        "singleton_env_returns_consistent_env_rc_for_dummy_env: start"
    );

    let singleton: SingletonEnv<DummyEnvForSingleton> = SingletonEnv::default();

    let env1 = singleton.env();
    let env2 = singleton.env();

    assert!(
        Rc::ptr_eq(&env1, &env2),
        "SingletonEnv::env should return the same Rc instance each time"
    );

    info!(
        "singleton_env_returns_consistent_env_rc_for_dummy_env: completed"
    );
}

#[traced_test]
fn singleton_env_exposes_functional_env_implementation() {
    trace!(
        "singleton_env_exposes_functional_env_implementation: start"
    );

    let singleton: SingletonEnv<DummyEnvForSingleton> = SingletonEnv::default();
    let env_rc = singleton.env();

    let (first_time, second_time, test_path) = {
        let mut env = env_rc.borrow_mut();

        let first_time = env.now_micros();
        let second_time = env.now_micros();

        let mut path = String::new();
        let path_ptr: *mut String = &mut path;
        let status = env.get_test_directory(path_ptr);
        assert!(
            status.is_ok(),
            "DummyEnvForSingleton::get_test_directory should succeed: {}",
            status.to_string()
        );

        (first_time, second_time, path)
    };

    assert!(
        second_time > first_time,
        "DummyEnvForSingleton::now_micros should be monotonically increasing"
    );

    assert!(
        !test_path.is_empty(),
        "DummyEnvForSingleton::get_test_directory should populate a non-empty path"
    );

    info!(
        first_time,
        second_time,
        path = %test_path,
        "singleton_env_exposes_functional_env_implementation: completed"
    );
}
