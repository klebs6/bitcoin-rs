// ---------------- [ File: bitcoinleveldbt-capi/src/bitcoinleveldbt_capi.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c_test.c]

pub const BITCOINLEVELDB_TEST_C_TEST_PHASE_FALLBACK: &str = "";

const phase: &'static str = "";

static BITCOINLEVELDB_TEST_C_TEST_PHASE_PTR: AtomicPtr<u8> =
    AtomicPtr::new(core::ptr::null_mut());

static BITCOINLEVELDB_TEST_C_TEST_FAKE_FILTER_RESULT: AtomicU8 =
    AtomicU8::new(1u8);

fn bitcoinleveldb_test__c_test_rs__phase_label() -> String {
    let p = BITCOINLEVELDB_TEST_C_TEST_PHASE_PTR.load(atomic::Ordering::SeqCst);

    if p.is_null() {
        return BITCOINLEVELDB_TEST_C_TEST_PHASE_FALLBACK.to_owned();
    }

    unsafe {
        CStr::from_ptr(p as *const c_char)
            .to_string_lossy()
            .into_owned()
    }
}

fn start_phase(name: *const u8) {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "start_phase_entry",
        name_ptr = (name as usize)
    );

    let label = if name.is_null() {
        String::new()
    } else {
        unsafe {
            CStr::from_ptr(name as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    eprintln!("=== Test {}", label);
    BITCOINLEVELDB_TEST_C_TEST_PHASE_PTR.store(name as *mut u8, atomic::Ordering::SeqCst);

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "start_phase_exit",
        phase = %label
    );
}

macro_rules! check_no_error {
    ($err:expr) => {{
        if !($err).is_null() {
            let phase_label_value = bitcoinleveldb_test__c_test_rs__phase_label();
            let err_msg = unsafe {
                CStr::from_ptr(($err) as *const c_char)
                    .to_string_lossy()
                    .into_owned()
            };

            error!(
                target: "bitcoinleveldbt_capi::c_test",
                event = "check_no_error_failed",
                file = file!(),
                line = line!(),
                phase = %phase_label_value,
                err = %err_msg
            );

            panic!("bitcoinleveldb_test__c_test_rs__check_no_error_failed");
        }
    }}
}

macro_rules! check_condition {
    ($cond:expr) => {{
        if !($cond) {
            let phase_label_value = bitcoinleveldb_test__c_test_rs__phase_label();

            error!(
                target: "bitcoinleveldbt_capi::c_test",
                event = "check_condition_failed",
                file = file!(),
                line = line!(),
                phase = %phase_label_value,
                condition = stringify!($cond)
            );

            panic!("bitcoinleveldb_test__c_test_rs__check_condition_failed");
        }
    }}
}

fn check_equal(
    expected: *const u8,
    v:        *const u8,
    n:        usize)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_equal_entry",
        expected_is_null = expected.is_null(),
        value_is_null = v.is_null(),
        value_len = n
    );

    let equal = if expected.is_null() && v.is_null() {
        true
    } else if !expected.is_null() && !v.is_null() {
        let expected_bytes = unsafe {
            CStr::from_ptr(expected as *const c_char).to_bytes()
        };
        let value_bytes = unsafe {
            core::slice::from_raw_parts(v as *const u8, n)
        };
        expected_bytes.len() == n && expected_bytes == value_bytes
    } else {
        false
    };

    if equal {
        trace!(
            target: "bitcoinleveldbt_capi::c_test",
            event = "check_equal_exit",
            matched = true
        );
        return;
    }

    let expected_label = if expected.is_null() {
        String::from("(null)")
    } else {
        unsafe {
            CStr::from_ptr(expected as *const c_char)
                .to_string_lossy()
                .into_owned()
        }
    };

    let got_label = if v.is_null() {
        String::from("(null)")
    } else {
        let bytes = unsafe { core::slice::from_raw_parts(v as *const u8, n) };
        String::from_utf8_lossy(bytes).into_owned()
    };

    error!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_equal_mismatch",
        phase = %bitcoinleveldb_test__c_test_rs__phase_label(),
        expected = %expected_label,
        got = %got_label,
        got_len = n
    );

    panic!("bitcoinleveldb_test__c_test_rs__check_equal_failed");
}

