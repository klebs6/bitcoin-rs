// ---------------- [ File: bitcoinleveldb-env/tests/write_string_to_file_tests.rs ]
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;
use bitcoin_support::*;

struct ReadFileTestSequentialFile {
    data:          Vec<u8>,
    offset:        usize,
    read_calls:    usize,
    fail_on_call:  Option<usize>,
}

impl ReadFileTestSequentialFile {
    fn new(data: Vec<u8>, fail_on_call: Option<usize>) -> Self {
        trace!(
            data_len = data.len(),
            fail_on_call,
            "ReadFileTestSequentialFile::new"
        );
        Self {
            data,
            offset: 0,
            read_calls: 0,
            fail_on_call,
        }
    }
}

impl Named for ReadFileTestSequentialFile {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[read-file-test-sequential-file]".to_string())
    }
}

impl SequentialFileSkip for ReadFileTestSequentialFile {
    fn skip(&mut self, n: u64) -> Status {
        trace!(
            n,
            "ReadFileTestSequentialFile::skip"
        );
        let remaining = self.data.len().saturating_sub(self.offset);
        let to_skip = std::cmp::min(remaining, n as usize);
        self.offset += to_skip;
        Status::ok()
    }
}

impl SequentialFile for ReadFileTestSequentialFile {}

impl SequentialFileRead for ReadFileTestSequentialFile {
    fn read(
        &mut self,
        n: usize,
        result: *mut Slice,
        scratch: *mut u8,
    ) -> Status {
        self.read_calls += 1;

        if let Some(target) = self.fail_on_call {
            if self.read_calls == target {
                trace!(
                    call = self.read_calls,
                    "ReadFileTestSequentialFile::read injecting failure"
                );
                unsafe {
                    *result = Slice::default();
                }
                let msg = "read error".to_string();
                let msg_slice = Slice::from(&msg);
                return Status::io_error(&msg_slice, None);
            }
        }

        let remaining = self.data.len().saturating_sub(self.offset);

        // Force small chunks so multiple read() calls happen even if
        // the caller requests a very large n. This lets us reliably
        // simulate "some data read, then error" in tests.
        const MAX_CHUNK: usize = 4;
        let to_read = std::cmp::min(std::cmp::min(n, remaining), MAX_CHUNK);

        trace!(
            call = self.read_calls,
            n,
            remaining,
            to_read,
            "ReadFileTestSequentialFile::read"
        );

        unsafe {
            if to_read > 0 {
                let src = self.data.as_ptr().add(self.offset);
                std::ptr::copy_nonoverlapping(src, scratch, to_read);
                *result = Slice::from_ptr_len(scratch as *const u8, to_read);
            } else {
                *result = Slice::default();
            }
        }

        self.offset += to_read;
        Status::ok()
    }
}

#[derive(Default)]
struct TestEnvForReadFile {
    file_data:                 Vec<u8>,
    new_sequential_status:     Option<Status>,
    fail_on_read_call:         Option<usize>,
    new_sequential_file_calls: usize,
}

impl TestEnvForReadFile {
    fn new(file_data: Vec<u8>) -> Self {
        trace!(
            data_len = file_data.len(),
            "TestEnvForReadFile::new"
        );
        Self {
            file_data,
            ..Self::default()
        }
    }
}

impl NewSequentialFile for TestEnvForReadFile {
    fn new_sequential_file(
        &mut self,
        _fname: &String,
        result: *mut *mut Box<dyn SequentialFile>,
    ) -> Status {
        self.new_sequential_file_calls += 1;

        if let Some(status) = self.new_sequential_status.take() {
            trace!(
                status_str = %status.to_string(),
                "TestEnvForReadFile::new_sequential_file returning injected status"
            );
            return status;
        }

        let seq = ReadFileTestSequentialFile::new(
            self.file_data.clone(),
            self.fail_on_read_call,
        );
        let obj: Box<dyn SequentialFile> = Box::new(seq);
        let holder: Box<Box<dyn SequentialFile>> = Box::new(obj);
        let raw = Box::into_raw(holder);
        unsafe {
            *result = raw;
        }

        Status::ok()
    }
}

impl NewRandomAccessFile for TestEnvForReadFile {
    fn new_random_access_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_random_access_file".to_string()), None)
    }
}

impl NewWritableFile for TestEnvForReadFile {
    fn new_writable_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_writable_file".to_string()), None)
    }
}

impl NewAppendableFile for TestEnvForReadFile {
    fn new_appendable_file(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn WritableFile>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_appendable_file".to_string()), None)
    }
}

impl FileExists for TestEnvForReadFile {
    fn file_exists(&mut self, _fname: &String) -> bool {
        true
    }
}

impl GetChildren for TestEnvForReadFile {
    fn get_children(&mut self, _dir: &String, _result: *mut Vec<String>) -> Status {
        Status::not_supported(&Slice::from(&"get_children".to_string()), None)
    }
}

impl DeleteFile for TestEnvForReadFile {
    fn delete_file(&mut self, _fname: &String) -> Status {
        Status::ok()
    }
}

