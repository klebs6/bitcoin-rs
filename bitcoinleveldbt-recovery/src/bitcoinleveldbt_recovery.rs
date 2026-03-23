// ---------------- [ File: bitcoinleveldbt-recovery/src/bitcoinleveldbt_recovery.rs ]
crate::ix!();

struct BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    inner: Box<dyn WritableFile>,
}

impl Named for BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    fn name(&self) -> Cow<'_,str> {
        self.inner.name()
    }
}

impl WritableFileAppend for BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    fn append(&mut self, data: &Slice) -> crate::Status {
        self.inner.append(data)
    }
}

impl WritableFileClose for BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    fn close(&mut self) -> crate::Status {
        self.inner.close()
    }
}

impl WritableFileFlush for BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    fn flush(&mut self) -> crate::Status {
        self.inner.flush()
    }
}

impl WritableFileSync for BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
    fn sync(&mut self) -> crate::Status {
        self.inner.sync()
    }
}

impl WritableFile for BitcoinLevelDbTestRecoveryWritableFileRcAdapter { }

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/recovery_test.cc]

struct RecoveryTest {
    dbname: String,
    env:    Rc<RefCell<dyn Env>>,
    db:     *mut dyn DB,
}

impl Default for RecoveryTest {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_default_entry"
        );

        let env = posix_default_env();
        let dbname = unique_db_path("/recovery_test");
        let _ = destroydb(&dbname, &Options::default());

        let mut out = Self {
            dbname,
            env,
            db: core::ptr::null_mut::<DBImpl>() as *mut dyn DB,
        };

        out.open(None);

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_default_exit",
            db_is_null = out.db.is_null()
        );

        out
    }
}

impl Drop for RecoveryTest {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_drop_entry",
            db_is_null = self.db.is_null()
        );

        self.close();
        let _ = destroydb(&self.dbname, &Options::default());

        debug!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_drop_exit"
        );
    }
}

impl RecoveryTest {

    pub fn dbfull(&self) -> *mut DBImpl {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_dbfull_entry",
            db_is_null = self.db.is_null()
        );

