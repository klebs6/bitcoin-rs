// ---------------- [ File: bitcoinleveldb-testfaultinjection/src/bitcoinleveldb_testfaultinjection.rs ]
/*!
  | This test uses a custom Env to keep track of
  | the state of a filesystem as of the last
  | "sync". It then checks for data loss errors by
  | purposely dropping file data (or entire files)
  | not protected by a "sync".
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/fault_injection_test.cc]

const VALUE_SIZE:     i32 = 1000;
const MAX_NUM_VALUES: i32 = 2000;
const NUM_ITERATIONS: usize = 3;

/**
  | Assume a filename, and not a directory
  | name like "/foo/bar/"
  |
  */
fn get_dir_name(filename: &String) -> String {
    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "get_dir_name_entry",
        filename_len = filename.len()
    );

    let out = match filename.rfind(|c| c == '/' || c == '\\') {
        Some(found) => filename[..found].to_string(),
        None => String::new(),
    };

    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "get_dir_name_exit",
        dir_len = out.len()
    );

    out
}

fn sync_dir(dir: &String) -> crate::Status {
    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "sync_dir_entry",
        dir_len = dir.len()
    );

    let status = Status::ok();

    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "sync_dir_exit",
        ok = status.is_ok()
    );

    status
}

/**
  | A basic file truncation function suitable
  | for this test.
  |
  */
fn truncate(
    filename: &String,
    length:   u64,
) -> crate::Status {
    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "truncate_entry",
        filename = %filename,
        length = length
    );

    let env = posix_default_env();

    let mut orig_file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
    let mut s = env.borrow_mut().new_sequential_file(
        filename,
        (&mut orig_file) as *mut *mut Box<dyn SequentialFile>,
    );

    if !s.is_ok() {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "truncate_exit",
            filename = %filename,
            ok = s.is_ok()
        );

        return s;
    }

    let mut scratch: Vec<u8> = vec![0u8; length as usize];
    let mut result: Slice = Slice::default();

    {
        let mut orig_file_holder: Box<Box<dyn SequentialFile>> = unsafe {
            Box::from_raw(orig_file)
        };
        let orig_file_ref: &mut Box<dyn SequentialFile> = orig_file_holder.as_mut();

        s = orig_file_ref.read(
            length as usize,
            (&mut result) as *mut Slice,
            scratch.as_mut_ptr(),
        );
    }

    if s.is_ok() {
        scratch.truncate(*result.size());

        let tmp_name = format!("{}/truncate.tmp", get_dir_name(filename));

        let mut tmp_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        s = env.borrow_mut().new_writable_file(
            &tmp_name,
            (&mut tmp_file) as *mut *mut Box<dyn WritableFile>,
        );

        if s.is_ok() {
            let mut tmp_file_holder: Box<Box<dyn WritableFile>> = unsafe {
                Box::from_raw(tmp_file)
            };
            let tmp_file_ref: &mut Box<dyn WritableFile> = tmp_file_holder.as_mut();

            let append_slice = Slice::from(scratch.as_slice());
            s = tmp_file_ref.append(&append_slice);

            if s.is_ok() {
                s = tmp_file_ref.close();
            }

            drop(tmp_file_holder);

            if s.is_ok() {
                s = env.borrow_mut().rename_file(&tmp_name, filename);
            } else {
                let _ = env.borrow_mut().delete_file(&tmp_name);
            }
        }
    }

    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "truncate_exit",
        filename = %filename,
        ok = s.is_ok()
    );

    s
}

struct FileState {
    filename:          String,
    pos:               i64,
    pos_at_last_sync:  i64,
    pos_at_last_flush: i64,
}

impl Default for FileState {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_default_entry"
        );

        let out = Self {
            filename: String::new(),
            pos: -1i64,
            pos_at_last_sync: -1i64,
            pos_at_last_flush: -1i64,
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_default_exit"
        );

        out
    }
}

impl FileState {
    pub fn new(filename: &String) -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_new_entry",
            filename_len = filename.len()
        );

        let out = Self {
            filename: filename.clone(),
            pos: -1i64,
            pos_at_last_sync: -1i64,
            pos_at_last_flush: -1i64,
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_new_exit"
        );

        out
    }

    pub fn is_fully_synced(&self) -> bool {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_is_fully_synced_entry",
            pos = self.pos,
            pos_at_last_sync = self.pos_at_last_sync
        );

        let out = self.pos <= 0i64 || self.pos == self.pos_at_last_sync;

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_is_fully_synced_exit",
            result = out
        );

        out
    }
}

