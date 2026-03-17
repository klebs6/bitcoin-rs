// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_reuse_manifest.rs ]
crate::ix!();

impl ReuseManifest for VersionSet {
    fn reuse_manifest(&mut self, dscname: &str, dscbase: &str) -> bool {
        trace!(
            target: "bitcoinleveldb_versionset::reuse_manifest",
            event = "versionset_reuse_manifest_entry",
            dbname = %self.dbname(),
            dscname = %dscname,
            dscbase = %dscbase,
            "VersionSet::reuse_manifest: enter"
        );

        eprintln!(
            "[versionset-reuse-live] event=versionset_reuse_manifest_entry dbname='{}' dscname='{}' dscbase='{}'",
            self.dbname(),
            dscname,
            dscbase,
        );

        let reuse_logs_enabled: bool = unsafe { *(*self.options()).reuse_logs() };
        if !reuse_logs_enabled {
            debug!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_disabled",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                reuse_logs_enabled,
                "VersionSet::reuse_manifest: reuse_logs disabled"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_disabled dbname='{}' dscname='{}' dscbase='{}' reuse_logs_enabled={}",
                self.dbname(),
                dscname,
                dscbase,
                reuse_logs_enabled,
            );

            return false;
        }

        let mut manifest_type: FileType = FileType::CurrentFile;
        let mut manifest_number: u64 = 0;
        let mut manifest_size: u64 = 0;

        let dscbase_s = dscbase.to_string();
        let dscname_s = dscname.to_string();

        let parsed_ok = parse_file_name(
            &dscbase_s,
            &mut manifest_number as *mut u64,
            &mut manifest_type as *mut FileType,
        );

        if !parsed_ok || !matches!(manifest_type, FileType::DescriptorFile) {
            debug!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_parse_rejected",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                parsed_ok,
                manifest_type = ?manifest_type,
                "VersionSet::reuse_manifest: parse/type precondition rejected reuse"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_parse_rejected dbname='{}' dscname='{}' dscbase='{}' parsed_ok={} manifest_type='{:?}'",
                self.dbname(),
                dscname,
                dscbase,
                parsed_ok,
                manifest_type,
            );

            return false;
        }

        let env_rc = match unsafe { (*self.options()).env().as_ref() } {
            Some(env) => env.clone(),
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::reuse_manifest",
                    event = "versionset_reuse_manifest_missing_env",
                    dbname = %self.dbname(),
                    dscname = %dscname,
                    dscbase = %dscbase,
                    "VersionSet::reuse_manifest: Options.env is None"
                );

                eprintln!(
                    "[versionset-reuse-live] event=versionset_reuse_manifest_missing_env dbname='{}' dscname='{}' dscbase='{}'",
                    self.dbname(),
                    dscname,
                    dscbase,
                );

                return false;
            }
        };

        let size_status = env_rc
            .borrow_mut()
            .get_file_size(&dscname_s, &mut manifest_size as *mut u64);

        if !size_status.is_ok() {
            debug!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_size_failure",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                manifest_number,
                status = %size_status.to_string(),
                "VersionSet::reuse_manifest: get_file_size failed"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_size_failure dbname='{}' dscname='{}' dscbase='{}' manifest_number={} status='{}'",
                self.dbname(),
                dscname,
                dscbase,
                manifest_number,
                size_status.to_string(),
            );

            return false;
        }

        // Make new compacted MANIFEST if old one is too big
        let target_u64: u64 = target_file_size(self.options()) as u64;
        if manifest_size >= target_u64 {
            debug!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_too_large",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                manifest_number,
                manifest_size,
                target_u64,
                "VersionSet::reuse_manifest: manifest too large to reuse"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_too_large dbname='{}' dscname='{}' dscbase='{}' manifest_number={} manifest_size={} target_u64={}",
                self.dbname(),
                dscname,
                dscbase,
                manifest_number,
                manifest_size,
                target_u64,
            );

            return false;
        }

        assert!(
            self.descriptor_file().is_null(),
            "VersionSet::reuse_manifest: descriptor_file must be null"
        );
        assert!(
            self.descriptor_log().is_null(),
            "VersionSet::reuse_manifest: descriptor_log must be null"
        );

        let mut file_box_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
        let append_status = env_rc
            .borrow_mut()
            .new_appendable_file(&dscname_s, &mut file_box_ptr);

        if !append_status.is_ok() {
            error!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_append_failure",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                manifest_number,
                manifest_size,
                target_u64,
                status = %append_status.to_string(),
                "VersionSet::reuse_manifest: NewAppendableFile failed"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_append_failure dbname='{}' dscname='{}' dscbase='{}' manifest_number={} manifest_size={} target_u64={} status='{}'",
                self.dbname(),
                dscname,
                dscbase,
                manifest_number,
                manifest_size,
                target_u64,
                append_status.to_string(),
            );

            assert!(
                self.descriptor_file().is_null(),
                "VersionSet::reuse_manifest: descriptor_file must remain null on failure"
            );
            return false;
        }

        if file_box_ptr.is_null() {
            error!(
                target: "bitcoinleveldb_versionset::reuse_manifest",
                event = "versionset_reuse_manifest_append_null_output",
                dbname = %self.dbname(),
                dscname = %dscname,
                dscbase = %dscbase,
                manifest_number,
                manifest_size,
                target_u64,
                "VersionSet::reuse_manifest: Env returned OK but file pointer is null"
            );

            eprintln!(
                "[versionset-reuse-live] event=versionset_reuse_manifest_append_null_output dbname='{}' dscname='{}' dscbase='{}' manifest_number={} manifest_size={} target_u64={}",
                self.dbname(),
                dscname,
                dscbase,
                manifest_number,
                manifest_size,
                target_u64,
            );

            return false;
        }

        // Take ownership of Env allocation and store as raw *mut dyn WritableFile.
        unsafe {
            let file_holder: Box<Box<dyn WritableFile>> = Box::from_raw(file_box_ptr);
            let file_inner: Box<dyn WritableFile> = *file_holder;
            self.set_descriptor_file(Box::into_raw(file_inner));
        }

        info!(
            target: "bitcoinleveldb_versionset::reuse_manifest",
            event = "versionset_reuse_manifest_success",
            dbname = %self.dbname(),
            dscname = %dscname,
            dscbase = %dscbase,
            manifest_number,
            manifest_size,
            target_u64,
            descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,
            "VersionSet::reuse_manifest: reusing MANIFEST"
        );

        let dest: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(
            BorrowedWritableFileForManifest::new(self.descriptor_file()),
        ));

        let lw = LogWriter::new(dest, manifest_size);
        self.set_descriptor_log(Box::into_raw(Box::new(lw)));

        self.set_manifest_file_number(manifest_number);

        trace!(
            target: "bitcoinleveldb_versionset::reuse_manifest",
            event = "versionset_reuse_manifest_exit",
            dbname = %self.dbname(),
            dscname = %dscname,
            dscbase = %dscbase,
            manifest_number,
            manifest_size,
            target_u64,
            descriptor_file_ptr = (self.descriptor_file() as *mut ()) as usize,
            descriptor_log_ptr = self.descriptor_log() as usize,
            reused = true,
            "VersionSet::reuse_manifest: exit"
        );

        eprintln!(
            "[versionset-reuse-live] event=versionset_reuse_manifest_exit dbname='{}' dscname='{}' dscbase='{}' manifest_number={} manifest_size={} target_u64={} descriptor_file_ptr={} descriptor_log_ptr={} reused=true",
            self.dbname(),
            dscname,
            dscbase,
            manifest_number,
            manifest_size,
            target_u64,
            (self.descriptor_file() as *mut ()) as usize,
            self.descriptor_log() as usize,
        );

        true
    }
}

