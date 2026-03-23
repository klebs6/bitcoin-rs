// ---------------- [ File: bitcoinleveldbt-corruption/src/bitcoinleveldbt_corruption.rs ]
crate::ix!();

const BITCOINLEVELDB_TEST_CORRUPTION_TEST_LOG_BLOCK_SIZE: i32 = 32 * 1024;
const BITCOINLEVELDB_TEST_CORRUPTION_TEST_MAX_MEM_COMPACT_LEVEL: i32 = 2;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/corruption_test.cc]

const VALUE_SIZE: i32 = 1000;

struct CorruptionTest {
    env:        Rc<RefCell<ErrorEnv>>,
    options:    Options,
    db:         *mut dyn DB,
    dbname:     String,
    tiny_cache: *mut Cache,
}

impl Default for CorruptionTest {

    fn default() -> Self {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_default_entry"
        );

        let env: Rc<RefCell<ErrorEnv>> = Rc::new(RefCell::new(ErrorEnv::default()));
        let dbname = "/memenv/corruption_test".to_string();
        let tiny_cache = new_lru_cache(100usize);

        let mut options = Options::default();
        let env_for_options: Rc<RefCell<dyn Env>> = env.clone();
        options.set_env(Some(env_for_options));
        options.set_block_cache(tiny_cache);

        let destroy_status = destroydb(&dbname, &options);
        assert!(destroy_status.is_ok());

        options.set_create_if_missing(true);

        let mut out = Self {
            env,
            options,
            db: core::ptr::null_mut::<DBImpl>() as *mut dyn DB,
            dbname,
            tiny_cache,
        };

        out.reopen();
        out.options.set_create_if_missing(false);

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_default_exit",
            db_is_null = out.db.is_null()
        );

        out
    }
}

impl Drop for CorruptionTest {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_drop_entry",
            db_is_null = self.db.is_null(),
            cache_is_null = self.tiny_cache.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        if !self.tiny_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.tiny_cache));
            }
            self.tiny_cache = core::ptr::null_mut();
        }

        debug!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_drop_exit"
        );
    }
}

impl CorruptionTest {
    
    pub fn try_reopen(&mut self) -> Status {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_try_reopen_entry",
            db_is_null = self.db.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let mut opener = DBImpl::new(&self.options, &self.dbname);
        let status = opener.open(
            &self.options,
            &self.dbname,
            (&mut self.db) as *mut *mut dyn DB,
        );

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_try_reopen_exit",
            ok = status.is_ok(),
            db_is_null = self.db.is_null()
        );