        let out = (self.db as *mut ()) as *mut DBImpl;

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_dbfull_exit",
            dbimpl_is_null = out.is_null()
        );

        out
    }
    
    pub fn env(&self) -> Rc<RefCell<dyn Env>> {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_env_entry"
        );

        let out = Rc::clone(&self.env);

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_env_exit",
            strong_count = Rc::strong_count(&out)
        );

        out
    }

    pub fn can_append(&mut self) -> bool {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_can_append_entry"
        );

        let current = current_file_name(&self.dbname);
        let mut tmp: *mut Box<dyn WritableFile> = core::ptr::null_mut();

        let status = self.env.borrow_mut().new_appendable_file(
            &current,
            (&mut tmp) as *mut *mut Box<dyn WritableFile>,
        );

        if !tmp.is_null() {
            let mut tmp_holder: Box<Box<dyn WritableFile>> = unsafe {
                Box::from_raw(tmp)
            };
            let tmp_ref: &mut Box<dyn WritableFile> = tmp_holder.as_mut();
            let _ = tmp_ref.close();
        }

        let out = if status.is_not_supported_error() {
            false
        } else {
            true
        };

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_can_append_exit",
            status_ok = status.is_ok(),
            status_not_supported = status.is_not_supported_error(),
            result = out
        );

        out
    }

    pub fn close(&mut self) {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_close_entry",
            db_is_null = self.db.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_close_exit",
            db_is_null = self.db.is_null()
        );
    }
   
    pub fn open_with_status(&mut self, options: Option<*mut Options>) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_open_with_status_entry",
            dbname = %self.dbname,
            has_options = options.is_some()
        );

        self.close();

        let mut opts = match options {
            Some(ptr) => unsafe { (*ptr).clone() },
            None => {
                let mut default_opts = Options::default();
                default_opts.set_reuse_logs(true);
                default_opts.set_create_if_missing(true);
                default_opts
            }
        };

        if opts.env().is_none() {
            opts.set_env(Some(Rc::clone(&self.env)));
        }

        let mut opener = DBImpl::new(&opts, &self.dbname);
        let status = opener.open(
            &opts,
            &self.dbname,
            (&mut self.db) as *mut *mut dyn DB,
        );

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_open_with_status_exit",
            ok = status.is_ok(),
            db_is_null = self.db.is_null()
        );

        status
    }

    pub fn open(&mut self, options: Option<*mut Options>) {
        let status = self.open_with_status(options);
        assert!(status.is_ok());
        assert_eq!(1i32, self.num_logs());
    }
   
    pub fn put(
        &mut self,
        k: &String,
        v: &String,
    ) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_put_entry",
            key_len = k.len(),
            value_len = v.len()
        );

        let status = unsafe {
            (&mut *self.db).put(
                &WriteOptions::default(),
                &Slice::from(k),
                &Slice::from(v),
            )
        };

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_put_exit",
            ok = status.is_ok()
        );

        status
    }
   
    pub fn get(
        &mut self,
        k:        &String,
        snapshot: Option<*const dyn Snapshot>,
    ) -> String {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_get_entry",
            key_len = k.len(),
            has_snapshot = snapshot.is_some()
        );

        let mut result = String::new();
        let s = unsafe {
            (&mut *self.db).get(
                &ReadOptions::default(),
                &Slice::from(k),
                (&mut result) as *mut String,
            )
        };

        if s.is_not_found() {
            result = "NOT_FOUND".to_string();
        } else if !s.is_ok() {
            result = s.to_string();
        }

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_get_exit",
            result_len = result.len()
        );

        result
    }

    pub fn manifest_file_name(&mut self) -> String {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_manifest_file_name_entry"
        );

        let current_file = current_file_name(&self.dbname);
        let mut current = String::new();
        let status = read_file_to_string(
            Rc::clone(&self.env),
            &current_file,
            (&mut current) as *mut String,
        );

        assert!(status.is_ok());

        if current.ends_with('\n') {
            let _ = current.pop();
        }

        let out = format!("{}/{}", self.dbname, current);

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_manifest_file_name_exit",
            manifest = %out
        );

        out
    }

    pub fn log_name(&mut self, number: u64) -> String {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_log_name_entry",
            number = number
        );

        let out = log_file_name(&self.dbname, number);

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_log_name_exit",
            filename = %out
        );

        out
    }

    pub fn delete_log_files(&mut self) -> usize {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_delete_log_files_entry"
        );

        // Linux allows unlinking open files, but Windows does not.
        // Closing the db allows for file deletion.
        self.close();

        let logs = self.get_files(FileType::LogFile);

        for number in logs.iter() {
            let fname = self.log_name(*number);
            let status = self.env.borrow_mut().delete_file(&fname);
            assert!(status.is_ok());
        }

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_delete_log_files_exit",
            deleted = logs.len()
        );

        logs.len()
    }

    pub fn delete_manifest_file(&mut self) {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_delete_manifest_file_entry"
        );

        let manifest = self.manifest_file_name();
        let status = self.env.borrow_mut().delete_file(&manifest);
        assert!(status.is_ok());

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_delete_manifest_file_exit"
        );
    }

    pub fn first_log_file(&mut self) -> u64 {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_first_log_file_entry"
        );

        let out = self.get_files(FileType::LogFile)[0];

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_first_log_file_exit",
            number = out
        );

        out
    }
   
    pub fn get_files(&mut self, t: FileType) -> Vec<u64> {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_get_files_entry"
        );

        let mut filenames: Vec<String> = Vec::new();
        let status = self.env.borrow_mut().get_children(
            &self.dbname,
            (&mut filenames) as *mut Vec<String>,
        );

        assert!(status.is_ok());

        let mut result: Vec<u64> = Vec::new();

        for filename in filenames.iter() {
            let mut number: u64 = 0u64;
            let mut ty = FileType::TempFile;

            if parse_file_name(
                filename,
                (&mut number) as *mut u64,
                (&mut ty) as *mut FileType,
            ) && core::mem::discriminant(&ty) == core::mem::discriminant(&t)
            {
                result.push(number);
            }
        }

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_get_files_exit",
            count = result.len()
        );

        result
    }
   
    pub fn num_logs(&mut self) -> i32 {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_num_logs_entry"
        );

        let out = self.get_files(FileType::LogFile).len() as i32;

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_num_logs_exit",
            count = out
        );

        out
    }

    pub fn num_tables(&mut self) -> i32 {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_num_tables_entry"
        );

        let out = self.get_files(FileType::TableFile).len() as i32;

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_num_tables_exit",
            count = out
        );

        out
    }

    pub fn file_size(&mut self, fname: &String) -> u64 {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_file_size_entry",
            filename = %fname
        );

        let mut result: u64 = 0u64;
        let status = self.env.borrow_mut().get_file_size(
            fname,
            (&mut result) as *mut u64,
        );

        assert!(status.is_ok());

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_file_size_exit",
            filename = %fname,
            size = result
        );

        result
    }
    
    pub fn compact_mem_table(&mut self) {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_compact_mem_table_entry"
        );

        let status = unsafe { (&mut *self.dbfull()).test_compact_mem_table() };
        assert!(status.is_ok());

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_compact_mem_table_exit",
            ok = status.is_ok()
        );
    }

    /// Directly construct a log file that sets key to val.
    ///
    pub fn make_log_file(
        &mut self,
        lognum: u64,
        seq:    SequenceNumber,
        key_:   Slice,
        val:    Slice,
    ) {
        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_make_log_file_entry",
            lognum = lognum,
            seq = seq
        );

        let fname = self.log_name(lognum);

        let mut file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status = self.env.borrow_mut().new_writable_file(
            &fname,
            (&mut file) as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());
        assert!(!file.is_null());

        let file_inner: Box<dyn WritableFile> = unsafe { *Box::from_raw(file) };

        let writer_dest: Rc<RefCell<dyn WritableFile>> =
            Rc::new(RefCell::new(BitcoinLevelDbTestRecoveryWritableFileRcAdapter {
                inner: file_inner,
            }));

        let mut writer = LogWriter::new(writer_dest.clone(), 0u64);

        let mut batch = WriteBatch::new();
        batch.put(&key_, &val);
        write_batch_internal::set_sequence(
            (&mut batch) as *mut WriteBatch,
            seq,
        );

        let contents = write_batch_internal::contents(
            (&batch) as *const WriteBatch,
        );

        let add_status = writer.add_record(&contents);
        assert!(add_status.is_ok());

        drop(writer);

        {
            let mut writable = writer_dest.borrow_mut();
            let flush_status = writable.flush();
            assert!(flush_status.is_ok());

            let close_status = writable.close();
            assert!(close_status.is_ok());
        }

        trace!(
            target: "bitcoinleveldbt_recovery::recovery_test",
            event = "recovery_test_make_log_file_exit",
            lognum = lognum
        );
    }
}

