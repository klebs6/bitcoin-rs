// ---------------- [ File: bitcoinleveldb-env/tests/set_current_file_tests.rs ]
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;
use bitcoin_support::*;

/// State shared between the test and the Env implementation so we do not need
/// to downcast `Rc<RefCell<dyn Env>>`.
#[derive(Default)]
struct SetCurrentFileEnvState {
    rename_calls:      Vec<(String, String)>,
    delete_file_calls: Vec<String>,
}

/// A minimal writable file used only so that `do_write_string_to_file` has a
/// valid `WritableFile` implementation to talk to.
///
/// It just accepts all operations and records no state.
struct SetCurrentFileWritableFile {
    name: String,
}

impl SetCurrentFileWritableFile {
    fn new(name: String) -> Self {
        trace!(file = %name, "SetCurrentFileWritableFile::new");
        Self { name }
    }
}

impl WritableFile for SetCurrentFileWritableFile {}

impl Named for SetCurrentFileWritableFile {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[set-current-file-writable]".to_string())
    }
}

impl WritableFileAppend for SetCurrentFileWritableFile {
    fn append(&mut self, data: &Slice) -> Status {
        trace!(
            file = %self.name,
            len = *data.size(),
            "SetCurrentFileWritableFile::append (no-op)"
        );
        Status::ok()
    }
}

impl WritableFileFlush for SetCurrentFileWritableFile {
    fn flush(&mut self) -> Status {
        trace!(
            file = %self.name,
            "SetCurrentFileWritableFile::flush (no-op)"
        );
        Status::ok()
    }
}

impl WritableFileSync for SetCurrentFileWritableFile {
    fn sync(&mut self) -> Status {
        trace!(
            file = %self.name,
            "SetCurrentFileWritableFile::sync (no-op)"
        );
        Status::ok()
    }
}

impl WritableFileClose for SetCurrentFileWritableFile {
    fn close(&mut self) -> Status {
        trace!(
            file = %self.name,
            "SetCurrentFileWritableFile::close (no-op)"
        );
        Status::ok()
    }
}

/// Env implementation used to test `set_current_file`. It tracks rename and
/// delete calls via the shared `SetCurrentFileEnvState`.
struct TestEnvForSetCurrentFile {
    state: Rc<RefCell<SetCurrentFileEnvState>>,
}

impl TestEnvForSetCurrentFile {
    fn new(state: Rc<RefCell<SetCurrentFileEnvState>>) -> Self {
        trace!("TestEnvForSetCurrentFile::new");
        Self { state }
    }
}

impl NewSequentialFile for TestEnvForSetCurrentFile {
    fn new_sequential_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_sequential_file".to_string()), None)
    }
}

impl NewRandomAccessFile for TestEnvForSetCurrentFile {
    fn new_random_access_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_random_access_file".to_string()), None)
    }
}

impl NewWritableFile for TestEnvForSetCurrentFile {
    fn new_writable_file(
        &mut self,
        fname: &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        trace!(
            file = %fname,
            "TestEnvForSetCurrentFile::new_writable_file"
        );

        let wf = SetCurrentFileWritableFile::new(fname.clone());
        let obj: Box<dyn WritableFile> = Box::new(wf);
        let holder: Box<Box<dyn WritableFile>> = Box::new(obj);
        let raw = Box::into_raw(holder);

        unsafe {
            *result = raw;
        }

        Status::ok()
    }
}

impl NewAppendableFile for TestEnvForSetCurrentFile {
    fn new_appendable_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_appendable_file".to_string()), None)
    }
}

impl FileExists for TestEnvForSetCurrentFile {
    fn file_exists(&mut self, _fname: &String) -> bool {
        true
    }
}

impl GetChildren for TestEnvForSetCurrentFile {
    fn get_children(&mut self, _dir: &String, _result: *mut Vec<String>) -> Status {
        Status::not_supported(&Slice::from(&"get_children".to_string()), None)
    }
}