        status
    }

    pub fn reopen(&mut self) {
        let status = self.try_reopen();
        assert!(status.is_ok());
    }

    pub fn repairdb(&mut self) {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_repairdb_entry",
            db_is_null = self.db.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let status = repairdb(&self.dbname, &self.options);
        assert!(status.is_ok());

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_repairdb_exit",
            ok = status.is_ok()
        );
    }

    pub fn build(&mut self, n: i32) {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_build_entry",
            n = n
        );

        let mut key_space = String::new();
        let mut value_space = String::new();
        let mut batch = WriteBatch::new();

        let mut i: i32 = 0i32;
        while i < n {
            let key = self.key(i, (&mut key_space) as *mut String);
            let value = self.value(i, (&mut value_space) as *mut String);

            batch.clear();
            batch.put(&key, &value);

            let mut options = WriteOptions::default();

            // Corrupt() doesn't work without this sync on windows; stat reports 0 for
            // the file size.
            if i == (n - 1) {
                options.set_sync(true);
            }

            let status = unsafe {
                (&mut *self.db).write(
                    &options,
                    (&mut batch) as *mut WriteBatch,
                )
            };
            assert!(status.is_ok());

            i += 1i32;
        }

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_build_exit",
            n = n
        );
    }

    pub fn check(
        &mut self,
        min_expected: i32,
        max_expected: i32,
    ) {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_check_entry",
            min_expected = min_expected,
            max_expected = max_expected
        );

        let mut next_expected: i32 = 0i32;
        let mut missed: i32 = 0i32;
        let mut bad_keys: i32 = 0i32;
        let mut bad_values: i32 = 0i32;
        let mut correct: i32 = 0i32;
        let mut value_space = String::new();

        let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
        assert!(!iter_ptr.is_null());

        {
            let iter = unsafe { &mut *iter_ptr };
            iter.seek_to_first();

            while iter.valid() {
                let key_text = iter.key().to_string();

                if key_text.is_empty() || key_text == "~" {
                    iter.next();
                    continue;
                }

                let parsed_key = match key_text.parse::<u64>() {
                    Ok(v) => v,
                    Err(_) => {
                        bad_keys += 1i32;
                        iter.next();
                        continue;
                    }
                };

                if parsed_key < next_expected as u64 {
                    bad_keys += 1i32;
                    iter.next();
                    continue;
                }

                missed += (parsed_key as i32) - next_expected;
                next_expected = (parsed_key as i32) + 1i32;

                let expected_value = self
                    .value(parsed_key as i32, (&mut value_space) as *mut String)
                    .to_string();

                if iter.value().to_string() != expected_value {
                    bad_values += 1i32;
                } else {
                    correct += 1i32;
                }

                iter.next();
            }
        }

        unsafe {
            drop(Box::from_raw(iter_ptr));
        }

        eprintln!(
            "expected={}..{}; got={}; bad_keys={}; bad_values={}; missed={}",
            min_expected,
            max_expected,
            correct,
            bad_keys,
            bad_values,
            missed
        );

        assert!(min_expected <= correct);
        assert!(max_expected >= correct);

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_check_exit",
            correct = correct,
            bad_keys = bad_keys,
            bad_values = bad_values,
            missed = missed
        );
    }

    pub fn corrupt(
        &mut self,
        filetype:         FileType,
        offset:           i32,
        bytes_to_corrupt: i32,
    ) {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_corrupt_entry",
            filetype = ?filetype,
            offset = offset,
            bytes_to_corrupt = bytes_to_corrupt
        );

        // Pick file to corrupt
        let mut filenames: Vec<String> = Vec::new();
        let children_status = self.env.borrow_mut().get_children(
            &self.dbname,
            (&mut filenames) as *mut Vec<String>,
        );
        assert!(children_status.is_ok());

        let mut number: u64 = 0u64;
        let mut parsed_type = FileType::TempFile;
        let mut fname = String::new();
        let mut picked_number: i32 = -1i32;

        for candidate in filenames.iter() {
            if parse_file_name(
                candidate,
                (&mut number) as *mut u64,
                (&mut parsed_type) as *mut FileType,
            ) && parsed_type == filetype
                && (number as i32) > picked_number
            {
                fname = format!("{}/{}", self.dbname, candidate);
                picked_number = number as i32;
            }
        }

        assert!(!fname.is_empty(), "{:?}", filetype);

        let mut file_size: u64 = 0u64;
        let size_status = self.env.borrow_mut().get_file_size(
            &fname,
            (&mut file_size) as *mut u64,
        );
        assert!(size_status.is_ok());

        let mut resolved_offset = offset;
        let mut resolved_bytes_to_corrupt = bytes_to_corrupt;

        if resolved_offset < 0i32 {
            // Relative to end of file; make it absolute
            if ((-resolved_offset) as u64) > file_size {
                resolved_offset = 0i32;
            } else {
                resolved_offset = (file_size as i64 + resolved_offset as i64) as i32;
            }
        }

        if (resolved_offset as u64) > file_size {
            resolved_offset = file_size as i32;
        }

        if ((resolved_offset as u64) + (resolved_bytes_to_corrupt as u64)) > file_size {
            resolved_bytes_to_corrupt = (file_size as i64 - resolved_offset as i64) as i32;
        }

        let mut seq_file: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        let open_status = self.env.borrow_mut().new_sequential_file(
            &fname,
            (&mut seq_file) as *mut *mut Box<dyn SequentialFile>,
        );
        assert!(open_status.is_ok());
        assert!(!seq_file.is_null());

        let mut contents: Vec<u8> = vec![0u8; file_size as usize];
        let mut read_result = Slice::default();

        {
            let mut seq_holder: Box<Box<dyn SequentialFile>> = unsafe {
                Box::from_raw(seq_file)
            };
            let seq_ref: &mut Box<dyn SequentialFile> = seq_holder.as_mut();

            let read_status = seq_ref.read(
                file_size as usize,
                (&mut read_result) as *mut Slice,
                contents.as_mut_ptr(),
            );
            assert!(read_status.is_ok());
        }

        contents.truncate(*read_result.size());

        let mut i: i32 = 0i32;
        while i < resolved_bytes_to_corrupt {
            let idx = (resolved_offset + i) as usize;
            contents[idx] ^= 0x80u8;
            i += 1i32;
        }

        let mut writable_file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let create_status = self.env.borrow_mut().new_writable_file(
            &fname,
            (&mut writable_file) as *mut *mut Box<dyn WritableFile>,
        );
        assert!(create_status.is_ok());
        assert!(!writable_file.is_null());

        {
            let mut writable_holder: Box<Box<dyn WritableFile>> = unsafe {
                Box::from_raw(writable_file)
            };
            let writable_ref: &mut Box<dyn WritableFile> = writable_holder.as_mut();

            let corrupt_slice = Slice::from(contents.as_slice());
            let append_status = writable_ref.append(&corrupt_slice);
            assert!(append_status.is_ok());

            let close_status = writable_ref.close();
            assert!(close_status.is_ok());
        }

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_corrupt_exit",
            file = %fname,
            file_size = file_size,
            resolved_offset = resolved_offset,
            resolved_bytes_to_corrupt = resolved_bytes_to_corrupt
        );
    }

    pub fn property(&mut self, name: &String) -> i32 {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_property_entry",
            name = %name
        );

        let mut property = String::new();

        let ok = unsafe {
            (&mut *self.db).get_property(
                name.as_str(),
                (&mut property) as *mut String,
            )
        };

        let result = if ok {
            match property.trim().parse::<i32>() {
                Ok(v) => v,
                Err(_) => -1i32,
            }
        } else {
            -1i32
        };

        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_property_exit",
            name = %name,
            result = result
        );

        result
    }

    /** Return the ith key */
    pub fn key(
        &mut self,
        i:       i32,
        storage: *mut String,
    ) -> Slice {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_key_entry",
            i = i,
            storage_is_null = storage.is_null()
        );

        if storage.is_null() {
            error!(
                target: "bitcoinleveldbt_corruption::corruption_test",
                event = "corruption_test_key_null_storage",
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
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_key_exit",
            key_len = formatted.len()
        );

        out
    }

    /** Return the value to associate with the specified key */
    pub fn value(
        &mut self,
        k:       i32,
        storage: *mut String,
    ) -> Slice {
        trace!(
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_value_entry",
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
            target: "bitcoinleveldbt_corruption::corruption_test",
            event = "corruption_test_value_exit",
            k = k
        );

        out
    }
}

