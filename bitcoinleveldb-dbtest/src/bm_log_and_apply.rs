// ---------------- [ File: bitcoinleveldb-dbtest/src/bm_log_and_apply.rs ]
crate::ix!();

/// Invariant: the returned key is exactly 16 ASCII decimal digits, left-padded with `0`.
/// Postcondition: `make_key(num).len() == 16` for all `num`.
pub fn make_key(num: u32) -> String {
    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "make_key.entry",
        num
    );

    let s = format!("{:016}", num);

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "make_key.exit",
        num,
        out_len = s.len()
    );

    s
}

pub fn bm_log_and_apply(
    iters:          i32,
    num_base_files: i32)  {

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "bm_log_and_apply.entry",
        iters,
        num_base_files
    );

    use lock_api::RawMutex as _;

    let mut dbname = bitcoinleveldb_test::tmp_dir();
    dbname.push_str("/leveldb_test_benchmark");
    let _ = destroy_db(&dbname, &Options::default());

    let mut db_slot: MaybeUninit<*mut dyn DB> = MaybeUninit::uninit();
    let mut open_opts = Options::default();
    open_opts.set_create_if_missing(true);

    let mut opener = DBImpl::new(&open_opts, &dbname);
    let s = opener.open(&open_opts, &dbname, db_slot.as_mut_ptr());
    assert!(s.is_ok());

    let db_ptr = unsafe { db_slot.assume_init() };
    assert!(!db_ptr.is_null());

    unsafe {
        drop(Box::from_raw(db_ptr));
    }

    let env = PosixEnv::shared();

    let mut mu = RawMutex::INIT;
    mu.lock();

    let options = Options::with_env(env.clone());
    let cmp = InternalKeyComparator::new(bytewise_comparator());
    let mut vset = VersionSet::new(
        &dbname,
        (&options) as *const Options,
        null_mut(),
        (&cmp) as *const InternalKeyComparator,
    );

    let mut save_manifest = false;
    assert!(vset.recover(&mut save_manifest as *mut bool).is_ok());

    let mut vbase = VersionEdit::default();
    let mut fnum: u64 = 1;

    let mut i: i32 = 0;
    while i < num_base_files {
        let start_key = make_key((2 * fnum) as u32);
        let limit_key = make_key((2 * fnum + 1) as u32);

        let start_slice = Slice::from(&start_key);
        let limit_slice = Slice::from(&limit_key);

        let start = InternalKey::new(&start_slice, 1 as SequenceNumber, ValueType::TypeValue);
        let limit = InternalKey::new(&limit_slice, 1 as SequenceNumber, ValueType::TypeDeletion);

        vbase.add_file(2, fnum, 1, &start, &limit);
        fnum += 1;
        i += 1;
    }

    assert!(vset.log_and_apply(&mut vbase as *mut VersionEdit, &mut mu as *mut RawMutex).is_ok());

    let start_micros = env.borrow_mut().now_micros();

    let mut iter: i32 = 0;
    while iter < iters {
        let mut vedit = VersionEdit::default();
        vedit.delete_file(2, fnum);

        let start_key = make_key((2 * fnum) as u32);
        let limit_key = make_key((2 * fnum + 1) as u32);

        let start_slice = Slice::from(&start_key);
        let limit_slice = Slice::from(&limit_key);

        let start = InternalKey::new(&start_slice, 1 as SequenceNumber, ValueType::TypeValue);
        let limit = InternalKey::new(&limit_slice, 1 as SequenceNumber, ValueType::TypeDeletion);

        vedit.add_file(2, fnum, 1, &start, &limit);
        assert!(vset.log_and_apply(&mut vedit as *mut VersionEdit, &mut mu as *mut RawMutex).is_ok());

        fnum += 1;
        iter += 1;
    }

    let stop_micros = env.borrow_mut().now_micros();
    let us = (stop_micros - start_micros) as u32;

    eprintln!(
        "BM_LogAndApply/{:<6}   {:8} iters : {:9} us ({:7.0} us / iter)",
        format!("{}", num_base_files),
        iters,
        us,
        (us as f32) / (iters as f32)
    );

    unsafe {
        mu.unlock();
    }

    trace!(
        target: "bitcoinleveldb-dbtest",
        label = "bm_log_and_apply.exit",
        iters,
        num_base_files,
        elapsed_us = us
    );
}

/// Precondition: `argv` is either null or points to an array of NUL-terminated C strings.
/// Postcondition: returns the benchmark exit code for `--benchmark`, otherwise delegates to the test harness.
/// Side effects: may execute the benchmark path or the registered LevelDB test harness.
pub fn dbdb_test_main(
    argc: i32,
    argv: *mut *mut u8,
) -> i32 {
    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.entry",
        argc
    );

    let benchmark_requested = match argc > 1 {
        true => match argv.is_null() {
            true => false,
            false => unsafe {
                let arg1 = *argv.add(1);
                match arg1.is_null() {
                    true => false,
                    false => CStr::from_ptr(arg1 as *const c_char).to_bytes() == b"--benchmark",
                }
            },
        },
        false => false,
    };

    let rc = match benchmark_requested {
        true => {
            bm_log_and_apply(1000, 1);
            bm_log_and_apply(1000, 100);
            bm_log_and_apply(1000, 10000);
            bm_log_and_apply(100, 100000);
            0
        }
        false => bitcoinleveldb_test::run_all_tests(),
    };

    tracing::trace!(
        target: "bitcoinleveldb-dbtest",
        label = "dbdb_test_main.exit",
        benchmark_requested,
        rc
    );

    rc
}