impl DeleteFile for TestEnvForSetCurrentFile {
    fn delete_file(&mut self, fname: &String) -> Status {
        trace!(
            file = %fname,
            "TestEnvForSetCurrentFile::delete_file"
        );
        self.state
            .borrow_mut()
            .delete_file_calls
            .push(fname.clone());
        Status::ok()
    }
}

impl CreateDir for TestEnvForSetCurrentFile {
    fn create_dir(&mut self, _dirname: &String) -> Status {
        Status::not_supported(&Slice::from(&"create_dir".to_string()), None)
    }
}

impl DeleteDir for TestEnvForSetCurrentFile {
    fn delete_dir(&mut self, _dirname: &String) -> Status {
        Status::not_supported(&Slice::from(&"delete_dir".to_string()), None)
    }
}

impl GetFileSize for TestEnvForSetCurrentFile {
    fn get_file_size(&mut self, _fname: &String, _file_size: *mut u64) -> Status {
        Status::not_supported(&Slice::from(&"get_file_size".to_string()), None)
    }
}

impl RenameFile for TestEnvForSetCurrentFile {
    fn rename_file(&mut self, src: &String, target: &String) -> Status {
        trace!(
            src = %src,
            target = %target,
            "TestEnvForSetCurrentFile::rename_file"
        );
        self.state
            .borrow_mut()
            .rename_calls
            .push((src.clone(), target.clone()));
        Status::ok()
    }
}

impl LockFile for TestEnvForSetCurrentFile {
    fn lock_file(
        &mut self,
        _fname: &String,
        _lock: *mut *mut Box<dyn FileLock>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"lock_file".to_string()), None)
    }
}

impl UnlockFile for TestEnvForSetCurrentFile {
    fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> Status {
        Status::not_supported(&Slice::from(&"unlock_file".to_string()), None)
    }
}

impl Schedule for TestEnvForSetCurrentFile {
    fn schedule(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        // not used
    }
}

impl StartThread for TestEnvForSetCurrentFile {
    fn start_thread(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        // not used
    }
}

impl GetTestDirectory for TestEnvForSetCurrentFile {
    fn get_test_directory(&mut self, _path: *mut String) -> Status {
        Status::not_supported(&Slice::from(&"get_test_directory".to_string()), None)
    }
}

impl NewLogger for TestEnvForSetCurrentFile {
    fn new_logger(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn Logger>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_logger".to_string()), None)
    }
}

impl NowMicros for TestEnvForSetCurrentFile {
    fn now_micros(&mut self) -> u64 {
        0
    }
}

impl SleepForMicroseconds for TestEnvForSetCurrentFile {
    fn sleep_for_microseconds(&mut self, _micros: i32) {
        // noop
    }
}

impl Env for TestEnvForSetCurrentFile {}

#[traced_test]
fn set_current_file_creates_and_moves_manifest_pointer() {
    trace!("set_current_file_creates_and_moves_manifest_pointer: start");

    let state = Rc::new(RefCell::new(SetCurrentFileEnvState::default()));
    let env_impl = TestEnvForSetCurrentFile::new(state.clone());
    let env: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));

    let dbname = "testdb".to_string();
    let descriptor_number = 7u64;

    let status = set_current_file(env.clone(), &dbname, descriptor_number);
    assert!(
        status.is_ok(),
        "set_current_file should succeed: {}",
        status.to_string()
    );

    let state_ref = state.borrow();

    // Expect one rename from temp manifest to CURRENT.
    assert_eq!(
        state_ref.rename_calls.len(),
        1,
        "exactly one rename should be issued"
    );

    let tmp_expected = temp_file_name(&dbname, descriptor_number);
    let current_expected = current_file_name(&dbname);

    assert_eq!(
        state_ref.rename_calls[0].0,
        tmp_expected,
        "rename source should be the temp manifest file"
    );
    assert_eq!(
        state_ref.rename_calls[0].1,
        current_expected,
        "rename target should be CURRENT"
    );
    assert!(
        state_ref.delete_file_calls.is_empty(),
        "no temp file deletion should happen on success"
    );

    info!("set_current_file_creates_and_moves_manifest_pointer: completed");
}
