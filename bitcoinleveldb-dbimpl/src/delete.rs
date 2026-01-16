// ---------------- [ File: bitcoinleveldb-dbimpl/src/delete.rs ]
crate::ix!();

impl DBDelete for DBImpl { }

#[cfg(test)]
mod db_delete_interface_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBDelete;

    fn assert_dbimpl_implements_db_delete() {
        fn _assert<T: DBDelete>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_delete_trait_object(_db: &mut dyn DBDelete) {}

    #[traced_test]
    fn db_delete_contract_dbimpl_implements_trait() {
        tracing::info!("Asserting DBImpl implements DBDelete");
        assert_dbimpl_implements_db_delete();
    }

    #[traced_test]
    fn db_delete_contract_trait_is_object_safe() {
        tracing::info!("Asserting DBDelete can be used as a trait object");
        let _accept = compile_only_accepts_db_delete_trait_object as fn(&mut dyn DBDelete);
        let _ = _accept;
    }
}
