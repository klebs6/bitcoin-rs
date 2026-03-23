// ---------------- [ File: bitcoinleveldbt-faultinjection/src/fault_injection_test_env.rs ]
crate::ix!();

pub struct FaultInjectionTestEnv {
    base:  EnvWrapper,
    mutex: Mutex<fault_injection_test_env::Inner>,
}

impl Default for FaultInjectionTestEnv {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_default_entry"
        );

        let out = Self {
            base: EnvWrapper::new(posix_default_env()),
            mutex: Mutex::new(fault_injection_test_env::Inner::default()),
        };

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_rename_file_trait_entry",
            source = %src,
            target = %target
        );

        let ret = self.base.rename_file(src, target);

        if ret.is_ok() {
            let mut guard = self.mutex.lock();

            if let Some(state) = guard.db_file_state.remove(src) {
                let file_state = 
                    FileStateBuilder::default()
                    .filename(target.clone())
                    .pos(*state.pos())
                    .pos_at_last_sync(*state.pos_at_last_sync())
                    .pos_at_last_flush(*state.pos_at_last_flush())
                    .build()
                    .unwrap();

                guard.db_file_state.insert(
                    target.clone(),
                    file_state
                );
            }

            if guard.new_files_since_last_dir_sync.remove(src) {
                guard.new_files_since_last_dir_sync.insert(target.clone());
            }
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_inner_default_entry"
        );

        let out = Self {
            db_file_state: HashMap::new(),
            new_files_since_last_dir_sync: HashSet::new(),
            filesystem_active: true,
        };

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_is_filesystem_active_entry"
        );

        let guard = self.mutex.lock();
        let out = guard.filesystem_active;

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_is_filesystem_active_exit",
            result = out
        );

        out
    }

    #[LOCKS_EXCLUDED(mutex_)]
    pub fn set_filesystem_active(&mut self, active: bool) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_set_filesystem_active_entry",
            active = active
        );

        let mut guard = self.mutex.lock();
        guard.filesystem_active = active;

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_set_filesystem_active_exit",
            active = active
        );
    }
}

impl TestWritableFile {

    pub fn sync_parent(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_sync_parent_entry",
            filename = %self.state().filename()
        );

        let parent_dir = get_dir_name(&self.state().filename());
        let s = sync_dir(&parent_dir);

        if s.is_ok() {
            unsafe {
                (&mut **self.env()).dir_was_synced();
            }
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "test_writable_file_sync_parent_exit",
            filename = %self.state().filename(),
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            state.set_pos(0);

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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            state.set_pos(0);

            {
                let mut guard = self.mutex.lock();

                if !guard.db_file_state.contains_key(fname) {
                    guard.new_files_since_last_dir_sync.insert(fname.clone());
                } else if let Some(existing_state) = guard.db_file_state.get(fname) {
                    state = FileStateBuilder::default()
                        .filename(existing_state.filename().clone())
                        .pos(*existing_state.pos())
                        .pos_at_last_sync(*existing_state.pos_at_last_sync())
                        .pos_at_last_flush(*existing_state.pos_at_last_flush())
                        .build()
                        .unwrap();
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_drop_unsynced_file_data_exit",
            ok = status.is_ok()
        );

        status
    }

    pub fn dir_was_synced(&mut self) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_dir_was_synced_entry"
        );

        let mut guard = self.mutex.lock();
        guard.new_files_since_last_dir_sync.clear();

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_dir_was_synced_exit"
        );
    }
 
    pub fn is_file_created_since_last_dir_sync(&mut self, filename: &String) -> bool {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_is_file_created_since_last_dir_sync_entry",
            filename = %filename
        );

        let guard = self.mutex.lock();
        let out = guard.new_files_since_last_dir_sync.contains(filename);

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_is_file_created_since_last_dir_sync_exit",
            filename = %filename,
            result = out
        );

        out
    }

    pub fn untrack_file(&mut self, f: &String) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_untrack_file_entry",
            filename = %f
        );

        let mut guard = self.mutex.lock();
        guard.db_file_state.remove(f);
        guard.new_files_since_last_dir_sync.remove(f);

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_untrack_file_exit",
            filename = %f
        );
    }

    pub fn delete_file(&mut self, f: &String) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_delete_file_entry",
            filename = %f
        );

        let status = self.base.delete_file(f);
        assert!(status.is_ok());

        if status.is_ok() {
            self.untrack_file(f);
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_rename_file_entry",
            source = %s,
            target = %t
        );

        let ret = self.base.rename_file(s, t);

        if ret.is_ok() {
            let mut guard = self.mutex.lock();

            if let Some(state) = guard.db_file_state.remove(s) {
                let file_state = FileStateBuilder::default()
                    .filename(t.clone())
                    .pos(*state.pos())
                    .pos_at_last_sync(*state.pos_at_last_sync())
                    .pos_at_last_flush(*state.pos_at_last_flush())
                    .build()
                    .unwrap();

                guard.db_file_state.insert(
                    t.clone(),
                    file_state
                );
            }

            if guard.new_files_since_last_dir_sync.remove(s) {
                guard.new_files_since_last_dir_sync.insert(t.clone());
            }
        }

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_rename_file_exit",
            source = %s,
            target = %t,
            ok = ret.is_ok()
        );

        ret
    }

    pub fn reset_state(&mut self) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_reset_state_entry"
        );

        // Since we are not destroying the database, the existing files
        // should keep their recorded synced/flushed state. Therefore
        // we do not reset db_file_state_ and new_files_since_last_dir_sync_.
        self.set_filesystem_active(true);

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_reset_state_exit"
        );
    }

    pub fn delete_files_created_after_last_dir_sync(&mut self) -> crate::Status {
        // Because DeleteFile access this container make a copy to avoid deadlock
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_delete_files_created_after_last_dir_sync_exit",
            ok = status.is_ok()
        );

        status
    }

    pub fn writable_file_closed(&mut self, state: &FileState) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_writable_file_closed_entry",
            filename = %state.filename()
        );

        let mut guard = self.mutex.lock();
        let file_state = FileStateBuilder::default()
            .filename(state.filename().clone())
            .pos(*state.pos())
            .pos_at_last_sync(*state.pos_at_last_sync())
            .pos_at_last_flush(*state.pos_at_last_flush())
            .build()
            .unwrap();

        guard.db_file_state.insert(
            state.filename().clone(),
            file_state
        );

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_env_writable_file_closed_exit",
            filename = %state.filename()
        );
    }
}
