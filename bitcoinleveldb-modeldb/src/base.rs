// ---------------- [ File: bitcoinleveldb-modeldb/src/base.rs ]
crate::ix!();

#[derive(Copy, Clone, Default)]
pub struct ModelDBBase;

impl ModelDBBase {
    pub fn put(
        &self,
        db:    &mut dyn DBWrite,
        opt:   &WriteOptions,
        key_:  &Slice,
        value: &Slice,
    ) -> crate::Status {
        tracing::trace!(
            key_len = key_.as_bytes().len(),
            value_len = value.as_bytes().len(),
            "ModelDBBase::put (DB::Put)"
        );

        let mut batch: WriteBatch = WriteBatch::new();
        batch.put(key_, value);

        db.write(opt, &mut batch as *mut WriteBatch)
    }

    pub fn delete(&self, db: &mut dyn DBWrite, opt: &WriteOptions, key_: &Slice) -> crate::Status {
        tracing::trace!(
            key_len = key_.as_bytes().len(),
            "ModelDBBase::delete (DB::Delete)"
        );

        let mut batch: WriteBatch = WriteBatch::new();
        batch.delete(key_);

        db.write(opt, &mut batch as *mut WriteBatch)
    }
}

#[cfg(test)]
mod model_db_base_delegation_suite {
    use super::*;

    fn collect_all_kvs_from_db_via_iterator(
        db: &mut dyn DBNewIterator,
        ro: &ReadOptions,
    ) -> Vec<(String, String)> {
        tracing::debug!("collect_all_kvs_from_db_via_iterator (base suite)");

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
    fn model_db_base_delegates_put_and_delete_via_db_traits() {
        tracing::info!("starting model_db_base_delegates_put_and_delete_via_db_traits");

        let options: Options = Options::default();
        let mut db: crate::ModelDB = crate::ModelDB::new(&options);

        let wo: WriteOptions = WriteOptions::default();
        let ro: ReadOptions = ReadOptions::default();

        let st1: crate::Status =
            DBPut::put(&mut db, &wo, &Slice::from("k1"), &Slice::from("v1"));
        assert!(st1.is_ok(), "put k1 failed: {}", st1.to_string());

        let st2: crate::Status =
            DBPut::put(&mut db, &wo, &Slice::from("k2"), &Slice::from("v2"));
        assert!(st2.is_ok(), "put k2 failed: {}", st2.to_string());

        let kvs1: Vec<(String, String)> = collect_all_kvs_from_db_via_iterator(&mut db, &ro);
        assert_eq!(
            kvs1,
            vec![(String::from("k1"), String::from("v1")), (String::from("k2"), String::from("v2"))]
        );

        let st3: crate::Status =
            DBPut::put(&mut db, &wo, &Slice::from("k1"), &Slice::from("v1b"));
        assert!(st3.is_ok(), "overwrite k1 failed: {}", st3.to_string());

        let kvs2: Vec<(String, String)> = collect_all_kvs_from_db_via_iterator(&mut db, &ro);
        assert_eq!(
            kvs2,
            vec![(String::from("k1"), String::from("v1b")), (String::from("k2"), String::from("v2"))]
        );

        let st4: crate::Status = DBDelete::delete(&mut db, &wo, &Slice::from("k1"));
        assert!(st4.is_ok(), "delete k1 failed: {}", st4.to_string());

        let kvs3: Vec<(String, String)> = collect_all_kvs_from_db_via_iterator(&mut db, &ro);
        assert_eq!(kvs3, vec![(String::from("k2"), String::from("v2"))]);

        let st5: crate::Status = DBDelete::delete(&mut db, &wo, &Slice::from("missing"));
        assert!(st5.is_ok(), "delete missing failed: {}", st5.to_string());

        let kvs4: Vec<(String, String)> = collect_all_kvs_from_db_via_iterator(&mut db, &ro);
        assert_eq!(kvs4, vec![(String::from("k2"), String::from("v2"))]);
    }
}
