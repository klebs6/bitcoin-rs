// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_recover.rs ]
crate::ix!();

struct VersionSetRecoverCorruptionReporter {
    status: *mut Status,
}

impl bitcoinleveldb_logreader::LogReaderReporter for VersionSetRecoverCorruptionReporter {
    fn corruption(&mut self, bytes: usize, st: &Status) {
        warn!(
            bytes,
            status = %st.to_string(),
            "VersionSet::recover: log corruption reported"
        );

        unsafe {
            if !self.status.is_null() && (*self.status).is_ok() {
                (*self.status) = Status::new_from_other_copy(st);
            }
        }
    }
}

impl Recover for VersionSet {
    /// Recover the last saved descriptor from persistent storage.
    fn recover(&mut self, save_manifest: *mut bool) -> Status {
        trace!(
            "VersionSet::recover: enter; save_manifest_ptr={:p}",
            save_manifest
        );

        assert!(
            !save_manifest.is_null(),
            "VersionSet::recover: save_manifest out-param is null"
        );

        unsafe {
            *save_manifest = false;
        }

        let env_rc = unsafe {
            (*self.options())
                .env()
                .as_ref()
                .expect("VersionSet::recover: Options.env is None")
                .clone()
        };

        let mut current: String = String::new();
        let current_name = current_file_name(&self.dbname());

        let mut s =
            read_file_to_string(env_rc.clone(), &current_name, &mut current as *mut String);

        if !s.is_ok() {
            let create_if_missing = unsafe { *(*self.options()).create_if_missing() };
            if s.is_not_found() && create_if_missing {
                info!(
                    dbname = %self.dbname(),
                    "VersionSet::recover: CURRENT missing and create_if_missing=true; creating new manifest"
                );

                self.set_manifest_file_number(1);
                self.set_next_file_number(2);
                self.set_last_sequence(0);
                self.set_log_number(0);
                self.set_prev_log_number(0);

                let manifest =
                    descriptor_file_name(self.dbname(), self.manifest_file_number());

                let mut file_box_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
                s = env_rc.borrow_mut().new_writable_file(&manifest, &mut file_box_ptr);

                if !s.is_ok() {
                    error!(
                        manifest = %manifest,
                        status = %s.to_string(),
                        "VersionSet::recover: failed to create initial MANIFEST"
                    );
                    return s;
                }

                assert!(
                    !file_box_ptr.is_null(),
                    "VersionSet::recover: Env returned OK but file pointer is null"
                );

                let raw_file: *mut dyn WritableFile = unsafe {
                    let file_holder: Box<Box<dyn WritableFile>> = Box::from_raw(file_box_ptr);
                    let file_inner: Box<dyn WritableFile> = *file_holder;
                    Box::into_raw(file_inner)
                };

                trace!(
                    manifest = %manifest,
                    raw_file_ptr = %format!("{:p}", raw_file),
                    "VersionSet::recover: created initial MANIFEST writable file"
                );

                let mut snap_status: Status = Status::ok();
                let mut sync_status: Status = Status::ok();

                {
                    let dest: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(
                        BorrowedWritableFileForManifest::new(raw_file),
                    ));

                    let mut lw = LogWriter::new(dest, 0);

                    snap_status = <VersionSet as WriteSnapshot>::write_snapshot(self, &mut lw);

                    trace!(
                        ok = snap_status.is_ok(),
                        status = %snap_status.to_string(),
                        "VersionSet::recover: write_snapshot into initial MANIFEST"
                    );

                    if snap_status.is_ok() {
                        sync_status = unsafe { (*raw_file).sync() };

                        trace!(
                            ok = sync_status.is_ok(),
                            status = %sync_status.to_string(),
                            "VersionSet::recover: synced initial MANIFEST"
                        );
                    }

                    // `lw` (and its borrowed wrapper) MUST drop before we free `raw_file`.
                    trace!(
                        raw_file_ptr = %format!("{:p}", raw_file),
                        "VersionSet::recover: dropping temporary LogWriter before freeing MANIFEST file"
                    );
                }

                unsafe {
                    drop(Box::<dyn WritableFile>::from_raw(raw_file));
                }

                if !snap_status.is_ok() {
                    return snap_status;
                }

                if !sync_status.is_ok() {
                    return sync_status;
                }

                let cur_status =
                    set_current_file(env_rc.clone(), self.dbname(), self.manifest_file_number());
                if !cur_status.is_ok() {
                    return cur_status;
                }

                trace!("VersionSet::recover: created new CURRENT+MANIFEST; exit ok");
                return Status::ok();
            }

            return s;
        }

        if current.is_empty() || !current.ends_with('\n') {
            let msg = Slice::from("CURRENT file does not end with newline");
            return Status::corruption(&msg, None);
        }
        current.pop();

