#![cfg(test)]

crate::ix!();

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn unique_dbname(tag: &str) -> String {
    let nanos: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();

    let mut p = std::env::temp_dir();
    p.push(format!("bitcoinleveldb_dbimpl_{}_{}", tag, nanos));
    p.to_string_lossy().to_string()
}

pub(crate) fn default_test_options() -> Options {
    let mut o: Options = Default::default();

    // Keep defaults unless explicitly needed by tests.
    o.create_if_missing = true;
    o.error_if_exists = false;
    o.paranoid_checks = true;
    o.reuse_logs = false;

    // Smaller buffer helps exercise memtable flushing paths.
    o.write_buffer_size = 64 * 1024;

    o
}

pub(crate) fn remove_db_dir_best_effort(dbname: &String) {
    let _ = fs::remove_dir_all(dbname);
}

pub(crate) fn open_dbimpl_for_test(tag: &str) -> (String, Box<DBImpl>) {
    let dbname: String = unique_dbname(tag);
    remove_db_dir_best_effort(&dbname);

    let opts: Options = default_test_options();

    let mut db: Box<DBImpl> = Box::new(DBImpl::new(&opts, &dbname));

    db.mutex_.lock();

    let mut edit: VersionEdit = Default::default();
    let mut save_manifest: bool = false;

    let mut s: Status = db.recover(
        (&mut edit) as *mut VersionEdit,
        (&mut save_manifest) as *mut bool,
    );

    if s.is_ok() && db.mem_.is_null() {
        // Create new log and a corresponding memtable.
        let new_log_number: u64 = unsafe { (*db.versions_).new_file_number() };

        let mut lfile: *mut dyn WritableFile = core::ptr::null_mut();
        let fname: String = log_file_name(&dbname, new_log_number);

        s = db.env_.borrow_mut().new_writable_file(&fname, &mut lfile);

        if s.is_ok() {
            edit.set_log_number(new_log_number);

            db.logfile_ = lfile;
            db.logfile_number_ = new_log_number;

            db.log_ = Box::into_raw(Box::new(LogWriter::new(db.logfile_)));

            db.mem_ = Box::into_raw(Box::new(MemTable::new(&db.internal_comparator_)));
            unsafe {
                (*db.mem_).ref_();
            }
        }
    }

    if s.is_ok() && save_manifest {
        edit.set_prev_log_number(0);
        edit.set_log_number(db.logfile_number_);
        s = unsafe { (*db.versions_).log_and_apply(&mut edit, &mut db.mutex_) };
    }

    if s.is_ok() {
        db.delete_obsolete_files();
        db.maybe_schedule_compaction();
    }

    db.mutex_.unlock();

    assert!(s.is_ok(), "open_dbimpl_for_test failed: {}", s.to_string());
    assert!(!db.mem_.is_null(), "memtable should be initialized");

    (dbname, db)
}

pub(crate) fn reopen_dbimpl_for_test(dbname: &String, mut opts: Options) -> Box<DBImpl> {
    // On reopen we do not require create_if_missing; keep behavior explicit.
    opts.create_if_missing = false;
    opts.error_if_exists = false;

    let mut db: Box<DBImpl> = Box::new(DBImpl::new(&opts, dbname));

    db.mutex_.lock();

    let mut edit: VersionEdit = Default::default();
    let mut save_manifest: bool = false;

    let mut s: Status = db.recover(
        (&mut edit) as *mut VersionEdit,
        (&mut save_manifest) as *mut bool,
    );

    if s.is_ok() && db.mem_.is_null() {
        let new_log_number: u64 = unsafe { (*db.versions_).new_file_number() };

        let mut lfile: *mut dyn WritableFile = core::ptr::null_mut();
        let fname: String = log_file_name(dbname, new_log_number);

        s = db.env_.borrow_mut().new_writable_file(&fname, &mut lfile);

        if s.is_ok() {
            edit.set_log_number(new_log_number);

            db.logfile_ = lfile;
            db.logfile_number_ = new_log_number;

            db.log_ = Box::into_raw(Box::new(LogWriter::new(db.logfile_)));

            db.mem_ = Box::into_raw(Box::new(MemTable::new(&db.internal_comparator_)));
            unsafe {
                (*db.mem_).ref_();
            }
        }
    }

    if s.is_ok() && save_manifest {
        edit.set_prev_log_number(0);
        edit.set_log_number(db.logfile_number_);
        s = unsafe { (*db.versions_).log_and_apply(&mut edit, &mut db.mutex_) };
    }

    if s.is_ok() {
        db.delete_obsolete_files();
        db.maybe_schedule_compaction();
    }

    db.mutex_.unlock();

    assert!(s.is_ok(), "reopen_dbimpl_for_test failed: {}", s.to_string());
    assert!(!db.mem_.is_null(), "memtable should be initialized on reopen");

    db
}

pub(crate) fn write_kv(db: &mut DBImpl, key: &str, val: &str) -> Status {
    let mut batch: WriteBatch = Default::default();
    batch.put(&Slice::from_str(key), &Slice::from_str(val));

    let mut boxed: Box<WriteBatch> = Box::new(batch);
    let s: Status = <DBImpl as DBWrite>::write(
        db,
        &WriteOptions::default(),
        (&mut *boxed) as *mut WriteBatch,
    );

    tracing::debug!(key = %key, status = %s.to_string(), "write_kv");
    s
}

pub(crate) fn delete_key(db: &mut DBImpl, key: &str) -> Status {
    let mut batch: WriteBatch = Default::default();
    batch.delete(&Slice::from_str(key));

    let mut boxed: Box<WriteBatch> = Box::new(batch);
    let s: Status = <DBImpl as DBWrite>::write(
        db,
        &WriteOptions::default(),
        (&mut *boxed) as *mut WriteBatch,
    );

    tracing::debug!(key = %key, status = %s.to_string(), "delete_key");
    s
}

pub(crate) fn read_value(db: &mut DBImpl, ro: &ReadOptions, key: &str) -> (Status, String) {
    let mut out: String = String::new();
    let s: Status = <DBImpl as DBGet>::get(db, ro, &Slice::from_str(key), (&mut out) as *mut String);
    tracing::debug!(key = %key, status = %s.to_string(), value_len = out.len(), "read_value");
    (s, out)
}

pub(crate) fn assert_read_eq(db: &mut DBImpl, key: &str, expected: &str) {
    let ro: ReadOptions = Default::default();
    let (s, v) = read_value(db, &ro, key);
    assert!(s.is_ok(), "read failed for key={}: {}", key, s.to_string());
    assert_eq!(v, expected, "unexpected value for key={}", key);
}

pub(crate) fn fill_sequential(db: &mut DBImpl, prefix: &str, n: usize, value_len: usize) {
    let val: String = "v".repeat(value_len);
    for i in 0..n {
        let k = format!("{}{:08}", prefix, i);
        let s = write_kv(db, &k, &val);
        assert!(s.is_ok(), "fill write failed: {}", s.to_string());
    }
    tracing::info!(prefix = %prefix, n, value_len, "fill_sequential done");
}

pub(crate) fn force_manual_compaction_full_range(db: &mut DBImpl) {
    // Use the public trait entry-point where available.
    let begin: *const Slice = core::ptr::null();
    let end: *const Slice = core::ptr::null();
    <DBImpl as CompactRange>::compact_range(db, begin, end);
    tracing::info!("force_manual_compaction_full_range done");
}

