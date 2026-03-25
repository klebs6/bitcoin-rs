// ---------------- [ File: bitcoinleveldb-modeldb/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

x!{modeldb}
x!{iter}
x!{model_db_write_batch_handler}
x!{base}
x!{model_snapshot}
x!{snapshot_vtable}

#[cfg(test)]
mod modeldb_crate_surface_sanity_suite {
    use super::*;

    #[traced_test]
    fn modeldb_crate_exports_allow_basic_round_trip_via_db_trait_object() {
        tracing::info!("starting modeldb_crate_exports_allow_basic_round_trip_via_db_trait_object");

        let options: Options = Options::default();
        let mut db: crate::ModelDB = crate::ModelDB::new(&options);

        let db_obj: &mut dyn DB = &mut db;

        let wo: WriteOptions = WriteOptions::default();
        let ro: ReadOptions = ReadOptions::default();

        let st: crate::Status =
            DBPut::put(db_obj, &wo, &Slice::from("k"), &Slice::from("v"));
        assert!(st.is_ok(), "put failed: {}", st.to_string());

        let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(db_obj, &ro);
        assert!(!it_ptr.is_null());

        let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };
        <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

        assert!(<LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box));
        assert_eq!(<LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string(), "k".to_string());
        assert_eq!(<LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string(), "v".to_string());

        <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
        assert!(!<LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box));
    }
}
#[cfg(test)]
mod modeldb_imports_visibility_smoke_suite {
    use super::*;

    #[traced_test]
    fn modeldb_imports_provide_expected_types_and_traits() {
        tracing::info!("starting modeldb_imports_provide_expected_types_and_traits");

        let _options: Options = Options::default();
        let mut _ro: ReadOptions = ReadOptions::default();
        let _wo: WriteOptions = WriteOptions::default();

        let _s: Slice = Slice::from("x");
        let _st_ok: Status = Status::ok();

        let mut batch: WriteBatch = WriteBatch::new();
        batch.put(&Slice::from("k"), &Slice::from("v"));
        batch.delete(&Slice::from("k"));

        _ro.set_snapshot(None);

        let mut db: crate::ModelDB = crate::ModelDB::new(&_options);
        let _ = DBGetProperty::get_property(&mut db, "property", core::ptr::null_mut());

        tracing::debug!("imports smoke complete");
    }
}