#[traced_test]
fn recovery_test_manifest_reused() {
    let mut t = RecoveryTest::default();

    if !t.can_append() {
        eprintln!("skipping test because env does not support appending");
        return;
    }

    assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());
    t.close();

    let old_manifest = t.manifest_file_name();

    t.open(None);
    assert_eq!(old_manifest, t.manifest_file_name());
    assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));

    t.open(None);
    assert_eq!(old_manifest, t.manifest_file_name());
    assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));
}

#[traced_test]
fn recovery_test_large_manifest_compacted() {
    let mut t = RecoveryTest::default();

    if !t.can_append() {
        eprintln!("skipping test because env does not support appending");
        return;
    }

    assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());
    t.close();

    let old_manifest = t.manifest_file_name();

    {
        let len = t.file_size(&old_manifest) as usize;

        let mut file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let status = t.env().borrow_mut().new_appendable_file(
            &old_manifest,
            (&mut file) as *mut *mut Box<dyn WritableFile>,
        );
        assert!(status.is_ok());
        assert!(!file.is_null());

        let mut file_holder: Box<Box<dyn WritableFile>> = unsafe {
            Box::from_raw(file)
        };
        let file_ref: &mut Box<dyn WritableFile> = file_holder.as_mut();

        let pad_len = (3usize * 1_048_576usize).saturating_sub(len);
        let zeroes = vec![0u8; pad_len];
        let zeroes_slice = Slice::from(zeroes.as_slice());

        let append_status = file_ref.append(&zeroes_slice);
        assert!(append_status.is_ok());

        let flush_status = file_ref.flush();
        assert!(flush_status.is_ok());

        let close_status = file_ref.close();
        assert!(close_status.is_ok());
    }

    t.open(None);

    let new_manifest = t.manifest_file_name();
    assert_ne!(old_manifest, new_manifest);
    assert!(10_000u64 > t.file_size(&new_manifest));
    assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));

    t.open(None);
    assert_eq!(new_manifest, t.manifest_file_name());
    assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));
}

#[traced_test]
fn recovery_test_no_log_files() {
    let mut t = RecoveryTest::default();

    assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());
    assert_eq!(1usize, t.delete_log_files());

    t.open(None);
    assert_eq!("NOT_FOUND".to_string(), t.get(&"foo".to_string(), None));

    t.open(None);
    assert_eq!("NOT_FOUND".to_string(), t.get(&"foo".to_string(), None));
}

#[traced_test]
fn recovery_test_log_file_reuse() {
    let mut t = RecoveryTest::default();

    if !t.can_append() {
        eprintln!("skipping test because env does not support appending");
        return;
    }

    let mut i: i32 = 0i32;
    while i < 2i32 {
        assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());

        if i == 0i32 {
            t.compact_mem_table();
        }

        t.close();
        assert_eq!(1i32, t.num_logs());

        let number = t.first_log_file();
        let log_name = t.log_name(number);
        if i == 0i32 {
            assert_eq!(0u64, t.file_size(&log_name));
        } else {
            assert!(0u64 < t.file_size(&log_name));
        }

        t.open(None);
        assert_eq!(1i32, t.num_logs());
        assert_eq!(number, t.first_log_file(), "did not reuse log file");
        assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));

        t.open(None);
        assert_eq!(1i32, t.num_logs());
        assert_eq!(number, t.first_log_file(), "did not reuse log file");
        assert_eq!("bar".to_string(), t.get(&"foo".to_string(), None));

        i += 1i32;
    }
}

