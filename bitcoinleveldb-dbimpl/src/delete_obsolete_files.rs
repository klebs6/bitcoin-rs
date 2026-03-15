// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete_obsolete_files.rs ]
crate::ix!();

impl DBImpl {
    /// Delete any unneeded files and stale in-memory entries.
    ///
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn delete_obsolete_files(&mut self) {
        self.mutex.assert_held();

        if !self.bg_error.is_ok() {
            // After a background error, we don't know whether a new version may
            // or may not have been committed, so we cannot safely garbage collect.
            tracing::debug!(
                dbname = %self.dbname,
                status = %self.bg_error.to_string(),
                "delete_obsolete_files: bg_error is set; skipping garbage collection pass"
            );
            return;
        }

        // Important: avoid holding the DB mutex while doing Env filesystem calls.
        // This reduces lock-ordering deadlocks involving Env scheduling/background work.
        //
        // Do not probe `file_exists(dbname)` here. Some Env implementations
        // (notably in-memory ones) do not model directories as first-class
        // filesystem entries, so `file_exists(dbname)` may be false even when
        // `get_children(dbname)` correctly returns live files.
        unsafe {
            self.mutex.unlock();
        }

        let mut filenames: Vec<String> = Vec::new();
        let children_status: Status = self
            .env
            .as_mut()
            .get_children(&self.dbname, &mut filenames);

        self.mutex.lock();

        if !children_status.is_ok() {
            tracing::debug!(
                dbname = %self.dbname,
                status = %children_status.to_string(),
                "delete_obsolete_files: get_children returned non-OK; aborting deletion pass"
            );
            return;
        }

        tracing::trace!(
            dbname = %self.dbname,
            files = filenames.len() as u64,
            "delete_obsolete_files: directory listing collected"
        );

        // If a background error was set while we dropped the lock, do not attempt GC.
        if !self.bg_error.is_ok() {
            tracing::debug!(
                dbname = %self.dbname,
                status = %self.bg_error.to_string(),
                "delete_obsolete_files: bg_error set during directory scan; aborting deletion pass"
            );
            return;
        }

        // Make a set of all of the live files.
        //
        // Important: In this Rust port, a freshly constructed DBImpl (pre-Open/recovery)
        // may have a VersionSet that is not safe to consult yet. In those states, mem is null.
        // We still allow deletion decisions based on pending_outputs alone (tests rely on this),
        // but we must not dereference VersionSet internals.
        let mut live: HashSet<u64> = self.pending_outputs.clone();

        let mut log_number: u64 = 0;
        let mut prev_log_number: u64 = 0;
        let mut manifest_file_number: u64 = 0;

        let versions_ptr: *mut VersionSet = self.versions;

        let can_consult_versions: bool = !versions_ptr.is_null() && !self.mem.is_null();

        if can_consult_versions {
            unsafe {
                (*versions_ptr).add_live_files(&mut live);
            }

            log_number = unsafe { (*versions_ptr).log_number() };
            prev_log_number = unsafe { (*versions_ptr).prev_log_number() };
            manifest_file_number = unsafe { (*versions_ptr).manifest_file_number() };
        } else {
            tracing::debug!(
                dbname = %self.dbname,
                versions_ptr = versions_ptr as usize,
                mem_ptr = self.mem as usize,
                pending_outputs = self.pending_outputs.len() as u64,
                "delete_obsolete_files: skipping VersionSet live-files scan; DB is not opened/initialized"
            );
        }

        let mut files_to_delete: Vec<String> = Vec::new();
        let mut tables_to_evict: Vec<u64> = Vec::new();

        let mut parsed: u64 = 0;
        let mut marked_for_deletion: u64 = 0;

        for filename in filenames.into_iter() {
            let mut number: u64 = 0;
            let mut ftype: FileType = FileType::LogFile;

            if parse_file_name(&filename, &mut number, &mut ftype) {
                parsed = parsed.saturating_add(1);

                let keep: bool = match ftype {
                    FileType::LogFile => number >= log_number || number == prev_log_number,
                    // Keep my manifest file, and any newer incarnations' (in case there is a race
                    // that allows other incarnations).
                    FileType::DescriptorFile => number >= manifest_file_number,
                    FileType::TableFile => live.contains(&number),
                    // Any temp files that are currently being written to must be recorded in
                    // pending_outputs, which is inserted into "live".
                    FileType::TempFile => live.contains(&number),
                    FileType::CurrentFile => true,
                    FileType::DBLockFile => true,
                    FileType::InfoLogFile => true,
                };

                if !keep {
                    marked_for_deletion = marked_for_deletion.saturating_add(1);
                    files_to_delete.push(filename.clone());

                    if matches!(ftype, FileType::TableFile) {
                        tables_to_evict.push(number);
                    }

                    let file_type: &'static str = match ftype {
                        FileType::LogFile => "log",
                        FileType::DBLockFile => "lock",
                        FileType::TableFile => "table",
                        FileType::DescriptorFile => "descriptor",
                        FileType::CurrentFile => "current",
                        FileType::TempFile => "temp",
                        FileType::InfoLogFile => "info_log",
                    };

                    tracing::info!(
                        dbname = %self.dbname,
                        file_type = %file_type,
                        file_number = number,
                        file_name = %filename,
                        "Delete obsolete file"
                    );
                }
            } else {
                tracing::trace!(
                    dbname = %self.dbname,
                    file_name = %filename,
                    "delete_obsolete_files: ignoring unparseable filename"
                );
            }
        }