#[traced_test]
fn corruption_test_recovery() {
    let mut t = CorruptionTest::default();

    t.build(100);
    t.check(100, 100);
    t.corrupt(FileType::LogFile, 19, 1);
    t.corrupt(
        FileType::LogFile,
        BITCOINLEVELDB_TEST_CORRUPTION_TEST_LOG_BLOCK_SIZE + 1000,
        1,
    );
    t.reopen();
    t.check(36, 36);
}

#[traced_test]
fn corruption_test_recover_write_error() {
    let mut t = CorruptionTest::default();

    t.env.borrow_mut().set_writable_file_error(true);
    let s = t.try_reopen();
    assert!(!s.is_ok());
}

#[traced_test]
fn corruption_test_new_file_error_during_write() {
    let mut t = CorruptionTest::default();

    t.env.borrow_mut().set_writable_file_error(true);

    let num = 3i32 + ((*Options::default().write_buffer_size()) / (VALUE_SIZE as usize)) as i32;
    let mut value_storage = String::new();
    let mut s = Status::ok();

    let mut i: i32 = 0i32;
    while s.is_ok() && i < num {
        let mut batch = WriteBatch::new();
        let key = Slice::from("a");
        let value = t.value(100, (&mut value_storage) as *mut String);
        batch.put(&key, &value);

        s = unsafe {
            (&mut *t.db).write(
                &WriteOptions::default(),
                (&mut batch) as *mut WriteBatch,
            )
        };

        i += 1i32;
    }

    assert!(!s.is_ok());
    assert!(t.env.borrow().num_writable_file_errors() >= 1);
    t.env.borrow_mut().set_writable_file_error(false);
    t.reopen();
}

#[traced_test]
fn corruption_test_table_file() {
    let mut t = CorruptionTest::default();

    t.build(100);

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    unsafe {
        (&mut *dbi).test_compact_range(0, core::ptr::null(), core::ptr::null());
        (&mut *dbi).test_compact_range(1, core::ptr::null(), core::ptr::null());
    }

    t.corrupt(FileType::TableFile, 100, 1);
    t.check(90, 99);
}

#[traced_test]
fn corruption_test_table_file_repair() {
    let mut t = CorruptionTest::default();

    t.options.set_block_size((2 * VALUE_SIZE) as usize);
    t.options.set_paranoid_checks(true);
    t.reopen();

    t.build(100);

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    unsafe {
        (&mut *dbi).test_compact_range(0, core::ptr::null(), core::ptr::null());
        (&mut *dbi).test_compact_range(1, core::ptr::null(), core::ptr::null());
    }

    t.corrupt(FileType::TableFile, 100, 1);
    t.repairdb();
    t.reopen();
    t.check(95, 99);
}

#[traced_test]
fn corruption_test_table_file_index_data() {
    let mut t = CorruptionTest::default();

    t.build(10000);

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    t.corrupt(FileType::TableFile, -2000, 500);
    t.reopen();
    t.check(5000, 9999);
}

#[traced_test]
fn corruption_test_missing_descriptor() {
    let mut t = CorruptionTest::default();

    t.build(1000);
    t.repairdb();
    t.reopen();
    t.check(1000, 1000);
}

