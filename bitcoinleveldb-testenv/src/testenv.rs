// ---------------- [ File: bitcoinleveldb-testenv/src/testenv.rs ]
crate::ix!();

/**
  | Test Env to override default Env behavior
  | for testing.
  |
  */
pub struct TestEnv {
    base:             EnvWrapper,
    ignore_dot_files: bool,
}

impl TestEnv {
    pub fn new(base: Rc<RefCell<dyn Env>>) -> Self {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            "TestEnv::new: constructing test environment wrapper"
        );

        let env_wrapper = EnvWrapper::new(base);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            ignore_dot_files = false,
            "TestEnv::new: constructed"
        );

        Self {
            base: env_wrapper,
            ignore_dot_files: false,
        }
    }

    pub fn set_ignore_dot_files(&mut self, ignored: bool) {
        let prev = self.ignore_dot_files;

        self.ignore_dot_files = ignored;

        info!(
            target: "bitcoinleveldb_testenv::testenv",
            previous = prev,
            current = self.ignore_dot_files,
            "TestEnv::set_ignore_dot_files: updated configuration"
        );
    }

    pub fn get_children(&mut self, dir: &String, result: *mut Vec<String>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %dir,
            ignore_dot_files = self.ignore_dot_files,
            "TestEnv::get_children: entering"
        );

        let s = self.base.get_children(dir, result);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %dir,
            ok = s.is_ok(),
            code = ?s.code(),
            ignore_dot_files = self.ignore_dot_files,
            "TestEnv::get_children: target GetChildren returned"
        );

        if !s.is_ok() || !self.ignore_dot_files {
            trace!(
                target: "bitcoinleveldb_testenv::testenv",
                dir = %dir,
                ok = s.is_ok(),
                ignore_dot_files = self.ignore_dot_files,
                "TestEnv::get_children: returning without filtering"
            );
            return s;
        }

        unsafe {
            let v: &mut Vec<String> = &mut *result;

            trace!(
                target: "bitcoinleveldb_testenv::testenv",
                dir = %dir,
                initial_len = v.len(),
                "TestEnv::get_children: filtering dot entries"
            );

            let mut i: usize = 0;
            while i != v.len() {
                if (v[i] == ".") || (v[i] == "..") {
                    trace!(
                        target: "bitcoinleveldb_testenv::testenv",
                        dir = %dir,
                        entry = %v[i],
                        index = i,
                        "TestEnv::get_children: removing dot entry"
                    );
                    v.remove(i);
                } else {
                    i += 1;
                }
            }

            debug!(
                target: "bitcoinleveldb_testenv::testenv",
                dir = %dir,
                final_len = v.len(),
                "TestEnv::get_children: finished filtering dot entries"
            );
        }

        s
    }
}

impl std::ops::Deref for TestEnv {
    type Target = EnvWrapper;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for TestEnv {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl GetChildren for TestEnv {
    fn get_children(&mut self, dir: &String, r: *mut Vec<String>) -> Status {
        TestEnv::get_children(self, dir, r)
    }
}

impl DeleteFile for TestEnv {
    fn delete_file(&mut self, f: &String) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            "TestEnv::delete_file: delegating"
        );

        let s = self.base.delete_file(f);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::delete_file: delegated"
        );

        s
    }
}

impl CreateDir for TestEnv {
    fn create_dir(&mut self, d: &String) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %d,
            "TestEnv::create_dir: delegating"
        );

        let s = self.base.create_dir(d);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %d,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::create_dir: delegated"
        );

        s
    }
}

impl DeleteDir for TestEnv {
    fn delete_dir(&mut self, d: &String) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %d,
            "TestEnv::delete_dir: delegating"
        );

        let s = self.base.delete_dir(d);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            dir = %d,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::delete_dir: delegated"
        );

        s
    }
}

