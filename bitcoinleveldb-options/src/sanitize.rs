// ---------------- [ File: bitcoinleveldb-options/src/sanitize.rs ]
crate::ix!();

/**
  | Fix user-supplied options to be reasonable
  |
  */
pub fn clip_to_range<T, V>(
        ptr:      *mut T,
        minvalue: V,
        maxvalue: V)  
where
    T: Copy + From<V>,
    V: Copy + PartialOrd + From<T>,
{
    unsafe {
        if V::from(*ptr) > maxvalue {
            *ptr = T::from(maxvalue);
        }
        if V::from(*ptr) < minvalue {
            *ptr = T::from(minvalue);
        }
    }
}

/**
  | Sanitize db options. The caller should
  | delete result.info_log if it is not
  | equal to src.info_log.
  |
  */
pub fn sanitize_options(
        dbname:  &String,
        icmp:    *const InternalKeyComparator,
        ipolicy: *const InternalFilterPolicy,
        src:     &Options) -> Options {
    
    // Options result = src;
    let mut result: Options = src.clone();

    // result.comparator = icmp;
    unsafe {
        let user_cmp_ptr: *const dyn SliceComparator = (*icmp).user_comparator();
        result.set_comparator(Arc::new(InternalKeyComparator::new(user_cmp_ptr)));
    }

    // result.filter_policy = (src.filter_policy != nullptr) ? ipolicy : nullptr;
    // In this Rust port, `filter_policy` is always present as a trait object.
    // Match the C++ behavior by installing the internal policy wrapper.
    let user_policy_ptr: *const dyn FilterPolicy =
        (&**src.filter_policy()) as *const dyn FilterPolicy;
    let _ = ipolicy;
    result.set_filter_policy(Arc::new(InternalFilterPolicy::new(user_policy_ptr)));

    // ClipToRange(&result.max_open_files, 64 + kNumNonTableCacheFiles, 50000);
    clip_to_range::<i32, i32>(
        result.max_open_files_mut() as *mut i32,
        64 + NUM_NON_TABLE_CACHE_FILES,
        50000,
    );

    // ClipToRange(&result.write_buffer_size, 64 << 10, 1 << 30);
    clip_to_range::<usize, usize>(
        result.write_buffer_size_mut() as *mut usize,
        64usize << 10,
        1usize << 30,
    );

    // ClipToRange(&result.max_file_size, 1 << 20, 1 << 30);
    clip_to_range::<usize, usize>(
        result.max_file_size_mut() as *mut usize,
        1usize << 20,
        1usize << 30,
    );

    // ClipToRange(&result.block_size, 1 << 10, 4 << 20);
    clip_to_range::<usize, usize>(
        result.block_size_mut() as *mut usize,
        1usize << 10,
        4usize << 20,
    );

    if result.info_log().is_none() {
        // Open a log file in the same directory as the db
        if let Some(env_rc) = src.env().clone() {
            // src.env->CreateDir(dbname);  // In case it does not exist
            let _ = env_rc.borrow_mut().create_dir(dbname);

            // src.env->RenameFile(InfoLogFileName(dbname), OldInfoLogFileName(dbname));
            let info_log = info_log_file_name(dbname);
            let old_info_log = old_info_log_file_name(dbname);
            let _ = env_rc.borrow_mut().rename_file(&info_log, &old_info_log);

            // Status s = src.env->NewLogger(InfoLogFileName(dbname), &result.info_log);
            let mut logger_ptr: *mut Box<dyn Logger> = core::ptr::null_mut();
            let s = env_rc.borrow_mut().new_logger(&info_log, &mut logger_ptr);

            if !s.is_ok() {
                // No place suitable for logging
                result.set_info_log(None);
            } else {
                unsafe {
                    if logger_ptr.is_null() {
                        result.set_info_log(None);
                    } else {
                        let outer: Box<Box<dyn Logger>> = Box::from_raw(logger_ptr);
                        let inner: Box<dyn Logger> = *outer;
                        let raw: *mut dyn Logger = Box::into_raw(inner);
                        result.set_info_log(Some(raw));
                    }
                }
            }
        } else {
            // No env available; match "no place suitable for logging".
            result.set_info_log(None);
        }
    }

    if (*result.block_cache()).is_null() {
        result.set_block_cache(new_lru_cache(8usize << 20));
    }

    result
}

