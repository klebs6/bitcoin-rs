// ---------------- [ File: bitcoinleveldb-dbimpl/src/recover.rs ]
crate::ix!();

impl DBImpl {
    /// Recover the descriptor from persistent storage.
    ///
    /// May do a significant amount of work to recover recently logged updates.
    ///
    /// Any changes to be made to the descriptor are added to *edit.
    ///
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn recover(
        &mut self,
        edit: *mut VersionEdit,
        save_manifest: *mut bool,
    ) -> crate::Status {
        self.mutex.assert_held();

        let tid = std::thread::current().id();
        let t_start = std::time::Instant::now();

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_entry",
            ?tid,
            dbname = %self.dbname,
            edit_ptr = edit as usize,
            save_manifest_ptr = save_manifest as usize,
            versions_ptr = self.versions as usize,
            db_lock_ptr = self.db_lock as usize,
            "DBImpl::recover: enter"
        );

        // Ignore error from CreateDir since the creation of the DB is
        // committed only when the descriptor is created, and this directory
        // may already exist from a previous failed creation attempt.
        let create_dir_status: Status = self.env.as_mut().create_dir(&self.dbname);

        tracing::debug!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_create_dir_result",
            ?tid,
            dbname = %self.dbname,
            status_ok = create_dir_status.is_ok(),
            status = %create_dir_status.to_string(),
            "DBImpl::recover: create_dir completed"
        );

        assert!(self.db_lock.is_null());

        let lock_path: String = lock_file_name(&self.dbname);

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_lock_file_begin",
            ?tid,
            dbname = %self.dbname,
            file = %lock_path,
            db_lock_ptr = self.db_lock as usize,
            "DBImpl::recover: about to call Env::lock_file"
        );

        let t_lock = std::time::Instant::now();
        let mut s: Status = self
            .env
            .as_mut()
            .lock_file(&lock_path, core::ptr::addr_of_mut!(self.db_lock));

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_lock_file_end",
            ?tid,
            dbname = %self.dbname,
            file = %lock_path,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            db_lock_null = self.db_lock.is_null(),
            elapsed_ms = t_lock.elapsed().as_millis() as u64,
            "DBImpl::recover: Env::lock_file returned"
        );

        if s.is_ok() && self.db_lock.is_null() {
            tracing::error!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_lock_file_null_output",
                ?tid,
                dbname = %self.dbname,
                file = %lock_path,
                "Env::lock_file returned ok but output lock handle was null"
            );

            let msg: Slice = Slice::from_str("lock_file returned ok but output was null");
            let fname_slice: Slice = Slice::from(&lock_path);
            return Status::corruption(&msg, Some(&fname_slice));
        }

        if !s.is_ok() {
            tracing::error!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_lock_file_failure",
                ?tid,
                dbname = %self.dbname,
                status = %s.to_string(),
                elapsed_ms = t_start.elapsed().as_millis() as u64,
                "DBImpl::recover: early return due to lock_file failure"
            );

            return s;
        }

        let current_path: String = current_file_name(&self.dbname);
        let current_exists: bool = self.env.as_mut().file_exists(&current_path);

        tracing::debug!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_current_exists",
            ?tid,
            dbname = %self.dbname,
            current = %current_path,
            current_exists,
            create_if_missing = *self.options.create_if_missing(),
            error_if_exists = *self.options.error_if_exists(),
            "DBImpl::recover: checked CURRENT existence"
        );

        if !current_exists {
            if *self.options.create_if_missing() {
                tracing::info!(
                    target: "bitcoinleveldb_dbimpl::recover",
                    event = "dbimpl_recover_current_missing_create_db",
                    ?tid,
                    dbname = %self.dbname,
                    current = %current_path,
                    "DBImpl::recover: CURRENT missing; create_if_missing=true; calling newdb()"
                );

                s = self.newdb();

                tracing::info!(
                    target: "bitcoinleveldb_dbimpl::recover",
                    event = "dbimpl_recover_newdb_result",
                    ?tid,
                    dbname = %self.dbname,
                    status_ok = s.is_ok(),
                    status = %s.to_string(),
                    elapsed_ms = t_start.elapsed().as_millis() as u64,
                    "DBImpl::recover: newdb returned"
                );

                if !s.is_ok() {
                    tracing::error!(
                        "[dbimpl-recover-live] event=dbimpl_recover_newdb_failure dbname='{}' status='{}' elapsed_ms={}",
                        self.dbname,
                        s.to_string(),
                        t_start.elapsed().as_millis() as u64,
                    );
                    return s;
                }
            } else {
                let msg: Slice = Slice::from(&self.dbname);
                let msg2: Slice = Slice::from_str("does not exist (create_if_missing is false)");

                tracing::warn!(
                    target: "bitcoinleveldb_dbimpl::recover",
                    event = "dbimpl_recover_current_missing_invalid_argument",
                    ?tid,
                    dbname = %self.dbname,
                    current = %current_path,
                    "DBImpl::recover: CURRENT missing; create_if_missing=false; returning InvalidArgument"
                );

                return Status::invalid_argument(&msg, Some(&msg2));
            }
        } else if *self.options.error_if_exists() {
            let msg: Slice = Slice::from(&self.dbname);
            let msg2: Slice = Slice::from_str("exists (error_if_exists is true)");

            tracing::warn!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_error_if_exists",
                ?tid,
                dbname = %self.dbname,
                current = %current_path,
                "DBImpl::recover: CURRENT exists; error_if_exists=true; returning InvalidArgument"
            );

            return Status::invalid_argument(&msg, Some(&msg2));
        }

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_versionset_recover_begin",
            ?tid,
            dbname = %self.dbname,
            versions_ptr = self.versions as usize,
            "DBImpl::recover: calling VersionSet::recover"
        );

        let t_vrecover = std::time::Instant::now();
        s = unsafe { (*self.versions).recover(save_manifest) };

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_versionset_recover_end",
            ?tid,
            dbname = %self.dbname,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            save_manifest_value = if save_manifest.is_null() { false } else { unsafe { *save_manifest } },
            elapsed_ms = t_vrecover.elapsed().as_millis() as u64,
            "DBImpl::recover: VersionSet::recover returned"
        );

        if !s.is_ok() {
            tracing::error!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_versionset_recover_failure",
                ?tid,
                dbname = %self.dbname,
                status = %s.to_string(),
                elapsed_ms = t_start.elapsed().as_millis() as u64,
                "DBImpl::recover: returning error from VersionSet::recover"
            );

            return s;
        }

        let mut max_sequence: SequenceNumber = 0;

        // Recover from all newer log files than the ones named in the
        // descriptor (new log files may have been added by the previous
        // incarnation without registering them in the descriptor).
        //
        // Note that PrevLogNumber() is no longer used, but we pay
        // attention to it in case we are recovering a database
        // produced by an older version of leveldb.
        let min_log: u64 = unsafe { (*self.versions).log_number() };
        let prev_log: u64 = unsafe { (*self.versions).prev_log_number() };

        let mut filenames: Vec<String> = Vec::new();

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_get_children_begin",
            ?tid,
            dbname = %self.dbname,
            "DBImpl::recover: calling Env::get_children"
        );

        let t_children = std::time::Instant::now();
        s = self.env.as_mut().get_children(&self.dbname, &mut filenames);

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_get_children_end",
            ?tid,
            dbname = %self.dbname,
            status_ok = s.is_ok(),
            status = %s.to_string(),
            elapsed_ms = t_children.elapsed().as_millis() as u64,
            child_count = filenames.len() as u64,
            "DBImpl::recover: Env::get_children returned"
        );

        if !s.is_ok() {
            return s;
        }

        let mut expected_live: std::collections::HashSet<u64> = std::collections::HashSet::new();

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_add_live_files_begin",
            ?tid,
            dbname = %self.dbname,
            versions_ptr = self.versions as usize,
            "DBImpl::recover: calling VersionSet::add_live_files"
        );

        let t_live = std::time::Instant::now();
        unsafe {
            (*self.versions).add_live_files(
                &mut expected_live as *mut std::collections::HashSet<u64>,
            );
        }

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_add_live_files_end",
            ?tid,
            dbname = %self.dbname,
            live_files = expected_live.len() as u64,
            elapsed_ms = t_live.elapsed().as_millis() as u64,
            "DBImpl::recover: VersionSet::add_live_files returned"
        );

        let mut expected: std::collections::BTreeSet<u64> = expected_live.into_iter().collect::<std::collections::BTreeSet<u64>>();
        let mut logs: Vec<u64> = Vec::new();

        for fname in filenames.into_iter() {
            let mut number: u64 = 0;
            let mut ftype: FileType = FileType::LogFile;

            if parse_file_name(&fname, &mut number, &mut ftype) {
                expected.remove(&number);

                if matches!(ftype, FileType::LogFile) && (number >= min_log || number == prev_log) {
                    logs.push(number);
                }
            }
        }

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_logs_selected",
            ?tid,
            dbname = %self.dbname,
            min_log,
            prev_log,
            logs_selected = logs.len() as u64,
            missing_expected = expected.len() as u64,
            "DBImpl::recover: selected candidate logs"
        );

        if !expected.is_empty() {
            let buf_string: String = format!("{} missing files; e.g.", expected.len());
            let msg: Slice = Slice::from_str(&buf_string);

            let first_missing_path: Option<String> = match expected.iter().next() {
                Some(first) => Some(table_file_name(&self.dbname, *first)),
                None => None,
            };

            tracing::error!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_missing_expected_live_files",
                ?tid,
                dbname = %self.dbname,
                missing_count = expected.len() as u64,
                example = %first_missing_path.clone().unwrap_or_else(|| String::from("<none>")),
                "DBImpl::recover: missing expected live files; returning corruption"
            );

            match first_missing_path {
                Some(first_fname) => {
                    let msg2 = Slice::from(&first_fname);
                    return Status::corruption(&msg, Some(&msg2));
                }
                None => {
                    return Status::corruption(&msg, None);
                }
            }
        }

        // Recover in the order in which the logs were generated
        logs.sort();

        if logs.is_empty() {
            tracing::info!(
                "[dbimpl-recover-live] event=dbimpl_recover_no_logs_to_replay dbname='{}' min_log={} prev_log={}",
                self.dbname,
                min_log,
                prev_log,
            );
        }

        for (i, log_number) in logs.iter().copied().enumerate() {
            tracing::info!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_log_file_begin",
                ?tid,
                dbname = %self.dbname,
                log_number,
                idx = i as u64,
                last = (i == logs.len().saturating_sub(1)),
                "DBImpl::recover: calling recover_log_file"
            );

            let t_log = std::time::Instant::now();
            s = self.recover_log_file(
                log_number,
                i == logs.len() - 1,
                save_manifest,
                edit,
                &mut max_sequence,
            );

            tracing::info!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_log_file_end",
                ?tid,
                dbname = %self.dbname,
                log_number,
                status_ok = s.is_ok(),
                status = %s.to_string(),
                elapsed_ms = t_log.elapsed().as_millis() as u64,
                max_sequence,
                "DBImpl::recover: recover_log_file returned"
            );

            if !s.is_ok() {
                return s;
            }

            unsafe {
                // The previous incarnation may not have written any MANIFEST
                // records after allocating this log number.  So we manually
                // update the file number allocation counter in VersionSet.
                (*self.versions).mark_file_number_used(log_number);
            }
        }

        let previous_last_sequence: SequenceNumber = unsafe { (*self.versions).last_sequence() };

        if previous_last_sequence < max_sequence {
            unsafe {
                (*self.versions).set_last_sequence(max_sequence);
            }

            tracing::info!(
                target: "bitcoinleveldb_dbimpl::recover",
                event = "dbimpl_recover_last_sequence_advanced",
                ?tid,
                dbname = %self.dbname,
                previous_last_sequence,
                max_sequence,
                "DBImpl::recover: advanced last_sequence after log replay"
            );
        }

        tracing::info!(
            target: "bitcoinleveldb_dbimpl::recover",
            event = "dbimpl_recover_exit",
            ?tid,
            dbname = %self.dbname,
            status_ok = true,
            max_sequence,
            previous_last_sequence,
            final_last_sequence = unsafe { (*self.versions).last_sequence() },
            save_manifest_value = if save_manifest.is_null() { false } else { unsafe { *save_manifest } },
            elapsed_ms = t_start.elapsed().as_millis() as u64,
            "DBImpl::recover: exit ok"
        );

        Status::ok()
    }
}