impl NewSequentialFile for TestEnv {
    fn new_sequential_file(&mut self, f: &String, r: *mut *mut Box<dyn SequentialFile>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            result_is_null = r.is_null(),
            "TestEnv::new_sequential_file: delegating"
        );

        let s = self.base.new_sequential_file(f, r);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::new_sequential_file: delegated"
        );

        s
    }
}

impl NewRandomAccessFile for TestEnv {
    fn new_random_access_file(
        &mut self,
        f: &String,
        r: *mut *mut Box<dyn RandomAccessFile>,
    ) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            result_is_null = r.is_null(),
            "TestEnv::new_random_access_file: delegating"
        );

        let s = self.base.new_random_access_file(f, r);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::new_random_access_file: delegated"
        );

        s
    }
}

impl NewWritableFile for TestEnv {
    fn new_writable_file(&mut self, f: &String, r: *mut *mut Box<dyn WritableFile>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            result_is_null = r.is_null(),
            "TestEnv::new_writable_file: delegating"
        );

        let s = self.base.new_writable_file(f, r);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::new_writable_file: delegated"
        );

        s
    }
}

impl NewAppendableFile for TestEnv {
    fn new_appendable_file(&mut self, f: &String, r: *mut *mut Box<dyn WritableFile>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            result_is_null = r.is_null(),
            "TestEnv::new_appendable_file: delegating"
        );

        let s = self.base.new_appendable_file(f, r);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = s.is_ok(),
            code = ?s.code(),
            "TestEnv::new_appendable_file: delegated"
        );

        s
    }
}

impl FileExists for TestEnv {
    fn file_exists(&mut self, f: &String) -> bool {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            "TestEnv::file_exists: delegating"
        );

        let exists = self.base.file_exists(f);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            exists,
            "TestEnv::file_exists: delegated"
        );

        exists
    }
}

impl GetFileSize for TestEnv {
    fn get_file_size(&mut self, f: &String, s: *mut u64) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            out_is_null = s.is_null(),
            "TestEnv::get_file_size: delegating"
        );

        let st = self.base.get_file_size(f, s);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::get_file_size: delegated"
        );

        st
    }
}

impl RenameFile for TestEnv {
    fn rename_file(&mut self, src: &String, target: &String) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            src = %src,
            target = %target,
            "TestEnv::rename_file: delegating"
        );

        let st = self.base.rename_file(src, target);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            src = %src,
            target = %target,
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::rename_file: delegated"
        );

        st
    }
}

impl LockFile for TestEnv {
    fn lock_file(&mut self, f: &String, l: *mut *mut Box<dyn FileLock>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            out_is_null = l.is_null(),
            "TestEnv::lock_file: delegating"
        );

        let st = self.base.lock_file(f, l);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %f,
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::lock_file: delegated"
        );

        st
    }
}

impl UnlockFile for TestEnv {
    fn unlock_file(&mut self, l: *mut Box<dyn FileLock>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            lock_is_null = l.is_null(),
            "TestEnv::unlock_file: delegating"
        );

        let st = self.base.unlock_file(l);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::unlock_file: delegated"
        );

        st
    }
}

impl Schedule for TestEnv {
    fn schedule(
        &mut self,
        function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        arg: *mut std::ffi::c_void,
    ) {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            arg_is_null = arg.is_null(),
            "TestEnv::schedule: delegating"
        );

        self.base.schedule(function, arg);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            "TestEnv::schedule: delegated"
        );
    }
}

impl StartThread for TestEnv {
    fn start_thread(
        &mut self,
        function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
        arg: *mut std::ffi::c_void,
    ) {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            arg_is_null = arg.is_null(),
            "TestEnv::start_thread: delegating"
        );

        self.base.start_thread(function, arg);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            "TestEnv::start_thread: delegated"
        );
    }
}

impl GetTestDirectory for TestEnv {
    fn get_test_directory(&mut self, path: *mut String) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            out_is_null = path.is_null(),
            "TestEnv::get_test_directory: delegating"
        );

        let st = self.base.get_test_directory(path);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::get_test_directory: delegated"
        );

        st
    }
}

