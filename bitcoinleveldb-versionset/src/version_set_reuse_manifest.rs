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

        true
    }
}

#[cfg(test)]
mod version_set_reuse_manifest_exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn reuse_manifest_returns_false_when_no_manifest_exists() {
        let dir = build_unique_temporary_database_directory_path("versionset_reuse_manifest_none");
        create_directory_tree_or_panic(&dir);
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(false);
        options.set_error_if_exists(false);
        options.set_reuse_logs(true);

        let icmp = Box::new(build_internal_key_comparator_from_database_options(options.as_ref()));

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

        debug!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_missing_manifest_result",
            reused = reused,
            descriptor_file_is_null = vs.descriptor_file().is_null(),
            descriptor_log_is_null = vs.descriptor_log().is_null()
        );

        assert!(
            !reused,
            "reuse_manifest must return false when the manifest file does not exist even if reuse_logs is enabled"
        );
        assert!(
            vs.descriptor_file().is_null(),
            "descriptor_file must remain null when manifest reuse fails"
        );
        assert!(
            vs.descriptor_log().is_null(),
            "descriptor_log must remain null when manifest reuse fails"
        );

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn reuse_manifest_true_for_small_existing_manifest_after_initial_recover() {
        let dir = build_unique_temporary_database_directory_path("versionset_reuse_manifest_small");
        create_directory_tree_or_panic(&dir);
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options.set_reuse_logs(true);

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
        info!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_small_initial_recover",
            save_manifest = save_manifest,
            status = ?st
        );
        assert_status_is_ok_or_panic(&st, "recover");

        let manifest = match find_manifest_file_in_directory(&dir) {
            Some(manifest) => manifest,
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
                    event = "versionset_reuse_manifest_small_manifest_missing_after_recover",
                    directory = %dir.display()
                );
                panic!("versionset_reuse_manifest_small_manifest_missing_after_recover");
            }
        };

        let manifest_size_before_reuse = read_file_size_or_panic(manifest.as_path());
        let reuse_threshold_bytes: u64 = target_file_size(options.as_ref()) as u64;

        debug!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_small_size_check",
            manifest_path = %manifest.display(),
            manifest_size_before_reuse = manifest_size_before_reuse,
            reuse_threshold_bytes = reuse_threshold_bytes
        );

        assert!(
            manifest_size_before_reuse < reuse_threshold_bytes,
            "the small-manifest reuse test requires a manifest smaller than target_file_size"
        );

        let dscname = manifest.to_string_lossy().to_string();
        let dscbase = match manifest.file_name().and_then(|file_name| file_name.to_str()) {
            Some(file_name) => file_name.to_string(),
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
                    event = "versionset_reuse_manifest_small_manifest_name_invalid_utf8",
                    manifest_path = %manifest.display()
                );
                panic!("versionset_reuse_manifest_small_manifest_name_invalid_utf8");
            }
        };

        let mut manifest_number: u64 = 0;
        let mut manifest_type: FileType = FileType::CurrentFile;

        let parsed_ok = parse_file_name(
            &dscbase,
            &mut manifest_number as *mut u64,
            &mut manifest_type as *mut FileType,
        );

        assert!(parsed_ok, "manifest file name must parse successfully");
        assert!(
            matches!(manifest_type, FileType::DescriptorFile),
            "parsed manifest file name must be classified as a descriptor file"
        );

        let reused = vs.reuse_manifest(&dscname, &dscbase);

        debug!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_small_reuse_result",
            reused = reused,
            manifest_number = manifest_number,
            descriptor_file_is_null = vs.descriptor_file().is_null(),
            descriptor_log_is_null = vs.descriptor_log().is_null(),
            stored_manifest_file_number = vs.manifest_file_number()
        );

        assert!(
            reused,
            "reuse_manifest must return true when reuse_logs is enabled and the existing manifest is below target_file_size"
        );
        assert!(
            !vs.descriptor_file().is_null(),
            "descriptor_file must be installed when manifest reuse succeeds"
        );
        assert!(
            !vs.descriptor_log().is_null(),
            "descriptor_log must be installed when manifest reuse succeeds"
        );
        assert_eq!(
            vs.manifest_file_number(),
            manifest_number,
            "manifest_file_number must track the reused descriptor number"
        );

        remove_directory_tree_best_effort(&dir);
    }

    #[traced_test]
    fn reuse_manifest_false_for_large_existing_manifest() {
        let dir = build_unique_temporary_database_directory_path("versionset_reuse_manifest_large");
        create_directory_tree_or_panic(&dir);
        let dbname = Box::new(dir.to_string_lossy().to_string());

        let env = PosixEnv::shared();
        let mut options = Box::new(Options::with_env(env));
        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options.set_reuse_logs(true);

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
        assert_status_is_ok_or_panic(&st, "recover");

        let manifest = match find_manifest_file_in_directory(&dir) {
            Some(manifest) => manifest,
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
                    event = "versionset_reuse_manifest_large_manifest_missing_after_recover",
                    directory = %dir.display()
                );
                panic!("versionset_reuse_manifest_large_manifest_missing_after_recover");
            }
        };

        let dscname = manifest.to_string_lossy().to_string();
        let dscbase = match manifest.file_name().and_then(|file_name| file_name.to_str()) {
            Some(file_name) => file_name.to_string(),
            None => {
                error!(
                    target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
                    event = "versionset_reuse_manifest_large_manifest_name_invalid_utf8",
                    manifest_path = %manifest.display()
                );
                panic!("versionset_reuse_manifest_large_manifest_name_invalid_utf8");
            }
        };

        let reuse_threshold_bytes: u64 = target_file_size(options.as_ref()) as u64;
        let manifest_size_before_growth = read_file_size_or_panic(manifest.as_path());

        let bytes_to_append_u64 = match manifest_size_before_growth >= reuse_threshold_bytes {
            true => 0_u64,
            false => reuse_threshold_bytes
                .saturating_sub(manifest_size_before_growth)
                .saturating_add(1),
        };

        let bytes_to_append: usize = match usize::try_from(bytes_to_append_u64) {
            Ok(value) => value,
            Err(error) => {
                error!(
                    target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
                    event = "versionset_reuse_manifest_large_append_size_conversion_error",
                    bytes_to_append_u64 = bytes_to_append_u64,
                    error = ?error
                );
                panic!("versionset_reuse_manifest_large_append_size_conversion_error");
            }
        };

        append_zero_bytes_to_file_or_panic(manifest.as_path(), bytes_to_append);

        let manifest_size_after_growth = read_file_size_or_panic(manifest.as_path());

        debug!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_large_size_check",
            manifest_path = %manifest.display(),
            reuse_threshold_bytes = reuse_threshold_bytes,
            manifest_size_before_growth = manifest_size_before_growth,
            bytes_to_append = bytes_to_append,
            manifest_size_after_growth = manifest_size_after_growth
        );

        assert!(
            manifest_size_after_growth >= reuse_threshold_bytes,
            "the large-manifest reuse test requires a manifest at or above target_file_size"
        );

        let reused = vs.reuse_manifest(&dscname, &dscbase);

        debug!(
            target: "bitcoinleveldb_versionset::version_set_reuse_manifest::test",
            event = "versionset_reuse_manifest_large_reuse_result",
            reused = reused,
            descriptor_file_is_null = vs.descriptor_file().is_null(),
            descriptor_log_is_null = vs.descriptor_log().is_null()
        );

        assert!(
            !reused,
            "reuse_manifest must return false when the existing manifest size reaches or exceeds target_file_size"
        );
        assert!(
            vs.descriptor_file().is_null(),
            "descriptor_file must remain null when a large manifest is rejected"
        );
        assert!(
            vs.descriptor_log().is_null(),
            "descriptor_log must remain null when a large manifest is rejected"
        );

        remove_directory_tree_best_effort(&dir);
    }
}
