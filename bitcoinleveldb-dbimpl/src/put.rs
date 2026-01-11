// ---------------- [ File: bitcoinleveldb-dbimpl/src/put.rs ]
crate::ix!();

impl DBPut for DBImpl {
    /// Convenience methods
    fn put(&mut self, o: &WriteOptions, key_: &Slice, val: &Slice) -> crate::Status {
        <DBImpl as DB>::put(self, o, key_, val)
    }
}

#[cfg(test)]
#[disable]
mod put_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn put_roundtrips_via_get_and_overwrites_value() {
        let (dbname, mut db) = open_dbimpl_for_test("put_roundtrips_via_get_and_overwrites_value");

        let s1: Status = <DBImpl as DBPut>::put(
            &mut *db,
            &WriteOptions::default(),
            &Slice::from_str("k"),
            &Slice::from_str("v1"),
        );
        tracing::info!(status = %s1.to_string(), "put v1");
        assert!(s1.is_ok(), "put v1 failed: {}", s1.to_string());
        assert_read_eq(&mut *db, "k", "v1");

        let s2: Status = <DBImpl as DBPut>::put(
            &mut *db,
            &WriteOptions::default(),
            &Slice::from_str("k"),
            &Slice::from_str("v2"),
        );
        tracing::info!(status = %s2.to_string(), "put v2");
        assert!(s2.is_ok(), "put v2 failed: {}", s2.to_string());
        assert_read_eq(&mut *db, "k", "v2");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
