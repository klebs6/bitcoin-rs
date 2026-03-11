// ---------------- [ File: bitcoinleveldb-db/src/leveldb_options.rs ]
crate::ix!();

pub fn leveldb_options_create() -> *mut LevelDBOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_create entry");

    let result = Box::new(LevelDBOptions::default());

    let p = Box::into_raw(result);

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_options_create exit"
    );
    p
}

pub fn leveldb_options_destroy(options: *mut LevelDBOptions) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        options_is_null = options.is_null(),
        "leveldb_options_destroy entry"
    );

    unsafe {
        if options.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_destroy called with null options"
            );
            return;
        }

        drop(Box::from_raw(options));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_destroy exit");
}

pub fn leveldb_options_set_comparator(opt: *mut LevelDBOptions, cmp: *mut LevelDBComparator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        cmp_is_null = cmp.is_null(),
        "leveldb_options_set_comparator entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_comparator: null opt"
            );
            return;
        }

        if cmp.is_null() {
            let default_cmp = Options::default().comparator().clone();
            (*opt).rep_mut().set_comparator(default_cmp);
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_comparator: null cmp; reset to default"
            );
            return;
        }

        let arc: Arc<LevelDBComparator> = Arc::from_raw(cmp as *const LevelDBComparator);
        (*opt).rep_mut().set_comparator(arc.clone());
        let _ = Arc::into_raw(arc);
    }

    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_comparator exit"
    );

}

pub fn leveldb_options_set_filter_policy(opt: *mut LevelDBOptions, policy: *mut LevelDBFilterPolicy) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        policy_is_null = policy.is_null(),
        "leveldb_options_set_filter_policy entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_filter_policy: null opt"
            );
            return;
        }

        if policy.is_null() {
            let null_policy: Arc<dyn FilterPolicy> = Arc::new(NullFilterPolicy::default());
            (*opt).rep_mut().set_filter_policy(null_policy);
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_filter_policy: null policy; reset to NullFilterPolicy"
            );
            return;
        }

        let arc: Arc<LevelDBFilterPolicy> = Arc::from_raw(policy as *const LevelDBFilterPolicy);
        (*opt).rep_mut().set_filter_policy(arc.clone());
        let _ = Arc::into_raw(arc);
    }

    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_filter_policy exit"
    );

}

pub fn leveldb_options_set_create_if_missing(opt: *mut LevelDBOptions, v: u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_options_set_create_if_missing entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_create_if_missing: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_create_if_missing(v != 0);
    }

}

pub fn leveldb_options_set_error_if_exists(opt: *mut LevelDBOptions, v: u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_options_set_error_if_exists entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_error_if_exists: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_error_if_exists(v != 0);
    }

}

pub fn leveldb_options_set_paranoid_checks(opt: *mut LevelDBOptions, v: u8) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        v = v,
        "leveldb_options_set_paranoid_checks entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_paranoid_checks: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_paranoid_checks(v != 0);
    }

}

pub fn leveldb_options_set_env(opt: *mut LevelDBOptions, env: *mut LevelDBEnv) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        env_is_null = env.is_null(),
        "leveldb_options_set_env entry"
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_env: null opt");
            return;
        }

        if env.is_null() {
            (*opt).rep_mut().set_env(None);
        } else {
            (*opt).rep_mut().set_env(Some((*env).rep().clone()));
        }
    }

}

pub fn leveldb_options_set_info_log(opt: *mut LevelDBOptions, l: *mut LevelDBLogger) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        logger_is_null = l.is_null(),
        "leveldb_options_set_info_log entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_info_log: null opt"
            );
            return;
        }

        if l.is_null() {
            (*opt).rep_mut().set_info_log(None);
        } else {
            let refcell_ptr: *const RefCell<dyn Logger> = Rc::as_ptr((*l).rep());
            let logger_ptr: *mut dyn Logger = (&*refcell_ptr).as_ptr();
            (*opt).rep_mut().set_info_log(Some(logger_ptr));
        }
    }

}

pub fn leveldb_options_set_write_buffer_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        s = s,
        "leveldb_options_set_write_buffer_size entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_write_buffer_size: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_write_buffer_size(s);
    }

}

pub fn leveldb_options_set_max_open_files(opt: *mut LevelDBOptions, n: i32) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        n = n,
        "leveldb_options_set_max_open_files entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_max_open_files: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_max_open_files(n);
    }

}

pub fn leveldb_options_set_cache(opt: *mut LevelDBOptions, c: *mut LevelDBCache) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        cache_is_null = c.is_null(),
        "leveldb_options_set_cache entry"
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_cache: null opt");
            return;
        }

        let cache_ptr: *mut Cache = if c.is_null() {
            core::ptr::null_mut()
        } else {
            let refcell_ptr: *const RefCell<Cache> = Rc::as_ptr((*c).rep());
            (&*refcell_ptr).as_ptr()
        };

        (*opt).rep_mut().set_block_cache(cache_ptr);
    }

}

