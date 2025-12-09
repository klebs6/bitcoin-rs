// ---------------- [ File: bitcoinleveldb-tablecache/src/test_support.rs ]
crate::ix!();

#[cfg(test)]
pub(crate) mod table_cache_test_support {
    use super::*;

    #[derive(Default)]
    pub struct InMemoryEnvState {
        pub files: HashMap<String, Vec<u8>>,
        pub directories: HashSet<String>,
        pub deleted_files: Vec<String>,
        pub renamed_files: Vec<(String, String)>,
        pub random_open_count: HashMap<String, usize>,
        pub sequential_open_count: HashMap<String, usize>,
        pub fail_new_writable: bool,
        pub fail_new_random_access: bool,
        pub fail_delete_file: bool,
        pub fail_rename_file: bool,
        pub fail_get_file_size: bool,
        pub reported_now_micros: u64,
    }

    #[derive(Clone)]
    pub struct InMemoryEnv {
        state: Arc<Mutex<InMemoryEnvState>>,
    }

    impl InMemoryEnv {
        pub fn new(state: Arc<Mutex<InMemoryEnvState>>) -> Self {
            trace!("InMemoryEnv::new");
            Self { state }
        }

        fn with_state<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&mut InMemoryEnvState) -> R,
        {
            let mut guard = self
                .state
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            f(&mut *guard)
        }
    }

    #[derive(Clone)]
    struct InMemoryFileHandle {
        name: String,
        state: Arc<Mutex<InMemoryEnvState>>,
        pos:   u64,
    }

    impl InMemoryFileHandle {
        fn new(name: String, state: Arc<Mutex<InMemoryEnvState>>) -> Self {
            Self { name, state, pos: 0 }
        }

        fn read_range(&self, offset: u64, n: usize) -> Vec<u8> {
            let guard = self
                .state
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            let data = guard.files.get(&self.name);
            match data {
                None => Vec::new(),
                Some(buf) => {
                    let offset_usize = offset as usize;
                    if offset_usize >= buf.len() {
                        Vec::new()
                    } else {
                        let end = core::cmp::min(buf.len(), offset_usize + n);
                        buf[offset_usize..end].to_vec()
                    }
                }
            }
        }

        fn append_bytes(&mut self, bytes: &[u8]) {
            let mut guard = self
                .state
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            let entry = guard
                .files
                .entry(self.name.clone())
                .or_insert_with(Vec::new);
            entry.extend_from_slice(bytes);
            self.pos = entry.len() as u64;
        }

        fn size(&self) -> u64 {
            let guard = self
                .state
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            guard
                .files
                .get(&self.name)
                .map(|v| v.len() as u64)
                .unwrap_or(0)
        }
    }

    impl WritableFile for InMemoryFileHandle {}

    impl WritableFileAppend for InMemoryFileHandle {
        fn append(&mut self, data: &Slice) -> Status {
            trace!(
                "InMemoryFileHandle::append: name='{}', len={}",
                self.name,
                *data.size()
            );
            unsafe {
                let len = *data.size();
                if len == 0 {
                    return Status::ok();
                }
                let ptr = *data.data();
                if ptr.is_null() {
                    return Status::ok();
                }
                let bytes = std::slice::from_raw_parts(ptr, len);
                self.append_bytes(bytes);
            }
            Status::ok()
        }
    }

    impl WritableFileClose for InMemoryFileHandle {
        fn close(&mut self) -> Status {
            trace!(
                "InMemoryFileHandle::close: name='{}', size={}",
                self.name,
                self.size()
            );
            Status::ok()
        }
    }

    impl WritableFileFlush for InMemoryFileHandle {
        fn flush(&mut self) -> Status {
            trace!(
                "InMemoryFileHandle::flush: name='{}', size={}",
                self.name,
                self.size()
            );
            Status::ok()
        }
    }

    impl WritableFileSync for InMemoryFileHandle {
        fn sync(&mut self) -> Status {
            trace!(
                "InMemoryFileHandle::sync: name='{}', size={}",
                self.name,
                self.size()
            );
            Status::ok()
        }
    }

    impl Named for InMemoryFileHandle {
        fn name(&self) -> std::borrow::Cow<'_, str> {
            std::borrow::Cow::Owned(self.name.clone())
        }
    }

    impl SequentialFile for InMemoryFileHandle {}

    impl SequentialFileRead for InMemoryFileHandle {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "InMemoryFileHandle::read (sequential): name='{}', pos={}, n={}",
                self.name,
                self.pos,
                n
            );
            let bytes = self.read_range(self.pos, n);
            let len = bytes.len();
            unsafe {
                if !scratch.is_null() && len > 0 {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), scratch, len);
                    if !result.is_null() {
                        *result = Slice::from_ptr_len(scratch, len);
                    }
                } else if !result.is_null() {
                    *result = Slice::default();
                }
            }
            self.pos = self.pos.saturating_add(len as u64);
            Status::ok()
        }
    }

    impl SequentialFileSkip for InMemoryFileHandle {
        fn skip(&mut self, n: u64) -> Status {
            trace!(
                "InMemoryFileHandle::skip: name='{}', pos={} + {}",
                self.name,
                self.pos,
                n
            );
            self.pos = self.pos.saturating_add(n);
            Status::ok()
        }
    }

    impl RandomAccessFile for InMemoryFileHandle {}

    impl RandomAccessFileRead for InMemoryFileHandle {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "InMemoryFileHandle::read (random): name='{}', offset={}, n={}",
                self.name,
                offset,
                n
            );
            let bytes = self.read_range(offset, n);
            let len = bytes.len();
            unsafe {
                if !scratch.is_null() && len > 0 {
                    std::ptr::copy_nonoverlapping(bytes.as_ptr(), scratch, len);
                    if !result.is_null() {
                        *result = Slice::from_ptr_len(scratch, len);
                    }
                } else if !result.is_null() {
                    *result = Slice::default();
                }
            }
            Status::ok()
        }
    }

    #[derive(Default)]
    struct InMemoryFileLockHandle;

    impl FileLock for InMemoryFileLockHandle {}

    struct InMemoryLogger;

    impl Logger for InMemoryLogger {
        fn logv(&mut self, format: *const u8, args: &[&str]) {
            let fmt_str = unsafe {
                if format.is_null() {
                    "<null>"
                } else {
                    match std::ffi::CStr::from_ptr(format as *const i8).to_str() {
                        Ok(s) => s,
                        Err(_) => "<non-utf8>",
                    }
                }
            };
            debug!(
                "InMemoryLogger::logv: format='{}', args={:?}",
                fmt_str,
                args
            );
        }
    }

    impl CreateDir for InMemoryEnv {
        fn create_dir(&mut self, dirname: &String) -> Status {
            trace!("InMemoryEnv::create_dir: dir='{}'", dirname);
            self.with_state(|st| {
                st.directories.insert(dirname.clone());
            });
            Status::ok()
        }
    }

    impl DeleteDir for InMemoryEnv {
        fn delete_dir(&mut self, dirname: &String) -> Status {
            trace!("InMemoryEnv::delete_dir: dir='{}'", dirname);
            self.with_state(|st| {
                st.directories.remove(dirname);
            });
            Status::ok()
        }
    }

    impl DeleteFile for InMemoryEnv {
        fn delete_file(&mut self, fname: &String) -> Status {
            trace!("InMemoryEnv::delete_file: file='{}'", fname);
            self.with_state(|st| {
                st.deleted_files.push(fname.clone());
                if st.fail_delete_file {
                    let msg = Slice::from("delete_file_fail");
                    return Status::io_error(&msg, None);
                }
                st.files.remove(fname);
                Status::ok()
            })
        }
    }

    impl FileExists for InMemoryEnv {
        fn file_exists(&mut self, fname: &String) -> bool {
            let exists = self.with_state(|st| st.files.contains_key(fname));
            trace!(
                "InMemoryEnv::file_exists: file='{}' -> {}",
                fname,
                exists
            );
            exists
        }
    }

    impl GetChildren for InMemoryEnv {
        fn get_children(
            &mut self,
            dir: &String,
            result: *mut Vec<String>,
        ) -> Status {
            trace!("InMemoryEnv::get_children: dir='{}'", dir);
            let children: Vec<String> = self.with_state(|st| {
                st.files
                    .keys()
                    .filter_map(|k| {
                        if k.starts_with(dir) {
                            Some(k.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            });
            unsafe {
                if !result.is_null() {
                    *result = children;
                }
            }
            Status::ok()
        }
    }

    impl GetFileSize for InMemoryEnv {
        fn get_file_size(
            &mut self,
            fname: &String,
            file_size: *mut u64,
        ) -> Status {
            trace!("InMemoryEnv::get_file_size: file='{}'", fname);
            self.with_state(|st| {
                if st.fail_get_file_size {
                    let msg = Slice::from("get_file_size_fail");
                    return Status::io_error(&msg, None);
                }
                let sz = st.files.get(fname).map(|v| v.len() as u64).unwrap_or(0);
                unsafe {
                    if !file_size.is_null() {
                        *file_size = sz;
                    }
                }
                Status::ok()
            })
        }
    }

    impl RenameFile for InMemoryEnv {
        fn rename_file(
            &mut self,
            src: &String,
            target: &String,
        ) -> Status {
            trace!(
                "InMemoryEnv::rename_file: src='{}', dst='{}'",
                src,
                target
            );
            self.with_state(|st| {
                st.renamed_files.push((src.clone(), target.clone()));
                if st.fail_rename_file {
                    let msg = Slice::from("rename_file_fail");
                    return Status::io_error(&msg, None);
                }
                if let Some(buf) = st.files.remove(src) {
                    st.files.insert(target.clone(), buf);
                }
                Status::ok()
            })
        }
    }

    impl NewSequentialFile for InMemoryEnv {
        fn new_sequential_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn SequentialFile>,
        ) -> Status {
            trace!(
                "InMemoryEnv::new_sequential_file: file='{}'",
                fname
            );
            self.with_state(|st| {
                *st.sequential_open_count
                    .entry(fname.clone())
                    .or_insert(0) += 1;
            });
            let handle = InMemoryFileHandle::new(fname.clone(), self.state.clone());
            let boxed: Box<dyn SequentialFile> = Box::new(handle);
            let outer: Box<Box<dyn SequentialFile>> = Box::new(boxed);
            let raw = Box::into_raw(outer);
            unsafe {
                if !result.is_null() {
                    *result = raw;
                }
            }
            Status::ok()
        }
    }

    impl NewRandomAccessFile for InMemoryEnv {
        fn new_random_access_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn RandomAccessFile>,
        ) -> Status {
            trace!(
                "InMemoryEnv::new_random_access_file: file='{}'",
                fname
            );
            self.with_state(|st| {
                *st.random_open_count
                    .entry(fname.clone())
                    .or_insert(0) += 1;
            });
            let fail = self.with_state(|st| st.fail_new_random_access);
            if fail {
                let msg = Slice::from("new_random_access_fail");
                unsafe {
                    if !result.is_null() {
                        *result = std::ptr::null_mut();
                    }
                }
                return Status::io_error(&msg, None);
            }

            let handle = InMemoryFileHandle::new(fname.clone(), self.state.clone());
            let boxed: Box<dyn RandomAccessFile> = Box::new(handle);
            let outer: Box<Box<dyn RandomAccessFile>> = Box::new(boxed);
            let raw = Box::into_raw(outer);
            unsafe {
                if !result.is_null() {
                    *result = raw;
                }
            }
            Status::ok()
        }
    }

    impl NewWritableFile for InMemoryEnv {
        fn new_writable_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            trace!(
                "InMemoryEnv::new_writable_file: file='{}'",
                fname
            );
            let fail = self.with_state(|st| st.fail_new_writable);
            if fail {
                let msg = Slice::from("new_writable_fail");
                unsafe {
                    if !result.is_null() {
                        *result = std::ptr::null_mut();
                    }
                }
                return Status::io_error(&msg, None);
            }

            self.with_state(|st| {
                st.files.insert(fname.clone(), Vec::new());
            });

            let handle = InMemoryFileHandle::new(fname.clone(), self.state.clone());
            let boxed: Box<dyn WritableFile> = Box::new(handle);
            let outer: Box<Box<dyn WritableFile>> = Box::new(boxed);
            let raw = Box::into_raw(outer);
            unsafe {
                if !result.is_null() {
                    *result = raw;
                }
            }
            Status::ok()
        }
    }

    impl NewAppendableFile for InMemoryEnv {
        fn new_appendable_file(
            &mut self,
            fname: &String,
            result: *mut *mut Box<dyn WritableFile>,
        ) -> Status {
            trace!(
                "InMemoryEnv::new_appendable_file: file='{}'",
                fname
            );
            self.with_state(|st| {
                st.files.entry(fname.clone()).or_insert_with(Vec::new);
            });
            let handle = InMemoryFileHandle::new(fname.clone(), self.state.clone());
            let boxed: Box<dyn WritableFile> = Box::new(handle);
            let outer: Box<Box<dyn WritableFile>> = Box::new(boxed);
            let raw = Box::into_raw(outer);
            unsafe {
                if !result.is_null() {
                    *result = raw;
                }
            }
            Status::ok()
        }
    }

    impl LockFile for InMemoryEnv {
        fn lock_file(
            &mut self,
            _fname: &String,
            lock: *mut *mut Box<dyn FileLock>,
        ) -> Status {
            trace!("InMemoryEnv::lock_file");
            let l = Box::new(InMemoryFileLockHandle::default());
            let outer: Box<Box<dyn FileLock>> = Box::new(l);
            let raw = Box::into_raw(outer);
            unsafe {
                if !lock.is_null() {
                    *lock = raw;
                }
            }
            Status::ok()
        }
    }

    impl UnlockFile for InMemoryEnv {
        fn unlock_file(
            &mut self,
            lock: *mut Box<dyn FileLock>,
        ) -> Status {
            trace!("InMemoryEnv::unlock_file");
            unsafe {
                if !lock.is_null() {
                    let _boxed: Box<Box<dyn FileLock>> = Box::from_raw(lock);
                    // dropped here
                }
            }
            Status::ok()
        }
    }

    impl Schedule for InMemoryEnv {
        fn schedule(
            &mut self,
            function: fn(arg: *mut c_void) -> c_void,
            arg: *mut c_void,
        ) {
            trace!("InMemoryEnv::schedule: executing synchronously");
            function(arg);
        }
    }

    impl StartThread for InMemoryEnv {
        fn start_thread(
            &mut self,
            function: fn(arg: *mut c_void) -> c_void,
            arg: *mut c_void,
        ) {
            trace!("InMemoryEnv::start_thread: executing synchronously");
            function(arg);
        }
    }

    impl GetTestDirectory for InMemoryEnv {
        fn get_test_directory(
            &mut self,
            path: *mut String,
        ) -> Status {
            trace!("InMemoryEnv::get_test_directory");
            unsafe {
                if !path.is_null() {
                    *path = "inmemory-test-db".to_string();
                }
            }
            Status::ok()
        }
    }

    impl NewLogger for InMemoryEnv {
        fn new_logger(
            &mut self,
            _fname: &String,
            result: *mut *mut Box<dyn Logger>,
        ) -> Status {
            trace!("InMemoryEnv::new_logger");
            let logger = Box::new(InMemoryLogger);
            let outer: Box<Box<dyn Logger>> = Box::new(logger);
            let raw = Box::into_raw(outer);
            unsafe {
                if !result.is_null() {
                    *result = raw;
                }
            }
            Status::ok()
        }
    }

    impl NowMicros for InMemoryEnv {
        fn now_micros(&mut self) -> u64 {
            let micros = self.with_state(|st| {
                st.reported_now_micros = st.reported_now_micros.wrapping_add(1);
                st.reported_now_micros
            });
            trace!("InMemoryEnv::now_micros -> {}", micros);
            micros
        }
    }

    impl SleepForMicroseconds for InMemoryEnv {
        fn sleep_for_microseconds(&mut self, micros: i32) {
            trace!(
                "InMemoryEnv::sleep_for_microseconds: micros={}",
                micros
            );
        }
    }

    impl Env for InMemoryEnv {}

    pub struct VecLevelDBIterator {
        entries: Vec<(Vec<u8>, Vec<u8>)>,
        index:   isize,
        status:  Status,
    }

    impl VecLevelDBIterator {
        pub fn new(entries: Vec<(Vec<u8>, Vec<u8>)>, status: Status) -> Self {
            Self {
                entries,
                index: -1,
                status,
            }
        }

        fn is_valid_index(&self) -> bool {
            self.index >= 0 && (self.index as usize) < self.entries.len()
        }

        fn seek_to_index(&mut self, idx: isize) {
            self.index = idx;
        }
    }

    impl LevelDBIteratorValid for VecLevelDBIterator {
        fn valid(&self) -> bool {
            let v = self.is_valid_index() && self.status.is_ok();
            trace!("VecLevelDBIterator::valid -> {}", v);
            v
        }
    }

    impl LevelDBIteratorSeekToFirst for VecLevelDBIterator {
        fn seek_to_first(&mut self) {
            trace!("VecLevelDBIterator::seek_to_first");
            if self.entries.is_empty() {
                self.seek_to_index(-1);
            } else {
                self.seek_to_index(0);
            }
        }
    }

    impl LevelDBIteratorSeekToLast for VecLevelDBIterator {
        fn seek_to_last(&mut self) {
            trace!("VecLevelDBIterator::seek_to_last");
            if self.entries.is_empty() {
                self.seek_to_index(-1);
            } else {
                self.seek_to_index((self.entries.len() - 1) as isize);
            }
        }
    }

    impl LevelDBIteratorSeek for VecLevelDBIterator {
        fn seek(&mut self, target: &Slice) {
            trace!(
                "VecLevelDBIterator::seek: target_len={}",
                *target.size()
            );
            let target_bytes = unsafe {
                let len = *target.size();
                let ptr = *target.data();
                if ptr.is_null() || len == 0 {
                    Vec::new()
                } else {
                    std::slice::from_raw_parts(ptr, len).to_vec()
                }
            };
            let mut idx: isize = -1;
            for (i, (k, _)) in self.entries.iter().enumerate() {
                if *k >= target_bytes {
                    idx = i as isize;
                    break;
                }
            }
            self.seek_to_index(idx);
        }
    }

    impl LevelDBIteratorNext for VecLevelDBIterator {
        fn next(&mut self) {
            trace!(
                "VecLevelDBIterator::next: index_before={}",
                self.index
            );
            if self.is_valid_index() {
                self.index += 1;
                if !self.is_valid_index() {
                    self.index = self.entries.len() as isize;
                }
            }
        }
    }

    impl LevelDBIteratorPrev for VecLevelDBIterator {
        fn prev(&mut self) {
            trace!(
                "VecLevelDBIterator::prev: index_before={}",
                self.index
            );
            if self.entries.is_empty() {
                self.index = -1;
                return;
            }
            if self.index <= 0 {
                self.index = -1;
            } else {
                self.index -= 1;
            }
        }
    }

    impl LevelDBIteratorKey for VecLevelDBIterator {
        fn key(&self) -> Slice {
            if !self.is_valid_index() {
                trace!("VecLevelDBIterator::key: invalid index; returning empty Slice");
                return Slice::default();
            }
            let idx = self.index as usize;
            let key_bytes = &self.entries[idx].0;
            trace!(
                "VecLevelDBIterator::key: index={} len={}",
                idx,
                key_bytes.len()
            );
            Slice::from(&key_bytes[..])
        }
    }

    impl LevelDBIteratorValue for VecLevelDBIterator {
        fn value(&self) -> Slice {
            if !self.is_valid_index() {
                trace!(
                    "VecLevelDBIterator::value: invalid index; returning empty Slice"
                );
                return Slice::default();
            }
            let idx = self.index as usize;
            let val_bytes = &self.entries[idx].1;
            trace!(
                "VecLevelDBIterator::value: index={} len={}",
                idx,
                val_bytes.len()
            );
            Slice::from(&val_bytes[..])
        }
    }

    impl LevelDBIteratorStatus for VecLevelDBIterator {
        fn status(&self) -> Status {
            trace!("VecLevelDBIterator::status");
            bitcoinleveldb_status::Status::new_from_other_copy(&self.status)
        }
    }

    impl LevelDBIteratorInterface for VecLevelDBIterator {}

    pub fn make_in_memory_env() -> (Rc<RefCell<dyn Env>>, Arc<Mutex<InMemoryEnvState>>) {
        let state = Arc::new(Mutex::new(InMemoryEnvState::default()));
        let env_impl = InMemoryEnv::new(state.clone());
        let env_dyn: Rc<RefCell<dyn Env>> = Rc::new(RefCell::new(env_impl));
        (env_dyn, state)
    }

    pub fn make_options_with_env(env: Rc<RefCell<dyn Env>>) -> Options {
        let mut opts = Options::default();
        opts.set_env(Some(env));
        opts
    }

    pub fn make_iterator_from_kv_pairs(pairs: &[(Vec<u8>, Vec<u8>)]) -> *mut LevelDBIterator {
        trace!(
            "make_iterator_from_kv_pairs: count={}",
            pairs.len()
        );
        let entries: Vec<(Vec<u8>, Vec<u8>)> = pairs
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        let iface: Box<dyn LevelDBIteratorInterface> =
            Box::new(VecLevelDBIterator::new(entries, Status::ok()));
        let wrapper = LevelDBIterator::new(Some(iface));
        Box::into_raw(Box::new(wrapper))
    }

    pub fn make_empty_iterator() -> *mut LevelDBIterator {
        trace!("make_empty_iterator");
        let iface: Box<dyn LevelDBIteratorInterface> =
            Box::new(VecLevelDBIterator::new(Vec::new(), Status::ok()));
        let wrapper = LevelDBIterator::new(Some(iface));
        Box::into_raw(Box::new(wrapper))
    }
}