#[traced_test]
fn corruption_test_sequence_number_recovery() {
    let mut t = CorruptionTest::default();

    let foo = Slice::from("foo");
    let v1 = Slice::from("v1");
    let v2 = Slice::from("v2");
    let v3 = Slice::from("v3");
    let v4 = Slice::from("v4");
    let v5 = Slice::from("v5");
    let v6 = Slice::from("v6");

    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v1) }.is_ok());
    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v2) }.is_ok());
    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v3) }.is_ok());
    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v4) }.is_ok());
    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v5) }.is_ok());

    t.repairdb();
    t.reopen();

    let mut v = String::new();
    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &foo,
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!("v5".to_string(), v);

    assert!(unsafe { (&mut *t.db).put(&WriteOptions::default(), &foo, &v6) }.is_ok());
    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &foo,
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!("v6".to_string(), v);

    t.reopen();
    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &foo,
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!("v6".to_string(), v);
}

#[traced_test]
fn corruption_test_corrupted_descriptor() {
    let mut t = CorruptionTest::default();

    assert!(unsafe {
        (&mut *t.db).put(
            &WriteOptions::default(),
            &Slice::from("foo"),
            &Slice::from("hello"),
        )
    }.is_ok());

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    unsafe {
        (&mut *dbi).test_compact_range(0, core::ptr::null(), core::ptr::null());
    }

    t.corrupt(FileType::DescriptorFile, 0, 1000);
    let s = t.try_reopen();
    assert!(!s.is_ok());

    t.repairdb();
    t.reopen();

    let mut v = String::new();
    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &Slice::from("foo"),
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!("hello".to_string(), v);
}

#[traced_test]
fn corruption_test_compaction_input_error() {
    let mut t = CorruptionTest::default();

    t.build(10);

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    let property_name = format!(
        "leveldb.num-files-at-level{}",
        BITCOINLEVELDB_TEST_CORRUPTION_TEST_MAX_MEM_COMPACT_LEVEL
    );
    assert_eq!(1i32, t.property(&property_name));

    t.corrupt(FileType::TableFile, 100, 1);
    t.check(5, 9);

    t.build(10000);
    t.check(10000, 10000);
}

#[traced_test]
fn corruption_test_compaction_input_error_paranoid() {
    let mut t = CorruptionTest::default();

    t.options.set_paranoid_checks(true);
    t.options.set_write_buffer_size((512usize) << 10);
    t.reopen();

    let dbi = t.db as *mut DBImpl;

    let mut i: i32 = 0i32;
    while i < 2i32 {
        t.build(10);

        let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
        assert!(compact_status.is_ok());

        t.corrupt(FileType::TableFile, 100, 1);
        t.env.borrow_mut().sleep_for_microseconds(100000);

        i += 1i32;
    }

    unsafe {
        (&mut *dbi).compact_range(core::ptr::null(), core::ptr::null());
    }

    let mut tmp1 = String::new();
    let mut tmp2 = String::new();
    let s = unsafe {
        (&mut *t.db).put(
            &WriteOptions::default(),
            &t.key(5, (&mut tmp1) as *mut String),
            &t.value(5, (&mut tmp2) as *mut String),
        )
    };
    assert!(!s.is_ok(), "write did not fail in corrupted paranoid db");
}

#[traced_test]
fn corruption_test_unrelated_keys() {
    let mut t = CorruptionTest::default();

    t.build(10);

    let dbi = t.db as *mut DBImpl;
    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    t.corrupt(FileType::TableFile, 100, 1);

    let mut tmp1 = String::new();
    let mut tmp2 = String::new();
    assert!(unsafe {
        (&mut *t.db).put(
            &WriteOptions::default(),
            &t.key(1000, (&mut tmp1) as *mut String),
            &t.value(1000, (&mut tmp2) as *mut String),
        )
    }.is_ok());

    let mut v = String::new();
    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &t.key(1000, (&mut tmp1) as *mut String),
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!(
        t.value(1000, (&mut tmp2) as *mut String).to_string(),
        v
    );

    let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
    assert!(compact_status.is_ok());

    assert!(unsafe {
        (&mut *t.db).get(
            &ReadOptions::default(),
            &t.key(1000, (&mut tmp1) as *mut String),
            (&mut v) as *mut String,
        )
    }.is_ok());
    assert_eq!(
        t.value(1000, (&mut tmp2) as *mut String).to_string(),
        v
    );
}

fn dbcorruption_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldbt_corruption::corruption_test",
        event = "dbcorruption_test_main_entry"
    );

    let rc = run_all_tests();

    trace!(
        target: "bitcoinleveldbt_corruption::corruption_test",
        event = "dbcorruption_test_main_exit",
        result = rc
    );

    rc
}
