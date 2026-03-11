// ---------------- [ File: bitcoinleveldb-test/src/util.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testutil.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testutil.cc]

/**
  | A wrapper that allows injection of errors.
  |
  */
pub struct ErrorEnv {
    base:                     EnvWrapper,
    writable_file_error:      bool,
    num_writable_file_errors: i32,
}

impl ErrorEnv {
    pub fn set_writable_file_error(&mut self, value: bool) {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_set_writable_file_error_entry",
            value = value
        );

        self.writable_file_error = value;

        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_set_writable_file_error_exit",
            value = self.writable_file_error
        );
    }

    pub fn writable_file_error(&self) -> bool {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_writable_file_error_entry",
            value = self.writable_file_error
        );

        self.writable_file_error
    }

    pub fn num_writable_file_errors(&self) -> i32 {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_num_writable_file_errors_entry",
            value = self.num_writable_file_errors
        );

        self.num_writable_file_errors
    }
}

impl Default for ErrorEnv {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_default_entry"
        );

        let out = Self {
            base: EnvWrapper::new(new_mem_env(posix_default_env())),
            writable_file_error: false,
            num_writable_file_errors: 0,
        };

        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_default_exit",
            writable_file_error = out.writable_file_error,
            num_writable_file_errors = out.num_writable_file_errors
        );

        out
    }
}

impl Drop for ErrorEnv {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_drop",
            writable_file_error = self.writable_file_error,
            num_writable_file_errors = self.num_writable_file_errors
        );
    }
}

impl Env for ErrorEnv { }

impl DeleteFile for ErrorEnv {
    fn delete_file(&mut self, fname: &String) -> crate::Status {
        self.base.delete_file(fname)
    }
}

impl CreateDir for ErrorEnv {
    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        self.base.create_dir(dirname)
    }
}

impl DeleteDir for ErrorEnv {
    fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        self.base.delete_dir(dirname)
    }
}

impl NewSequentialFile for ErrorEnv {
    fn new_sequential_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn SequentialFile>,
    ) -> crate::Status {
        self.base.new_sequential_file(fname, result)
    }
}

impl NewRandomAccessFile for ErrorEnv {
    fn new_random_access_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> crate::Status {
        self.base.new_random_access_file(fname, result)
    }
}

impl NewWritableFile for ErrorEnv {
    fn new_writable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_new_writable_file_entry",
            filename = %fname,
            writable_file_error = self.writable_file_error
        );

        if self.writable_file_error {
            self.num_writable_file_errors += 1;

            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }

            let fname_slice = Slice::from(fname);
            let msg_string = "fake error".to_string();
            let msg_slice = Slice::from(&msg_string);
            let status = Status::io_error(&fname_slice, Some(&msg_slice));

            trace!(
                target: "bitcoinleveldb_test::util",
                event = "error_env_new_writable_file_exit",
                filename = %fname,
                ok = status.is_ok(),
                num_writable_file_errors = self.num_writable_file_errors
            );

            return status;
        }

        let status = self.base.new_writable_file(fname, result);

        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_new_writable_file_exit",
            filename = %fname,
            ok = status.is_ok(),
            num_writable_file_errors = self.num_writable_file_errors
        );

        status
    }
}

impl NewAppendableFile for ErrorEnv {
    fn new_appendable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_new_appendable_file_entry",
            filename = %fname,
            writable_file_error = self.writable_file_error
        );

        if self.writable_file_error {
            self.num_writable_file_errors += 1;

            unsafe {
                if !result.is_null() {
                    *result = core::ptr::null_mut();
                }
            }

            let fname_slice = Slice::from(fname);
            let msg_string = "fake error".to_string();
            let msg_slice = Slice::from(&msg_string);
            let status = Status::io_error(&fname_slice, Some(&msg_slice));

            trace!(
                target: "bitcoinleveldb_test::util",
                event = "error_env_new_appendable_file_exit",
                filename = %fname,
                ok = status.is_ok(),
                num_writable_file_errors = self.num_writable_file_errors
            );

            return status;
        }

        let status = self.base.new_appendable_file(fname, result);

        trace!(
            target: "bitcoinleveldb_test::util",
            event = "error_env_new_appendable_file_exit",
            filename = %fname,
            ok = status.is_ok(),
            num_writable_file_errors = self.num_writable_file_errors
        );

        status
    }
}

impl FileExists for ErrorEnv {
    fn file_exists(&mut self, fname: &String) -> bool {
        self.base.file_exists(fname)
    }
}

impl GetChildren for ErrorEnv {
    fn get_children(
        &mut self,
        dir:    &String,
        result: *mut Vec<String>,
    ) -> crate::Status {
        self.base.get_children(dir, result)
    }
}

impl GetFileSize for ErrorEnv {
    fn get_file_size(
        &mut self,
        fname:     &String,
        file_size: *mut u64,
    ) -> crate::Status {
        self.base.get_file_size(fname, file_size)
    }
}

impl RenameFile for ErrorEnv {
    fn rename_file(
        &mut self,
        src:    &String,
        target: &String,
    ) -> crate::Status {
        self.base.rename_file(src, target)
    }
}

