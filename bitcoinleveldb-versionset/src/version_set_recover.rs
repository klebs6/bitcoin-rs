// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_recover.rs ]
crate::ix!();

struct VersionSetRecoverCorruptionReporter {
    status: *mut Status,
}

impl bitcoinleveldb_logreader::LogReaderReporter for VersionSetRecoverCorruptionReporter {
    fn corruption(&mut self, bytes: usize, st: &Status) {
        warn!(
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_manifest_corruption",
            bytes,
            status = %st.to_string(),
            "VersionSet::recover: log corruption reported"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_manifest_corruption bytes={} status='{}'",
            bytes,
            st.to_string(),
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
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_entry",
            dbname = %self.dbname(),
            save_manifest_ptr = save_manifest as usize,
            "VersionSet::recover: enter"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_entry dbname='{}' save_manifest_ptr={}",
            self.dbname(),
            save_manifest as usize,
        );

        assert!(
            !save_manifest.is_null(),
            "VersionSet::recover: save_manifest out-param is null"
        );

        unsafe {
            *save_manifest = false;
        }

        let env_rc = match unsafe { (*self.options()).env().as_ref() } {
            Some(env) => env.clone(),
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_missing_env",
                    dbname = %self.dbname(),
                    "VersionSet::recover: Options.env is None"
                );

                eprintln!(
                    "[versionset-recover-live] event=versionset_recover_missing_env dbname='{}'",
                    self.dbname(),
                );

                let msg = Slice::from("VersionSet::recover: Options.env is None");
                return Status::corruption(&msg, None);
            }
        };

        let mut current: String = String::new();
        let current_name = current_file_name(&self.dbname());

        let mut s =
            read_file_to_string(env_rc.clone(), &current_name, &mut current as *mut String);

        tracing::debug!(
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_read_current",
            dbname = %self.dbname(),
            current_name = %current_name,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            "VersionSet::recover: read CURRENT"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_read_current dbname='{}' current_name='{}' status_ok={} status='{}'",
            self.dbname(),
            current_name,
            s.is_ok(),
            s.to_string(),
        );

        if !s.is_ok() {
            let create_if_missing = unsafe { *(*self.options()).create_if_missing() };
            if s.is_not_found() && create_if_missing {
                info!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_current_missing_create_manifest",
                    dbname = %self.dbname(),
                    "VersionSet::recover: CURRENT missing and create_if_missing=true; creating new manifest"
                );

                eprintln!(
                    "[versionset-recover-live] event=versionset_recover_current_missing_create_manifest dbname='{}'",
                    self.dbname(),
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

                tracing::info!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_create_initial_manifest",
                    dbname = %self.dbname(),
                    manifest = %manifest,
                    status_ok = s.is_ok(),
                    status = %s.to_string(),
                    "VersionSet::recover: create initial MANIFEST"
                );

                if !s.is_ok() {
                    error!(
                        target: "bitcoinleveldb_versionset::recover",
                        event = "versionset_recover_create_initial_manifest_failure",
                        dbname = %self.dbname(),
                        manifest = %manifest,
                        status = %s.to_string(),
                        "VersionSet::recover: failed to create initial MANIFEST"
                    );

                    eprintln!(
                        "[versionset-recover-live] event=versionset_recover_create_initial_manifest_failure dbname='{}' manifest='{}' status='{}'",
                        self.dbname(),
                        manifest,
                        s.to_string(),
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
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_initial_manifest_file_created",
                    manifest = %manifest,
                    raw_file_ptr = (raw_file as *mut ()) as usize,
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
                        target: "bitcoinleveldb_versionset::recover",
                        event = "versionset_recover_initial_write_snapshot",
                        dbname = %self.dbname(),
                        manifest = %manifest,
                        status_ok = snap_status.is_ok(),
                        status = %snap_status.to_string(),
                        "VersionSet::recover: write_snapshot into initial MANIFEST"
                    );

                    if snap_status.is_ok() {
                        sync_status = unsafe { (*raw_file).sync() };

                        trace!(
                            target: "bitcoinleveldb_versionset::recover",
                            event = "versionset_recover_initial_manifest_sync",
                            dbname = %self.dbname(),
                            manifest = %manifest,
                            status_ok = sync_status.is_ok(),
                            status = %sync_status.to_string(),
                            "VersionSet::recover: synced initial MANIFEST"
                        );
                    }

                    // `lw` (and its borrowed wrapper) MUST drop before we free `raw_file`.
                    trace!(
                        target: "bitcoinleveldb_versionset::recover",
                        event = "versionset_recover_initial_manifest_drop_writer",
                        dbname = %self.dbname(),
                        raw_file_ptr = (raw_file as *mut ()) as usize,
                        "VersionSet::recover: dropping temporary LogWriter before freeing MANIFEST file"
                    );
                }

                unsafe {
                    drop(Box::<dyn WritableFile>::from_raw(raw_file));
                }

                if !snap_status.is_ok() {
                    eprintln!(
                        "[versionset-recover-live] event=versionset_recover_initial_write_snapshot_failure dbname='{}' manifest='{}' status='{}'",
                        self.dbname(),
                        manifest,
                        snap_status.to_string(),
                    );
                    return snap_status;
                }

                if !sync_status.is_ok() {
                    eprintln!(
                        "[versionset-recover-live] event=versionset_recover_initial_manifest_sync_failure dbname='{}' manifest='{}' status='{}'",
                        self.dbname(),
                        manifest,
                        sync_status.to_string(),
                    );
                    return sync_status;
                }

                let cur_status =
                    set_current_file(env_rc.clone(), self.dbname(), self.manifest_file_number());

                tracing::info!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_set_current_initial_manifest",
                    dbname = %self.dbname(),
                    manifest_file_number = self.manifest_file_number(),
                    status_ok = cur_status.is_ok(),
                    status = %cur_status.to_string(),
                    "VersionSet::recover: set CURRENT for initial MANIFEST"
                );

                if !cur_status.is_ok() {
                    return cur_status;
                }

                trace!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_created_new_manifest_exit",
                    dbname = %self.dbname(),
                    status_ok = true,
                    save_manifest = unsafe { *save_manifest },
                    "VersionSet::recover: created new CURRENT+MANIFEST; exit ok"
                );

                eprintln!(
                    "[versionset-recover-live] event=versionset_recover_created_new_manifest_exit dbname='{}' status_ok=true save_manifest={}",
                    self.dbname(),
                    unsafe { *save_manifest },
                );

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

        tracing::info!(
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_open_manifest",
            dbname = %self.dbname(),
            descriptor = %dscname,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            file_ptr = file_ptr as usize,
            "VersionSet::recover: opened descriptor file"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_open_manifest dbname='{}' descriptor='{}' status_ok={} status='{}' file_ptr={}",
            self.dbname(),
            dscname,
            s.is_ok(),
            s.to_string(),
            file_ptr as usize,
        );

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
        let mut manifest_records_seen: u64 = 0;
        let mut manifest_edits_applied: u64 = 0;

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
                manifest_records_seen = manifest_records_seen.saturating_add(1);

                if manifest_records_seen == 1 || (manifest_records_seen % 64) == 0 {
                    tracing::debug!(
                        target: "bitcoinleveldb_versionset::recover",
                        event = "versionset_recover_manifest_progress",
                        dbname = %self.dbname(),
                        descriptor = %dscname,
                        manifest_records_seen,
                        manifest_edits_applied,
                        "VersionSet::recover: manifest replay progress"
                    );

                    eprintln!(
                        "[versionset-recover-live] event=versionset_recover_manifest_progress dbname='{}' descriptor='{}' manifest_records_seen={} manifest_edits_applied={}",
                        self.dbname(),
                        dscname,
                        manifest_records_seen,
                        manifest_edits_applied,
                    );
                }

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
                    manifest_edits_applied = manifest_edits_applied.saturating_add(1);
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

        tracing::info!(
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_manifest_replay_summary",
            dbname = %self.dbname(),
            descriptor = %dscname,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            manifest_records_seen,
            manifest_edits_applied,
            have_log_number,
            have_prev_log_number,
            have_next_file,
            have_last_sequence,
            next_file,
            last_sequence,
            log_number,
            prev_log_number,
            "VersionSet::recover: manifest replay summary"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_manifest_replay_summary dbname='{}' descriptor='{}' status_ok={} status='{}' manifest_records_seen={} manifest_edits_applied={} have_log_number={} have_prev_log_number={} have_next_file={} have_last_sequence={} next_file={} last_sequence={} log_number={} prev_log_number={}",
            self.dbname(),
            dscname,
            s.is_ok(),
            s.to_string(),
            manifest_records_seen,
            manifest_edits_applied,
            have_log_number,
            have_prev_log_number,
            have_next_file,
            have_last_sequence,
            next_file,
            last_sequence,
            log_number,
            prev_log_number,
        );

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

            let built_version = VersionBuilder::default()
                .vset(vset_iface_ptr)
                .next(core::ptr::null_mut())
                .prev(core::ptr::null_mut())
                .refs(0)
                .files(v_files)
                .file_to_compact(core::ptr::null_mut())
                .file_to_compact_level(-1)
                .compaction_score(-1.0)
                .compaction_level(-1)
                .build();

            let v_ptr: *mut Version = match built_version {
                Ok(v) => Box::into_raw(Box::new(v)),
                Err(build_error) => {
                    error!(
                        target: "bitcoinleveldb_versionset::recover",
                        event = "versionset_recover_build_version_failure",
                        dbname = %self.dbname(),
                        error = ?build_error,
                        "VersionSet::recover: failed to build recovered Version"
                    );

                    eprintln!(
                        "[versionset-recover-live] event=versionset_recover_build_version_failure dbname='{}'",
                        self.dbname(),
                    );

                    let msg = Slice::from("failed to build recovered Version");
                    return Status::corruption(&msg, None);
                }
            };

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
            let manifest_reused: bool = self.reuse_manifest(&dscname_s, &current_s);

            if manifest_reused {
                trace!(
                    target: "bitcoinleveldb_versionset::recover",
                    event = "versionset_recover_reused_manifest",
                    dbname = %self.dbname(),
                    descriptor = %dscname_s,
                    "VersionSet::recover: reused existing MANIFEST"
                );
            } else {
                unsafe {
                    *save_manifest = true;
                }
            }

            trace!(
                target: "bitcoinleveldb_versionset::recover",
                event = "versionset_recover_exit",
                dbname = %self.dbname(),
                status_ok = s.is_ok(),
                save_manifest = unsafe { *save_manifest },
                manifest_reused,
                manifest_records_seen,
                manifest_edits_applied,
                next_file,
                last_sequence,
                log_number,
                prev_log_number,
                "VersionSet::recover: exit"
            );

            eprintln!(
                "[versionset-recover-live] event=versionset_recover_exit dbname='{}' status_ok={} save_manifest={} manifest_reused={} manifest_records_seen={} manifest_edits_applied={} next_file={} last_sequence={} log_number={} prev_log_number={}",
                self.dbname(),
                s.is_ok(),
                unsafe { *save_manifest },
                manifest_reused,
                manifest_records_seen,
                manifest_edits_applied,
                next_file,
                last_sequence,
                log_number,
                prev_log_number,
            );

            return s;
        }

        trace!(
            target: "bitcoinleveldb_versionset::recover",
            event = "versionset_recover_exit_error",
            dbname = %self.dbname(),
            status_ok = s.is_ok(),
            status = %s.to_string(),
            save_manifest = unsafe { *save_manifest },
            manifest_records_seen,
            manifest_edits_applied,
            "VersionSet::recover: exit"
        );

        eprintln!(
            "[versionset-recover-live] event=versionset_recover_exit_error dbname='{}' status_ok={} status='{}' save_manifest={} manifest_records_seen={} manifest_edits_applied={}",
            self.dbname(),
            s.is_ok(),
            s.to_string(),
            unsafe { *save_manifest },
            manifest_records_seen,
            manifest_edits_applied,
        );

        s
    }
}