#[cfg(test)]
mod sanitize_options_exhaustive_suite {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    use tracing::{debug, info, trace, warn};

    static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn unique_temp_db_dir(prefix: &str) -> (String, PathBuf) {
        let pid = std::process::id() as u64;
        let ctr = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0));

        let mut p = std::env::temp_dir();
        p.push(format!(
            "bitcoinleveldb-options-{}-pid{}-ctr{}-ns{}",
            prefix,
            pid,
            ctr,
            now.as_nanos()
        ));

        let s = p.to_string_lossy().into_owned();
        (s, p)
    }

    fn cleanup_best_effort(dir: &PathBuf) {
        let d = dir.to_string_lossy().into_owned();
        match fs::remove_dir_all(dir) {
            Ok(_) => debug!(dir = %d, "removed temp dir"),
            Err(e) => warn!(dir = %d, err = %e, "failed to remove temp dir (ignored)"),
        }
    }

    fn path_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    unsafe fn drop_owned_logger_ptr(p: Option<*mut dyn Logger>) {
        if let Some(ptr) = p {
            debug!(logger = %format!("{:p}", ptr), "dropping owned Logger");
            drop(Box::from_raw(ptr));
        }
    }

    unsafe fn drop_owned_cache_ptr(p: *mut Cache) {
        if !p.is_null() {
            debug!(cache = %format!("{:p}", p), "dropping owned Cache");
            drop(Box::from_raw(p));
        }
    }

    #[traced_test]
    fn clip_to_range_clamps_values_in_place() {
        trace!("sanitize_options_exhaustive_suite: start");

        let mut a: i32 = 10;
        clip_to_range::<i32, i32>(&mut a as *mut i32, 20, 30);
        assert_eq!(a, 20);

        let mut b: i32 = 40;
        clip_to_range::<i32, i32>(&mut b as *mut i32, 20, 30);
        assert_eq!(b, 30);

        let mut c: i32 = 25;
        clip_to_range::<i32, i32>(&mut c as *mut i32, 20, 30);
        assert_eq!(c, 25);

        let mut x: usize = 1;
        clip_to_range::<usize, usize>(&mut x as *mut usize, 64usize, 128usize);
        assert_eq!(x, 64);

        let mut y: usize = 1024;
        clip_to_range::<usize, usize>(&mut y as *mut usize, 64usize, 128usize);
        assert_eq!(y, 128);

        let mut z: usize = 96;
        clip_to_range::<usize, usize>(&mut z as *mut usize, 64usize, 128usize);
        assert_eq!(z, 96);

        trace!("sanitize_options_exhaustive_suite: done");
    }

    #[traced_test]
    fn sanitize_options_clamps_numeric_fields_and_installs_default_block_cache_and_logger() {
        trace!("sanitize_options_exhaustive_suite: start");

        let (dbname, dir) = unique_temp_db_dir("sanitize-clamps");
        fs::create_dir_all(&dir).unwrap();

        let mut src = Options::default();

        // Force out-of-range values.
        src.set_max_open_files(1);
        src.set_write_buffer_size(1);
        src.set_max_file_size(1);
        src.set_block_size(1);

        // Ensure defaults are taken.
        src.set_info_log(None);
        src.set_block_cache(core::ptr::null_mut());

        let has_env = src.env().is_some();
        debug!(has_env, "source env availability for sanitize_options");

        let icmp = src.internal_key_comparator();

        let user_policy_ptr: *const dyn FilterPolicy =
            (&**src.filter_policy()) as *const dyn FilterPolicy;
        let ipolicy = InternalFilterPolicy::new(user_policy_ptr);

        // Seed an existing LOG so RenameFile(LOG -> OLD_LOG) has an effect (when env is available).
        let log_path = info_log_file_name(&dbname);
        fs::write(&log_path, b"old-log").unwrap();

        let sanitized = sanitize_options(
            &dbname,
            &icmp as *const InternalKeyComparator,
            &ipolicy as *const InternalFilterPolicy,
            &src,
        );

        info!(
            dbname = %dbname,
            max_open_files = *sanitized.max_open_files(),
            write_buffer_size = *sanitized.write_buffer_size(),
            max_file_size = *sanitized.max_file_size(),
            block_size = *sanitized.block_size(),
            has_env,
            info_log_is_some = sanitized.info_log().is_some(),
            block_cache = %format!("{:p}", *sanitized.block_cache()),
            "sanitized options snapshot"
        );

        assert!(
            *sanitized.max_open_files() >= 64 + NUM_NON_TABLE_CACHE_FILES,
            "max_open_files must be clipped up to minimum"
        );
        assert!(
            *sanitized.write_buffer_size() >= (64usize << 10),
            "write_buffer_size must be clipped up to minimum"
        );
        assert!(
            *sanitized.max_file_size() >= (1usize << 20),
            "max_file_size must be clipped up to minimum"
        );
        assert!(
            *sanitized.block_size() >= (1usize << 10),
            "block_size must be clipped up to minimum"
        );

        assert!(
            !(*sanitized.block_cache()).is_null(),
            "expected default block_cache to be installed"
        );

        let old_log_path = old_info_log_file_name(&dbname);
        info!(old_log_path = %old_log_path, "checking LOG/LOG.old behavior");

        if has_env {
            assert!(
                sanitized.info_log().is_some(),
                "expected info_log to be created when env is available"
            );
            assert!(
                path_exists(&old_log_path),
                "expected old LOG to exist after rename when env is available"
            );
            assert!(
                path_exists(&log_path),
                "expected new LOG path to exist after NewLogger when env is available"
            );
        } else {
            assert!(
                sanitized.info_log().is_none(),
                "expected no info_log when env is unavailable"
            );
            assert!(
                !path_exists(&old_log_path),
                "did not expect LOG.old to exist when env is unavailable"
            );
            assert!(
                path_exists(&log_path),
                "expected pre-existing LOG path to remain when env is unavailable"
            );
        }

        unsafe {
            drop_owned_cache_ptr(*sanitized.block_cache());
            drop_owned_logger_ptr(*sanitized.info_log());
        }

        cleanup_best_effort(&dir);

        trace!("sanitize_options_exhaustive_suite: done");
    }

    #[traced_test]
    fn sanitize_options_does_not_replace_non_null_block_cache_pointer() {
        trace!("sanitize_options_exhaustive_suite: start");

        let (dbname, dir) = unique_temp_db_dir("sanitize-cache-preserve");
        fs::create_dir_all(&dir).unwrap();

        let mut src = Options::default();

        let cache_ptr = new_lru_cache(1usize << 20);
        assert!(!cache_ptr.is_null());
        src.set_block_cache(cache_ptr);

        let icmp = src.internal_key_comparator();

        let user_policy_ptr: *const dyn FilterPolicy =
            (&**src.filter_policy()) as *const dyn FilterPolicy;
        let ipolicy = InternalFilterPolicy::new(user_policy_ptr);

        let sanitized = sanitize_options(
            &dbname,
            &icmp as *const InternalKeyComparator,
            &ipolicy as *const InternalFilterPolicy,
            &src,
        );

        let got_cache = *sanitized.block_cache();
        info!(
            src_cache = %format!("{:p}", cache_ptr),
            got_cache = %format!("{:p}", got_cache),
            "block_cache pointer preservation"
        );

        assert_eq!(got_cache, cache_ptr);

        unsafe {
            // Logger may have been created (src.info_log was None).
            drop_owned_logger_ptr(*sanitized.info_log());
            // Cache was provided by us; drop exactly once.
            drop_owned_cache_ptr(cache_ptr);
        }

        cleanup_best_effort(&dir);

        trace!("sanitize_options_exhaustive_suite: done");
    }

    #[traced_test]
    fn sanitize_options_does_not_replace_existing_info_log_pointer() {

        #[derive(Default)]
        struct InfoLogPointerPreservationLoggerStub {
            calls: usize,
        }

        impl Logv for InfoLogPointerPreservationLoggerStub {
            fn logv(&mut self, format: *const u8, ap: &[&str]) {
                self.calls += 1;
                tracing::trace!(
                    calls = self.calls,
                    format_ptr = %format!("{:p}", format),
                    argc = ap.len(),
                    "InfoLogPointerPreservationLoggerStub::logv"
                );
            }
        }

        impl Logger for InfoLogPointerPreservationLoggerStub {}

        trace!("sanitize_options_exhaustive_suite: start");

        let (dbname, dir) = unique_temp_db_dir("sanitize-infolog-preserve");
        fs::create_dir_all(&dir).unwrap();

        let mut src = Options::default();

        // Install a logger explicitly (without requiring an Env) so sanitize_options must preserve it.
        let boxed: Box<dyn Logger> = Box::new(InfoLogPointerPreservationLoggerStub::default());
        let raw_logger: *mut dyn Logger = Box::into_raw(boxed);

        src.set_info_log(Some(raw_logger));

        // Allow sanitize_options to allocate a cache so we can verify it doesn't disturb info_log.
        src.set_block_cache(core::ptr::null_mut());

        let icmp = src.internal_key_comparator();

        let user_policy_ptr: *const dyn FilterPolicy =
            (&**src.filter_policy()) as *const dyn FilterPolicy;
        let ipolicy = InternalFilterPolicy::new(user_policy_ptr);

        let sanitized = sanitize_options(
            &dbname,
            &icmp as *const InternalKeyComparator,
            &ipolicy as *const InternalFilterPolicy,
            &src,
        );

        let got_logger = *sanitized.info_log();
        info!(
            src_logger = %format!("{:p}", raw_logger),
            got_logger = %format!("{:p}", got_logger.expect("expected Some logger")),
            "info_log pointer preservation"
        );

        assert_eq!(got_logger, Some(raw_logger));

        unsafe {
            drop_owned_cache_ptr(*sanitized.block_cache());
            drop_owned_logger_ptr(Some(raw_logger));
        }

        cleanup_best_effort(&dir);

        trace!("sanitize_options_exhaustive_suite: done");
    }

    #[traced_test]
    fn sanitize_options_leaves_info_log_unset_when_env_is_none() {
        trace!("sanitize_options_exhaustive_suite: start");

        let (dbname, dir) = unique_temp_db_dir("sanitize-no-env");
        fs::create_dir_all(&dir).unwrap();

        let mut src = Options::default();
        src.set_env(None);
        src.set_info_log(None);
        src.set_block_cache(core::ptr::null_mut());

        let icmp = src.internal_key_comparator();

        let user_policy_ptr: *const dyn FilterPolicy =
            (&**src.filter_policy()) as *const dyn FilterPolicy;
        let ipolicy = InternalFilterPolicy::new(user_policy_ptr);

        let sanitized = sanitize_options(
            &dbname,
            &icmp as *const InternalKeyComparator,
            &ipolicy as *const InternalFilterPolicy,
            &src,
        );

        info!(
            info_log_is_some = sanitized.info_log().is_some(),
            "sanitized info_log with env=None"
        );

        assert!(sanitized.info_log().is_none(), "expected no info_log when env is None");

        unsafe {
            drop_owned_cache_ptr(*sanitized.block_cache());
        }

        cleanup_best_effort(&dir);

        trace!("sanitize_options_exhaustive_suite: done");
    }
}