        let dscname = format!("{}/{}", self.dbname(), current);

        let mut file_ptr: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
        s = env_rc.borrow_mut().new_sequential_file(&dscname, &mut file_ptr);

        if !s.is_ok() {
            if s.is_not_found() {
                let msg1 = Slice::from("CURRENT points to a non-existent file");
                let msg2s = s.to_string();
                let msg2 = Slice::from(msg2s.as_bytes());
                return Status::corruption(&msg1, Some(&msg2));
            }
            return s;
        }

        let mut have_log_number: bool = false;
        let mut have_prev_log_number: bool = false;
        let mut have_next_file: bool = false;
        let mut have_last_sequence: bool = false;

        let mut next_file: u64 = 0;
        let mut last_sequence: u64 = 0;
        let mut log_number: u64 = 0;
        let mut prev_log_number: u64 = 0;

        let mut builder = VersionSetBuilder::new(self, self.current());

        {
            let reporter: Box<dyn bitcoinleveldb_logreader::LogReaderReporter> =
                Box::new(VersionSetRecoverCorruptionReporter {
                    status: &mut s as *mut Status,
                });

            let file_holder: Box<Box<dyn SequentialFile>> = unsafe {
                assert!(
                    !file_ptr.is_null(),
                    "VersionSet::recover: Env returned OK but file_ptr is null"
                );
                Box::from_raw(file_ptr)
            };

            let file: Box<dyn SequentialFile> = *file_holder;

            let mut reader = bitcoinleveldb_logreader::LogReader::new(
                file,
                reporter,
                true, /*checksum*/
                0,    /*initial_offset*/
            );

            let mut record = Slice::default();
            let mut scratch: Vec<u8> = Vec::new();

            while reader.read_record(&mut record, &mut scratch) && s.is_ok() {
                let mut edit = VersionEdit::default();
                s = edit.decode_from(&record);

                if s.is_ok() && *edit.has_comparator() {
                    let existing_name: String = unsafe {
                        let ucmp_ptr = self.icmp().user_comparator();
                        if ucmp_ptr.is_null() {
                            String::new()
                        } else {
                            match (*ucmp_ptr).name() {
                                Cow::Borrowed(x) => x.to_owned(),
                                Cow::Owned(x) => x,
                            }
                        }
                    };

                    if edit.comparator().as_str() != existing_name.as_str() {
                        let mut msg1s = String::new();
                        msg1s.push_str(edit.comparator());
                        msg1s.push_str(" does not match existing comparator ");

                        let msg1 = Slice::from(msg1s.as_bytes());
                        let msg2 = Slice::from(existing_name.as_bytes());

                        s = Status::invalid_argument(&msg1, Some(&msg2));
                    }
                }

                if s.is_ok() {
                    builder.apply(&mut edit as *mut VersionEdit);
                }

                if *edit.has_log_number() {
                    log_number = *edit.log_number();
                    have_log_number = true;
                }

                if *edit.has_prev_log_number() {
                    prev_log_number = *edit.prev_log_number();
                    have_prev_log_number = true;
                }

                if *edit.has_next_file_number() {
                    next_file = *edit.next_file_number();
                    have_next_file = true;
                }

                if *edit.has_last_sequence() {
                    last_sequence = *edit.last_sequence();
                    have_last_sequence = true;
                }
            }
        }

        if s.is_ok() {
            if !have_next_file {
                let msg = Slice::from("no meta-nextfile entry in descriptor");
                s = Status::corruption(&msg, None);
            } else if !have_log_number {
                let msg = Slice::from("no meta-lognumber entry in descriptor");
                s = Status::corruption(&msg, None);
            } else if !have_last_sequence {
                let msg = Slice::from("no last-sequence-number entry in descriptor");
                s = Status::corruption(&msg, None);
            }

            if !have_prev_log_number {
                prev_log_number = 0;
            }

            self.mark_file_number_used(prev_log_number);
            self.mark_file_number_used(log_number);
        }

        if s.is_ok() {
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

            builder.save_to(v_ptr);

            self.finalize(v_ptr);
            self.append_version(v_ptr);

            self.set_manifest_file_number(next_file);
            self.set_next_file_number(next_file + 1);
            self.set_last_sequence(last_sequence);
            self.set_log_number(log_number);
            self.set_prev_log_number(prev_log_number);

            let dscname_s = dscname.clone();
            let current_s = current.clone();

            if self.reuse_manifest(&dscname_s, &current_s) {
                trace!("VersionSet::recover: reused existing MANIFEST");
            } else {
                unsafe {
                    *save_manifest = true;
                }
            }
        }

