// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_log_and_apply.rs ]
crate::ix!();

impl VersionEditLogAndApply for VersionSet {
    /// Apply *edit to the current version to form a new descriptor that is both saved to
    /// persistent state and installed as the new current version.  Will release *mu while actually
    /// writing to the file.
    /// 
    /// REQUIRES: *mu is held on entry.
    /// 
    /// REQUIRES: no other thread concurrently calls LogAndApply()
    ///
    #[EXCLUSIVE_LOCKS_REQUIRED(mu)]
    fn log_and_apply(&mut self, edit: *mut VersionEdit, mu: *mut RawMutex) -> Status {
        let tid = std::thread::current().id();
        let t_enter = std::time::Instant::now();

        tracing::trace!(
            ?tid,
            edit_ptr = edit as usize,
            mu_ptr = mu as usize,
            current_ptr = self.current() as usize,
            "VersionSet::log_and_apply: enter"
        );

        assert!(!edit.is_null(), "VersionSet::log_and_apply: edit is null");
        assert!(!mu.is_null(), "VersionSet::log_and_apply: mu is null");
        assert!(
            !self.current().is_null(),
            "VersionSet::log_and_apply: current is null"
        );

        unsafe {
            let e: &mut VersionEdit = &mut *edit;

            if *e.has_log_number() {
                assert!(
                    *e.log_number() >= self.log_number(),
                    "VersionSet::log_and_apply: edit.log_number < log_number_"
                );
                assert!(
                    *e.log_number() < self.next_file_number(),
                    "VersionSet::log_and_apply: edit.log_number >= next_file_number_"
                );
            } else {
                e.set_log_number(self.log_number());
            }

            if !*e.has_prev_log_number() {
                e.set_prev_log_number(self.prev_log_number());
            }

            e.set_next_file(self.next_file_number());
            e.set_last_sequence(self.last_sequence());

            // Version* v = new Version(this);
            let vset_iface_ptr: *mut dyn VersionSetInterface =
                (self as &mut dyn VersionSetInterface) as *mut dyn VersionSetInterface;

            let v_files: [Vec<*mut FileMetaData>; NUM_LEVELS] =
                core::array::from_fn(|_| Vec::new());

            let v_ptr: *mut Version = Box::into_raw(Box::new(
                VersionBuilder::default()
                    .vset(vset_iface_ptr)
                    .next(core::ptr::null_mut())
                    .prev(core::ptr::null_mut())
                    .refs(0)
                    .files(v_files)
                    .file_to_compact(core::ptr::null_mut())
                    .file_to_compact_level(-1)
                    .compaction_score(-1.0)
                    .compaction_level(-1)
                    .build()
                    .unwrap(),
            ));

            {
                // Builder builder(this, current_);
                tracing::trace!(
                    ?tid,
                    v_ptr = v_ptr as usize,
                    base_ptr = self.current() as usize,
                    "VersionSet::log_and_apply: constructing builder/applying edit"
                );

                let mut builder = VersionSetBuilder::new(self, self.current());
                builder.apply(edit);

                tracing::trace!(
                    ?tid,
                    v_ptr = v_ptr as usize,
                    "VersionSet::log_and_apply: builder.save_to begin"
                );
                builder.save_to(v_ptr);
                tracing::trace!(
                    ?tid,
                    v_ptr = v_ptr as usize,
                    "VersionSet::log_and_apply: builder.save_to end"
                );
            }

            self.finalize(v_ptr);

            // Initialize new descriptor log file if necessary by creating
            // a temporary file that contains a snapshot of the current version.
            let mut new_manifest_file: String = String::new();
            let mut s: Status = Status::ok();

            if self.descriptor_log().is_null() {
                tracing::trace!(
                    ?tid,
                    dbname = %self.dbname(),
                    "VersionSet::log_and_apply: descriptor_log is null; creating new MANIFEST"
                );

                // No reason to unlock *mu here since we only hit this path in the
                // first call to LogAndApply (when opening the database).
                assert!(
                    self.descriptor_file().is_null(),
                    "VersionSet::log_and_apply: descriptor_log is null but descriptor_file is not null"
                );

                new_manifest_file =
                    descriptor_file_name(self.dbname(), self.manifest_file_number());
                e.set_next_file(self.next_file_number());

                let env_rc = (*self.options())
                    .env()
                    .as_ref()
                    .expect("VersionSet::log_and_apply: Options.env is None")
                    .clone();

                let mut file_box_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

                tracing::info!(
                    ?tid,
                    dbname = %self.dbname(),
                    manifest = %new_manifest_file,
                    "VersionSet::log_and_apply: Env::new_writable_file begin"
                );

                let t_create_manifest = std::time::Instant::now();
                s = env_rc
                    .borrow_mut()
                    .new_writable_file(&new_manifest_file, &mut file_box_ptr);

                tracing::info!(
                    ?tid,
                    dbname = %self.dbname(),
                    manifest = %new_manifest_file,
                    status_ok = s.is_ok(),
                    status = %s.to_string(),
                    elapsed_ms = t_create_manifest.elapsed().as_millis() as u64,
                    "VersionSet::log_and_apply: Env::new_writable_file end"
                );

                if s.is_ok() {
                    assert!(
                        !file_box_ptr.is_null(),
                        "VersionSet::log_and_apply: Env returned OK but file pointer is null"
                    );

                    // Take ownership of Env allocation and store as a raw *mut dyn WritableFile.
                    let file_holder: Box<Box<dyn WritableFile>> = Box::from_raw(file_box_ptr);
                    let file_inner: Box<dyn WritableFile> = *file_holder;
                    let raw_file: *mut dyn WritableFile = Box::into_raw(file_inner);

                    self.set_descriptor_file(raw_file);

                    let dest: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(
                        BorrowedWritableFileForManifest::new(self.descriptor_file()),
                    ));

                    let lw = LogWriter::new(dest, 0);
                    self.set_descriptor_log(Box::into_raw(Box::new(lw)));

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        descriptor_log_ptr = self.descriptor_log() as usize,
                        descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,
                        "VersionSet::log_and_apply: write_snapshot begin"
                    );

                    let t_snapshot = std::time::Instant::now();
                    s = self.write_snapshot();

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        status_ok = s.is_ok(),
                        status = %s.to_string(),
                        elapsed_ms = t_snapshot.elapsed().as_millis() as u64,
                        "VersionSet::log_and_apply: write_snapshot end"
                    );
                }
            }

            // Unlock during expensive MANIFEST log write
            {
                tracing::info!(
                    ?tid,
                    mu_ptr = mu as usize,
                    "VersionSet::log_and_apply: about to unlock mu for MANIFEST write"
                );

                (*mu).unlock();

                let t_unlocked = std::time::Instant::now();

                tracing::info!(
                    ?tid,
                    mu_ptr = mu as usize,
                    "VersionSet::log_and_apply: mu unlocked"
                );

                // Write new record to MANIFEST log
                if s.is_ok() {
                    let mut record: String = String::new();
                    e.encode_to(&mut record as *mut String);

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        bytes = record.len() as u64,
                        descriptor_log_ptr = self.descriptor_log() as usize,
                        "VersionSet::log_and_apply: descriptor_log.add_record begin"
                    );

                    let t_add_record = std::time::Instant::now();
                    let rec_slice = Slice::from(record.as_bytes());
                    s = (*self.descriptor_log()).add_record(&rec_slice);

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        status_ok = s.is_ok(),
                        status = %s.to_string(),
                        elapsed_ms = t_add_record.elapsed().as_millis() as u64,
                        "VersionSet::log_and_apply: descriptor_log.add_record end"
                    );

                    if s.is_ok() {
                        tracing::info!(
                            ?tid,
                            dbname = %self.dbname(),
                            descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,
                            "VersionSet::log_and_apply: descriptor_file.sync begin"
                        );

                        let t_sync = std::time::Instant::now();
                        s = (*self.descriptor_file()).sync();

                        tracing::info!(
                            ?tid,
                            dbname = %self.dbname(),
                            status_ok = s.is_ok(),
                            status = %s.to_string(),
                            elapsed_ms = t_sync.elapsed().as_millis() as u64,
                            "VersionSet::log_and_apply: descriptor_file.sync end"
                        );
                    }

                    if !s.is_ok() {
                        tracing::error!(
                            ?tid,
                            dbname = %self.dbname(),
                            status = %s.to_string(),
                            "VersionSet::log_and_apply: MANIFEST write failed"
                        );
                    }
                }

                // If we just created a new descriptor file, install it by writing a
                // new CURRENT file that points to it.
                if s.is_ok() && !new_manifest_file.is_empty() {
                    let env_rc = (*self.options())
                        .env()
                        .as_ref()
                        .expect("VersionSet::log_and_apply: Options.env is None")
                        .clone();

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        manifest_file_number = self.manifest_file_number(),
                        "VersionSet::log_and_apply: set_current_file begin"
                    );

                    let t_set_current = std::time::Instant::now();
                    s = set_current_file(env_rc, self.dbname(), self.manifest_file_number());

                    tracing::info!(
                        ?tid,
                        dbname = %self.dbname(),
                        status_ok = s.is_ok(),
                        status = %s.to_string(),
                        elapsed_ms = t_set_current.elapsed().as_millis() as u64,
                        "VersionSet::log_and_apply: set_current_file end"
                    );
                }

                tracing::info!(
                    ?tid,
                    mu_ptr = mu as usize,
                    unlocked_elapsed_ms = t_unlocked.elapsed().as_millis() as u64,
                    "VersionSet::log_and_apply: about to re-lock mu after MANIFEST write"
                );

                let t_relock_wait = std::time::Instant::now();
                (*mu).lock();

                tracing::info!(
                    ?tid,
                    mu_ptr = mu as usize,
                    relock_wait_ms = t_relock_wait.elapsed().as_millis() as u64,
                    "VersionSet::log_and_apply: mu re-locked"
                );
            }

            // Install the new version
            if s.is_ok() {
                tracing::info!(
                    ?tid,
                    dbname = %self.dbname(),
                    v_ptr = v_ptr as usize,
                    "VersionSet::log_and_apply: append_version begin"
                );

                let t_append = std::time::Instant::now();
                self.append_version(v_ptr);

                tracing::info!(
                    ?tid,
                    dbname = %self.dbname(),
                    v_ptr = v_ptr as usize,
                    elapsed_ms = t_append.elapsed().as_millis() as u64,
                    "VersionSet::log_and_apply: append_version end"
                );

                self.set_log_number(*e.log_number());
                self.set_prev_log_number(*e.prev_log_number());
            } else {
                drop(Box::from_raw(v_ptr));

                if !new_manifest_file.is_empty() {
                    if !self.descriptor_log().is_null() {
                        drop(Box::from_raw(self.descriptor_log()));
                        self.set_descriptor_log(core::ptr::null_mut());
                    }

                    if !self.descriptor_file().is_null() {
                        drop(Box::<dyn WritableFile>::from_raw(self.descriptor_file()));
                        self.set_descriptor_file(VersionSet::null_writable_file_ptr());
                    }

                    let env_rc = (*self.options())
                        .env()
                        .as_ref()
                        .expect("VersionSet::log_and_apply: Options.env is None")
                        .clone();

                    let del_status = env_rc.borrow_mut().delete_file(&new_manifest_file);
                    if !del_status.is_ok() {
                        tracing::warn!(
                            ?tid,
                            dbname = %self.dbname(),
                            manifest = %new_manifest_file,
                            status = %del_status.to_string(),
                            "VersionSet::log_and_apply: failed to delete new manifest after error"
                        );
                    }
                }
            }

            tracing::info!(
                ?tid,
                dbname = %self.dbname(),
                status_ok = s.is_ok(),
                status = %s.to_string(),
                elapsed_ms = t_enter.elapsed().as_millis() as u64,
                "VersionSet::log_and_apply: exit"
            );

            s
        }
    }
}

