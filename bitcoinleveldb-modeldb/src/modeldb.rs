// ---------------- [ File: bitcoinleveldb-modeldb/src/modeldb.rs ]
crate::ix!();

pub struct ModelDB {
    base:    ModelDBBase,
    options: Options,
    map:     KVMap,
}

impl ModelDB {
    pub fn new(options: &Options) -> Self {
        tracing::debug!("ModelDB::new");

        Self {
            base: ModelDBBase::default(),
            options: options.clone(),
            map: KVMap::default(),
        }
    }
}

impl DB for ModelDB {}

impl DBOpen for ModelDB {
    fn open(
        &mut self,
        options: &Options,
        dbname:  &String,
        dbptr:   *mut *mut dyn DB,
    ) -> Status {
        tracing::debug!(
            dbname = dbname.as_str(),
            dbptr_is_null = dbptr.is_null(),
            "ModelDB::open"
        );

        let _ = dbname;

        if dbptr.is_null() {
            tracing::error!("ModelDB::open called with null dbptr");

            let msg: Slice = Slice::from("dbptr");
            return Status::invalid_argument(&msg, None);
        }

        let db: Box<dyn DB> = Box::new(ModelDB::new(options));
        let raw: *mut dyn DB = Box::into_raw(db);

        unsafe {
            *dbptr = raw;
        }

        Status::ok()
    }
}

impl DBPut for ModelDB {
    fn put(&mut self, opt: &WriteOptions, key_: &Slice, value: &Slice) -> crate::Status {
        tracing::trace!(
            key_len = key_.as_bytes().len(),
            value_len = value.as_bytes().len(),
            "ModelDB::put"
        );

        let base: ModelDBBase = self.base;
        base.put(self, opt, key_, value)
    }
}

impl DBDelete for ModelDB {
    fn delete(&mut self, opt: &WriteOptions, key_: &Slice) -> crate::Status {
        tracing::trace!(key_len = key_.as_bytes().len(), "ModelDB::delete");

        let base: ModelDBBase = self.base;
        base.delete(self, opt, key_)
    }
}

impl DBGet for ModelDB {
    fn get(
        &mut self,
        options: &ReadOptions,
        key_:    &Slice,
        value:   *mut String,
    ) -> crate::Status {
        let _ = options;
        let _ = value;

        tracing::error!(
            key_len = key_.as_bytes().len(),
            "ModelDB::get (not implemented)"
        );

        debug_assert!(false);
        crate::Status::not_found(key_, None)
    }
}

impl DBNewIterator for ModelDB {
    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        let has_snapshot: bool = options.snapshot().is_some();

        tracing::debug!(
            has_snapshot = has_snapshot,
            map_len = self.map.len(),
            "ModelDB::new_iterator"
        );

        if options.snapshot().is_none() {
            let saved: Box<KVMap> = Box::new(self.map.clone());
            let saved_ptr: *const KVMap = Box::into_raw(saved) as *const KVMap;

            let miter: ModelIter<'static> = ModelIter::<'static>::new(saved_ptr, true);
            let iter: LevelDBIterator = LevelDBIterator::new(Some(Box::new(miter)));

            Box::into_raw(Box::new(iter))
        } else {
            let snapshot_arc: &std::sync::Arc<dyn Snapshot> = options
                .snapshot()
                .as_ref()
                .unwrap();

            let snapshot_ptr: *const dyn Snapshot = std::sync::Arc::as_ptr(snapshot_arc);
            let model_snapshot_ptr: *const ModelSnapshot = snapshot_ptr as *const ModelSnapshot;

            let snapshot_state_ptr: *const KVMap =
                unsafe { (*model_snapshot_ptr).map_ref() as *const KVMap };

            let miter: ModelIter<'static> = ModelIter::<'static>::new(snapshot_state_ptr, false);
            let iter: LevelDBIterator = LevelDBIterator::new(Some(Box::new(miter)));

            Box::into_raw(Box::new(iter))
        }
    }
}

impl DBGetSnapshot for ModelDB {
    fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
        tracing::debug!("ModelDB::get_snapshot");

        Box::new(ModelSnapshot::new_from_map(&self.map))
    }
}

impl DBReleaseSnapshot for ModelDB {
    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
        tracing::debug!("ModelDB::release_snapshot");

        drop(snapshot);
    }
}