/**
  | A wrapper around WritableFile which
  | informs another Env whenever this file
  | is written to or sync'ed.
  |
  */
struct TestWritableFile {
    state:                FileState,
    target:               *mut Box<dyn WritableFile>,
    writable_file_opened: bool,
    env:                  *mut FaultInjectionTestEnv,
}

impl TestWritableFile {
    pub fn new(
        state: &FileState,
        f:     *mut Box<dyn WritableFile>,
        env:   *mut FaultInjectionTestEnv,
    ) -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_new_entry",
            target_is_null = f.is_null(),
            env_is_null = env.is_null()
        );

        assert!(!f.is_null());

        let out = Self {
            state: FileState {
                filename: state.filename.clone(),
                pos: state.pos,
                pos_at_last_sync: state.pos_at_last_sync,
                pos_at_last_flush: state.pos_at_last_flush,
            },
            target: f,
            writable_file_opened: true,
            env,
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_new_exit",
            writable_file_opened = out.writable_file_opened
        );

        out
    }
}

impl Drop for TestWritableFile {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_drop_entry",
            writable_file_opened = self.writable_file_opened,
            target_is_null = self.target.is_null(),
            env_is_null = self.env.is_null()
        );

        if self.writable_file_opened {
            let _ = self.close();
        }

        if !self.target.is_null() {
            unsafe {
                drop(Box::from_raw(self.target));
            }
        }

        debug!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_drop_exit"
        );
    }
}

impl WritableFile for TestWritableFile {

}

impl WritableFileAppend for TestWritableFile {
    fn append(&mut self, data: &Slice) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_append_entry",
            filename = %self.state.filename,
            data_size = *data.size()
        );

        let s = unsafe { (&mut *self.target).append(data) };

        if s.is_ok() && unsafe { (&mut *self.env).is_filesystem_active() } {
            self.state.pos += *data.size() as i64;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_append_exit",
            filename = %self.state.filename,
            ok = s.is_ok(),
            pos = self.state.pos
        );

        s
    }
}

impl WritableFileClose for TestWritableFile {
    fn close(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_close_entry",
            filename = %self.state.filename,
            writable_file_opened = self.writable_file_opened
        );

        self.writable_file_opened = false;
        let s = unsafe { (&mut *self.target).close() };

        if s.is_ok() {
            unsafe {
                (&mut *self.env).writable_file_closed(&self.state);
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_close_exit",
            filename = %self.state.filename,
            ok = s.is_ok(),
            writable_file_opened = self.writable_file_opened
        );

        s
    }
}

impl WritableFileFlush for TestWritableFile {
    fn flush(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_flush_entry",
            filename = %self.state.filename,
            pos = self.state.pos
        );

        let s = unsafe { (&mut *self.target).flush() };

        if s.is_ok() && unsafe { (&mut *self.env).is_filesystem_active() } {
            self.state.pos_at_last_flush = self.state.pos;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_flush_exit",
            filename = %self.state.filename,
            ok = s.is_ok(),
            pos_at_last_flush = self.state.pos_at_last_flush
        );

        s
    }
}

impl WritableFileSync for TestWritableFile {
    fn sync(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_sync_entry",
            filename = %self.state.filename,
            pos = self.state.pos
        );

        if !unsafe { (&mut *self.env).is_filesystem_active() } {
            trace!(
                target: "bitcoinleveldb_test::fault_injection_test",
                event = "test_writable_file_sync_exit",
                filename = %self.state.filename,
                ok = true,
                filesystem_active = false
            );

            return Status::ok();
        }

        // Ensure new files referred to by the manifest are in the filesystem.
        let mut s = unsafe { (&mut *self.target).sync() };

        if s.is_ok() {
            self.state.pos_at_last_sync = self.state.pos;
        }

        if unsafe {
            (&mut *self.env).is_file_created_since_last_dir_sync(&self.state.filename)
        } {
            let ps = self.sync_parent();
            if s.is_ok() && !ps.is_ok() {
                s = ps;
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_sync_exit",
            filename = %self.state.filename,
            ok = s.is_ok(),
            pos_at_last_sync = self.state.pos_at_last_sync
        );

        s
    }
}

impl Named for TestWritableFile {
    
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("".to_string())
    }
}

///-------------------------
struct FaultInjectionTestEnv {
    base:  EnvWrapper,
    mutex: Mutex<fault_injection_test_env::Inner>,
}

impl Default for FaultInjectionTestEnv {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_default_entry"
        );

        let out = Self {
            base: EnvWrapper::new(posix_default_env()),
            mutex: Mutex::new(fault_injection_test_env::Inner::default()),
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_default_exit"
        );

        out
    }
}