fn free(ptr: *mut *mut u8)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "free_entry",
        ptr_ptr_is_null = ptr.is_null()
    );

    unsafe {
        if ptr.is_null() {
            trace!(
                target: "bitcoinleveldbt_capi::c_test",
                event = "free_exit",
                freed = false
            );
            return;
        }

        if !(*ptr).is_null() {
            leveldb_free(*ptr as *mut c_void);
            *ptr = core::ptr::null_mut();
        }
    }

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "free_exit",
        freed = true
    );
}

fn check_get(
    db:       *mut LevelDB,
    options:  *const LevelDBReadOptions,
    key_:     *const u8,
    expected: *const u8)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_get_entry",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        key_is_null = key_.is_null(),
        expected_is_null = expected.is_null()
    );

    let mut err: *mut u8 = core::ptr::null_mut();
    let mut val_len: usize = 0;

    let key_len = if key_.is_null() {
        0usize
    } else {
        unsafe {
            CStr::from_ptr(key_ as *const c_char).to_bytes().len()
        }
    };

    let val = leveldb_get(
        db,
        options,
        key_,
        key_len,
        (&mut val_len) as *mut usize,
        (&mut err) as *mut *mut u8,
    );

    check_no_error!(err);
    check_equal(expected, val, val_len);
    free((&mut err) as *mut *mut u8);

    let mut val_mut = val;
    free((&mut val_mut) as *mut *mut u8);

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_get_exit",
        key_len = key_len,
        val_len = val_len
    );
}

fn check_iter(
    iter: *mut LevelDBIterator,
    key_: *const u8,
    val:  *const u8)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_iter_entry",
        iter_is_null = iter.is_null()
    );

    let mut len: usize = 0;
    let key_ptr = leveldb_iter_key(iter as *const LevelDBIterator, (&mut len) as *mut usize);
    check_equal(key_, key_ptr, len);

    let val_ptr = leveldb_iter_value(iter as *const LevelDBIterator, (&mut len) as *mut usize);
    check_equal(val, val_ptr, len);

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_iter_exit"
    );
}

/**
  | Callback from leveldb_writebatch_iterate()
  |
  */
fn check_put(
    ptr:  *mut c_void,
    k:    *const u8,
    klen: usize,
    v:    *const u8,
    vlen: usize)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_put_entry",
        state_ptr = (ptr as usize),
        key_len = klen,
        value_len = vlen
    );

    check_condition!(!ptr.is_null());

    let state = ptr as *mut i32;

    unsafe {
        check_condition!(*state < 2);

        match *state {
            0 => {
                check_equal(b"bar\0".as_ptr(), k, klen);
                check_equal(b"b\0".as_ptr(), v, vlen);
            }
            1 => {
                check_equal(b"box\0".as_ptr(), k, klen);
                check_equal(b"c\0".as_ptr(), v, vlen);
            }
            _ => {
                check_condition!(false);
            }
        }

        *state += 1;
    }

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_put_exit"
    );
}

/**
  | Callback from leveldb_writebatch_iterate()
  |
  */
fn check_del(
    ptr:  *mut c_void,
    k:    *const u8,
    klen: usize)  {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_del_entry",
        state_ptr = (ptr as usize),
        key_len = klen
    );

    check_condition!(!ptr.is_null());

    let state = ptr as *mut i32;

    unsafe {
        check_condition!(*state == 2);
        check_equal(b"bar\0".as_ptr(), k, klen);
        *state += 1;
    }

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "check_del_exit"
    );
}

fn cmp_destroy(arg: *mut c_void)  {
    debug!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "cmp_destroy",
        arg = (arg as usize)
    );
}

fn cmp_compare(
    arg:  *mut c_void,
    a:    *const u8,
    alen: usize,
    b:    *const u8,
    blen: usize) -> i32 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "cmp_compare_entry",
        arg = (arg as usize),
        alen = alen,
        blen = blen
    );

    let a_bytes: &[u8] = if a.is_null() && alen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(a, alen) }
    };

    let b_bytes: &[u8] = if b.is_null() && blen == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(b, blen) }
    };

    let n = if alen < blen { alen } else { blen };
    let mut i = 0usize;
    let mut r = 0i32;

    while i < n {
        let av = a_bytes[i];
        let bv = b_bytes[i];

        if av < bv {
            r = -1;
            break;
        } else if av > bv {
            r = 1;
            break;
        }

        i += 1;
    }

    if r == 0 {
        if alen < blen {
            r = -1;
        } else if alen > blen {
            r = 1;
        } else {
            r = 0;
        }
    }

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "cmp_compare_exit",
        result = r
    );

    r
}