impl DBWrite for ModelDB {
    fn write(&mut self, options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status {
        let _ = options;

        tracing::debug!(
            updates_ptr = ?updates,
            map_len_before = self.map.len(),
            "ModelDB::write"
        );

        let mut handler = ModelDBWriteBatchHandler {
            map: &mut self.map as *mut KVMap,
        };

        let st: crate::Status = unsafe {
            (*updates).iterate(&mut handler as *mut dyn WriteBatchHandler)
        };

        tracing::debug!(
            ok = st.is_ok(),
            map_len_after = self.map.len(),
            "ModelDB::write done"
        );

        st
    }
}

impl DBGetProperty for ModelDB {
    fn get_property(&mut self, property: &str, value: *mut String) -> bool {
        let _ = value;

        tracing::trace!(property = property, "ModelDB::get_property");
        false
    }
}

impl DBGetApproximateSizes for ModelDB {
    fn get_approximate_sizes(
        &mut self,
        range: *const bitcoinleveldb_slice::Range,
        n:     i32,
        sizes: *mut u64,
    ) {
        let _ = range;

        tracing::trace!(n = n, sizes_ptr = ?sizes, "ModelDB::get_approximate_sizes");

        for i in 0..n {
            unsafe {
                *sizes.offset(i as isize) = 0;
            }
        }
    }
}

impl DBCompactRange for ModelDB {
    fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
        let _ = begin;
        let _ = end;

        tracing::trace!("ModelDB::compact_range");
    }
}

#[cfg(test)]
mod model_db_interface_behavior_suite {
    use super::*;

    fn collect_all_kvs_from_db_via_iterator(
        db: &mut dyn DBNewIterator,
        ro: &ReadOptions,
    ) -> Vec<(String, String)> {
        tracing::debug!("collect_all_kvs_from_db_via_iterator");

        let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(db, ro);
        assert!(!it_ptr.is_null(), "new_iterator returned null");

        let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };

        <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

        let mut out: Vec<(String, String)> = Vec::new();

        while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
            let k: String = <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
            let v: String = <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
            out.push((k, v));

