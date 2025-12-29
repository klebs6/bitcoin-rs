// ---------------- [ File: bitcoinleveldb-repair/src/create.rs ]
crate::ix!();

impl Repairer {

    pub fn new(dbname: &str, options: &Options) -> Self {
        trace!(dbname = %dbname, "Repairer::new: start");

        let dbname_owned = dbname.clone();

        let user_cmp_ptr: *const dyn SliceComparator = &**options.comparator();
        let icmp = InternalKeyComparator::new(user_cmp_ptr);

        let user_policy_ptr: *const dyn FilterPolicy = &**options.filter_policy();
        let ipolicy = InternalFilterPolicy::new(user_policy_ptr);

        let sanitized = sanitize_options(
            &dbname_owned,
            &icmp as *const InternalKeyComparator,
            &ipolicy as *const InternalFilterPolicy,
            options,
        );

        let owns_info_log = sanitized.info_log() != options.info_log();
        let owns_cache = sanitized.block_cache() != options.block_cache();

        let env_rc = sanitized
            .env()
            .clone()
            .or_else(|| options.env().clone())
            .unwrap_or_else(|| {
                warn!("Repairer::new: Options.env is None; using default_env()");
                default_env()
            });

        let env_box: Box<dyn Env> = Box::new(EnvWrapper::new(env_rc));

        // TableCache can be small since we expect each table to be opened once.
        let table_cache_ptr: *mut TableCache =
            Box::into_raw(Box::new(TableCache::new(&dbname_owned, sanitized.clone(), 10)));

        debug!(
            owns_info_log,
            owns_cache,
            table_cache = ?table_cache_ptr,
            "Repairer::new: constructed core state"
        );

        trace!(dbname = %dbname, "Repairer::new: done");
        Repairer {
            dbname:           dbname_owned,
            env:              env_box,
            icmp,
            ipolicy,
            options:          sanitized,
            owns_info_log,
            owns_cache,
            table_cache:      table_cache_ptr,
            edit:             VersionEdit::default(),
            manifests:        Vec::new(),
            table_numbers:    Vec::new(),
            logs:             Vec::new(),
            tables:           Vec::new(),
            next_file_number: 1,
        }
    }
}