impl LockFile for ErrorEnv {
    fn lock_file(
        &mut self,
        fname: &String,
        lock:  *mut *mut Box<dyn FileLock>,
    ) -> crate::Status {
        self.base.lock_file(fname, lock)
    }
}

impl UnlockFile for ErrorEnv {
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        self.base.unlock_file(lock)
    }
}

impl Schedule for ErrorEnv {
    fn schedule(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        self.base.schedule(function, arg);
    }
}

impl StartThread for ErrorEnv {
    fn start_thread(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        self.base.start_thread(function, arg);
    }
}

impl GetTestDirectory for ErrorEnv {
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        self.base.get_test_directory(path)
    }
}

impl NewLogger for ErrorEnv {
    fn new_logger(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn Logger>,
    ) -> crate::Status {
        self.base.new_logger(fname, result)
    }
}

impl NowMicros for ErrorEnv {
    fn now_micros(&mut self) -> u64 {
        self.base.now_micros()
    }
}

impl SleepForMicroseconds for ErrorEnv {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        self.base.sleep_for_microseconds(micros);
    }
}
/**
  | Store in *dst a random string of length
  | "len" and return a Slice that references
  | the generated data.
  |
  */
pub fn random_string(
    rnd: *mut Random,
    len: i32,
    dst: *mut String,
) -> Slice {
    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_string_entry",
        rnd_is_null = rnd.is_null(),
        dst_is_null = dst.is_null(),
        len = len
    );

    if rnd.is_null() || dst.is_null() {
        error!(
            target: "bitcoinleveldb_test::util",
            event = "random_string_null_input",
            rnd_is_null = rnd.is_null(),
            dst_is_null = dst.is_null()
        );

        return Slice::from_ptr_len(core::ptr::null::<u8>(), 0usize);
    }

    let target_len: usize = if len <= 0 { 0usize } else { len as usize };
    let mut bytes: Vec<u8> = Vec::with_capacity(target_len);

    let mut i: usize = 0usize;
    while i < target_len {
        // ' ' .. '~'
        let sample = unsafe { (&mut *rnd).uniform(95) } as u8;
        bytes.push(b' ' + sample);
        i += 1usize;
    }

    let generated = match String::from_utf8(bytes) {
        Ok(v) => v,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    };

    unsafe {
        *dst = generated;
    }

    let out = unsafe { Slice::from(&*dst) };

    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_string_exit",
        result_len = target_len
    );

    out
}

/**
  | Return a random key with the specified
  | length that may contain interesting
  | characters (e.g. \x00, \xff, etc.).
  |
  */
pub fn random_key(
    rnd: *mut Random,
    len: i32,
) -> String {
    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_key_entry",
        rnd_is_null = rnd.is_null(),
        len = len
    );

    // Make sure to generate a wide variety of characters so we
    // test the boundary conditions for short-key optimizations.
    const BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS: [u8; 10] = [
        b'\0',
        b'\x01',
        b'a',
        b'b',
        b'c',
        b'd',
        b'e',
        b'\xfd',
        b'\xfe',
        b'\xff',
    ];

    if rnd.is_null() || len <= 0 {
        trace!(
            target: "bitcoinleveldb_test::util",
            event = "random_key_exit",
            result_len = 0
        );

        return String::new();
    }

    let mut result = String::new();
    let mut i: i32 = 0i32;

    while i < len {
        let idx = unsafe {
            (&mut *rnd).uniform(BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS.len() as i32)
        } as usize;

        let ch = char::from(BITCOINLEVELDB_TEST_UTIL_RANDOM_KEY_TEST_CHARS[idx]);
        result.push(ch);

        i += 1i32;
    }

    trace!(
        target: "bitcoinleveldb_test::util",
        event = "random_key_exit",
        result_len = result.len()
    );

    result
}

/**
  | Store in *dst a string of length "len" that
  | will compress to "N*compressed_fraction" bytes
  | and return a Slice that references the
  | generated data.
  */
pub fn compressible_string(
    rnd:                 *mut Random,
    compressed_fraction: f64,
    len:                 usize,
    dst:                 *mut String,
) -> Slice {
    trace!(
        target: "bitcoinleveldb_test::util",
        event = "compressible_string_entry",
        rnd_is_null = rnd.is_null(),
        dst_is_null = dst.is_null(),
        compressed_fraction = compressed_fraction,
        len = len
    );

    if dst.is_null() {
        error!(
            target: "bitcoinleveldb_test::util",
            event = "compressible_string_null_dst"
        );

        return Slice::from_ptr_len(core::ptr::null::<u8>(), 0usize);
    }

    let mut raw: i32 = (len as f64 * compressed_fraction) as i32;
    if raw < 1 {
        raw = 1;
    }

    let mut raw_data = String::new();
    let _raw_slice = random_string(
        rnd,
        raw,
        (&mut raw_data) as *mut String,
    );

    // Duplicate the random data until we have filled "len" bytes
    unsafe {
        (*dst).clear();

        while (&(*dst)).len() < len {
            (*dst).push_str(raw_data.as_str());
        }

        (*dst).truncate(len);
    }

    let out = unsafe { Slice::from(&*dst) };

    trace!(
        target: "bitcoinleveldb_test::util",
        event = "compressible_string_exit",
        raw_len = raw,
        result_len = len
    );

    out
}
