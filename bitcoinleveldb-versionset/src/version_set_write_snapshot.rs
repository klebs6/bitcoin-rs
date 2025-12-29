// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_write_snapshot.rs ]
crate::ix!();

impl WriteSnapshot for VersionSet {

    /// Save current contents to *log
    /// 
    fn write_snapshot(&mut self, log: *mut LogWriter) -> Status {
        trace!("VersionSet::write_snapshot(log): enter; log={:p}", log);

        assert!(!log.is_null(), "VersionSet::write_snapshot(log): log is null");

        let cur: *mut Version = self.current();

        assert!(
            !cur.is_null(),
            "VersionSet::write_snapshot(log): current is null"
        );

        let mut edit = VersionEdit::default();

        let cmp_name_owned: String = unsafe {
            let ucmp_ptr = self.icmp().user_comparator();
            if ucmp_ptr.is_null() {
                String::new()
            } else {
                match (*ucmp_ptr).name() {
                    Cow::Borrowed(s) => s.to_owned(),
                    Cow::Owned(s) => s,
                }
            }
        };

        let cmp_name_slice = Slice::from(cmp_name_owned.as_bytes());
        edit.set_comparator_name(&cmp_name_slice);

        for level in 0..(NUM_LEVELS as i32) {
            let cp_string: String = self.compact_pointer_mut()[level as usize].clone();
            if !cp_string.is_empty() {
                let mut key = InternalKey::default();

                unsafe {
                    let cp_slice = Slice::from_ptr_len(cp_string.as_ptr(), cp_string.len());
                    let _ = key.decode_from(&cp_slice);
                }

                edit.set_compact_pointer(level, &key);
            }
        }

        unsafe {
            let v: &mut Version = &mut *cur;

            for level in 0..(NUM_LEVELS as i32) {
                let files = &v.files()[level as usize];

                for &fptr in files.iter() {
                    assert!(
                        !fptr.is_null(),
                        "VersionSet::write_snapshot(log): null FileMetaData pointer at level {}",
                        level
                    );

                    let f: &FileMetaData = &*fptr;

                    edit.add_file(
                        level,
                        *f.number(),
                        *f.file_size(),
                        f.smallest(),
                        f.largest(),
                    );
                }
            }
        }

        let mut record = String::new();
        edit.encode_to(&mut record as *mut String);

        let rec_slice = Slice::from(record.as_bytes());
        let st = unsafe { (*log).add_record(&rec_slice) };

        trace!("VersionSet::write_snapshot(log): exit; ok={}", st.is_ok());

        st
    }
}

impl VersionSet {

    pub fn write_snapshot(&mut self) -> Status {
        let log_ptr: *mut LogWriter = self.descriptor_log();

        trace!(
            descriptor_log_ptr = %format!("{:p}", log_ptr),
            "VersionSet::write_snapshot: wrapper enter"
        );

        if log_ptr.is_null() {
            let msg = Slice::from("descriptor_log is null");
            error!(
                "VersionSet::write_snapshot: {}", 
                Status::invalid_argument(&msg, None).to_string()
            );
            return Status::invalid_argument(&msg, None);
        }

        <VersionSet as WriteSnapshot>::write_snapshot(self, log_ptr)
    }
}

#[cfg(test)]
mod version_set_write_snapshot_exhaustive_test_suite {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, error, info, trace, warn};

    fn make_unique_temp_db_dir(prefix: &str) -> PathBuf {
        let pid = std::process::id();
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);

        let mut p = std::env::temp_dir();
        p.push(format!("{prefix}_{pid}_{nanos}"));
        p
    }

    fn remove_dir_all_best_effort(dir: &Path) {
        match std::fs::remove_dir_all(dir) {
            Ok(()) => trace!(dir = %dir.display(), "removed temp db dir"),
            Err(e) => warn!(dir = %dir.display(), error = ?e, "failed to remove temp db dir (best effort)"),
        }
    }

    fn assert_status_ok(st: &Status, context: &'static str) {
        if !st.is_ok() {
            error!(?st, context, "unexpected non-ok Status");
            panic!("unexpected non-ok Status in {context}");
        }
        trace!(context, "Status OK");
    }

    fn make_ikey(user_key: &str, seq: u64) -> InternalKey {
        InternalKey::new(&Slice::from(user_key), seq, ValueType::TypeValue)
    }

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    struct RawMutexTestGuard {
        mu: *mut RawMutex,
    }

    impl RawMutexTestGuard {
        fn lock(mu: *mut RawMutex) -> Self {
            trace!(mu_ptr = %format!("{:p}", mu), "RawMutexTestGuard::lock");
            unsafe { (*mu).lock() };
            Self { mu }
        }
    }

    impl Drop for RawMutexTestGuard {
        fn drop(&mut self) {
            trace!(mu_ptr = %format!("{:p}", self.mu), "RawMutexTestGuard::drop (unlock)");
            unsafe { (*self.mu).unlock() };
        }
    }

    #[traced_test]
    fn write_snapshot_then_recover_preserves_file_state() {
        let dir = make_unique_temp_db_dir("versionset_write_snapshot_preserves_state");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let mut options = Box::new(Options::default());
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));
        let mut mu = Box::new(RawMutex::INIT);

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        let mut edit = VersionEdit::default();
        let fnum = vs.new_file_number();
        edit.add_file(2, fnum, 555, &make_ikey("aa", 1), &make_ikey("zz", 1));

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);
        let st1 = vs.log_and_apply(&mut edit as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
        assert_status_ok(&st1, "log_and_apply");

        let st_snap = vs.write_snapshot();
        info!(status = ?st_snap, "write_snapshot");
        assert_status_ok(&st_snap, "write_snapshot");

        let mut options2 = Box::new(Options::default());
        options2.set_create_if_missing(false);
        options2.set_error_if_exists(false);

        let icmp2 = Box::new(make_internal_key_comparator_from_options(options2.as_ref()));

        let mut table_cache2 = Box::new(TableCache::new(dbname.as_ref(), options2.as_ref(), 128));

        let mut vs2 = VersionSet::new(
            dbname.as_ref(),
            options2.as_ref(),
            table_cache2.as_mut() as *mut TableCache,
            icmp2.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest2: bool = false;
        let st2 = vs2.recover(&mut save_manifest2 as *mut bool);
        assert_status_ok(&st2, "recover after snapshot");

        let n2 = vs2.num_level_files(2);
        debug!(n2, "num_level_files(2) after snapshot+recover");
        assert!(n2 >= 1, "expected at least one L2 file after snapshot+recover");

        remove_dir_all_best_effort(&dir);
    }
}