impl CreateDir for TestEnvForReadFile {
    fn create_dir(&mut self, _dirname: &String) -> Status {
        Status::not_supported(&Slice::from(&"create_dir".to_string()), None)
    }
}

impl DeleteDir for TestEnvForReadFile {
    fn delete_dir(&mut self, _dirname: &String) -> Status {
        Status::not_supported(&Slice::from(&"delete_dir".to_string()), None)
    }
}

impl GetFileSize for TestEnvForReadFile {
    fn get_file_size(&mut self, _fname: &String, _file_size: *mut u64) -> Status {
        Status::not_supported(&Slice::from(&"get_file_size".to_string()), None)
    }
}

impl RenameFile for TestEnvForReadFile {
    fn rename_file(&mut self, _src: &String, _target: &String) -> Status {
        Status::not_supported(&Slice::from(&"rename_file".to_string()), None)
    }
}

impl LockFile for TestEnvForReadFile {
    fn lock_file(
        &mut self,
        _fname: &String,
        _lock: *mut *mut Box<dyn FileLock>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"lock_file".to_string()), None)
    }
}

impl UnlockFile for TestEnvForReadFile {
    fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> Status {
        Status::not_supported(&Slice::from(&"unlock_file".to_string()), None)
    }
}

impl Schedule for TestEnvForReadFile {
    fn schedule(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        // not used
    }
}

impl StartThread for TestEnvForReadFile {
    fn start_thread(
        &mut self,
        _function: fn(arg: *mut c_void) -> c_void,
        _arg: *mut c_void,
    ) {
        // not used
    }
}

impl GetTestDirectory for TestEnvForReadFile {
    fn get_test_directory(&mut self, _path: *mut String) -> Status {
        Status::not_supported(&Slice::from(&"get_test_directory".to_string()), None)
    }
}

impl NewLogger for TestEnvForReadFile {
    fn new_logger(
        &mut self,
        _fname: &String,
        _result: *mut *mut Box<dyn Logger>,
    ) -> Status {
        Status::not_supported(&Slice::from(&"new_logger".to_string()), None)
    }
}

impl NowMicros for TestEnvForReadFile {
    fn now_micros(&mut self) -> u64 {
        0
    }
}

impl SleepForMicroseconds for TestEnvForReadFile {
    fn sleep_for_microseconds(&mut self, _micros: i32) {
        // noop
    }
}

impl Env for TestEnvForReadFile {}

#[traced_test]
fn read_file_to_string_reads_full_content_in_multiple_chunks() {
    trace!("read_file_to_string_reads_full_content_in_multiple_chunks: start");

    // Build data longer than K_BUFFER_SIZE (8192) to ensure multiple iterations.
    let base = "abc123XYZ";
    let mut source = String::new();
    while source.len() < 10_000 {
        source.push_str(base);
    }

    let data_bytes = source.as_bytes().to_vec();
    let env_impl = TestEnvForReadFile::new(data_bytes);
    let env: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));

    let fname = "in-memory".to_string();
    let mut output = String::new();
    let output_ptr: *mut String = &mut output;

    let status = read_file_to_string(env.clone(), &fname, output_ptr);
    assert!(
        status.is_ok(),
        "read_file_to_string should succeed: {}",
        status.to_string()
    );
    assert_eq!(output, source, "output content should match source");

    info!("read_file_to_string_reads_full_content_in_multiple_chunks: completed");
}

#[traced_test]
fn read_file_to_string_propagates_new_sequential_file_failure() {
    trace!("read_file_to_string_propagates_new_sequential_file_failure: start");

    let mut env_impl = TestEnvForReadFile::new(Vec::new());
    let msg = "no such file".to_string();
    let msg_slice = Slice::from(&msg);
    env_impl.new_sequential_status = Some(Status::not_found(&msg_slice, None));

    let env: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));

    let fname = "missing".to_string();
    let mut output = String::new();
    let output_ptr: *mut String = &mut output;

    let status = read_file_to_string(env.clone(), &fname, output_ptr);
    assert!(
        status.is_not_found(),
        "Status should be NotFound: {}",
        status.to_string()
    );
    assert!(
        output.is_empty(),
        "output string should remain empty on NewSequentialFile failure"
    );

    info!("read_file_to_string_propagates_new_sequential_file_failure: completed");
}

#[traced_test]
fn read_file_to_string_stops_on_read_error() {
    trace!("read_file_to_string_stops_on_read_error: start");

    let source = "partial-then-error".to_string();
    let bytes = source.as_bytes().to_vec();
    let mut env_impl = TestEnvForReadFile::new(bytes);
    env_impl.fail_on_read_call = Some(2); // fail on second read

    let env: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));

    let fname = "error-after-first-chunk".to_string();
    let mut output = String::new();
    let output_ptr: *mut String = &mut output;

    let status = read_file_to_string(env.clone(), &fname, output_ptr);
    assert!(
        status.is_io_error(),
        "read error should be reported as IO error: {}",
        status.to_string()
    );
    assert!(
        !output.is_empty(),
        "some data may have been read before the error, output should not be empty"
    );
    assert!(
        output.len() < source.len(),
        "output should be partial when read stops on error"
    );

    info!("read_file_to_string_stops_on_read_error: completed");
}