impl NewLogger for TestEnv {
    fn new_logger(&mut self, fname: &String, result: *mut *mut Box<dyn Logger>) -> Status {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %fname,
            out_is_null = result.is_null(),
            "TestEnv::new_logger: delegating"
        );

        let st = self.base.new_logger(fname, result);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            file = %fname,
            ok = st.is_ok(),
            code = ?st.code(),
            "TestEnv::new_logger: delegated"
        );

        st
    }
}

impl NowMicros for TestEnv {
    fn now_micros(&mut self) -> u64 {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            "TestEnv::now_micros: delegating"
        );

        let v = self.base.now_micros();

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            now_micros = v,
            "TestEnv::now_micros: delegated"
        );

        v
    }
}

impl SleepForMicroseconds for TestEnv {
    fn sleep_for_microseconds(&mut self, micros: i32) {
        trace!(
            target: "bitcoinleveldb_testenv::testenv",
            micros,
            "TestEnv::sleep_for_microseconds: delegating"
        );

        self.base.sleep_for_microseconds(micros);

        debug!(
            target: "bitcoinleveldb_testenv::testenv",
            micros,
            "TestEnv::sleep_for_microseconds: delegated"
        );
    }
}

impl Env for TestEnv {}

#[cfg(test)]
mod testenv_exhaustive_behavior_suite {
    use super::*;

    #[derive(Debug, Clone)]
    struct GetChildrenScriptStep {
        status: Status,
        children: Vec<String>,
    }

    impl GetChildrenScriptStep {
        fn ok(children: &[&str]) -> Self {
            let status = Status::ok();
            let v: Vec<String> = children.iter().map(|s| (*s).to_string()).collect();

            debug!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                ok = status.is_ok(),
                code = ?status.code(),
                children_len = v.len(),
                "GetChildrenScriptStep::ok"
            );

            Self { status, children: v }
        }

        fn with_status(status: Status, children: &[&str]) -> Self {
            let v: Vec<String> = children.iter().map(|s| (*s).to_string()).collect();

            debug!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                ok = status.is_ok(),
                code = ?status.code(),
                children_len = v.len(),
                "GetChildrenScriptStep::with_status"
            );