impl Env for FaultInjectionTestEnv { }

impl NewSequentialFile for FaultInjectionTestEnv {
    fn new_sequential_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn SequentialFile>,
    ) -> crate::Status {
        self.base.new_sequential_file(fname, result)
    }
}

impl NewRandomAccessFile for FaultInjectionTestEnv {
    fn new_random_access_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn RandomAccessFile>,
    ) -> crate::Status {
        self.base.new_random_access_file(fname, result)
    }
}

impl FileExists for FaultInjectionTestEnv {
    fn file_exists(&mut self, fname: &String) -> bool {
        self.base.file_exists(fname)
    }
}

impl GetChildren for FaultInjectionTestEnv {
    fn get_children(
        &mut self,
        dir:    &String,
        result: *mut Vec<String>,
    ) -> crate::Status {
        self.base.get_children(dir, result)
    }
}

impl CreateDir for FaultInjectionTestEnv {
    fn create_dir(&mut self, dirname: &String) -> crate::Status {
        self.base.create_dir(dirname)
    }
}

impl DeleteDir for FaultInjectionTestEnv {
    fn delete_dir(&mut self, dirname: &String) -> crate::Status {
        self.base.delete_dir(dirname)
    }
}

impl GetFileSize for FaultInjectionTestEnv {
    fn get_file_size(
        &mut self,
        fname:     &String,
        file_size: *mut u64,
    ) -> crate::Status {
        self.base.get_file_size(fname, file_size)
    }
}

impl DeleteFile for FaultInjectionTestEnv {
    fn delete_file(&mut self, f: &String) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_file_trait_entry",
            filename = %f
        );

        let status = self.base.delete_file(f);
        assert!(status.is_ok());

        if status.is_ok() {
            let mut guard = self.mutex.lock();
            guard.db_file_state.remove(f);
            guard.new_files_since_last_dir_sync.remove(f);
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_file_trait_exit",
            filename = %f,
            ok = status.is_ok()
        );

        status
    }
}

impl RenameFile for FaultInjectionTestEnv {
    fn rename_file(
        &mut self,
        src:    &String,
        target: &String,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_rename_file_trait_entry",
            source = %src,
            target = %target
        );

        let ret = self.base.rename_file(src, target);

        if ret.is_ok() {
            let mut guard = self.mutex.lock();

            if let Some(state) = guard.db_file_state.remove(src) {
                guard.db_file_state.insert(
                    target.clone(),
                    FileState {
                        filename: target.clone(),
                        pos: state.pos,
                        pos_at_last_sync: state.pos_at_last_sync,
                        pos_at_last_flush: state.pos_at_last_flush,
                    },
                );
            }

            if guard.new_files_since_last_dir_sync.remove(src) {
                guard.new_files_since_last_dir_sync.insert(target.clone());
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_rename_file_trait_exit",
            source = %src,
            target = %target,
            ok = ret.is_ok()
        );

        ret
    }
}

impl LockFile for FaultInjectionTestEnv {
    fn lock_file(
        &mut self,
        fname: &String,
        lock:  *mut *mut Box<dyn FileLock>,
    ) -> crate::Status {
        self.base.lock_file(fname, lock)
    }
}

impl UnlockFile for FaultInjectionTestEnv {
    fn unlock_file(&mut self, lock: *mut Box<dyn FileLock>) -> crate::Status {
        self.base.unlock_file(lock)
    }
}

impl Schedule for FaultInjectionTestEnv {
    fn schedule(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        self.base.schedule(function, arg);
    }
}

impl StartThread for FaultInjectionTestEnv {
    fn start_thread(
        &mut self,
        function: fn(arg: *mut c_void) -> c_void,
        arg:      *mut c_void,
    ) {
        self.base.start_thread(function, arg);
    }
}

impl GetTestDirectory for FaultInjectionTestEnv {
    fn get_test_directory(&mut self, path: *mut String) -> crate::Status {
        self.base.get_test_directory(path)
    }
}

impl NewLogger for FaultInjectionTestEnv {
    fn new_logger(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn Logger>,
    ) -> crate::Status {
        self.base.new_logger(fname, result)
    }
}

impl NowMicros for FaultInjectionTestEnv {
    fn now_micros(&mut self) -> u64 {
        self.base.now_micros()
    }
}

impl SleepForMicroseconds for FaultInjectionTestEnv {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        self.base.sleep_for_microseconds(micros);
    }
}
mod fault_injection_test_env {

    use super::*;

