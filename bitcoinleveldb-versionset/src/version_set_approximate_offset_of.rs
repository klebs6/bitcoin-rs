// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_approximate_offset_of.rs ]
crate::ix!();

impl ApproximateOffsetOf for VersionSet {

    /// Return the approximate offset in the database of the data for "key" as of version "v".
    ///
    fn approximate_offset_of(
        &mut self,
        v:     *mut Version,
        ikey_: &InternalKey
    ) -> u64 {

        trace!(
            "VersionSet::approximate_offset_of: enter; v={:p} ikey_len={}",
            v,
            ikey_.encode().size()
        );

        assert!(!v.is_null(), "VersionSet::approximate_offset_of: v is null");

        let mut result: u64 = 0;

        for level in 0..(NUM_LEVELS as i32) {

            unsafe {
                let vref: &mut Version = &mut *v;
                let files: &Vec<*mut FileMetaData> = &vref.files()[level as usize];

                trace!(
                    "VersionSet::approximate_offset_of: level={} files_len={}",
                    level,
                    files.len()
                );

                for (i, &fptr) in files.iter().enumerate() {

                    assert!(
                        !fptr.is_null(),
                        "VersionSet::approximate_offset_of: null FileMetaData pointer at level {} index {}",
                        level,
                        i
                    );

                    let f: &FileMetaData = &*fptr;

                    let cmp_largest = self.icmp().compare_internal_key(f.largest(), ikey_);
                    if cmp_largest <= 0 {
                        // Entire file is before "ikey", so just add the file size
                        result = result.saturating_add(*f.file_size());
                        trace!(
                            "VersionSet::approximate_offset_of: level={} idx={} file={} before_key -> add file_size={} result={}",
                            level,
                            i,
                            *f.number(),
                            *f.file_size(),
                            result
                        );
                        continue;
                    }

                    let cmp_smallest = self.icmp().compare_internal_key(f.smallest(), ikey_);
                    if cmp_smallest > 0 {
                        // Entire file is after "ikey", so ignore
                        trace!(
                            "VersionSet::approximate_offset_of: level={} idx={} file={} after_key; level0_special={} -> {}",
                            level,
                            i,
                            *f.number(),
                            (level == 0),
                            if level > 0 { "break" } else { "continue" }
                        );
                        if level > 0 {
                            // Files other than level 0 are sorted by meta->smallest, so
                            // no further files in this level will contain data for
                            // "ikey".
                            break;
                        }
                        continue;
                    }

                    // "ikey" falls in the range for this table.  Add the
                    // approximate offset of "ikey" within the table.
                    let tc_ptr: *mut TableCache = self.table_cache() as *mut TableCache;
                    assert!(
                        !tc_ptr.is_null(),
                        "VersionSet::approximate_offset_of: table_cache is null"
                    );

                    let mut tableptr: *mut bitcoinleveldb_table::Table = core::ptr::null_mut();

                    let it_ptr: *mut LevelDBIterator = (*tc_ptr).new_iterator(
                        &ReadOptions::default(),
                        *f.number(),
                        *f.file_size(),
                        &mut tableptr as *mut *mut bitcoinleveldb_table::Table,
                    );

                    if !tableptr.is_null() {
                        let key_slice = ikey_.encode();
                        // LevelDB Table API: ApproximateOffsetOf(key)
                        let add = (*tableptr).approximate_offset_of(&key_slice);
                        result = result.saturating_add(add);

                        trace!(
                            "VersionSet::approximate_offset_of: level={} idx={} file={} in_range -> table_offset={} result={}",
                            level,
                            i,
                            *f.number(),
                            add,
                            result
                        );
                    } else {
                        trace!(
                            "VersionSet::approximate_offset_of: level={} idx={} file={} in_range but tableptr=null; no add",
                            level,
                            i,
                            *f.number()
                        );
                    }

                    if !it_ptr.is_null() {
                        drop(Box::from_raw(it_ptr));
                    }
                }
            }
        }

        trace!(
            "VersionSet::approximate_offset_of: exit; result={}",
            result
        );

        result
    }
}

#[cfg(test)]
mod version_set_approximate_offset_of_exhaustive_test_suite {
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
            unsafe {
                (*mu).lock();
            }
            Self { mu }
        }
    }

    impl Drop for RawMutexTestGuard {
        fn drop(&mut self) {
            trace!(mu_ptr = %format!("{:p}", self.mu), "RawMutexTestGuard::drop (unlock)");
            unsafe {
                (*self.mu).unlock();
            }
        }
    }

    #[traced_test]
    fn approximate_offset_is_zero_for_empty_versions() {
        let dir = make_unique_temp_db_dir("versionset_approx_offset_empty");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let mut options = Box::new(Options::default());
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        let v = vs.current();
        assert!(!v.is_null(), "current version must not be null after recover");

        let ikey = make_ikey("any", 1);
        let off = vs.approximate_offset_of(v, &ikey);
        debug!(off, "approximate_offset_of on empty db");
        assert_eq!(off, 0, "empty db must yield offset 0");

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn approximate_offset_sums_file_sizes_when_tables_missing() {
        let dir = make_unique_temp_db_dir("versionset_approx_offset_missing_tables");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let mut options = Box::new(Options::default());
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 128));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st0 = vs.recover(&mut save_manifest as *mut bool);
        assert_status_ok(&st0, "recover");

        let mut mu = Box::new(RawMutex::INIT);
        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        let mut e1 = VersionEdit::default();
        e1.add_file(
            1,
            vs.new_file_number(),
            100,
            &make_ikey("a", 1),
            &make_ikey("k", 1),
        );
        assert_status_ok(
            &vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e1",
        );

        let mut e2 = VersionEdit::default();
        e2.add_file(
            1,
            vs.new_file_number(),
            200,
            &make_ikey("l", 1),
            &make_ikey("z", 1),
        );
        assert_status_ok(
            &vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex),
            "log_and_apply e2",
        );

        let v = vs.current();
        assert!(!v.is_null(), "current version must not be null");

        let ik1 = make_ikey("b", 1);
        let off1 = vs.approximate_offset_of(v, &ik1);
        debug!(off1, "offset for key in first file");
        assert!(
            off1 <= 100,
            "when table open fails, offset should conservatively reach end-of-file (<= 100) for first file"
        );

        let ik2 = make_ikey("m", 1);
        let off2 = vs.approximate_offset_of(v, &ik2);
        debug!(off2, "offset for key in second file");
        assert!(off2 >= 100, "offset for second file key must include at least first file size");

        let ik3 = make_ikey("zzzz", 1);
        let off3 = vs.approximate_offset_of(v, &ik3);
        debug!(off3, "offset for key after last file");
        assert!(
            off3 >= 300,
            "offset after last file should be at least the sum of file sizes (300)"
        );

        remove_dir_all_best_effort(&dir);
    }
}