fn cmp_name(arg: *mut c_void) -> *const u8 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "cmp_name",
        arg = (arg as usize)
    );

    b"foo\0".as_ptr()
}

/* ------------- Custom filter policy  ------------- */
lazy_static!{
    /*
    static uint8_t fake_filter_result = 1;
    */
}

fn filter_destroy(arg: *mut c_void)  {
    debug!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "filter_destroy",
        arg = (arg as usize)
    );
}

fn filter_name(arg: *mut c_void) -> *const u8 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "filter_name",
        arg = (arg as usize)
    );

    b"TestFilter\0".as_ptr()
}

fn filter_create(
    arg:              *mut c_void,
    key_array:        *const *const u8,
    key_length_array: *const usize,
    num_keys:         i32,
    filter_length:    *mut usize) -> *mut u8 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "filter_create_entry",
        arg = (arg as usize),
        key_array = (key_array as usize),
        key_length_array = (key_length_array as usize),
        num_keys = num_keys,
        filter_length_ptr = (filter_length as usize)
    );

    unsafe {
        if !filter_length.is_null() {
            *filter_length = 4usize;
        }

        let result = libc::malloc(4usize) as *mut u8;
        if result.is_null() {
            error!(
                target: "bitcoinleveldbt_capi::c_test",
                event = "filter_create_malloc_failed"
            );

            if !filter_length.is_null() {
                *filter_length = 0usize;
            }

            return core::ptr::null_mut();
        }

        core::ptr::copy_nonoverlapping(b"fake".as_ptr(), result, 4usize);

        trace!(
            target: "bitcoinleveldbt_capi::c_test",
            event = "filter_create_exit",
            result_ptr = (result as usize)
        );

        result
    }
}

fn filter_key_match(
    arg:           *mut c_void,
    k:             *const u8,
    length:        usize,
    filter:        *const u8,
    filter_length: usize,
) -> u8 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "filter_key_match_entry",
        arg = (arg as usize),
        key_ptr = (k as usize),
        key_len = length,
        filter_ptr = (filter as usize),
        filter_len = filter_length
    );

    check_condition!(filter_length == 4usize);

    let filter_bytes = if filter.is_null() && filter_length == 0 {
        &[]
    } else {
        unsafe { core::slice::from_raw_parts(filter, filter_length) }
    };

    check_condition!(filter_bytes == b"fake");

    let result = BITCOINLEVELDB_TEST_C_TEST_FAKE_FILTER_RESULT.load(atomic::Ordering::SeqCst);

    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "filter_key_match_exit",
        result = result
    );

    result
}

