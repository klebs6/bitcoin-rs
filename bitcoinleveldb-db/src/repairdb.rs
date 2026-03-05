// ---------------- [ File: bitcoinleveldb-db/src/repairdb.rs ]
crate::ix!();

/**
  | If a DB cannot be opened, you may attempt to
  | call this method to resurrect as much of the
  | contents of the database as possible.
  |
  | Some data may be lost, so be careful when
  | calling this function on a database that
  | contains important information.
  */
pub fn repairdb(dbname: &String, options: &Options) -> crate::Status {
    trace!(
        target: "bitcoinleveldb_db::db",
        dbname = %dbname,
        "RepairDB entry"
    );

    // NOTE: A full line-for-line port of LevelDB's RepairDB requires the translated
    // repairer implementation (repair.cc and its dependencies). If/when that module
    // exists in this workspace, this should forward to it in the same way DestroyDB does.
    //
    // For now, surface an explicit NotSupported status rather than panicking or silently
    // returning OK.
    let msg = Slice::from_str("RepairDB not supported (repair implementation unavailable)");
    let result = crate::Status::not_supported(&msg, None);

    warn!(
        target: "bitcoinleveldb_db::db",
        dbname = %dbname,
        status = %result.to_string(),
        has_env = options.env().is_some(),
        "RepairDB not supported"
    );

    result
}

#[cfg(test)]
#[disable]
mod bitcoinleveldb_db__repairdb_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__repairdb_rs__returns_not_supported_status() {
        let env = PosixEnv::shared();
        let opts: Options = Options::with_env(env);
        let name: String = String::from("bitcoinleveldb_db__repairdb_rs__in_memory_name");

        let st: crate::Status = repairdb(&name, &opts);

        assert!(!st.is_ok());

        let not_supported: bool = st.is_not_supported();
        assert!(not_supported);
    }
}
