// ---------------- [ File: bitcoinleveldb-repair/src/repair.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/repair.cc]

/// We recover the contents of the descriptor from the other files we find.
///
///```ignore
/// (1) Any log files are first converted to tables
///
/// (2) We scan every table to compute
///     (a) smallest/largest for the table
///     (b) largest sequence number in the table
///
/// (3) We generate descriptor contents: - log number is set to zero
///
///      - next-file-number is set to 1 + largest file number we found
///      - last-sequence-number is set to largest sequence# found across all tables (see 2c)
///      - compaction pointers are cleared
///      - every table file is added at level 0
///
/// Possible optimization 1:
///
///   (a) Compute total size and use to pick appropriate max-level M
///   (b) Sort tables by largest sequence# in the table
///   (c) For each table: if it overlaps earlier table, place in level-0, else place in level-M.
///
/// Possible optimization 2:
///
///   Store per-table metadata (smallest, largest, largest-seq#, ...) in the table's meta section
///   to speed up ScanTable.
///
///```
///
#[derive(Getters, MutGetters)]
#[getset(get="pub", get_mut="pub(crate)")]
pub struct Repairer {
    dbname:           String,
    env:              Box<dyn Env>,
    env_rc:           std::rc::Rc<std::cell::RefCell<dyn Env>>,
    icmp:             InternalKeyComparator,
    ipolicy:          InternalFilterPolicy,
    options:          Options,
    owns_info_log:    bool,
    owns_cache:       bool,
    table_cache:      *mut TableCache,
    edit:             VersionEdit,
    manifests:        Vec<String>,
    table_numbers:    Vec<u64>,
    logs:             Vec<u64>,
    tables:           Vec<RepairerTableInfo>,
    next_file_number: u64,
}

impl Repairer {
    pub(crate) fn new_from_parts(
        dbname: String,
        env: Box<dyn Env>,
        env_rc: std::rc::Rc<std::cell::RefCell<dyn Env>>,
        icmp: InternalKeyComparator,
        ipolicy: InternalFilterPolicy,
        options: Options,
        owns_info_log: bool,
        owns_cache: bool,
        table_cache: *mut TableCache,
    ) -> Self {
        Repairer {
            dbname,
            env,
            env_rc,
            icmp,
            ipolicy,
            options,
            owns_info_log,
            owns_cache,
            table_cache,
            edit: VersionEdit::default(),
            manifests: Vec::new(),
            table_numbers: Vec::new(),
            logs: Vec::new(),
            tables: Vec::new(),
            next_file_number: 1,
        }
    }
}


#[cfg(test)]
mod repairer_construction_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn repairer_new_constructs_and_can_run_on_minimal_directory() {
        let db = EphemeralDbDir::new("repairer-new-run-minimal");
        let dbname: String = db.path_string();

        // Ensure non-empty for deterministic find_files behavior.
        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
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
        let env = PosixEnv::shared();
        let options = Options::with_env(env);

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
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn repairer_struct_is_usable_via_public_constructor_and_run() {
        let db = EphemeralDbDir::new("repairer-struct-smoke");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let env = PosixEnv::shared();
        let options = Options::with_env(env);
        let mut repairer = Repairer::new(&dbname, &options);

        trace!(dbname = %dbname, "calling run on Repairer");
        let st = repairer.run();

        info!(ok = st.is_ok(), status = %st.to_string(), "Repairer::run returned");
        assert!(st.is_ok(), "expected ok run: {}", st.to_string());
    }

    #[traced_test]
    fn options_with_env_sets_env_slot_and_preserves_pointer_identity() {
        trace!("options_defaults_and_accessors_suite: start");

        let env_rc = PosixEnv::shared();
        let opts = Options::with_env(env_rc.clone());

        let got = opts.env().as_ref().expect("env must be Some");
        let got_ptr = Rc::as_ptr(got);
        let exp_ptr = Rc::as_ptr(&env_rc);

        info!(
            got = %format!("{:p}", got_ptr),
            exp = %format!("{:p}", exp_ptr),
            "Options::with_env pointer identity"
        );

        assert!(core::ptr::eq(got_ptr, exp_ptr));

        trace!("options_defaults_and_accessors_suite: done");
    }
}