            Self { status, children: v }
        }
    }

    #[derive(Debug)]
    struct ScriptedEnvState {
        get_children_steps: Vec<GetChildrenScriptStep>,
        get_children_calls: usize,
        get_children_dirs: Vec<String>,

        delete_file_calls: Vec<String>,
        create_dir_calls: Vec<String>,
        delete_dir_calls: Vec<String>,

        file_exists_calls: Vec<String>,
        file_exists_result: bool,

        get_file_size_calls: Vec<String>,
        get_file_size_value: u64,

        rename_file_calls: Vec<(String, String)>,

        lock_file_calls: Vec<String>,
        unlock_file_calls: usize,

        new_sequential_file_calls: Vec<String>,
        new_random_access_file_calls: Vec<String>,
        new_writable_file_calls: Vec<String>,
        new_appendable_file_calls: Vec<String>,

        schedule_calls: usize,
        start_thread_calls: usize,

        get_test_directory_calls: usize,
        test_directory_value: String,

        new_logger_calls: Vec<String>,

        now_micros_calls: usize,
        now_micros_value: u64,

        sleep_calls: Vec<i32>,
    }

    impl ScriptedEnvState {
        fn new(get_children_steps: Vec<GetChildrenScriptStep>) -> Self {
            info!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                steps_len = get_children_steps.len(),
                "ScriptedEnvState::new"
            );

            Self {
                get_children_steps,
                get_children_calls: 0,
                get_children_dirs: Vec::new(),

                delete_file_calls: Vec::new(),
                create_dir_calls: Vec::new(),
                delete_dir_calls: Vec::new(),

                file_exists_calls: Vec::new(),
                file_exists_result: false,

                get_file_size_calls: Vec::new(),
                get_file_size_value: 0,

                rename_file_calls: Vec::new(),

                lock_file_calls: Vec::new(),
                unlock_file_calls: 0,

                new_sequential_file_calls: Vec::new(),
                new_random_access_file_calls: Vec::new(),
                new_writable_file_calls: Vec::new(),
                new_appendable_file_calls: Vec::new(),

                schedule_calls: 0,
                start_thread_calls: 0,

                get_test_directory_calls: 0,
                test_directory_value: "testdir".to_string(),

                new_logger_calls: Vec::new(),

                now_micros_calls: 0,
                now_micros_value: 123,

                sleep_calls: Vec::new(),
            }
        }

        fn set_file_exists_result(&mut self, v: bool) {
            self.file_exists_result = v;
        }

        fn set_get_file_size_value(&mut self, v: u64) {
            self.get_file_size_value = v;
        }

        fn set_test_directory_value(&mut self, v: &str) {
            self.test_directory_value = v.to_string();
        }

        fn set_now_micros_value(&mut self, v: u64) {
            self.now_micros_value = v;
        }
    }

    #[derive(Debug)]
    struct ScriptedEnv {
        state: Rc<RefCell<ScriptedEnvState>>,
    }

    impl ScriptedEnv {
        fn new(state: Rc<RefCell<ScriptedEnvState>>) -> Self {
            debug!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                "ScriptedEnv::new"
            );
            Self { state }
        }

        fn next_get_children_step_locked(
            st: &mut ScriptedEnvState,
        ) -> GetChildrenScriptStep {
            let call_index = st.get_children_calls;
            let steps_len = st.get_children_steps.len();

            if steps_len == 0 {
                return GetChildrenScriptStep::ok(&[]);
            }

            let step_index = call_index.min(steps_len.saturating_sub(1));
            st.get_children_steps[step_index].clone()
        }
    }

    impl Env for ScriptedEnv {}

    impl GetChildren for ScriptedEnv {
        fn get_children(&mut self, dir: &String, result: *mut Vec<String>) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                dir = %dir,
                result_is_null = result.is_null(),
                "ScriptedEnv::get_children: begin"
            );

            if result.is_null() {
                let st = Status::invalid_argument(&"null get_children result".into(), None);

                warn!(
                    target: "bitcoinleveldb_testenv::tests::fixture",
                    dir = %dir,
                    ok = st.is_ok(),
                    code = ?st.code(),
                    "ScriptedEnv::get_children: null result pointer"
                );

                return st;
            }

            let step = {
                let mut st = self.state.borrow_mut();
                st.get_children_dirs.push(dir.clone());
                let step = ScriptedEnv::next_get_children_step_locked(&mut st);
                st.get_children_calls += 1;
                step
            };

            unsafe {
                let out: &mut Vec<String> = &mut *result;
                out.clear();
                out.extend(step.children.iter().cloned());
            }

            debug!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                dir = %dir,
                ok = step.status.is_ok(),
                code = ?step.status.code(),
                "ScriptedEnv::get_children: filled result"
            );

            Status::new_from_other_copy(&step.status)
        }
    }

    impl DeleteFile for ScriptedEnv {
        fn delete_file(&mut self, fname: &String) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                "ScriptedEnv::delete_file"
            );

            self.state.borrow_mut().delete_file_calls.push(fname.clone());

            Status::not_found(&"delete_file".into(), None)
        }
    }

    impl CreateDir for ScriptedEnv {
        fn create_dir(&mut self, dirname: &String) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                dir = %dirname,
                "ScriptedEnv::create_dir"
            );

            self.state.borrow_mut().create_dir_calls.push(dirname.clone());

            Status::ok()
        }
    }

    impl DeleteDir for ScriptedEnv {
        fn delete_dir(&mut self, dirname: &String) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                dir = %dirname,
                "ScriptedEnv::delete_dir"
            );

            self.state.borrow_mut().delete_dir_calls.push(dirname.clone());

            Status::not_supported(&"delete_dir".into(), None)
        }
    }

    impl NewSequentialFile for ScriptedEnv {
        fn new_sequential_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn SequentialFile>,
        ) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                result_is_null = result.is_null(),
                "ScriptedEnv::new_sequential_file"
            );

            self.state
                .borrow_mut()
                .new_sequential_file_calls
                .push(fname.clone());

            if result.is_null() {
                return Status::invalid_argument(&"null sequential out".into(), None);
            }

            unsafe {
                *result = std::ptr::null_mut();
            }

            Status::not_supported(&"new_sequential_file".into(), None)
        }
    }

    impl NewRandomAccessFile for ScriptedEnv {
        fn new_random_access_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                result_is_null = result.is_null(),
                "ScriptedEnv::new_random_access_file"
            );

            self.state
                .borrow_mut()
                .new_random_access_file_calls
                .push(fname.clone());

            if result.is_null() {
                return Status::invalid_argument(&"null random-access out".into(), None);
            }

            unsafe {
                *result = std::ptr::null_mut();
            }

            Status::not_supported(&"new_random_access_file".into(), None)
        }
    }

    impl NewWritableFile for ScriptedEnv {
        fn new_writable_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                result_is_null = result.is_null(),
                "ScriptedEnv::new_writable_file"
            );

            self.state
                .borrow_mut()
                .new_writable_file_calls
                .push(fname.clone());

            if result.is_null() {
                return Status::invalid_argument(&"null writable out".into(), None);
            }

            unsafe {
                *result = std::ptr::null_mut();
            }

            Status::not_supported(&"new_writable_file".into(), None)
        }
    }

    impl NewAppendableFile for ScriptedEnv {
        fn new_appendable_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                result_is_null = result.is_null(),
                "ScriptedEnv::new_appendable_file"
            );

            self.state
                .borrow_mut()
                .new_appendable_file_calls
                .push(fname.clone());

            if result.is_null() {
                return Status::invalid_argument(&"null appendable out".into(), None);
            }

            unsafe {
                *result = std::ptr::null_mut();
            }

            Status::not_supported(&"new_appendable_file".into(), None)
        }
    }

    impl FileExists for ScriptedEnv {
        fn file_exists(&mut self, fname: &String) -> bool {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                "ScriptedEnv::file_exists"
            );

            let mut st = self.state.borrow_mut();
            st.file_exists_calls.push(fname.clone());
            st.file_exists_result
        }
    }

    impl GetFileSize for ScriptedEnv {
        fn get_file_size(&mut self, fname: &String, file_size: *mut u64) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                out_is_null = file_size.is_null(),
                "ScriptedEnv::get_file_size"
            );

            let v = {
                let mut st = self.state.borrow_mut();
                st.get_file_size_calls.push(fname.clone());
                st.get_file_size_value
            };

            if file_size.is_null() {
                return Status::invalid_argument(&"null file_size out".into(), None);
            }

            unsafe {
                *file_size = v;
            }

            Status::ok()
        }
    }

    impl RenameFile for ScriptedEnv {
        fn rename_file(&mut self, src: &String, target: &String) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                src = %src,
                target = %target,
                "ScriptedEnv::rename_file"
            );

            self.state
                .borrow_mut()
                .rename_file_calls
                .push((src.clone(), target.clone()));

            Status::invalid_argument(&"rename_file".into(), None)
        }
    }

    impl LockFile for ScriptedEnv {
        fn lock_file(&mut self, fname: &String, lock: *mut *mut Box<dyn FileLock>) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                out_is_null = lock.is_null(),
                "ScriptedEnv::lock_file"
            );

            self.state.borrow_mut().lock_file_calls.push(fname.clone());

            if lock.is_null() {
                return Status::invalid_argument(&"null lock out".into(), None);
            }

            unsafe {
                *lock = std::ptr::null_mut();
            }

            Status::ok()
        }
    }

    impl UnlockFile for ScriptedEnv {
        fn unlock_file(&mut self, _lock: *mut Box<dyn FileLock>) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                "ScriptedEnv::unlock_file"
            );

            self.state.borrow_mut().unlock_file_calls += 1;

            Status::ok()
        }
    }

    impl Schedule for ScriptedEnv {
        fn schedule(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            arg: *mut std::ffi::c_void,
        ) {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                arg_is_null = arg.is_null(),
                "ScriptedEnv::schedule"
            );

            self.state.borrow_mut().schedule_calls += 1;
        }
    }

    impl StartThread for ScriptedEnv {
        fn start_thread(
            &mut self,
            _function: fn(arg: *mut std::ffi::c_void) -> std::ffi::c_void,
            arg: *mut std::ffi::c_void,
        ) {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                arg_is_null = arg.is_null(),
                "ScriptedEnv::start_thread"
            );

            self.state.borrow_mut().start_thread_calls += 1;
        }
    }

    impl GetTestDirectory for ScriptedEnv {
        fn get_test_directory(&mut self, path: *mut String) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                out_is_null = path.is_null(),
                "ScriptedEnv::get_test_directory"
            );

            let v = {
                let mut st = self.state.borrow_mut();
                st.get_test_directory_calls += 1;
                st.test_directory_value.clone()
            };

            if path.is_null() {
                return Status::invalid_argument(&"null testdir out".into(), None);
            }

            unsafe {
                *path = v;
            }

            Status::ok()
        }
    }

    impl NewLogger for ScriptedEnv {
        fn new_logger(&mut self, fname: &String, result: *mut *mut Box<dyn Logger>) -> Status {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                file = %fname,
                out_is_null = result.is_null(),
                "ScriptedEnv::new_logger"
            );

            self.state.borrow_mut().new_logger_calls.push(fname.clone());

            if result.is_null() {
                return Status::invalid_argument(&"null logger out".into(), None);
            }

            unsafe {
                *result = std::ptr::null_mut();
            }

            Status::not_supported(&"new_logger".into(), None)
        }
    }

    impl NowMicros for ScriptedEnv {
        fn now_micros(&mut self) -> u64 {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                "ScriptedEnv::now_micros"
            );

            let mut st = self.state.borrow_mut();
            st.now_micros_calls += 1;
            st.now_micros_value
        }
    }

    impl SleepForMicroseconds for ScriptedEnv {
        fn sleep_for_microseconds(&mut self, micros: i32) {
            trace!(
                target: "bitcoinleveldb_testenv::tests::fixture",
                micros,
                "ScriptedEnv::sleep_for_microseconds"
            );

            self.state.borrow_mut().sleep_calls.push(micros);
        }
    }

    fn scripted_env_as_dyn_env(
        steps: Vec<GetChildrenScriptStep>,
    ) -> (Rc<RefCell<dyn Env>>, Rc<RefCell<ScriptedEnvState>>) {
        let state = Rc::new(RefCell::new(ScriptedEnvState::new(steps)));
        let env = ScriptedEnv::new(state.clone());

        let rc: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env));

        (rc, state)
    }

    fn invoke_get_children(env: &mut TestEnv, dir: &str) -> (Status, Vec<String>) {
        let dir_string = dir.to_string();
        let mut out: Vec<String> = Vec::new();

        trace!(
            target: "bitcoinleveldb_testenv::tests",
            dir = %dir_string,
            "invoke_get_children: calling TestEnv::get_children"
        );

        let s = env.get_children(&dir_string, &mut out as *mut Vec<String>);
        (s, out)
    }

    fn invoke_get_children_via_trait(env: &mut TestEnv, dir: &str) -> (Status, Vec<String>) {
        let dir_string = dir.to_string();
        let mut out: Vec<String> = Vec::new();

        trace!(
            target: "bitcoinleveldb_testenv::tests",
            dir = %dir_string,
            "invoke_get_children_via_trait: calling GetChildren::get_children"
        );

        let s = GetChildren::get_children(env, &dir_string, &mut out as *mut Vec<String>);
        (s, out)
    }

    fn never_returning_callback(_arg: *mut std::ffi::c_void) -> std::ffi::c_void {
        loop {}
    }

    #[traced_test]
    fn testenv_defaults_do_not_filter_dot_entries_and_propagate_ok_status() {
        let step = GetChildrenScriptStep::ok(&[".", "..", "alpha", ".hidden", "beta"]);
        let (base_env, state) = scripted_env_as_dyn_env(vec![step]);

        let mut env = TestEnv::new(base_env);

        let (s, out) = invoke_get_children(&mut env, "dir0");
        assert!(s.is_ok());
        assert_eq!(
            out,
            vec![
                ".".to_string(),
                "..".to_string(),
                "alpha".to_string(),
                ".hidden".to_string(),
                "beta".to_string(),
            ]
        );

        let st = state.borrow();
        assert_eq!(st.get_children_calls, 1);
        assert_eq!(st.get_children_dirs, vec!["dir0".to_string()]);
    }

    #[traced_test]
    fn testenv_filters_only_dot_and_dotdot_when_configured_and_base_returns_ok() {
        let step = GetChildrenScriptStep::ok(&[
            ".", "..", ".", "alpha", "..", "beta", ".", ".hidden", "...", "..bar",
        ]);
        let (base_env, state) = scripted_env_as_dyn_env(vec![step]);

        let mut env = TestEnv::new(base_env);
        env.set_ignore_dot_files(true);

        let (s, out) = invoke_get_children(&mut env, "dir1");
        assert!(s.is_ok());
        assert_eq!(
            out,
            vec![
                "alpha".to_string(),
                "beta".to_string(),
                ".hidden".to_string(),
                "...".to_string(),
                "..bar".to_string(),
            ]
        );

        let st = state.borrow();
        assert_eq!(st.get_children_calls, 1);
        assert_eq!(st.get_children_dirs, vec!["dir1".to_string()]);
    }

    #[traced_test]
    fn testenv_does_not_filter_when_base_returns_error_even_if_configured_to_ignore_dots() {
        let err = Status::io_error(&"io-error".into(), None);
        let step = GetChildrenScriptStep::with_status(
            Status::new_from_other_copy(&err),
            &[".", "..", "alpha", "beta"],
        );
        let (base_env, state) = scripted_env_as_dyn_env(vec![step]);

        let mut env = TestEnv::new(base_env);
        env.set_ignore_dot_files(true);

        let (s, out) = invoke_get_children(&mut env, "dir2");
        assert!(!s.is_ok());
        assert_eq!(s.code(), StatusCode::IOError);

        assert_eq!(
            out,
            vec![
                ".".to_string(),
                "..".to_string(),
                "alpha".to_string(),
                "beta".to_string(),
            ]
        );

        let st = state.borrow();
        assert_eq!(st.get_children_calls, 1);
        assert_eq!(st.get_children_dirs, vec!["dir2".to_string()]);
    }

    #[traced_test]
    fn testenv_trait_get_children_invocation_matches_inherent_method() {
        let step = GetChildrenScriptStep::ok(&[".", "..", "alpha", "beta"]);
        let (base_env, _state) = scripted_env_as_dyn_env(vec![step]);

        let mut env = TestEnv::new(base_env);
        env.set_ignore_dot_files(true);

        let (s_inherent, out_inherent) = invoke_get_children(&mut env, "dir3");
        let (s_trait, out_trait) = invoke_get_children_via_trait(&mut env, "dir3");

        assert!(s_inherent.is_ok());
        assert!(s_trait.is_ok());
        assert_eq!(out_inherent, vec!["alpha".to_string(), "beta".to_string()]);
        assert_eq!(out_trait, vec!["alpha".to_string(), "beta".to_string()]);
    }

    #[traced_test]
    fn testenv_forwards_env_supertrait_methods_to_base_env_wrapper() {
        let (base_env, state) = scripted_env_as_dyn_env(vec![]);
        {
            let mut st = state.borrow_mut();
            st.set_file_exists_result(true);
            st.set_get_file_size_value(42);
            st.set_test_directory_value("fixture_test_dir");
            st.set_now_micros_value(999_001);
        }

        let mut env = TestEnv::new(base_env);

        let file = "some_file".to_string();
        let dir = "some_dir".to_string();

        let st_del = env.delete_file(&file);
        assert_eq!(st_del.code(), StatusCode::NotFound);

        let st_mkdir = env.create_dir(&dir);
        assert!(st_mkdir.is_ok());

        let st_rmdir = env.delete_dir(&dir);
        assert_eq!(st_rmdir.code(), StatusCode::NotSupported);

        let exists = env.file_exists(&file);
        assert!(exists);

        let now = env.now_micros();
        assert_eq!(now, 999_001);

        env.sleep_for_microseconds(1234);

        let mut file_size: u64 = 0;
        let st_size = env.get_file_size(&file, &mut file_size as *mut u64);
        assert!(st_size.is_ok());
        assert_eq!(file_size, 42);

        let mut testdir = String::new();
        let st_testdir = env.get_test_directory(&mut testdir as *mut String);
        assert!(st_testdir.is_ok());
        assert_eq!(testdir, "fixture_test_dir".to_string());

        let src = "src".to_string();
        let dst = "dst".to_string();
        let st_rename = env.rename_file(&src, &dst);
        assert_eq!(st_rename.code(), StatusCode::InvalidArgument);

        let mut lock_out: *mut Box<dyn FileLock> = std::ptr::null_mut();
        let st_lock = env.lock_file(&file, &mut lock_out as *mut *mut Box<dyn FileLock>);
        assert!(st_lock.is_ok());
        assert!(lock_out.is_null());

        env.schedule(never_returning_callback, std::ptr::null_mut());
        env.start_thread(never_returning_callback, std::ptr::null_mut());

        let mut logger_out: *mut Box<dyn Logger> = std::ptr::null_mut();
        let st_logger = env.new_logger(&file, &mut logger_out as *mut *mut Box<dyn Logger>);
        assert_eq!(st_logger.code(), StatusCode::NotSupported);
        assert!(logger_out.is_null());

        let st = state.borrow();

        assert_eq!(st.delete_file_calls, vec![file.clone()]);
        assert_eq!(st.create_dir_calls, vec![dir.clone()]);
        assert_eq!(st.delete_dir_calls, vec![dir.clone()]);

        assert_eq!(st.file_exists_calls, vec![file.clone()]);
        assert_eq!(st.get_file_size_calls, vec![file.clone()]);
        assert_eq!(st.get_test_directory_calls, 1);

        assert_eq!(st.rename_file_calls, vec![(src.clone(), dst.clone())]);
        assert_eq!(st.lock_file_calls, vec![file.clone()]);
        assert_eq!(st.unlock_file_calls, 0);

        assert_eq!(st.schedule_calls, 1);
        assert_eq!(st.start_thread_calls, 1);

        assert_eq!(st.new_logger_calls, vec![file.clone()]);

        assert_eq!(st.now_micros_calls, 1);
        assert_eq!(st.sleep_calls, vec![1234]);
    }

    #[traced_test]
    fn testenv_deref_exposes_envwrapper_target_rc_identity() {
        let step = GetChildrenScriptStep::ok(&["alpha"]);
        let (base_env, _state) = scripted_env_as_dyn_env(vec![step]);

        let base_env_clone = base_env.clone();
        let env = TestEnv::new(base_env);

        let wrapper_target = env.target();
        assert!(Rc::ptr_eq(&wrapper_target, &base_env_clone));
    }
}
