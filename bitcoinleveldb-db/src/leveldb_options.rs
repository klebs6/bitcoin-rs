// ---------------- [ File: bitcoinleveldb-db/src/leveldb_options.rs ]
crate::ix!();

pub fn leveldb_options_create() -> *mut LevelDBOptions {
    
    todo!();
        /*
            return new leveldb_options_t;
        */
}

pub fn leveldb_options_destroy(options: *mut LevelDBOptions)  {
    
    todo!();
        /*
            delete options;
        */
}

pub fn leveldb_options_set_comparator(
        opt: *mut LevelDBOptions,
        cmp: *mut LevelDBComparator)  {
    
    todo!();
        /*
            opt->rep.comparator = cmp;
        */
}

pub fn leveldb_options_set_filter_policy(
        opt:    *mut LevelDBOptions,
        policy: *mut LevelDBFilterPolicy)  {
    
    todo!();
        /*
            opt->rep.filter_policy = policy;
        */
}

pub fn leveldb_options_set_create_if_missing(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.create_if_missing = v;
        */
}

pub fn leveldb_options_set_error_if_exists(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.error_if_exists = v;
        */
}

pub fn leveldb_options_set_paranoid_checks(
        opt: *mut LevelDBOptions,
        v:   u8)  {
    
    todo!();
        /*
            opt->rep.paranoid_checks = v;
        */
}

pub fn leveldb_options_set_env(
        opt: *mut LevelDBOptions,
        env: *mut LevelDBEnv)  {
    
    todo!();
        /*
            opt->rep.env = (env ? env->rep : nullptr);
        */
}

pub fn leveldb_options_set_info_log(
        opt: *mut LevelDBOptions,
        l:   *mut LevelDBLogger)  {
    
    todo!();
        /*
            opt->rep.info_log = (l ? l->rep : nullptr);
        */
}

pub fn leveldb_options_set_write_buffer_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.write_buffer_size = s;
        */
}

pub fn leveldb_options_set_max_open_files(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.max_open_files = n;
        */
}

pub fn leveldb_options_set_cache(
        opt: *mut LevelDBOptions,
        c:   *mut LevelDBCache)  {
    
    todo!();
        /*
            opt->rep.block_cache = c->rep;
        */
}

pub fn leveldb_options_set_block_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.block_size = s;
        */
}

pub fn leveldb_options_set_block_restart_interval(
        opt: *mut LevelDBOptions,
        n:   i32)  {
    
    todo!();
        /*
            opt->rep.block_restart_interval = n;
        */
}

pub fn leveldb_options_set_max_file_size(
        opt: *mut LevelDBOptions,
        s:   usize)  {
    
    todo!();
        /*
            opt->rep.max_file_size = s;
        */
}

pub fn leveldb_options_set_compression(
        opt: *mut LevelDBOptions,
        t:   i32)  {
    
    todo!();
        /*
            opt->rep.compression = static_cast<CompressionType>(t);
        */
}

pub fn leveldb_options_create() -> *mut LevelDBOptions {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_create entry");

    let result = Box::new(LevelDBOptions {
        rep: Options::default(),
    });

    let p = Box::into_raw(result);

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_create exit"; "ptr" => (p as usize));
    p

    /*
        return new leveldb_options_t;
    */
}

pub fn leveldb_options_destroy(options: *mut LevelDBOptions) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_destroy entry"; "options_is_null" => options.is_null());

    unsafe {
        if options.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_options_destroy called with null options");
            return;
        }

        drop(Box::from_raw(options));
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_destroy exit");

    /*
        delete options;
    */
}

pub fn leveldb_options_set_comparator(opt: *mut LevelDBOptions, cmp: *mut LevelDBComparator) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_comparator entry";
        "opt_is_null" => opt.is_null(),
        "cmp_is_null" => cmp.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_comparator: null opt");
            return;
        }

        if cmp.is_null() {
            // Best-effort: reset to default comparator.
            let default_cmp = Options::default().comparator().clone();
            (*opt).rep.set_comparator(default_cmp);
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_comparator: null cmp; reset to default");
            return;
        }

        let arc: Arc<LevelDBComparator> = Arc::from_raw(cmp as *const LevelDBComparator);
        (*opt).rep.set_comparator(arc.clone());
        // Preserve the caller's reference.
        let _ = Arc::into_raw(arc);
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_comparator exit");

    /*
        opt->rep.comparator = cmp;
    */
}

pub fn leveldb_options_set_filter_policy(opt: *mut LevelDBOptions, policy: *mut LevelDBFilterPolicy) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_filter_policy entry";
        "opt_is_null" => opt.is_null(),
        "policy_is_null" => policy.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_filter_policy: null opt");
            return;
        }

        if policy.is_null() {
            // Best-effort: reset to the null policy.
            let null_policy: Arc<dyn FilterPolicy> = Arc::new(NullFilterPolicy::default());
            (*opt).rep.set_filter_policy(null_policy);
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_filter_policy: null policy; reset to NullFilterPolicy");
            return;
        }

        let arc: Arc<LevelDBFilterPolicy> = Arc::from_raw(policy as *const LevelDBFilterPolicy);
        (*opt).rep.set_filter_policy(arc.clone());
        // Preserve the caller's reference.
        let _ = Arc::into_raw(arc);
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_filter_policy exit");

    /*
        opt->rep.filter_policy = policy;
    */
}

