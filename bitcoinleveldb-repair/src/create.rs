// ---------------- [ File: bitcoinleveldb-repair/src/create.rs ]
crate::ix!();

impl Repairer {

    pub fn new(dbname: &str, options: &Options) -> Self {
        trace!(dbname = %dbname, "Repairer::new: start");

        let dbname_owned: String = dbname.to_string();

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
                error!(
                    dbname = %dbname_owned,
                    "Repairer::new: Options.env is None (sanitized and original); cannot proceed"
                );
                panic!("Repairer::new: Options.env is None");
            });

        let env_box: Box<dyn Env> = Box::new(EnvWrapper::new(env_rc.clone()));

        // TableCache can be small since we expect each table to be opened once.
        let table_cache_ptr: *mut TableCache =
            Box::into_raw(Box::new(TableCache::new(&dbname_owned, &sanitized, 10)));

        debug!(
            owns_info_log,
            owns_cache,
            table_cache = ?table_cache_ptr,
            "Repairer::new: constructed core state"
        );

        let repairer = Self::new_from_parts(
            dbname_owned,
            env_box,
            env_rc,
            icmp,
            ipolicy,
            sanitized,
            owns_info_log,
            owns_cache,
            table_cache_ptr,
        );

        trace!(dbname = %dbname, "Repairer::new: done");
        repairer
    }
}

#[cfg(test)]
mod repairer_construction_suite {
    use super::*;
    use crate::repairer_test_harness::*;

    #[traced_test]
    fn repairer_new_constructs_and_can_run_on_minimal_directory() {
        let db = EphemeralDbDir::new("repairer-new-run-minimal");
        let dbname: String = db.path_string();

        // Ensure non-empty for deterministic find_files behavior.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling Repairer::run");
        let st = repairer.run();

        info!(
            dbname = %dbname,
            ok = st.is_ok(),
            status = %st.to_string(),
            "Repairer::run returned"
        );

        assert!(st.is_ok(), "expected Repairer::run ok: {}", st.to_string());

        let manifest = descriptor_file_name(&dbname, 1);
        assert!(path_exists(&manifest), "expected manifest created: {}", manifest);
    }

    #[traced_test]
    fn repairer_new_is_drop_safe_in_tight_loop() {
        let options = Options::default();

        for i in 0..32u32 {
            let db = EphemeralDbDir::new(&format!("repairer-new-drop-loop-{}", i));
            let dbname = db.path_string();

            // Provide a sentinel file to avoid dependence on env-created artifacts.
            let sentinel = format!("{}/SENTINEL", dbname);
            touch_file(&sentinel);

            trace!(iter = i, dbname = %dbname, "constructing Repairer");
            let mut repairer = Repairer::new(&dbname, &options);

            // Exercise a small part of the pipeline and then drop.
            let st = repairer.find_files();
            debug!(
                iter = i,
                ok = st.is_ok(),
                status = %st.to_string(),
                "find_files in construction loop"
            );
            drop(repairer);
        }
    }
}

#[cfg(test)]
mod repairer_type_smoke_suite {
    use super::*;
    use crate::repairer_test_harness::*;

    #[traced_test]
    fn repairer_struct_is_usable_via_public_constructor_and_run() {
        let db = EphemeralDbDir::new("repairer-struct-smoke");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling run on Repairer");
        let st = repairer.run();

        info!(ok = st.is_ok(), status = %st.to_string(), "Repairer::run returned");
        assert!(st.is_ok(), "expected ok run: {}", st.to_string());
    }
}