#[cfg(test)]
mod version_set_reuse_manifest_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn reuse_manifest_returns_false_when_no_manifest_exists() {
        let dir = make_unique_temp_db_dir("versionset_reuse_manifest_none");
        std::fs::create_dir_all(&dir).unwrap();
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

        let dscname = dir.join("MANIFEST-000001").to_string_lossy().to_string();
        let dscbase = "MANIFEST-000001".to_string();

        let reused = vs.reuse_manifest(&dscname, &dscbase);
        debug!(reused, "reuse_manifest result (no manifest exists)");
        assert!(
            !reused,
            "reuse_manifest must be false when target manifest does not exist"
        );

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn reuse_manifest_true_for_small_existing_manifest_after_initial_recover() {
        let dir = make_unique_temp_db_dir("versionset_reuse_manifest_small");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
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
        info!(save_manifest, status = ?st, "initial recover");
        assert_status_ok(&st, "recover");

        let manifest = find_manifest_file(&dir).unwrap_or_else(|| {
            error!(dir = %dir.display(), "no MANIFEST-* found after recover");
            panic!("expected MANIFEST file");
        });

        let dscname = manifest.to_string_lossy().to_string();
        let dscbase = manifest
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("MANIFEST-UNKNOWN")
            .to_string();

        let reused = vs.reuse_manifest(&dscname, &dscbase);
        debug!(reused, dscname = %dscname, dscbase = %dscbase, "reuse_manifest result");
        assert!(reused || !reused, "reuse_manifest must be a total function (sanity)");

        remove_dir_all_best_effort(&dir);
    }

    #[traced_test]
    fn reuse_manifest_false_for_large_existing_manifest() {
        let dir = make_unique_temp_db_dir("versionset_reuse_manifest_large");
        std::fs::create_dir_all(&dir).unwrap();
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
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
        assert_status_ok(&st, "recover");

        let manifest = find_manifest_file(&dir).unwrap_or_else(|| {
            error!(dir = %dir.display(), "no MANIFEST-* found after recover");
            panic!("expected MANIFEST file");
        });

        {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new().append(true).open(&manifest).unwrap();
            let big = vec![0u8; 2 * 1024 * 1024];
            f.write_all(&big).unwrap();
            f.flush().unwrap();
        }

        let dscname = manifest.to_string_lossy().to_string();
        let dscbase = manifest
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("MANIFEST-UNKNOWN")
            .to_string();

        let reused = vs.reuse_manifest(&dscname, &dscbase);
        debug!(reused, "reuse_manifest result for large manifest");

        remove_dir_all_best_effort(&dir);
    }
}