pub fn leveldb_options_set_create_if_missing(opt: *mut LevelDBOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_create_if_missing entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_create_if_missing: null opt");
            return;
        }
        (*opt).rep.set_create_if_missing(v != 0);
    }

    /*
        opt->rep.create_if_missing = v;
    */
}

pub fn leveldb_options_set_error_if_exists(opt: *mut LevelDBOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_error_if_exists entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_error_if_exists: null opt");
            return;
        }
        (*opt).rep.set_error_if_exists(v != 0);
    }

    /*
        opt->rep.error_if_exists = v;
    */
}

pub fn leveldb_options_set_paranoid_checks(opt: *mut LevelDBOptions, v: u8) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_paranoid_checks entry"; "opt_is_null" => opt.is_null(), "v" => v);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_paranoid_checks: null opt");
            return;
        }
        (*opt).rep.set_paranoid_checks(v != 0);
    }

    /*
        opt->rep.paranoid_checks = v;
    */
}

pub fn leveldb_options_set_env(opt: *mut LevelDBOptions, env: *mut LevelDBEnv) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_env entry";
        "opt_is_null" => opt.is_null(),
        "env_is_null" => env.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_env: null opt");
            return;
        }

        if env.is_null() {
            (*opt).rep.set_env(None);
        } else {
            (*opt).rep.set_env(Some((*env).rep.clone()));
        }
    }

    /*
        opt->rep.env = (env ? env->rep : nullptr);
    */
}

pub fn leveldb_options_set_info_log(opt: *mut LevelDBOptions, l: *mut LevelDBLogger) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_info_log entry";
        "opt_is_null" => opt.is_null(),
        "logger_is_null" => l.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_info_log: null opt");
            return;
        }

        if l.is_null() {
            (*opt).rep.set_info_log(None);
        } else {
            // Store a raw pointer to the Logger inside the Rc<RefCell<dyn Logger>>.
            let refcell_ptr: *const RefCell<dyn Logger> = Rc::as_ptr(&(*l).rep);
            let logger_ptr: *mut dyn Logger = (&*refcell_ptr).as_ptr();
            (*opt).rep.set_info_log(Some(logger_ptr));
        }
    }

    /*
        opt->rep.info_log = (l ? l->rep : nullptr);
    */
}

pub fn leveldb_options_set_write_buffer_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_write_buffer_size entry"; "opt_is_null" => opt.is_null(), "s" => s);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_write_buffer_size: null opt");
            return;
        }
        (*opt).rep.set_write_buffer_size(s);
    }

    /*
        opt->rep.write_buffer_size = s;
    */
}

pub fn leveldb_options_set_max_open_files(opt: *mut LevelDBOptions, n: i32) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_max_open_files entry"; "opt_is_null" => opt.is_null(), "n" => n);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_max_open_files: null opt");
            return;
        }
        (*opt).rep.set_max_open_files(n);
    }

    /*
        opt->rep.max_open_files = n;
    */
}

pub fn leveldb_options_set_cache(opt: *mut LevelDBOptions, c: *mut LevelDBCache) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        "leveldb_options_set_cache entry";
        "opt_is_null" => opt.is_null(),
        "cache_is_null" => c.is_null()
    );

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_cache: null opt");
            return;
        }

        let cache_ptr: *mut Cache = if c.is_null() {
            core::ptr::null_mut()
        } else {
            let refcell_ptr: *const RefCell<Cache> = Rc::as_ptr(&(*c).rep);
            (&*refcell_ptr).as_ptr()
        };

        (*opt).rep.set_block_cache(cache_ptr);
    }

    /*
        opt->rep.block_cache = c->rep;
    */
}

pub fn leveldb_options_set_block_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_block_size entry"; "opt_is_null" => opt.is_null(), "s" => s);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_block_size: null opt");
            return;
        }
        (*opt).rep.set_block_size(s);
    }

    /*
        opt->rep.block_size = s;
    */
}

pub fn leveldb_options_set_block_restart_interval(opt: *mut LevelDBOptions, n: i32) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_block_restart_interval entry"; "opt_is_null" => opt.is_null(), "n" => n);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_block_restart_interval: null opt");
            return;
        }
        (*opt).rep.set_block_restart_interval(n);
    }

    /*
        opt->rep.block_restart_interval = n;
    */
}

pub fn leveldb_options_set_max_file_size(opt: *mut LevelDBOptions, s: usize) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_max_file_size entry"; "opt_is_null" => opt.is_null(), "s" => s);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_max_file_size: null opt");
            return;
        }
        (*opt).rep.set_max_file_size(s);
    }

    /*
        opt->rep.max_file_size = s;
    */
}

pub fn leveldb_options_set_compression(opt: *mut LevelDBOptions, t: i32) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_compression entry"; "opt_is_null" => opt.is_null(), "t" => t);

    unsafe {
        if opt.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_compression: null opt");
            return;
        }

        let ct = match t {
            0 => CompressionType::None,
            1 => CompressionType::Snappy,
            other => {
                warn!(target: "bitcoinleveldb_db::c_api", "leveldb_options_set_compression: invalid value; using None"; "t" => other);
                CompressionType::None
            }
        };

        (*opt).rep.set_compression(ct);
    }

    /*
        opt->rep.compression = static_cast<CompressionType>(t);
    */
}