    pub struct Inner {
        pub db_file_state:                 HashMap<String,FileState>,
        pub new_files_since_last_dir_sync: HashSet<String>,

        /// Record flushes, syncs, writes
        pub filesystem_active:             bool,
    }
}

impl Default for fault_injection_test_env::Inner {

    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_inner_default_entry"
        );

        let out = Self {
            db_file_state: HashMap::new(),
            new_files_since_last_dir_sync: HashSet::new(),
            filesystem_active: true,
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_inner_default_exit",
            filesystem_active = out.filesystem_active
        );

        out
    }
}

impl FaultInjectionTestEnv {

    /// Setting the filesystem to inactive is the test equivalent to simulating a system reset. 
    ///
    /// Setting to inactive will freeze our saved filesystem state so that it will stop being
    /// recorded. It can then be reset back to the state at the time of the reset.
    ///
    #[LOCKS_EXCLUDED(mutex_)]
    pub fn is_filesystem_active(&mut self) -> bool {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_is_filesystem_active_entry"
        );

        let guard = self.mutex.lock();
        let out = guard.filesystem_active;

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_is_filesystem_active_exit",
            result = out
        );

        out
    }

    #[LOCKS_EXCLUDED(mutex_)]
    pub fn set_filesystem_active(&mut self, active: bool) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_set_filesystem_active_entry",
            active = active
        );

        let mut guard = self.mutex.lock();
        guard.filesystem_active = active;

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_set_filesystem_active_exit",
            active = active
        );
    }
}

impl TestWritableFile {

    fn sync_parent(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_sync_parent_entry",
            filename = %self.state.filename
        );

        let parent_dir = get_dir_name(&self.state.filename);
        let s = sync_dir(&parent_dir);

        if s.is_ok() {
            unsafe {
                (&mut *self.env).dir_was_synced();
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "test_writable_file_sync_parent_exit",
            filename = %self.state.filename,
            ok = s.is_ok()
        );

        s
    }
}

impl NewWritableFile for FaultInjectionTestEnv {

    fn new_writable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_new_writable_file_entry",
            filename = %fname,
            result_ptr_is_null = result.is_null()
        );

        let mut actual_writable_file =
            core::mem::MaybeUninit::<*mut Box<dyn WritableFile>>::uninit();

        let s = self.base.new_writable_file(
            fname,
            actual_writable_file.as_mut_ptr(),
        );

        if s.is_ok() {
            assert!(!result.is_null());

            let actual_writable_file = unsafe { actual_writable_file.assume_init() };
            let mut state = FileState::new(fname);
            state.pos = 0;

            let wrapped: Box<dyn WritableFile> = Box::new(
                TestWritableFile::new(
                    &state,
                    actual_writable_file,
                    self as *mut FaultInjectionTestEnv,
                ),
            );

            unsafe {
                *result = Box::into_raw(Box::new(wrapped));
            }

            // NewWritableFile doesn't append to files, so if the same file is
            // opened again then it will be truncated - so forget our saved
            // state.
            self.untrack_file(fname);

            let mut guard = self.mutex.lock();
            guard.new_files_since_last_dir_sync.insert(fname.clone());
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_new_writable_file_exit",
            filename = %fname,
            ok = s.is_ok()
        );

        s
    }
}

impl NewAppendableFile for FaultInjectionTestEnv {

    fn new_appendable_file(
        &mut self,
        fname:  &String,
        result: *mut *mut Box<dyn WritableFile>,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_new_appendable_file_entry",
            filename = %fname,
            result_ptr_is_null = result.is_null()
        );

        let mut actual_writable_file =
            core::mem::MaybeUninit::<*mut Box<dyn WritableFile>>::uninit();

        let s = self.base.new_appendable_file(
            fname,
            actual_writable_file.as_mut_ptr(),
        );

        if s.is_ok() {
            assert!(!result.is_null());

            let actual_writable_file = unsafe { actual_writable_file.assume_init() };
            let mut state = FileState::new(fname);
            state.pos = 0;

            {
                let mut guard = self.mutex.lock();

                if !guard.db_file_state.contains_key(fname) {
                    guard.new_files_since_last_dir_sync.insert(fname.clone());
                } else if let Some(existing_state) = guard.db_file_state.get(fname) {
                    state = FileState {
                        filename: existing_state.filename.clone(),
                        pos: existing_state.pos,
                        pos_at_last_sync: existing_state.pos_at_last_sync,
                        pos_at_last_flush: existing_state.pos_at_last_flush,
                    };
                }
            }

            let wrapped: Box<dyn WritableFile> = Box::new(
                TestWritableFile::new(
                    &state,
                    actual_writable_file,
                    self as *mut FaultInjectionTestEnv,
                ),
            );

            unsafe {
                *result = Box::into_raw(Box::new(wrapped));
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_new_appendable_file_exit",
            filename = %fname,
            ok = s.is_ok()
        );

        s
    }
}

