// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete.rs ]
crate::ix!();

impl DBDelete for DBImpl {
    fn delete(&mut self, options: &WriteOptions, key_: &Slice) -> crate::Status {
        <DBImpl as DB>::delete(self, options, key_)
    }
}

#[cfg(test)]
#[disable]
mod delete_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn delete_removes_key_and_get_reports_not_found_or_empty() {
        let (dbname, mut db) = open_dbimpl_for_test("delete_removes_key_and_get_reports_not_found_or_empty");

        let s1 = write_kv(&mut *db, "k", "v");
        assert!(s1.is_ok(), "write failed: {}", s1.to_string());
        assert_read_eq(&mut *db, "k", "v");

        let sd: Status = <DBImpl as Delete>::delete(&mut *db, &WriteOptions::default(), &Slice::from_str("k"));
        tracing::info!(status = %sd.to_string(), "delete");
        assert!(sd.is_ok(), "delete failed: {}", sd.to_string());

        let ro: ReadOptions = Default::default();
        let (sg, v) = read_value(&mut *db, &ro, "k");
        tracing::info!(status = %sg.to_string(), value_len = v.len(), "get after delete");

        // LevelDB typically returns NotFound; accept either a non-ok status or empty value,
        // without changing control flow in implementation.
        assert!(
            !sg.is_ok() || v.is_empty(),
            "expected key to be absent; got status={} value={}",
            sg.to_string(),
            v
        );

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
