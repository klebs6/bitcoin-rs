// ---------------- [ File: bitcoinleveldbt-faultinjection/src/fault_injection_test.rs ]
crate::ix!();

pub struct FaultInjectionTest {
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_drop_exit"
        );
    }
}

impl FaultInjectionTest {

    pub fn reuse_logs(&mut self, reuse: bool) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_reuse_logs_entry",
            reuse = reuse
        );

        *self.options.reuse_logs_mut() = reuse;

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_key_entry",
            i = i,
            storage_is_null = storage.is_null()
        );

        if storage.is_null() {
            error!(
                target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_value_exit",
            k = k
        );

        out
    }

    pub fn opendb(&mut self) -> crate::Status {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_opendb_exit",
            ok = status.is_ok(),
            db_is_null = self.db.is_null()
        );

        status
    }

    pub fn closedb(&mut self) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_closedb_exit",
            db_is_null = self.db.is_null()
        );
    }

    pub fn delete_all_data(&mut self) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_delete_all_data_exit",
            deleted = keys.len()
        );
    }
    
    pub fn reset_db_state(&mut self, reset_method: ResetMethod) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_partial_compact_test_reopen_with_fault_exit"
        );
    }
   
    pub fn no_write_test_pre_fault(&mut self)  { }

    pub fn no_write_test_reopen_with_fault(&mut self, reset_method: ResetMethod) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_no_write_test_reopen_with_fault_entry",
            reset_method = ?reset_method
        );

        self.closedb();
        self.reset_db_state(reset_method);

        let open_status = self.opendb();
        assert!(open_status.is_ok());

        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_no_write_test_reopen_with_fault_exit"
        );
    }

    pub fn do_test(&mut self) {
        trace!(
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
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
            target: "bitcoinleveldbt_faultinjection::fault_injection_test",
            event = "fault_injection_test_do_test_exit"
        );
    }
}