#[cfg(test)]
mod version_set_recover_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn recover_creates_new_db_when_allowed() {
        let mut h = VersionSetRecoveryScenarioHarness::open_for_create_if_missing_flag(
            "versionset_recover_creates_db",
            true,
        );

        let (st, _save_manifest) = h.recover_into_current_version_set();
        assert_status_is_ok_or_panic(&st, "recover");

        let cleanup_path = h.database_directory_path().to_path_buf();

        assert!(
            cleanup_path.join("CURRENT").exists(),
            "CURRENT should exist after recover"
        );

        h.drop_version_set_instance();
        remove_directory_tree_best_effort(cleanup_path.as_path());
    }

    #[traced_test]
    fn recover_fails_on_missing_db_when_not_allowed() {
        let mut h = VersionSetRecoveryScenarioHarness::open_for_create_if_missing_flag(
            "versionset_recover_disallowed_missing_db",
            false,
        );

        let (st, _save_manifest) = h.recover_into_current_version_set();
        debug!(?st, "recover result");
        assert!(
            !st.is_ok(),
            "recover must fail when db missing and create_if_missing=false"
        );

        let cleanup_path = h.database_directory_path().to_path_buf();

        h.drop_version_set_instance();
        remove_directory_tree_best_effort(cleanup_path.as_path());
    }

    #[traced_test]
    fn recover_fails_on_corrupt_current_file_contents() {
        let dir = build_unique_temporary_database_directory_path("versionset_recover_corrupt_current");
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

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

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

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn recover_fails_when_current_points_to_missing_manifest() {
        let dir = build_unique_temporary_database_directory_path("versionset_recover_missing_manifest");
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

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

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

        remove_directory_tree_best_effort(&dir);
    }
}