#[traced_test]
fn recovery_test_multiple_mem_tables() {
    let mut t = RecoveryTest::default();

    const BITCOINLEVELDB_TEST_RECOVERY_TEST_K_NUM: i32 = 1000;

    let mut i: i32 = 0i32;
    while i < BITCOINLEVELDB_TEST_RECOVERY_TEST_K_NUM {
        let buf = format!("{:050}", i);
        assert!(t.put(&buf, &buf).is_ok());
        i += 1i32;
    }

    assert_eq!(0i32, t.num_tables());
    t.close();
    assert_eq!(0i32, t.num_tables());
    assert_eq!(1i32, t.num_logs());

    let old_log_file = t.first_log_file();

    let mut opt = Options::default();
    opt.set_reuse_logs(true);
    opt.set_write_buffer_size((BITCOINLEVELDB_TEST_RECOVERY_TEST_K_NUM * 100 / 2) as usize);
    t.open(Some((&mut opt) as *mut Options));

    assert!(2i32 <= t.num_tables());
    assert_eq!(1i32, t.num_logs());
    assert_ne!(old_log_file, t.first_log_file(), "must not reuse log");

    i = 0i32;
    while i < BITCOINLEVELDB_TEST_RECOVERY_TEST_K_NUM {
        let buf = format!("{:050}", i);
        assert_eq!(buf, t.get(&buf, None));
        i += 1i32;
    }
}

#[traced_test]
fn recovery_test_multiple_log_files() {
    let mut t = RecoveryTest::default();

    assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());
    t.close();
    assert_eq!(1i32, t.num_logs());

    let old_log = t.first_log_file();

    let hello = "hello".to_string();
    let world = "world".to_string();
    t.make_log_file(
        old_log + 1u64,
        1000u64,
        Slice::from(&hello),
        Slice::from(&world),
    );

    let hi = "hi".to_string();
    let there = "there".to_string();
    t.make_log_file(
        old_log + 2u64,
        1001u64,
        Slice::from(&hi),
        Slice::from(&there),
    );

    let foo = "foo".to_string();
    let bar2 = "bar2".to_string();
    t.make_log_file(
        old_log + 3u64,
        1002u64,
        Slice::from(&foo),
        Slice::from(&bar2),
    );

    t.open(None);
    assert!(1i32 <= t.num_tables());
    assert_eq!(1i32, t.num_logs());

    let new_log = t.first_log_file();
    assert!(old_log + 3u64 <= new_log);
    assert_eq!("bar2".to_string(), t.get(&"foo".to_string(), None));
    assert_eq!("world".to_string(), t.get(&"hello".to_string(), None));
    assert_eq!("there".to_string(), t.get(&"hi".to_string(), None));

    t.open(None);
    assert!(1i32 <= t.num_tables());
    assert_eq!(1i32, t.num_logs());
    if t.can_append() {
        assert_eq!(new_log, t.first_log_file());
    }
    assert_eq!("bar2".to_string(), t.get(&"foo".to_string(), None));
    assert_eq!("world".to_string(), t.get(&"hello".to_string(), None));
    assert_eq!("there".to_string(), t.get(&"hi".to_string(), None));

    t.close();

    let stale_write = "stale write".to_string();
    t.make_log_file(
        old_log + 1u64,
        2000u64,
        Slice::from(&hello),
        Slice::from(&stale_write),
    );

    t.open(None);
    assert!(1i32 <= t.num_tables());
    assert_eq!(1i32, t.num_logs());
    if t.can_append() {
        assert_eq!(new_log, t.first_log_file());
    }
    assert_eq!("bar2".to_string(), t.get(&"foo".to_string(), None));
    assert_eq!("world".to_string(), t.get(&"hello".to_string(), None));
    assert_eq!("there".to_string(), t.get(&"hi".to_string(), None));
}

#[traced_test]
fn recovery_test_manifest_missing() {
    let mut t = RecoveryTest::default();

    assert!(t.put(&"foo".to_string(), &"bar".to_string()).is_ok());
    t.close();
    t.delete_manifest_file();

    let status = t.open_with_status(None);
    assert!(status.is_corruption());
}

fn dbrecovery_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldbt_recovery::recovery_test",
        event = "dbrecovery_test_main_entry"
    );

    let rc = run_all_tests();

    trace!(
        target: "bitcoinleveldbt_recovery::recovery_test",
        event = "dbrecovery_test_main_exit",
        result = rc
    );

    rc
}
