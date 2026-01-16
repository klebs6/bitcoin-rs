// ---------------- [ File: bitcoinleveldb-dbimpl/src/put.rs ]
crate::ix!();

impl DBPut for DBImpl { }

#[cfg(test)]
mod db_put_interface_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBPut;

    fn assert_dbimpl_implements_db_put() {
        fn _assert<T: DBPut>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_put_trait_object(_db: &mut dyn DBPut) {}

    #[traced_test]
    fn db_put_contract_dbimpl_implements_trait() {
        tracing::info!("Asserting DBImpl implements DBPut");
        assert_dbimpl_implements_db_put();
    }

    #[traced_test]
    fn db_put_contract_trait_is_object_safe() {
        tracing::info!("Asserting DBPut can be used as a trait object");
        let _accept = compile_only_accepts_db_put_trait_object as fn(&mut dyn DBPut);
        let _ = _accept;
    }
}