fn dbc_test_main(
    _argc: i32,
    _argv: *mut *mut u8,
) -> i32 {
    trace!(
        target: "bitcoinleveldbt_capi::c_test",
        event = "dbc_test_main_entry"
    );

    unsafe {
        let mut db: *mut LevelDB = core::ptr::null_mut();
        let cmp: *mut LevelDBComparator;
        let cache: *mut LevelDBCache;
        let env: *mut LevelDBEnv;
        let options: *mut LevelDBOptions;
        let roptions: *mut LevelDBReadOptions;
        let woptions: *mut LevelDBWriteOptions;
        let mut dbname = unique_db_path("leveldb_c_api_roundtrip").into_bytes();
        let mut err: *mut u8 = core::ptr::null_mut();
        let mut run: i32 = -1;

        dbname.push(0u8);

        check_condition!(leveldb_major_version() >= 1);
        check_condition!(leveldb_minor_version() >= 1);

        start_phase(b"create_objects\0".as_ptr());
        cmp = leveldb_comparator_create(core::ptr::null_mut(), cmp_destroy, cmp_compare, cmp_name);
        env = leveldb_create_default_env();
        cache = leveldb_cache_create_lru(100000usize);

        check_condition!(!cmp.is_null());
        check_condition!(!env.is_null());
        check_condition!(!cache.is_null());

        let test_directory = leveldb_env_get_test_directory(env);
        check_condition!(!test_directory.is_null());
        leveldb_free(test_directory as *mut c_void);

        options = leveldb_options_create();
        leveldb_options_set_comparator(options, cmp);
        leveldb_options_set_error_if_exists(options, 1u8);
        leveldb_options_set_cache(options, cache);
        leveldb_options_set_env(options, env);
        leveldb_options_set_info_log(options, core::ptr::null_mut());
        leveldb_options_set_write_buffer_size(options, 100000usize);
        leveldb_options_set_paranoid_checks(options, 1u8);
        leveldb_options_set_max_open_files(options, 10);
        leveldb_options_set_block_size(options, 1024usize);
        leveldb_options_set_block_restart_interval(options, 8);
        leveldb_options_set_max_file_size(options, 3usize << 20);
        leveldb_options_set_compression(options, 0);

        roptions = leveldb_readoptions_create();
        leveldb_readoptions_set_verify_checksums(roptions, 1u8);
        leveldb_readoptions_set_fill_cache(roptions, 0u8);

        woptions = leveldb_writeoptions_create();
        leveldb_writeoptions_set_sync(woptions, 1u8);

        start_phase(b"destroy\0".as_ptr());
        leveldb_destroy_db(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
        free((&mut err) as *mut *mut u8);

        start_phase(b"open_error\0".as_ptr());
        db = leveldb_open(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
        check_condition!(!err.is_null());
        free((&mut err) as *mut *mut u8);

        start_phase(b"leveldb_free\0".as_ptr());
        db = leveldb_open(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
        check_condition!(!err.is_null());
        leveldb_free(err as *mut c_void);
        err = core::ptr::null_mut();

        start_phase(b"open\0".as_ptr());
        leveldb_options_set_create_if_missing(options, 1u8);
        db = leveldb_open(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
        check_no_error!(err);
        check_get(db, roptions, b"foo\0".as_ptr(), core::ptr::null());

        start_phase(b"put\0".as_ptr());
        leveldb_put(
            db,
            woptions,
            b"foo".as_ptr(),
            3usize,
            b"hello".as_ptr(),
            5usize,
            (&mut err) as *mut *mut u8,
        );
        check_no_error!(err);
        check_get(db, roptions, b"foo\0".as_ptr(), b"hello\0".as_ptr());

        start_phase(b"compactall\0".as_ptr());
        leveldb_compact_range(db, core::ptr::null(), 0usize, core::ptr::null(), 0usize);
        check_get(db, roptions, b"foo\0".as_ptr(), b"hello\0".as_ptr());

        start_phase(b"compactrange\0".as_ptr());
        leveldb_compact_range(db, b"a".as_ptr(), 1usize, b"z".as_ptr(), 1usize);
        check_get(db, roptions, b"foo\0".as_ptr(), b"hello\0".as_ptr());

        start_phase(b"writebatch\0".as_ptr());
        {
            let wb: *mut LevelDBWriteBatch = leveldb_writebatch_create();
            leveldb_writebatch_put(wb, b"foo".as_ptr(), 3usize, b"a".as_ptr(), 1usize);
            leveldb_writebatch_clear(wb);
            leveldb_writebatch_put(wb, b"bar".as_ptr(), 3usize, b"b".as_ptr(), 1usize);
            leveldb_writebatch_put(wb, b"box".as_ptr(), 3usize, b"c".as_ptr(), 1usize);

            let wb2: *mut LevelDBWriteBatch = leveldb_writebatch_create();
            leveldb_writebatch_delete(wb2, b"bar".as_ptr(), 3usize);
            leveldb_writebatch_append(wb, wb2);
            leveldb_writebatch_destroy(wb2);

            leveldb_write(db, woptions, wb, (&mut err) as *mut *mut u8);
            check_no_error!(err);
            check_get(db, roptions, b"foo\0".as_ptr(), b"hello\0".as_ptr());
            check_get(db, roptions, b"bar\0".as_ptr(), core::ptr::null());
            check_get(db, roptions, b"box\0".as_ptr(), b"c\0".as_ptr());

            let mut pos: i32 = 0;
            leveldb_writebatch_iterate(
                wb,
                (&mut pos as *mut i32).cast::<c_void>(),
                check_put,
                check_del,
            );
            check_condition!(pos == 3);
            leveldb_writebatch_destroy(wb);
        }

        start_phase(b"iter\0".as_ptr());
        {
            let iter: *mut LevelDBIterator = leveldb_create_iterator(db, roptions);
            check_condition!(leveldb_iter_valid(iter as *const LevelDBIterator) == 0u8);
            leveldb_iter_seek_to_first(iter);
            check_condition!(leveldb_iter_valid(iter as *const LevelDBIterator) != 0u8);
            check_iter(iter, b"box\0".as_ptr(), b"c\0".as_ptr());
            leveldb_iter_next(iter);
            check_iter(iter, b"foo\0".as_ptr(), b"hello\0".as_ptr());
            leveldb_iter_prev(iter);
            check_iter(iter, b"box\0".as_ptr(), b"c\0".as_ptr());
            leveldb_iter_prev(iter);
            check_condition!(leveldb_iter_valid(iter as *const LevelDBIterator) == 0u8);
            leveldb_iter_seek_to_last(iter);
            check_iter(iter, b"foo\0".as_ptr(), b"hello\0".as_ptr());
            leveldb_iter_seek(iter, b"b".as_ptr(), 1usize);
            check_iter(iter, b"box\0".as_ptr(), b"c\0".as_ptr());
            leveldb_iter_get_error(iter as *const LevelDBIterator, (&mut err) as *mut *mut u8);
            check_no_error!(err);
            leveldb_iter_destroy(iter);
        }

        start_phase(b"approximate_sizes\0".as_ptr());
        {
            let n: i32 = 20000;
            let mut sizes: [u64; 2] = [0u64; 2];
            let start0: [u8; 1] = [b'a'];
            let start1 = b"k00000000000000010000";
            let limit0 = b"k00000000000000010000";
            let limit1: [u8; 1] = [b'z'];

            let start: [*const u8; 2] = [start0.as_ptr(), start1.as_ptr()];
            let start_len: [usize; 2] = [1usize, 21usize];
            let limit: [*const u8; 2] = [limit0.as_ptr(), limit1.as_ptr()];
            let limit_len: [usize; 2] = [21usize, 1usize];

            leveldb_writeoptions_set_sync(woptions, 0u8);

            let mut i: i32 = 0;
            while i < n {
                let keybuf = format!("k{:020}", i);
                let valbuf = format!("v{:020}", i);

                leveldb_put(
                    db,
                    woptions,
                    keybuf.as_bytes().as_ptr(),
                    keybuf.as_bytes().len(),
                    valbuf.as_bytes().as_ptr(),
                    valbuf.as_bytes().len(),
                    (&mut err) as *mut *mut u8,
                );
                check_no_error!(err);

                i += 1;
            }

            leveldb_approximate_sizes(
                db,
                2,
                start.as_ptr(),
                start_len.as_ptr(),
                limit.as_ptr(),
                limit_len.as_ptr(),
                sizes.as_mut_ptr(),
            );

            check_condition!(sizes[0] > 0u64);
            check_condition!(sizes[1] > 0u64);
        }

        start_phase(b"property\0".as_ptr());
        {
            let mut prop: *mut u8 = leveldb_property_value(db, b"nosuchprop\0".as_ptr());
            check_condition!(prop.is_null());

            prop = leveldb_property_value(db, b"leveldb.stats\0".as_ptr());
            check_condition!(!prop.is_null());
            free((&mut prop) as *mut *mut u8);
        }

        start_phase(b"snapshot\0".as_ptr());
        {
            let snap: *const LevelDBSnapshot = leveldb_create_snapshot(db);
            leveldb_delete(
                db,
                woptions,
                b"foo".as_ptr(),
                3usize,
                (&mut err) as *mut *mut u8,
            );
            check_no_error!(err);
            leveldb_readoptions_set_snapshot(roptions, snap);
            check_get(db, roptions, b"foo\0".as_ptr(), b"hello\0".as_ptr());
            leveldb_readoptions_set_snapshot(roptions, core::ptr::null());
            check_get(db, roptions, b"foo\0".as_ptr(), core::ptr::null());
            leveldb_release_snapshot(db, snap);
        }

        start_phase(b"repair\0".as_ptr());
        {
            leveldb_close(db);
            db = core::ptr::null_mut();

            leveldb_options_set_create_if_missing(options, 0u8);
            leveldb_options_set_error_if_exists(options, 0u8);
            leveldb_repair_db(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
            check_no_error!(err);

            db = leveldb_open(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
            check_no_error!(err);
            check_get(db, roptions, b"foo\0".as_ptr(), core::ptr::null());
            check_get(db, roptions, b"bar\0".as_ptr(), core::ptr::null());
            check_get(db, roptions, b"box\0".as_ptr(), b"c\0".as_ptr());
            leveldb_options_set_create_if_missing(options, 1u8);
            leveldb_options_set_error_if_exists(options, 1u8);
        }

        start_phase(b"filter\0".as_ptr());
        run = 0;
        while run < 2 {
            check_no_error!(err);

            let policy: *mut LevelDBFilterPolicy = if run == 0 {
                leveldb_filterpolicy_create(
                    core::ptr::null_mut(),
                    filter_destroy,
                    filter_create,
                    filter_key_match,
                    filter_name,
                )
            } else {
                leveldb_filterpolicy_create_bloom(10)
            };

            leveldb_close(db);
            db = core::ptr::null_mut();

            leveldb_destroy_db(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
            leveldb_options_set_filter_policy(options, policy);
            db = leveldb_open(options, dbname.as_ptr(), (&mut err) as *mut *mut u8);
            check_no_error!(err);

            leveldb_put(
                db,
                woptions,
                b"foo".as_ptr(),
                3usize,
                b"foovalue".as_ptr(),
                8usize,
                (&mut err) as *mut *mut u8,
            );
            check_no_error!(err);

            leveldb_put(
                db,
                woptions,
                b"bar".as_ptr(),
                3usize,
                b"barvalue".as_ptr(),
                8usize,
                (&mut err) as *mut *mut u8,
            );
            check_no_error!(err);

            leveldb_compact_range(db, core::ptr::null(), 0usize, core::ptr::null(), 0usize);

            BITCOINLEVELDB_TEST_C_TEST_FAKE_FILTER_RESULT.store(1u8, atomic::Ordering::SeqCst);
            check_get(db, roptions, b"foo\0".as_ptr(), b"foovalue\0".as_ptr());
            check_get(db, roptions, b"bar\0".as_ptr(), b"barvalue\0".as_ptr());

            if BITCOINLEVELDB_TEST_C_TEST_PHASE_PTR
                .load(atomic::Ordering::SeqCst)
                .is_null()
            {
                BITCOINLEVELDB_TEST_C_TEST_FAKE_FILTER_RESULT.store(0u8, atomic::Ordering::SeqCst);
                check_get(db, roptions, b"foo\0".as_ptr(), core::ptr::null());
                check_get(db, roptions, b"bar\0".as_ptr(), core::ptr::null());
                BITCOINLEVELDB_TEST_C_TEST_FAKE_FILTER_RESULT.store(1u8, atomic::Ordering::SeqCst);
                check_get(db, roptions, b"foo\0".as_ptr(), b"foovalue\0".as_ptr());
                check_get(db, roptions, b"bar\0".as_ptr(), b"barvalue\0".as_ptr());
            }

            leveldb_options_set_filter_policy(options, core::ptr::null_mut());
            leveldb_filterpolicy_destroy(policy);

            run += 1;
        }

        start_phase(b"cleanup\0".as_ptr());
        leveldb_close(db);
        leveldb_options_destroy(options);
        leveldb_readoptions_destroy(roptions);
        leveldb_writeoptions_destroy(woptions);
        leveldb_cache_destroy(cache);
        leveldb_comparator_destroy(cmp);
        leveldb_env_destroy(env);

        eprintln!("PASS");

        trace!(
            target: "bitcoinleveldbt_capi::c_test",
            event = "dbc_test_main_exit",
            result = 0
        );

        0
    }
}

#[traced_test]
fn bitcoinleveldb_test__c_test_rs__upstream_c_api_roundtrip_passes() {
    let rc = dbc_test_main(0i32, core::ptr::null_mut());
    assert_eq!(rc, 0i32);
}