#[cfg(test)]
mod version_set_log_and_apply_exhaustive_test_suite {
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

    fn find_manifest_file(dir: &Path) -> Option<PathBuf> {
        let rd = std::fs::read_dir(dir).ok()?;
        for ent in rd.flatten() {
            let p = ent.path();
            if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
                if name.starts_with("MANIFEST-") {
                    return Some(p);
                }
            }
        }
        None
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
    fn log_and_apply_persists_edit_so_recover_sees_it() {
        let dir = make_unique_temp_db_dir("versionset_log_and_apply_persist");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env.clone()));
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
        assert_status_ok(&st0, "recover before log_and_apply");

        let manifest_before = find_manifest_file(&dir).expect("MANIFEST-* must exist after recover");
        let size_before = std::fs::metadata(&manifest_before).unwrap().len();
        debug!(path = %manifest_before.display(), size_before, "manifest before");

        let mut edit = VersionEdit::default();
        let fnum = vs.new_file_number();
        let smallest = make_ikey("a", 1);
        let largest = make_ikey("k", 1);
        edit.add_file(1, fnum, 100, &smallest, &largest);

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);
        let st1 = vs.log_and_apply(&mut edit as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
        assert_status_ok(&st1, "log_and_apply");

        let size_after = std::fs::metadata(&manifest_before).unwrap().len();
        debug!(size_after, "manifest after");
        assert!(size_after >= size_before, "manifest must not shrink after log_and_apply");

        let mut options2 = Box::new(Options::with_env(env));
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
        info!(save_manifest2, status = ?st2, "recover after log_and_apply");
        assert_status_ok(&st2, "recover after log_and_apply");

        let n = vs2.num_level_files(1);
        debug!(n, "num_level_files(1) after recover");
        assert!(n >= 1, "expected at least one file in level 1 after recover");

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn log_and_apply_multiple_edits_monotonically_increase_file_numbers() {
        let dir = make_unique_temp_db_dir("versionset_log_and_apply_file_numbers");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
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

        let f1 = vs.new_file_number();
        let f2 = vs.new_file_number();
        debug!(f1, f2, "allocated new file numbers");
        assert!(f2 > f1, "file numbers must be monotonically increasing");

        let _guard = RawMutexTestGuard::lock(mu.as_mut() as *mut RawMutex);

        let mut e1 = VersionEdit::default();
        e1.add_file(0, f1, 10, &make_ikey("a", 1), &make_ikey("b", 1));
        let st1 = vs.log_and_apply(&mut e1 as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
        assert_status_ok(&st1, "log_and_apply e1");

        let mut e2 = VersionEdit::default();
        e2.add_file(0, f2, 10, &make_ikey("c", 1), &make_ikey("d", 1));
        let st2 = vs.log_and_apply(&mut e2 as *mut VersionEdit, mu.as_mut() as *mut RawMutex);
        assert_status_ok(&st2, "log_and_apply e2");

        let l0 = vs.num_level_files(0);
        debug!(l0, "num_level_files(0)");
        assert!(l0 >= 2, "expected at least two L0 files after two edits");

        remove_dir_all_best_effort(&dir);
    }
}
