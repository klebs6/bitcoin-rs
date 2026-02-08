// ---------------- [ File: bitcoinleveldb-modeldb/src/model_db_write_batch_handler.rs ]
crate::ix!();

pub struct ModelDBWriteBatchHandler {
    pub(crate) map: *mut KVMap,
}

impl WriteBatchHandler for ModelDBWriteBatchHandler {}

impl WriteBatchPut for ModelDBWriteBatchHandler {
    fn put(&mut self, key_: &Slice, value: &Slice) {
        tracing::trace!(
            key_len = key_.as_bytes().len(),
            value_len = value.as_bytes().len(),
            "ModelDBWriteBatchHandler::put"
        );

        unsafe {
            (*self.map).insert(key_.to_string(), value.to_string());
        }
    }
}

impl WriteBatchDelete for ModelDBWriteBatchHandler {
    fn delete(&mut self, key_: &Slice) {
        tracing::trace!(
            key_len = key_.as_bytes().len(),
            "ModelDBWriteBatchHandler::delete"
        );

        unsafe {
            (*self.map).remove(&key_.to_string());
        }
    }
}

#[cfg(test)]
mod model_db_write_batch_handler_trait_semantics_suite {
    use super::*;

    #[traced_test]
    fn model_db_write_batch_handler_put_overwrites_value() {
        tracing::info!("starting model_db_write_batch_handler_put_overwrites_value");

        let mut map: KVMap = KVMap::default();
        let mut handler: ModelDBWriteBatchHandler = ModelDBWriteBatchHandler {
            map: &mut map as *mut KVMap,
        };

        <ModelDBWriteBatchHandler as WriteBatchPut>::put(&mut handler, &Slice::from("k"), &Slice::from("v1"));
        assert_eq!(map.get("k").cloned(), Some(String::from("v1")));

        <ModelDBWriteBatchHandler as WriteBatchPut>::put(&mut handler, &Slice::from("k"), &Slice::from("v2"));
        assert_eq!(map.get("k").cloned(), Some(String::from("v2")));
    }

    #[traced_test]
    fn model_db_write_batch_handler_delete_removes_key_and_is_idempotent() {
        tracing::info!("starting model_db_write_batch_handler_delete_removes_key_and_is_idempotent");

        let mut map: KVMap = KVMap::default();
        map.insert(String::from("k"), String::from("v"));

        let mut handler: ModelDBWriteBatchHandler = ModelDBWriteBatchHandler {
            map: &mut map as *mut KVMap,
        };

        <ModelDBWriteBatchHandler as WriteBatchDelete>::delete(&mut handler, &Slice::from("k"));
        assert!(map.get("k").is_none());

        <ModelDBWriteBatchHandler as WriteBatchDelete>::delete(&mut handler, &Slice::from("k"));
        assert!(map.get("k").is_none());
    }

    #[traced_test]
    fn model_db_write_applies_write_batch_operations_in_order() {
        tracing::info!("starting model_db_write_applies_write_batch_operations_in_order");

        let options: Options = Options::default();
        let mut db: crate::ModelDB = crate::ModelDB::new(&options);

        let wo: WriteOptions = WriteOptions::default();
        let ro: ReadOptions = ReadOptions::default();

        let mut batch: WriteBatch = WriteBatch::new();
        batch.put(&Slice::from("k1"), &Slice::from("v1"));
        batch.put(&Slice::from("k2"), &Slice::from("v2"));
        batch.delete(&Slice::from("k1"));
        batch.put(&Slice::from("k1"), &Slice::from("v3"));

        let st: crate::Status = DBWrite::write(&mut db, &wo, &mut batch as *mut WriteBatch);
        assert!(st.is_ok(), "write returned non-ok: {}", st.to_string());

        let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(&mut db, &ro);
        assert!(!it_ptr.is_null());

        let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };
        <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

        let mut out: Vec<(String, String)> = Vec::new();
        while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
            let k: String = <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
            let v: String = <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
            out.push((k, v));
            <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
        }

        assert_eq!(
            out,
            vec![
                (String::from("k1"), String::from("v3")),
                (String::from("k2"), String::from("v2")),
            ]
        );
    }
}