#[cfg(test)]
mod recover_interface_and_precondition_suite {
    use super::*;

    fn build_temp_db_path_for_recover_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_with_env_or_panic_for_recover_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run recover suite");
            panic!();
        }

        options
    }

    #[traced_test]
    fn recover_signature_is_stable() {
        tracing::info!("Asserting DBImpl::recover signature is stable");
        type Sig = fn(&mut DBImpl, *mut VersionEdit, *mut bool) -> Status;
        let _sig: Sig = DBImpl::recover;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn recover_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::recover method item is addressable");
        let _m = DBImpl::recover;
        let _ = _m;
    }

    #[traced_test]
    fn recover_returns_invalid_argument_when_db_missing_and_create_if_missing_is_false() {
        let dbname = build_temp_db_path_for_recover_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let mut options = build_options_with_env_or_panic_for_recover_suite();
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);

        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let mut edit: VersionEdit = Default::default();
        let mut save_manifest: bool = false;

        db.mutex.lock();

        tracing::info!(
            dbname = %dbname,
            create_if_missing = *options.create_if_missing(),
            "Calling recover() on directory without CURRENT; expecting non-OK Status"
        );

        let s: Status = db.recover(&mut edit as *mut VersionEdit, &mut save_manifest as *mut bool);

        tracing::debug!(status = %s.to_string(), save_manifest, "recover() returned");

        unsafe { db.mutex.unlock() };

        assert!(
            !s.is_ok(),
            "recover must return non-OK when CURRENT is missing and create_if_missing is false"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn recover_returns_invalid_argument_when_db_exists_and_error_if_exists_is_true() {
        let dbname = build_temp_db_path_for_recover_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let current_path = current_file_name(&dbname);
        std::fs::write(&current_path, "MANIFEST-000001\n").unwrap_or_else(|e| {
            tracing::error!(path = %current_path, error = %format!("{:?}", e), "Failed to create CURRENT");
            panic!();
        });

        let mut options = build_options_with_env_or_panic_for_recover_suite();
        options.set_error_if_exists(true);
        options.set_create_if_missing(true);

        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        let mut edit: VersionEdit = Default::default();
        let mut save_manifest: bool = false;

        db.mutex.lock();

        tracing::info!(
            dbname = %dbname,
            error_if_exists = *options.error_if_exists(),
            "Calling recover() with CURRENT present and error_if_exists=true; expecting non-OK Status"
        );

        let s: Status = db.recover(&mut edit as *mut VersionEdit, &mut save_manifest as *mut bool);

        tracing::debug!(status = %s.to_string(), save_manifest, "recover() returned");

        unsafe { db.mutex.unlock() };

        assert!(
            !s.is_ok(),
            "recover must return non-OK when CURRENT exists and error_if_exists is true"
        );

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