        tracing::debug!(
            dbname = %self.dbname,
            parsed,
            marked_for_deletion,
            live_files = live.len() as u64,
            pending_outputs = self.pending_outputs.len() as u64,
            log_number,
            prev_log_number,
            manifest_file_number,
            "delete_obsolete_files: computed deletion plan"
        );

        // While deleting all files unblock other threads. All files being deleted have unique names
        // which will not collide with newly created files and are therefore safe to delete while
        // allowing other threads to proceed.
        unsafe {
            self.mutex.unlock();
        }

        // Evict table cache entries (table cache provides its own synchronization).
        if self.table_cache.is_null() {
            tracing::error!(
                dbname = %self.dbname,
                "delete_obsolete_files: table_cache pointer was null; skipping evictions"
            );
        } else {
            for num in tables_to_evict.into_iter() {
                unsafe {
                    (*(self.table_cache as *mut TableCache)).evict(num);
                }
                tracing::trace!(
                    dbname = %self.dbname,
                    table_number = num,
                    "delete_obsolete_files: evicted table from cache"
                );
            }
        }

        for filename in files_to_delete.into_iter() {
            let full = format!("{}/{}", self.dbname, filename);
            let s: Status = self.env.as_mut().delete_file(&full);

            if !s.is_ok() {
                tracing::warn!(
                    dbname = %self.dbname,
                    path = %full,
                    status = %s.to_string(),
                    "delete_obsolete_files: delete_file failed (ignoring)"
                );
            } else {
                tracing::trace!(
                    dbname = %self.dbname,
                    path = %full,
                    "delete_obsolete_files: deleted file"
                );
            }
        }

        self.mutex.lock();
    }
}

#[cfg(test)]
mod obsolete_file_deletion_contract_suite {
    use super::*;

    fn build_temp_db_path_for_delete_obsolete_files_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn create_file_at_path(path: &str, payload: &[u8]) {
        tracing::debug!(path = %path, bytes = payload.len(), "Creating test file");
        let mut f = std::fs::File::create(path).unwrap_or_else(|e| {
            tracing::error!(path = %path, error = %format!("{:?}", e), "Failed to create test file");
            panic!();
        });
        use std::io::Write;
        f.write_all(payload).unwrap_or_else(|e| {
            tracing::error!(path = %path, error = %format!("{:?}", e), "Failed to write test payload");
            panic!();
        });
        let _ = f.sync_all();
    }

    fn assert_path_exists(path: &str) {
        let ok = std::fs::metadata(path).is_ok();
        tracing::trace!(path = %path, exists = ok, "Asserting path exists");
        assert!(ok, "Expected path to exist: {path}");
    }

    fn assert_path_missing(path: &str) {
        let ok = std::fs::metadata(path).is_ok();
        tracing::trace!(path = %path, exists = ok, "Asserting path missing");
        assert!(!ok, "Expected path to be deleted/missing: {path}");
    }

    fn build_options_with_env_or_panic_for_delete_obsolete_files_suite() -> Options {
        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        if options.env().is_none() {
            tracing::error!("Options::default() did not supply an Env; delete_obsolete_files suite cannot proceed");
            panic!();
        }
        options
    }