        trace!(
            "VersionSet::recover: exit; status_ok={} save_manifest={}",
            s.is_ok(),
            unsafe { *save_manifest }
        );

        s
    }
}

#[cfg(test)]
mod version_set_recover_exhaustive_test_suite {
    use super::*;
    use std::io::Write;
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

    fn make_internal_key_comparator_from_options(options: &Options) -> InternalKeyComparator {
        let ucmp_ptr: *const dyn SliceComparator =
            options.comparator().as_ref() as *const dyn SliceComparator;
        InternalKeyComparator::new(ucmp_ptr)
    }

    struct VersionSetRecoverHarness {
        dir: PathBuf,
        dbname: Box<String>,
        options: Box<Options>,
        icmp: Box<InternalKeyComparator>,
        table_cache: Box<TableCache>,
        versionset: Option<VersionSet>,
    }

    impl VersionSetRecoverHarness {
        fn new(prefix: &str, create_if_missing: bool) -> Self {
            let dir = make_unique_temp_db_dir(prefix);
            std::fs::create_dir_all(&dir).unwrap();

            let dbname = Box::new(dir.to_string_lossy().to_string());

            let env = PosixEnv::shared();
            let mut options = Box::new(Options::with_env(env));
            options.set_create_if_missing(create_if_missing);
            options.set_error_if_exists(false);

            let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

            let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

            let versionset = VersionSet::new(
                dbname.as_ref(),
                options.as_ref(),
                table_cache.as_mut() as *mut TableCache,
                icmp.as_ref() as *const InternalKeyComparator,
            );

            Self {
                dir,
                dbname,
                options,
                icmp,
                table_cache,
                versionset: Some(versionset),
            }
        }

        fn recover(&mut self) -> (Status, bool) {
            let vs = self.versionset.as_mut().unwrap();
            let mut save_manifest: bool = false;
            let st = vs.recover(&mut save_manifest as *mut bool);
            info!(save_manifest, status = ?st, "recover completed");
            (st, save_manifest)
        }

        fn drop_versionset_now(&mut self) {
            let _ = self.versionset.take();
        }
    }

    #[traced_test]
    fn recover_creates_new_db_when_allowed() {
        let mut h = VersionSetRecoverHarness::new("versionset_recover_creates_db", true);
        let (st, _save_manifest) = h.recover();
        assert_status_ok(&st, "recover");

        assert!(
            h.dir.join("CURRENT").exists(),
            "CURRENT should exist after recover"
        );

        h.drop_versionset_now();
        remove_dir_all_best_effort(&h.dir);
    }

    #[traced_test]
    fn recover_fails_on_missing_db_when_not_allowed() {
        let mut h = VersionSetRecoverHarness::new("versionset_recover_disallowed_missing_db", false);
        let (st, _save_manifest) = h.recover();
        debug!(?st, "recover result");
        assert!(
            !st.is_ok(),
            "recover must fail when db missing and create_if_missing=false"
        );

        h.drop_versionset_now();
        remove_dir_all_best_effort(&h.dir);
    }

    #[traced_test]
    fn recover_fails_on_corrupt_current_file_contents() {
        let dir = make_unique_temp_db_dir("versionset_recover_corrupt_current");
        std::fs::create_dir_all(&dir).unwrap();

        let current = dir.join("CURRENT");
        {
            let mut f = std::fs::File::create(&current).unwrap();
            f.write_all(b"NOT_A_MANIFEST_NAME\n").unwrap();
            f.flush().unwrap();
        }

        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);

        debug!(save_manifest, status = ?st, "recover result on corrupt CURRENT");
        assert!(
            !st.is_ok(),
            "recover must fail when CURRENT is corrupt"
        );

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn recover_fails_when_current_points_to_missing_manifest() {
        let dir = make_unique_temp_db_dir("versionset_recover_missing_manifest");
        std::fs::create_dir_all(&dir).unwrap();

        let current = dir.join("CURRENT");
        {
            let mut f = std::fs::File::create(&current).unwrap();
            f.write_all(b"MANIFEST-999999\n").unwrap();
            f.flush().unwrap();
        }

        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);

        let icmp = Box::new(make_internal_key_comparator_from_options(options.as_ref()));

        let mut table_cache = Box::new(TableCache::new(dbname.as_ref(), options.as_ref(), 64));

        let mut vs = VersionSet::new(
            dbname.as_ref(),
            options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            icmp.as_ref() as *const InternalKeyComparator,
        );

        let mut save_manifest: bool = false;
        let st = vs.recover(&mut save_manifest as *mut bool);

        debug!(save_manifest, status = ?st, "recover result when manifest missing");
        assert!(
            !st.is_ok(),
            "recover must fail when CURRENT points to missing manifest"
        );

        remove_dir_all_best_effort(&dir);
    }
}