impl FaultInjectionTestEnv {

    pub fn drop_unsynced_file_data(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_drop_unsynced_file_data_entry"
        );

        let guard = self.mutex.lock();
        let mut status = Status::ok();

        for (_filename, state) in guard.db_file_state.iter() {
            let state: &FileState = state;

            if !status.is_ok() {
                break;
            }

            if !state.is_fully_synced() {
                status = state.drop_unsynced_data();
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_drop_unsynced_file_data_exit",
            ok = status.is_ok()
        );

        status
    }

    pub fn dir_was_synced(&mut self) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_dir_was_synced_entry"
        );

        let mut guard = self.mutex.lock();
        guard.new_files_since_last_dir_sync.clear();

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_dir_was_synced_exit"
        );
    }
 
    pub fn is_file_created_since_last_dir_sync(&mut self, filename: &String) -> bool {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_is_file_created_since_last_dir_sync_entry",
            filename = %filename
        );

        let guard = self.mutex.lock();
        let out = guard.new_files_since_last_dir_sync.contains(filename);

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_is_file_created_since_last_dir_sync_exit",
            filename = %filename,
            result = out
        );

        out
    }

    pub fn untrack_file(&mut self, f: &String) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_untrack_file_entry",
            filename = %f
        );

        let mut guard = self.mutex.lock();
        guard.db_file_state.remove(f);
        guard.new_files_since_last_dir_sync.remove(f);

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_untrack_file_exit",
            filename = %f
        );
    }

    pub fn delete_file(&mut self, f: &String) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_file_entry",
            filename = %f
        );

        let status = self.base.delete_file(f);
        assert!(status.is_ok());

        if status.is_ok() {
            self.untrack_file(f);
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_file_exit",
            filename = %f,
            ok = status.is_ok()
        );

        status
    }

    pub fn rename_file(
        &mut self,
        s: &String,
        t: &String,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_rename_file_entry",
            source = %s,
            target = %t
        );

        let ret = self.base.rename_file(s, t);

        if ret.is_ok() {
            let mut guard = self.mutex.lock();

            if let Some(state) = guard.db_file_state.remove(s) {
                guard.db_file_state.insert(
                    t.clone(),
                    FileState {
                        filename: t.clone(),
                        pos: state.pos,
                        pos_at_last_sync: state.pos_at_last_sync,
                        pos_at_last_flush: state.pos_at_last_flush,
                    },
                );
            }

            if guard.new_files_since_last_dir_sync.remove(s) {
                guard.new_files_since_last_dir_sync.insert(t.clone());
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_rename_file_exit",
            source = %s,
            target = %t,
            ok = ret.is_ok()
        );

        ret
    }

    pub fn reset_state(&mut self) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_reset_state_entry"
        );

        // Since we are not destroying the database, the existing files
        // should keep their recorded synced/flushed state. Therefore
        // we do not reset db_file_state_ and new_files_since_last_dir_sync_.
        self.set_filesystem_active(true);

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_reset_state_exit"
        );
    }

    pub fn delete_files_created_after_last_dir_sync(&mut self) -> crate::Status {
        // Because DeleteFile access this container make a copy to avoid deadlock
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_files_created_after_last_dir_sync_entry"
        );

        let new_files: Vec<String> = {
            let guard = self.mutex.lock();
            guard.new_files_since_last_dir_sync.iter().cloned().collect()
        };

        let mut status = Status::ok();

        for new_file in new_files.iter() {
            let delete_status = self.delete_file(new_file);
            if !delete_status.is_ok() && status.is_ok() {
                status = delete_status;
            }
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_delete_files_created_after_last_dir_sync_exit",
            ok = status.is_ok()
        );

        status
    }

    pub fn writable_file_closed(&mut self, state: &FileState) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_writable_file_closed_entry",
            filename = %state.filename
        );

        let mut guard = self.mutex.lock();
        guard.db_file_state.insert(
            state.filename.clone(),
            FileState {
                filename: state.filename.clone(),
                pos: state.pos,
                pos_at_last_sync: state.pos_at_last_sync,
                pos_at_last_flush: state.pos_at_last_flush,
            },
        );

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_env_writable_file_closed_exit",
            filename = %state.filename
        );
    }
}

