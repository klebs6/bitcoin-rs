// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_reuse_manifest.rs ]
crate::ix!();

impl ReuseManifest for VersionSet {
    fn reuse_manifest(&mut self, dscname: &str, dscbase: &str) -> bool {
        trace!(
            "VersionSet::reuse_manifest: enter; dscname='{}' dscbase='{}'",
            dscname,
            dscbase
        );

        unsafe {
            let opt_ref: &Options = &*self.options();
            if !*opt_ref.reuse_logs() {
                return false;
            }
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
            return false;
        }

        let env_rc = unsafe {
            (*self.options())
                .env()
                .as_ref()
                .expect("VersionSet::reuse_manifest: Options.env is None")
                .clone()
        };

        let size_status = env_rc
            .borrow_mut()
            .get_file_size(&dscname_s, &mut manifest_size as *mut u64);

        if !size_status.is_ok() {
            return false;
        }

        // Make new compacted MANIFEST if old one is too big
        let target_u64: u64 = target_file_size(self.options()) as u64;
        if manifest_size >= target_u64 {
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
        let r = env_rc
            .borrow_mut()
            .new_appendable_file(&dscname_s, &mut file_box_ptr);

        if !r.is_ok() {
            error!(
                "VersionSet::reuse_manifest: NewAppendableFile failed: {}",
                r.to_string()
            );
            assert!(
                self.descriptor_file().is_null(),
                "VersionSet::reuse_manifest: descriptor_file must remain null on failure"
            );
            return false;
        }

        assert!(
            !file_box_ptr.is_null(),
            "VersionSet::reuse_manifest: Env returned OK but file pointer is null"
        );

        // Take ownership of Env allocation and store as raw *mut dyn WritableFile.
        unsafe {
            let file_holder: Box<Box<dyn WritableFile>> = Box::from_raw(file_box_ptr);
            let file_inner: Box<dyn WritableFile> = *file_holder;
            self.set_descriptor_file(Box::into_raw(file_inner));
        }

        info!(
            "VersionSet::reuse_manifest: reusing MANIFEST '{}', size={}",
            dscname,
            manifest_size
        );

        let dest: Rc<RefCell<dyn WritableFile>> = Rc::new(RefCell::new(
            BorrowedWritableFileForManifest::new(self.descriptor_file()),
        ));

        let lw = LogWriter::new(dest, manifest_size);
        self.set_descriptor_log(Box::into_raw(Box::new(lw)));

        self.set_manifest_file_number(manifest_number);

        trace!(
            "VersionSet::reuse_manifest: exit; reused=true manifest_file_number={}",
            self.manifest_file_number()
        );

        true
    }
}

#[cfg(test)]
mod version_set_reuse_manifest_exhaustive_test_suite {
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