pub fn leveldb_options_set_block_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        s = s,
        "leveldb_options_set_block_size entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_block_size: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_block_size(s);
    }

}

pub fn leveldb_options_set_block_restart_interval(opt: *mut LevelDBOptions, n: i32) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        n = n,
        "leveldb_options_set_block_restart_interval entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_block_restart_interval: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_block_restart_interval(n);
    }

}

pub fn leveldb_options_set_max_file_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        s = s,
        "leveldb_options_set_max_file_size entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_max_file_size: null opt"
            );
            return;
        }
        (*opt).rep_mut().set_max_file_size(s);
    }

}

pub fn leveldb_options_set_compression(opt: *mut LevelDBOptions, t: i32) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        opt_is_null = opt.is_null(),
        t = t,
        "leveldb_options_set_compression entry"
    );

    unsafe {
        if opt.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_options_set_compression: null opt"
            );
            return;
        }

        let ct = match t {
            0 => CompressionType::None,
            1 => CompressionType::Snappy,
            other => {
                warn!(
                    target: "bitcoinleveldb_db::c_api",
                    t = other,
                    "leveldb_options_set_compression: invalid value; using None"
                );
                CompressionType::None
            }
        };

        (*opt).rep_mut().set_compression(ct);
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_options_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_options_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__options_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_options_rs__create_destroy_roundtrip_is_safe() {
        unsafe {
            let opt: *mut LevelDBOptions = leveldb_options_create();
            assert!(!opt.is_null());
            leveldb_options_destroy(opt);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_options_rs__all_setters_handle_null_opt_pointer_without_crashing() {
        unsafe {
            leveldb_options_set_comparator(core::ptr::null_mut(), core::ptr::null_mut());
            leveldb_options_set_filter_policy(core::ptr::null_mut(), core::ptr::null_mut());
            leveldb_options_set_create_if_missing(core::ptr::null_mut(), 1u8);
            leveldb_options_set_error_if_exists(core::ptr::null_mut(), 1u8);
            leveldb_options_set_paranoid_checks(core::ptr::null_mut(), 1u8);
            leveldb_options_set_env(core::ptr::null_mut(), core::ptr::null_mut());
            leveldb_options_set_info_log(core::ptr::null_mut(), core::ptr::null_mut());
            leveldb_options_set_write_buffer_size(core::ptr::null_mut(), 4096usize);
            leveldb_options_set_max_open_files(core::ptr::null_mut(), 100);
            leveldb_options_set_cache(core::ptr::null_mut(), core::ptr::null_mut());
            leveldb_options_set_block_size(core::ptr::null_mut(), 4096usize);
            leveldb_options_set_block_restart_interval(core::ptr::null_mut(), 16);
            leveldb_options_set_max_file_size(core::ptr::null_mut(), 2 * 1024 * 1024usize);
            leveldb_options_set_compression(core::ptr::null_mut(), 0);
        }

        assert!(true);
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_options_rs__setters_accept_valid_handles_and_open_smoke_works() {
        unsafe {
            let opt: *mut LevelDBOptions = leveldb_options_create();
            assert!(!opt.is_null());

            leveldb_options_set_create_if_missing(opt, 1u8);
            leveldb_options_set_error_if_exists(opt, 0u8);
            leveldb_options_set_paranoid_checks(opt, 0u8);
            leveldb_options_set_write_buffer_size(opt, 64 * 1024usize);
            leveldb_options_set_max_open_files(opt, 32);
            leveldb_options_set_block_size(opt, 4096usize);
            leveldb_options_set_block_restart_interval(opt, 16);
            leveldb_options_set_max_file_size(opt, 2 * 1024 * 1024usize);
            leveldb_options_set_compression(opt, 99);

            let env: *mut LevelDBEnv = crate::leveldb_env::leveldb_create_default_env();
            assert!(!env.is_null());
            leveldb_options_set_env(opt, env);
            crate::leveldb_env::leveldb_env_destroy(env);

            let cache: *mut LevelDBCache = crate::leveldb_cache::leveldb_cache_create_lru(8 * 1024usize);
            assert!(!cache.is_null());
            leveldb_options_set_cache(opt, cache);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_options_rs__make_unique_dbname_bytes();

            let mut err: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                opt,
                dbname_bytes.as_ptr(),
                (&mut err) as *mut *mut u8,
            );

            assert!(err.is_null());
            assert!(!db.is_null());

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                opt,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            if !derr.is_null() {
                crate::leveldb_free::leveldb_free(derr as *mut core::ffi::c_void);
            }

            leveldb_options_destroy(opt);

            crate::leveldb_cache::leveldb_cache_destroy(cache);
        }
    }
}