impl FileState {
    pub fn drop_unsynced_data(&self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_drop_unsynced_data_entry",
            filename = %self.filename,
            pos = self.pos,
            pos_at_last_sync = self.pos_at_last_sync
        );

        let sync_pos: i64 = if self.pos_at_last_sync == -1i64 {
            0i64
        } else {
            self.pos_at_last_sync
        };

        let status = truncate(
            &self.filename,
            sync_pos as u64,
        );

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "file_state_drop_unsynced_data_exit",
            filename = %self.filename,
            ok = status.is_ok(),
            sync_pos = sync_pos
        );

        status
    }
}

///-------------------
struct FaultInjectionTest {
    env:        *mut FaultInjectionTestEnv,
    dbname:     String,
    tiny_cache: *mut Cache,
    options:    Options,
    db:         *mut dyn DB,
}

#[derive(Debug)]
enum ExpectedVerifResult { 
    VAL_EXPECT_NO_ERROR, 
    VAL_EXPECT_ERROR 
}

#[derive(Debug)]
enum ResetMethod { 
    RESET_DROP_UNSYNCED_DATA, 
    RESET_DELETE_UNSYNCED_FILES 
}

impl Default for FaultInjectionTest {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_default_entry"
        );

        let env_holder: Rc<RefCell<dyn Env>> =
            Rc::new(RefCell::new(FaultInjectionTestEnv::default()));

        let env_ptr = {
            let env_dyn_ptr: *mut dyn Env = env_holder.as_ref().as_ptr();
            (env_dyn_ptr as *mut ()) as *mut FaultInjectionTestEnv
        };

        let tiny_cache = new_lru_cache(100usize);
        let dbname = unique_db_path("/fault_test");

        let destroy_status = destroydb(&dbname, &Options::default());
        assert!(destroy_status.is_ok());

        let mut options = Options::default();
        options.set_reuse_logs(true);
        options.set_env(Some(env_holder));
        options.set_paranoid_checks(true);
        options.set_block_cache(tiny_cache);
        options.set_create_if_missing(true);

        let out = Self {
            env: env_ptr,
            dbname,
            tiny_cache,
            options,
            db: core::ptr::null_mut::<DBImpl>() as *mut dyn DB,
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_default_exit",
            env_is_null = out.env.is_null(),
            db_is_null = out.db.is_null()
        );

        out
    }
}

impl Drop for FaultInjectionTest {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_drop_entry",
            env_is_null = self.env.is_null(),
            db_is_null = self.db.is_null(),
            cache_is_null = self.tiny_cache.is_null()
        );

        self.closedb();
        let _ = destroydb(&self.dbname, &Options::default());

        if !self.tiny_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.tiny_cache));
            }
            self.tiny_cache = core::ptr::null_mut();
        }

        debug!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_drop_exit"
        );
    }
}

impl FaultInjectionTest {

