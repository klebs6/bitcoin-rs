// ---------------- [ File: bitcoinleveldb-testautocompaction/src/bitcoinleveldb_testautocompaction.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/autocompact_test.cc]

struct AutoCompactTest {
    dbname:     String,
    tiny_cache: *mut Cache,
    options:    Options,
    db:         *mut dyn DB,
}

impl Default for AutoCompactTest {
    fn default() -> Self {
        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_default_entry"
        );

        let dbname = unique_db_path("/autocompact_test");
        let tiny_cache = new_lru_cache(100usize);

        let mut options = Options::default();
        options.set_env(Some(posix_default_env()));
        options.set_block_cache(tiny_cache);

        let destroy_status = destroydb(&dbname, &options);
        assert!(destroy_status.is_ok());

        options.set_create_if_missing(true);
        options.set_compression(CompressionType::None);

        let mut db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        let mut opener = DBImpl::new(&options, &dbname);
        let open_status = opener.open(
            &options,
            &dbname,
            (&mut db) as *mut *mut dyn DB,
        );
        assert!(open_status.is_ok());

        let out = Self {
            dbname,
            tiny_cache,
            options,
            db,
        };

        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_default_exit",
            db_is_null = out.db.is_null()
        );

        out
    }
}

impl Drop for AutoCompactTest {
    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_drop_entry",
            db_is_null = self.db.is_null(),
            cache_is_null = self.tiny_cache.is_null()
        );

        if !self.db.is_null() {
            unsafe {
                drop(Box::from_raw(self.db));
            }
            self.db = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;
        }

        let _ = destroydb(&self.dbname, &Options::default());

        if !self.tiny_cache.is_null() {
            unsafe {
                drop(Box::from_raw(self.tiny_cache));
            }
            self.tiny_cache = core::ptr::null_mut();
        }

        debug!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_drop_exit"
        );
    }
}

const VALUE_SIZE: i32 = 200 * 1024;
const TOTAL_SIZE: i32 = 100 * 1024 * 1024;
const COUNT:      i32 = TOTAL_SIZE / VALUE_SIZE;

impl AutoCompactTest {
    pub fn key(&mut self, i: i32) -> String {
        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_key_entry",
            i = i
        );

        let out = format!("key{:06}", i);

        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_key_exit",
            key_len = out.len()
        );

        out
    }
    
    pub fn size(
        &mut self,
        start: &Slice,
        limit: &Slice,
    ) -> u64 {
        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_size_entry",
            start_len = *start.size(),
            limit_len = *limit.size()
        );

        let range = Range::new(
            Slice::from_ptr_len(*start.data(), *start.size()),
            Slice::from_ptr_len(*limit.data(), *limit.size()),
        );

        let mut size: u64 = 0u64;
        unsafe {
            (&mut *self.db).get_approximate_sizes(
                (&range) as *const Range,
                1i32,
                (&mut size) as *mut u64,
            );
        }

        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_size_exit",
            size = size
        );

        size
    }
   
    /**
      | Read through the first n keys repeatedly
      | and check that they get compacted (verified
      | by checking the size of the key space).
      |
      */
    pub fn do_reads(&mut self, n: i32) {
        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_do_reads_entry",
            n = n
        );

        let value = "x".repeat(VALUE_SIZE as usize);
        let dbi: *mut DBImpl = (self.db as *mut ()) as *mut DBImpl;

        // Fill database
        let mut i: i32 = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).put(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                    &Slice::from(&value),
                )
            };
            assert!(status.is_ok());
            i += 1i32;
        }

        let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
        assert!(compact_status.is_ok());

        // Delete everything
        i = 0i32;
        while i < COUNT {
            let key = self.key(i);
            let status = unsafe {
                (&mut *self.db).delete(
                    &WriteOptions::default(),
                    &Slice::from(&key),
                )
            };
            assert!(status.is_ok());
            i += 1i32;
        }

        let compact_status = unsafe { (&mut *dbi).test_compact_mem_table() };
        assert!(compact_status.is_ok());

        let key0 = self.key(0);
        let keyn = self.key(n);
        let last_key = self.key(COUNT);

        // Get initial measurement of the space we will be reading.
        let initial_size = self.size(
            &Slice::from(&key0),
            &Slice::from(&keyn),
        );
        let initial_other_size = self.size(
            &Slice::from(&keyn),
            &Slice::from(&last_key),
        );

        // Read until size drops significantly.
        let limit_key = self.key(n);

        let mut read: i32 = 0i32;
        loop {
            assert!(read < 100, "Taking too long to compact");

            let iter_ptr = unsafe { (&mut *self.db).new_iterator(&ReadOptions::default()) };
            assert!(!iter_ptr.is_null());

            {
                let iter = unsafe { &mut *iter_ptr };
                iter.seek_to_first();
                while iter.valid() && iter.key().to_string() < limit_key {
                    // Drop data
                    iter.next();
                }
            }

            unsafe {
                drop(Box::from_raw(iter_ptr));
            }

            // Wait a little bit to allow any triggered compactions to complete.
            let env = posix_default_env();
            env.borrow_mut().sleep_for_microseconds(1_000_000);

            let key0 = self.key(0);
            let keyn = self.key(n);
            let last_key = self.key(COUNT);


            let size = self.size(
                &Slice::from(&key0),
                &Slice::from(&keyn)
            );

            eprintln!(
                "iter {:3} => {:7.3} MB [other {:7.3} MB]",
                read + 1,
                size as f64 / 1_048_576.0,
                self.size(
                    &Slice::from(&keyn),
                    &Slice::from(&last_key)
                ) as f64 / 1_048_576.0,
            );

            if size <= initial_size / 10 {
                break;
            }

            read += 1i32;
        }

        let keyn = self.key(n);
        let last_key = self.key(COUNT);

        // Verify that the size of the key space not touched by the reads
        // is pretty much unchanged.
        let final_other_size = self.size(
            &Slice::from(&keyn),
            &Slice::from(&last_key)
        );

        assert!(final_other_size <= initial_other_size + 1_048_576u64);
        assert!(final_other_size >= initial_other_size / 5u64 - 1_048_576u64);

        trace!(
            target: "bitcoinleveldb_test::autocompact_test",
            event = "auto_compact_test_do_reads_exit",
            n = n
        );
    }
}

#[traced_test]
fn auto_compact_test_read_all() {
    let mut t = AutoCompactTest::default();
    t.do_reads(COUNT);
}

#[traced_test]
fn auto_compact_test_read_half() {
    let mut t = AutoCompactTest::default();
    t.do_reads(COUNT / 2);
}

fn dbautocompact_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldb_test::autocompact_test",
        event = "dbautocompact_test_main_entry"
    );

    let rc = run_all_tests();

    trace!(
        target: "bitcoinleveldb_test::autocompact_test",
        event = "dbautocompact_test_main_exit",
        result = rc
    );

    rc
}