    #[traced_test]
    fn delete_obsolete_files_returns_early_and_does_not_touch_disk_when_bg_error_is_set() {
        let dbname = build_temp_db_path_for_delete_obsolete_files_suite();
        let options = build_options_with_env_or_panic_for_delete_obsolete_files_suite();

        let _ = std::fs::create_dir_all(&dbname);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        // Create a table file that would otherwise be eligible for deletion.
        let doomed_table_num: u64 = 12345;
        let doomed_table_path = table_file_name(&dbname, doomed_table_num);
        create_file_at_path(&doomed_table_path, b"table-payload");
        assert_path_exists(&doomed_table_path);

        // Force early return.
        db.bg_error = Status::io_error(&Slice::from_str("bg_error"), None);

        db.mutex.lock();
        tracing::info!("Calling delete_obsolete_files with bg_error set; must early-return");
        db.delete_obsolete_files();
        unsafe { db.mutex.unlock() };

        // The file must still exist since GC should not run when bg_error is non-OK.
        assert_path_exists(&doomed_table_path);

        let _ = std::fs::remove_file(&doomed_table_path);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn delete_obsolete_files_deletes_unreferenced_table_and_temp_files_and_keeps_live_ones() {
        let dbname = build_temp_db_path_for_delete_obsolete_files_suite();
        let options = build_options_with_env_or_panic_for_delete_obsolete_files_suite();

        let _ = std::fs::create_dir_all(&dbname);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        // Choose file numbers for table/temp.
        let live_table_num: u64 = 20001;
        let dead_table_num: u64 = 20002;
        let live_temp_num: u64 = 30001;
        let dead_temp_num: u64 = 30002;

        // Mark "live" ones via pending_outputs.
        db.pending_outputs.insert(live_table_num);
        db.pending_outputs.insert(live_temp_num);

        let live_table_path = table_file_name(&dbname, live_table_num);
        let dead_table_path = table_file_name(&dbname, dead_table_num);
        let live_temp_path = temp_file_name(&dbname, live_temp_num);
        let dead_temp_path = temp_file_name(&dbname, dead_temp_num);

        create_file_at_path(&live_table_path, b"live-table");
        create_file_at_path(&dead_table_path, b"dead-table");
        create_file_at_path(&live_temp_path, b"live-temp");
        create_file_at_path(&dead_temp_path, b"dead-temp");

        // Always-kept files (by type).
        let current_path = current_file_name(&dbname);
        let lock_path = lock_file_name(&dbname);
        let info_log_path = info_log_file_name(&dbname);
        let old_info_log_path = old_info_log_file_name(&dbname);

        create_file_at_path(&current_path, b"CURRENT");
        create_file_at_path(&lock_path, b"LOCK");
        create_file_at_path(&info_log_path, b"LOG");
        create_file_at_path(&old_info_log_path, b"LOG.old");

        // Unparseable file name must be ignored (kept).
        let unparseable_path = format!("{}/{}", dbname, "UNPARSEABLE_FILENAME");
        create_file_at_path(&unparseable_path, b"junk");

        assert_path_exists(&live_table_path);
        assert_path_exists(&dead_table_path);
        assert_path_exists(&live_temp_path);
        assert_path_exists(&dead_temp_path);

        assert_path_exists(&current_path);
        assert_path_exists(&lock_path);
        assert_path_exists(&info_log_path);
        assert_path_exists(&old_info_log_path);
        assert_path_exists(&unparseable_path);

        db.mutex.lock();
        tracing::info!("Calling delete_obsolete_files; expected to delete dead table/temp files only");
        db.delete_obsolete_files();
        // The function unlocks and then re-locks; on return, mutex must be held.
        let still_locked = db.mutex.is_locked();
        tracing::debug!(still_locked, "Mutex lock state after delete_obsolete_files");
        assert!(still_locked, "Mutex must be re-locked before returning from delete_obsolete_files");
        unsafe { db.mutex.unlock() };

        // Validate deletion decisions.
        assert_path_exists(&live_table_path);
        assert_path_missing(&dead_table_path);

        assert_path_exists(&live_temp_path);
        assert_path_missing(&dead_temp_path);

        assert_path_exists(&current_path);
        assert_path_exists(&lock_path);
        assert_path_exists(&info_log_path);
        assert_path_exists(&old_info_log_path);

        assert_path_exists(&unparseable_path);

        // Cleanup remaining artifacts.
        let _ = std::fs::remove_file(&live_table_path);
        let _ = std::fs::remove_file(&live_temp_path);
        let _ = std::fs::remove_file(&current_path);
        let _ = std::fs::remove_file(&lock_path);
        let _ = std::fs::remove_file(&info_log_path);
        let _ = std::fs::remove_file(&old_info_log_path);
        let _ = std::fs::remove_file(&unparseable_path);

        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn delete_obsolete_files_tolerates_missing_directory_without_panicking() {
        let dbname = build_temp_db_path_for_delete_obsolete_files_suite();
        let options = build_options_with_env_or_panic_for_delete_obsolete_files_suite();

        // Intentionally do not create the directory.
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        // Ensure bg_error is OK so it attempts the scan; GetChildren may fail and must be ignored.
        db.bg_error = Status::ok();

        db.mutex.lock();
        tracing::info!(
            dbname = %dbname,
            "Calling delete_obsolete_files on a non-existent directory; must not panic"
        );
        db.delete_obsolete_files();
        unsafe { db.mutex.unlock() };

        // No filesystem artifacts should exist; best-effort remove in case env created anything.
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn delete_obsolete_files_returns_early_when_bg_error_is_set() {
        let dbname = build_temp_db_path_for_delete_obsolete_files_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        let doomed_table_num: u64 = 91001;
        let doomed_table_path = table_file_name(&dbname, doomed_table_num);
        create_file_at_path(&doomed_table_path, b"table-payload");
        assert_path_exists(&doomed_table_path);

        db.bg_error = Status::io_error(&Slice::from_str("bg_error"), None);

        db.mutex.lock();
        tracing::info!("Calling delete_obsolete_files with bg_error set; must early-return");
        db.delete_obsolete_files();
        unsafe { db.mutex.unlock() };

        assert_path_exists(&doomed_table_path);

        let _ = std::fs::remove_file(&doomed_table_path);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn delete_obsolete_files_deletes_dead_table_and_keeps_live_table() {
        let dbname = build_temp_db_path_for_delete_obsolete_files_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));
        db.clear_background_error_for_test();

        let live_table_num: u64 = 92001;
        let dead_table_num: u64 = 92002;

        db.pending_outputs.insert(live_table_num);

        let live_table_path = table_file_name(&dbname, live_table_num);
        let dead_table_path = table_file_name(&dbname, dead_table_num);

        create_file_at_path(&live_table_path, b"live");
        create_file_at_path(&dead_table_path, b"dead");

        assert_path_exists(&live_table_path);
        assert_path_exists(&dead_table_path);

        db.mutex.lock();
        tracing::info!("Calling delete_obsolete_files; expected to delete dead table only");
        db.delete_obsolete_files();
        let still_locked = db.mutex.is_locked();
        tracing::debug!(still_locked, "Mutex lock state after delete_obsolete_files");
        assert!(still_locked);
        unsafe { db.mutex.unlock() };

        assert_path_exists(&live_table_path);
        assert_path_missing(&dead_table_path);

        let _ = std::fs::remove_file(&live_table_path);
        let _ = std::fs::remove_dir_all(&dbname);
    }

    #[traced_test]
    fn delete_obsolete_files_signature_is_stable() {
        tracing::info!("Asserting DBImpl::delete_obsolete_files signature is stable");
        type Sig = fn(&mut DBImpl);
        let _sig: Sig = DBImpl::delete_obsolete_files;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn delete_obsolete_files_uses_get_children_for_memenv_even_when_dbname_file_exists_is_false() {
        let dbname = "/memenv/delete_obsolete_files_memenv_regression".to_string();

        let env = new_mem_env(posix_default_env());
        let options: Options = Options::with_env(env.clone());

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        let dbname_exists_before = env.borrow_mut().file_exists(&dbname);
        tracing::debug!(
            dbname = %dbname,
            dbname_exists_before,
            "MemEnv directory existence probe before creating files"
        );
        assert!(
            !dbname_exists_before,
            "Regression precondition failed: memenv unexpectedly reports dbname as existing"
        );

        let dead_table_num: u64 = 93001;
        let dead_table_path = table_file_name(&dbname, dead_table_num);

        let mut file: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let create_status = env
            .borrow_mut()
            .new_writable_file(&dead_table_path, (&mut file) as *mut *mut Box<dyn WritableFile>);
        assert!(create_status.is_ok());
        assert!(!file.is_null());

        {
            let mut file_holder: Box<Box<dyn WritableFile>> = unsafe { Box::from_raw(file) };
            let file_ref: &mut Box<dyn WritableFile> = file_holder.as_mut();

            let payload = Slice::from(b"dead-table".as_slice());
            let append_status = file_ref.append(&payload);
            assert!(append_status.is_ok());

            let close_status = file_ref.close();
            assert!(close_status.is_ok());
        }

        let dead_table_exists_before = env.borrow_mut().file_exists(&dead_table_path);
        tracing::debug!(
            path = %dead_table_path,
            dead_table_exists_before,
            "Dead table existence before delete_obsolete_files"
        );
        assert!(dead_table_exists_before);

        db.mutex.lock();
        tracing::info!(
            "Calling delete_obsolete_files in memenv regression scenario; dead table must be collected"
        );
        db.delete_obsolete_files();
        let still_locked = db.mutex.is_locked();
        tracing::debug!(still_locked, "Mutex lock state after memenv delete_obsolete_files");
        assert!(still_locked);
        unsafe { db.mutex.unlock() };

        let dead_table_exists_after = env.borrow_mut().file_exists(&dead_table_path);
        tracing::debug!(
            path = %dead_table_path,
            dead_table_exists_after,
            "Dead table existence after delete_obsolete_files"
        );
        assert!(
            !dead_table_exists_after,
            "Expected dead table to be deleted via get_children scan in memenv"
        );
    }
}