    pub fn reuse_logs(&mut self, reuse: bool) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_reuse_logs_entry",
            reuse = reuse
        );

        *self.options.reuse_logs_mut() = reuse;

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_reuse_logs_exit",
            reuse = *self.options.reuse_logs()
        );
    }

    pub fn build(
        &mut self,
        start_idx: i32,
        num_vals:  i32,
    ) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_build_entry",
            start_idx = start_idx,
            num_vals = num_vals
        );

        let mut key_space = String::new();
        let mut value_space = String::new();
        let mut batch = WriteBatch::new();

        let mut i: i32 = start_idx;
        while i < (start_idx + num_vals) {
            let key = self.key(i, (&mut key_space) as *mut String);
            let value = self.value(i, (&mut value_space) as *mut String);

            batch.clear();
            batch.put(&key, &value);

            let status = unsafe {
                (&mut *self.db).write(
                    &WriteOptions::default(),
                    (&mut batch) as *mut WriteBatch,
                )
            };
            assert!(status.is_ok());

            i += 1i32;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_build_exit",
            start_idx = start_idx,
            num_vals = num_vals
        );
    }

    pub fn read_value(
        &self,
        i:   i32,
        val: *mut String,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_read_value_entry",
            i = i,
            val_is_null = val.is_null()
        );

        let mut key_space = String::new();
        let mut value_space = String::new();

        let key = self.key(i, (&mut key_space) as *mut String);
        let _expected = self.value(i, (&mut value_space) as *mut String);

        let status = unsafe {
            (&mut *self.db).get(
                &ReadOptions::default(),
                &key,
                val,
            )
        };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_read_value_exit",
            i = i,
            ok = status.is_ok()
        );

        status
    }
   
    pub fn verify(
        &self,
        start_idx: i32,
        num_vals:  i32,
        expected:  ExpectedVerifResult,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_verify_entry",
            start_idx = start_idx,
            num_vals = num_vals,
            expected = ?expected
        );

        let mut val = String::new();
        let mut value_space = String::new();
        let mut s = Status::ok();

        let mut i: i32 = start_idx;
        while i < (start_idx + num_vals) && s.is_ok() {
            let expected_value = self
                .value(i, (&mut value_space) as *mut String)
                .to_string();

            s = self.read_value(i, (&mut val) as *mut String);

            match expected {
                ExpectedVerifResult::VAL_EXPECT_NO_ERROR => {
                    if s.is_ok() {
                        assert_eq!(expected_value, val);
                    }
                }
                ExpectedVerifResult::VAL_EXPECT_ERROR => {
                    if s.is_ok() {
                        eprintln!("Expected an error at {}, but was OK", i);

                        let dbname_slice = Slice::from(&self.dbname);
                        let msg = "Expected value error:".to_string();
                        let msg_slice = Slice::from(&msg);
                        s = Status::io_error(&dbname_slice, Some(&msg_slice));
                    } else {
                        s = Status::ok();  // An expected error
                    }
                }
            }

            i += 1i32;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_verify_exit",
            ok = s.is_ok()
        );

        s
    }

    /**
      | Return the ith key
      |
      */
    pub fn key(
        &self,
        i:       i32,
        storage: *mut String,
    ) -> Slice {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_key_entry",
            i = i,
            storage_is_null = storage.is_null()
        );

        if storage.is_null() {
            error!(
                target: "bitcoinleveldb_test::fault_injection_test",
                event = "fault_injection_test_key_null_storage",
                i = i
            );

            return Slice::from_ptr_len(core::ptr::null::<u8>(), 0usize);
        }

        let formatted = format!("{:016}", i);

        unsafe {
            (*storage).clear();
            (*storage).push_str(formatted.as_str());
        }

        let out = unsafe { Slice::from(&*storage) };

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_key_exit",
            key_len = formatted.len()
        );

        out
    }

    /**
      | Return the value to associate with the
      | specified key
      |
      */
    pub fn value(
        &self,
        k:       i32,
        storage: *mut String,
    ) -> Slice {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_value_entry",
            k = k,
            storage_is_null = storage.is_null()
        );

        let mut rnd = Random::new(k as u32);
        let out = random_string(
            (&mut rnd) as *mut Random,
            VALUE_SIZE,
            storage,
        );

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_value_exit",
            k = k
        );

        out
    }

    pub fn opendb(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_opendb_entry",
            env_is_null = self.env.is_null(),
            db_is_null = self.db.is_null()
        );

        self.closedb();

        unsafe {
            (&mut *self.env).reset_state();
        }

        let mut opener = DBImpl::new(&self.options, &self.dbname);
        let status = opener.open(
            &self.options,
            &self.dbname,
            (&mut self.db) as *mut *mut dyn DB,
        );

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_opendb_exit",
            ok = status.is_ok(),
            db_is_null = self.db.is_null()
        );

        status
    }

    pub fn closedb(&mut self) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_closedb_entry",
            db_is_null = self.db.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_closedb_exit",
            db_is_null = self.db.is_null()
        );
    }

    pub fn delete_all_data(&mut self) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_delete_all_data_entry",
            db_is_null = self.db.is_null()
        );

        let mut keys: Vec<String> = Vec::new();

        let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
        assert!(!iter_ptr.is_null());

        {
            let iter = unsafe { &mut *iter_ptr };
            iter.seek_to_first();

            while iter.valid() {
                keys.push(iter.key().to_string());
                iter.next();
            }
        }

        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        for key in keys.iter() {
            let status = unsafe {
                (&mut *self.db).delete(
                    &WriteOptions::default(),
                    &Slice::from(key),
                )
            };
            assert!(status.is_ok());
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_delete_all_data_exit",
            deleted = keys.len()
        );
    }
    
    pub fn reset_db_state(&mut self, reset_method: ResetMethod) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_reset_db_state_entry",
            env_is_null = self.env.is_null()
        );

        assert!(!self.env.is_null());

        let status = match reset_method {
            ResetMethod::RESET_DROP_UNSYNCED_DATA => unsafe {
                (&mut *self.env).drop_unsynced_file_data()
            },
            ResetMethod::RESET_DELETE_UNSYNCED_FILES => unsafe {
                (&mut *self.env).delete_files_created_after_last_dir_sync()
            },
        };

        assert!(status.is_ok());

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_reset_db_state_exit",
            ok = status.is_ok()
        );
    }

    pub fn partial_compact_test_pre_fault(
        &mut self,
        num_pre_sync:  i32,
        num_post_sync: i32,
    ) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_partial_compact_test_pre_fault_entry",
            num_pre_sync = num_pre_sync,
            num_post_sync = num_post_sync
        );

        self.delete_all_data();
        self.build(0, num_pre_sync);
        unsafe {
            (&mut *self.db).compact_range(core::ptr::null(), core::ptr::null());
        }
        self.build(num_pre_sync, num_post_sync);

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_partial_compact_test_pre_fault_exit"
        );
    }

    pub fn partial_compact_test_reopen_with_fault(
        &mut self,
        reset_method:  ResetMethod,
        num_pre_sync:  i32,
        num_post_sync: i32,
    ) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_partial_compact_test_reopen_with_fault_entry",
            num_pre_sync = num_pre_sync,
            num_post_sync = num_post_sync,
            reset_method = ?reset_method
        );

        unsafe {
            (&mut *self.env).set_filesystem_active(false);
        }

        self.closedb();
        self.reset_db_state(reset_method);

        let open_status = self.opendb();
        assert!(open_status.is_ok());

        let verify_pre = self.verify(
            0,
            num_pre_sync,
            ExpectedVerifResult::VAL_EXPECT_NO_ERROR,
        );
        assert!(verify_pre.is_ok());

        let verify_post = self.verify(
            num_pre_sync,
            num_post_sync,
            ExpectedVerifResult::VAL_EXPECT_ERROR,
        );
        assert!(verify_post.is_ok());

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_partial_compact_test_reopen_with_fault_exit"
        );
    }
   
    pub fn no_write_test_pre_fault(&mut self)  { }

    pub fn no_write_test_reopen_with_fault(&mut self, reset_method: ResetMethod) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_no_write_test_reopen_with_fault_entry",
            reset_method = ?reset_method
        );

        self.closedb();
        self.reset_db_state(reset_method);

        let open_status = self.opendb();
        assert!(open_status.is_ok());

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_no_write_test_reopen_with_fault_exit"
        );
    }

    pub fn do_test(&mut self) {
        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_do_test_entry"
        );

        let mut rnd = Random::new(0u32);

        let open_status = self.opendb();
        assert!(open_status.is_ok());

        let mut idx: usize = 0usize;
        while idx < NUM_ITERATIONS {
            let num_pre_sync = rnd.uniform(MAX_NUM_VALUES);
            let num_post_sync = rnd.uniform(MAX_NUM_VALUES);

            self.partial_compact_test_pre_fault(num_pre_sync.try_into().unwrap(), num_post_sync.try_into().unwrap());
            self.partial_compact_test_reopen_with_fault(
                ResetMethod::RESET_DROP_UNSYNCED_DATA,
                num_pre_sync.try_into().unwrap(),
                num_post_sync.try_into().unwrap(),
            );

            self.no_write_test_pre_fault();
            self.no_write_test_reopen_with_fault(
                ResetMethod::RESET_DROP_UNSYNCED_DATA,
            );

            self.partial_compact_test_pre_fault(num_pre_sync.try_into().unwrap(), num_post_sync.try_into().unwrap());
            // No new files created so we expect all values since no files will be
            // dropped.
            self.partial_compact_test_reopen_with_fault(
                ResetMethod::RESET_DELETE_UNSYNCED_FILES,
                (num_pre_sync + num_post_sync).try_into().unwrap(),
                0,
            );

            self.no_write_test_pre_fault();
            self.no_write_test_reopen_with_fault(
                ResetMethod::RESET_DELETE_UNSYNCED_FILES,
            );

            idx += 1usize;
        }

        trace!(
            target: "bitcoinleveldb_test::fault_injection_test",
            event = "fault_injection_test_do_test_exit"
        );
    }
}

#[traced_test]
fn fault_injection_test_no_log_reuse() {
    let mut t = FaultInjectionTest::default();
    t.reuse_logs(false);
    t.do_test();
}

#[traced_test]
fn fault_injection_test_with_log_reuse() {
    let mut t = FaultInjectionTest::default();
    t.reuse_logs(true);
    t.do_test();
}

fn dbfault_injection_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "dbfault_injection_test_main_entry"
    );

    let rc = run_all_tests();

    trace!(
        target: "bitcoinleveldb_test::fault_injection_test",
        event = "dbfault_injection_test_main_exit",
        result = rc
    );

    rc
}