            <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
        }

        let st: crate::Status = <LevelDBIterator as LevelDBIteratorStatus>::status(&*it_box);
        assert!(st.is_ok(), "iterator status not ok: {}", st.to_string());

        out
    }

    #[traced_test]
    fn model_db_open_sets_dbptr_and_produces_usable_db() {
        tracing::info!("starting model_db_open_sets_dbptr_and_produces_usable_db");

        let options: Options = Options::default();
        let mut factory: ModelDB = ModelDB::new(&options);

        let dbname: String = "modeldb-open-test".to_string();
        let mut out: core::mem::MaybeUninit<*mut dyn DB> = core::mem::MaybeUninit::uninit();

        let st: Status =
            DBOpen::open(&mut factory, &options, &dbname, out.as_mut_ptr());

        assert!(st.is_ok(), "open returned non-ok: {}", st.to_string());

        let dbptr: *mut dyn DB = unsafe { out.assume_init() };
        let mut db_box: Box<dyn DB> = unsafe { Box::from_raw(dbptr) };

        let wo: WriteOptions = WriteOptions::default();
        let ro: ReadOptions = ReadOptions::default();

        let put_st: crate::Status =
            DBPut::put(&mut *db_box, &wo, &Slice::from("k"), &Slice::from("v"));
        assert!(put_st.is_ok(), "put returned non-ok: {}", put_st.to_string());

        let kvs: Vec<(String, String)> =
            collect_all_kvs_from_db_via_iterator(&mut *db_box, &ro);

        assert_eq!(kvs, vec![(String::from("k"), String::from("v"))]);
    }

    #[traced_test]
    fn model_db_open_with_null_dbptr_returns_invalid_argument() {
        tracing::info!("starting model_db_open_with_null_dbptr_returns_invalid_argument");

        let options: Options = Options::default();
        let mut factory: ModelDB = ModelDB::new(&options);

        let dbname: String = "modeldb-open-null-dst".to_string();

        let st: Status = DBOpen::open(
            &mut factory,
            &options,
            &dbname,
            core::ptr::null_mut(),
        );

        assert!(
            st.is_invalid_argument(),
            "expected invalid argument, got: {}",
            st.to_string()
        );
    }

    #[traced_test]
    fn model_db_put_delete_iterator_snapshot_isolation_without_snapshot_option() {
        tracing::info!("starting model_db_put_delete_iterator_snapshot_isolation_without_snapshot_option");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        let wo: WriteOptions = WriteOptions::default();
        let ro: ReadOptions = ReadOptions::default();

        assert!(DBPut::put(&mut db, &wo, &Slice::from("a"), &Slice::from("va")).is_ok());
        assert!(DBPut::put(&mut db, &wo, &Slice::from("b"), &Slice::from("vb")).is_ok());

        let it_ptr_before: *mut LevelDBIterator = DBNewIterator::new_iterator(&mut db, &ro);
        assert!(!it_ptr_before.is_null(), "new_iterator returned null");

        assert!(DBPut::put(&mut db, &wo, &Slice::from("c"), &Slice::from("vc")).is_ok());
        assert!(DBDelete::delete(&mut db, &wo, &Slice::from("a")).is_ok());

        let kvs_before: Vec<(String, String)> = {
            let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr_before) };
            <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

            let mut out: Vec<(String, String)> = Vec::new();
            while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
                let k: String = <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
                let v: String = <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
                out.push((k, v));
                <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
            }

            out
        };

        assert_eq!(
            kvs_before,
            vec![(String::from("a"), String::from("va")), (String::from("b"), String::from("vb"))]
        );

        let kvs_after: Vec<(String, String)> = collect_all_kvs_from_db_via_iterator(&mut db, &ro);
        assert_eq!(
            kvs_after,
            vec![(String::from("b"), String::from("vb")), (String::from("c"), String::from("vc"))]
        );
    }

    #[traced_test]
    fn model_db_snapshot_option_iterator_sees_snapshot_state_only() {
        tracing::info!("starting model_db_snapshot_option_iterator_sees_snapshot_state_only");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        let wo: WriteOptions = WriteOptions::default();

        assert!(DBPut::put(&mut db, &wo, &Slice::from("a"), &Slice::from("va")).is_ok());
        assert!(DBPut::put(&mut db, &wo, &Slice::from("b"), &Slice::from("vb")).is_ok());

        let snap_box: Box<dyn Snapshot> = DBGetSnapshot::get_snapshot(&mut db);
        let snap_arc: std::sync::Arc<dyn Snapshot> = std::sync::Arc::from(snap_box);

        assert!(DBPut::put(&mut db, &wo, &Slice::from("c"), &Slice::from("vc")).is_ok());
        assert!(DBDelete::delete(&mut db, &wo, &Slice::from("a")).is_ok());

        let mut ro_snap: ReadOptions = ReadOptions::default();
        ro_snap.set_snapshot(Some(snap_arc.clone()));

        let kvs_snap: Vec<(String, String)> =
            collect_all_kvs_from_db_via_iterator(&mut db, &ro_snap);

        assert_eq!(
            kvs_snap,
            vec![(String::from("a"), String::from("va")), (String::from("b"), String::from("vb"))]
        );

        let ro_live: ReadOptions = ReadOptions::default();
        let kvs_live: Vec<(String, String)> =
            collect_all_kvs_from_db_via_iterator(&mut db, &ro_live);

        assert_eq!(
            kvs_live,
            vec![(String::from("b"), String::from("vb")), (String::from("c"), String::from("vc"))]
        );
    }

    #[traced_test]
    fn model_db_get_is_not_implemented_panics_or_returns_not_found() {
        tracing::info!("starting model_db_get_is_not_implemented_panics_or_returns_not_found");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        let ro: ReadOptions = ReadOptions::default();
        let key: Slice = Slice::from("k");
        let mut value: String = String::new();

        let res: Result<crate::Status, Box<dyn core::any::Any + Send>> =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                DBGet::get(&mut db, &ro, &key, &mut value as *mut String)
            }));

        match res {
            Ok(st) => {
                assert!(
                    st.is_not_found(),
                    "expected NotFound when not panicking; got: {}",
                    st.to_string()
                );
            }
            Err(_) => {
                tracing::debug!("DBGet::get panicked as expected in debug configurations");
            }
        }
    }

    #[traced_test]
    fn model_db_get_property_returns_false_and_preserves_value() {
        tracing::info!("starting model_db_get_property_returns_false_and_preserves_value");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        let mut value: String = "sentinel".to_string();
        let ok: bool = DBGetProperty::get_property(
            &mut db,
            "some.property",
            &mut value as *mut String,
        );

        assert!(!ok);
        assert_eq!(value, "sentinel".to_string());
    }

    #[traced_test]
    fn model_db_get_approximate_sizes_sets_only_first_n_to_zero() {
        tracing::info!("starting model_db_get_approximate_sizes_sets_only_first_n_to_zero");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        let mut sizes: [u64; 5] = [11, 22, 33, 44, 55];

        DBGetApproximateSizes::get_approximate_sizes(
            &mut db,
            core::ptr::null(),
            3,
            sizes.as_mut_ptr(),
        );

        assert_eq!(sizes[0], 0);
        assert_eq!(sizes[1], 0);
        assert_eq!(sizes[2], 0);
        assert_eq!(sizes[3], 44);
        assert_eq!(sizes[4], 55);
    }

    #[traced_test]
    fn model_db_compact_range_is_noop_and_safe_with_null_bounds() {
        tracing::info!("starting model_db_compact_range_is_noop_and_safe_with_null_bounds");

        let options: Options = Options::default();
        let mut db: ModelDB = ModelDB::new(&options);

        DBCompactRange::compact_range(&mut db, core::ptr::null(), core::ptr::null());
    }
}
