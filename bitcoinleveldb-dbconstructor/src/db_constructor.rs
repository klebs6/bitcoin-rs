// ---------------- [ File: bitcoinleveldb-dbconstructor/src/db_constructor.rs ]
crate::ix!();
 
///--------------------
pub struct ArcSliceComparatorAdapter {
    inner: std::sync::Arc<dyn SliceComparator>,
}

impl SliceComparator for ArcSliceComparatorAdapter {
    fn compare(&self, a: &Slice, b: &Slice) -> i32 {
        self.inner.compare(a, b)
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn find_shortest_separator(&self, start: &mut String, limit: &Slice) {
        self.inner.find_shortest_separator(start, limit)
    }

    fn find_short_successor(&self, key: &mut String) {
        self.inner.find_short_successor(key)
    }
}

///--------------------
pub struct DBConstructor {
    base:       Constructor,
    comparator: std::sync::Arc<dyn SliceComparator>,
    db:         Option<*mut dyn DB>,
}

impl Drop for DBConstructor {
    fn drop(&mut self) {
        tracing::trace!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::drop(begin)"
        );

        if let Some(db_ptr) = self.db.take() {
            tracing::debug!(
                target: "bitcoinleveldb_dbconstructor::db_constructor",
                db_ptr = ?db_ptr,
                "DBConstructor::drop: dropping owned DB instance"
            );

            unsafe {
                drop(Box::from_raw(db_ptr));
            }
        }

        tracing::trace!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::drop(end)"
        );
    }
}

impl DBConstructor {
    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
        tracing::trace!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::new(begin)"
        );

        let comparator: std::sync::Arc<dyn SliceComparator> = std::sync::Arc::from(cmp);

        let base = Constructor::new(Box::new(ArcSliceComparatorAdapter {
            inner: comparator.clone(),
        }));

        let mut out = Self {
            base,
            comparator,
            db: None,
        };

        out.newdb();

        tracing::trace!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            db_is_some = out.db.is_some(),
            "DBConstructor::new(end)"
        );

        out
    }

    pub fn finish_impl(&mut self, options: &Options, data: &KVMap) -> Status {
        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            data_len = data.len(),
            options_ptr = ?(options as *const Options),
            "DBConstructor::finish_impl(begin)"
        );

        if let Some(old_db_ptr) = self.db.take() {
            tracing::debug!(
                target: "bitcoinleveldb_dbconstructor::db_constructor",
                old_db_ptr = ?old_db_ptr,
                "DBConstructor::finish_impl: deleting existing DB instance"
            );

            unsafe {
                drop(Box::from_raw(old_db_ptr));
            }
        }

        self.newdb();

        let db_ptr: *mut dyn DB =
            self.db
                .expect("DBConstructor::finish_impl: NewDB must initialize db");

        for (k, v) in data.iter() {
            tracing::trace!(
                target: "bitcoinleveldb_dbconstructor::db_constructor",
                "DBConstructor::finish_impl: writing one KV via WriteBatch"
            );

            let key_slice = Slice::from(k);
            let val_slice = Slice::from(v);

            let mut batch = WriteBatch::new();
            batch.put(&key_slice, &val_slice);

            let write_options = WriteOptions::default();

            let st =
                unsafe { (&mut *db_ptr).write(&write_options, &mut batch as *mut WriteBatch) };

            tracing::debug!(
                target: "bitcoinleveldb_dbconstructor::db_constructor",
                write_ok = st.is_ok(),
                "DBConstructor::finish_impl: db.write(...) completed"
            );

            assert!(st.is_ok());
        }

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::finish_impl(end)"
        );

        Status::ok()
    }

    pub fn new_iterator(&self) -> *mut LevelDBIterator {
        let db_ptr: *mut dyn DB =
            self.db
                .expect("DBConstructor::new_iterator: db must be initialized");

        let read_options = ReadOptions::default();

        tracing::trace!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            db_ptr = ?db_ptr,
            "DBConstructor::new_iterator"
        );

        unsafe { (&mut *db_ptr).new_iterator(&read_options) }
    }

    pub fn db(&self) -> *mut dyn DB {
        self.db.expect("DBConstructor::db: db must be initialized")
    }

    pub fn newdb(&mut self) {
        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::newdb(begin)"
        );

        let mut path = std::env::temp_dir();
        path.push("table_testdb");
        let name = path.to_string_lossy().into_owned();

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            db_name = %name,
            "DBConstructor::newdb: using DB path"
        );

        let mut options = Options::default();
        options.set_comparator(self.comparator.clone());

        let mut status = destroy_db(&name, &options);

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            destroy_ok = status.is_ok(),
            "DBConstructor::newdb: destroy_db(...) completed"
        );

        assert!(status.is_ok());

        options.set_create_if_missing(true);
        options.set_error_if_exists(true);
        options.set_write_buffer_size(10000); // Something small to force merging

        let mut db_ptr: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let mut opener = DBImpl::new(&options, &name);

        status = opener.open(&options, &name, &mut db_ptr as *mut *mut dyn DB);

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            open_ok = status.is_ok(),
            db_is_null = db_ptr.is_null(),
            "DBConstructor::newdb: open(...) completed"
        );

        assert!(status.is_ok());
        assert!(!db_ptr.is_null());

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            db_ptr = ?db_ptr,
            "DBConstructor::newdb: DB opened"
        );

        self.db = Some(db_ptr);

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor",
            "DBConstructor::newdb(end)"
        );
    }
}

#[cfg(test)]
mod dbconstructor_exhaustive_suite {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn dbconstructor_global_serial_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn make_dbconstructor_with_default_comparator() -> DBConstructor {
        let cmp: Box<dyn SliceComparator> = Box::new(BytewiseComparatorImpl::default());
        DBConstructor::new(cmp)
    }

    fn kvmap_from_ascii_pairs(pairs: &[(&str, &str)]) -> KVMap {
        let mut m = KVMap::new();
        for (k, v) in pairs.iter() {
            m.insert((*k).into(), (*v).into());
        }
        m
    }

    fn slice_to_vec(s: &Slice) -> Vec<u8> {
        let p = *s.data();
        let n = *s.size();
        unsafe { core::slice::from_raw_parts(p, n) }.to_vec()
    }

    unsafe fn collect_entries_from_iterator(ptr: *mut LevelDBIterator) -> Vec<(Vec<u8>, Vec<u8>)> {
        assert!(!ptr.is_null());

        let it = &mut *ptr;

        it.seek_to_first();

        let mut out: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

        while it.valid() {
            let k = it.key();
            let v = it.value();

            out.push((slice_to_vec(&k), slice_to_vec(&v)));

            it.next();
        }

        let st = it.status();
        assert!(st.is_ok());

        drop(Box::from_raw(ptr));

        out
    }

    #[traced_test]
    fn dbconstructor_new_creates_empty_db_iterable() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_new_creates_empty_db_iterable(begin)"
        );

        let ctor = make_dbconstructor_with_default_comparator();

        let iter_ptr = ctor.new_iterator();

        let entries = unsafe { collect_entries_from_iterator(iter_ptr) };

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            entries_len = entries.len(),
            "iterator entries collected"
        );

        assert!(entries.is_empty());

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_new_creates_empty_db_iterable(end)"
        );
    }

    #[traced_test]
    fn dbconstructor_finish_impl_with_empty_map_produces_empty_db() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_with_empty_map_produces_empty_db(begin)"
        );

        let mut ctor = make_dbconstructor_with_default_comparator();

        let options = Options::default();
        let data = KVMap::new();

        let st = ctor.finish_impl(&options, &data);

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            finish_ok = st.is_ok(),
            "finish_impl returned"
        );

        assert!(st.is_ok());

        let entries = unsafe { collect_entries_from_iterator(ctor.new_iterator()) };

        assert!(entries.is_empty());

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_with_empty_map_produces_empty_db(end)"
        );
    }

    #[traced_test]
    fn dbconstructor_finish_impl_writes_single_kv_visible_via_iterator() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_writes_single_kv_visible_via_iterator(begin)"
        );

        let mut ctor = make_dbconstructor_with_default_comparator();

        let options = Options::default();
        let data = kvmap_from_ascii_pairs(&[("k1", "v1")]);

        let st = ctor.finish_impl(&options, &data);
        assert!(st.is_ok());

        let entries = unsafe { collect_entries_from_iterator(ctor.new_iterator()) };

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].0, b"k1".to_vec());
        assert_eq!(entries[0].1, b"v1".to_vec());

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_writes_single_kv_visible_via_iterator(end)"
        );
    }

    #[traced_test]
    fn dbconstructor_finish_impl_writes_multiple_kvs_and_iterates_all() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_writes_multiple_kvs_and_iterates_all(begin)"
        );

        let mut ctor = make_dbconstructor_with_default_comparator();

        let options = Options::default();
        let data = kvmap_from_ascii_pairs(&[
            ("alpha", "1"),
            ("beta", "2"),
            ("gamma", "3"),
            ("delta", "4"),
        ]);

        let st = ctor.finish_impl(&options, &data);
        assert!(st.is_ok());

        let entries = unsafe { collect_entries_from_iterator(ctor.new_iterator()) };

        tracing::debug!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            entries_len = entries.len(),
            "collected entries after finish_impl"
        );

        assert_eq!(entries.len(), 4);

        let got_keys: Vec<Vec<u8>> = entries.iter().map(|(k, _)| k.clone()).collect();

        assert!(got_keys.contains(&b"alpha".to_vec()));
        assert!(got_keys.contains(&b"beta".to_vec()));
        assert!(got_keys.contains(&b"gamma".to_vec()));
        assert!(got_keys.contains(&b"delta".to_vec()));

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_writes_multiple_kvs_and_iterates_all(end)"
        );
    }

    #[traced_test]
    fn dbconstructor_finish_impl_resets_db_between_calls() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_resets_db_between_calls(begin)"
        );

        let mut ctor = make_dbconstructor_with_default_comparator();

        let options = Options::default();

        let data1 = kvmap_from_ascii_pairs(&[("k_old", "v_old")]);
        let st1 = ctor.finish_impl(&options, &data1);
        assert!(st1.is_ok());

        let entries1 = unsafe { collect_entries_from_iterator(ctor.new_iterator()) };
        assert_eq!(entries1.len(), 1);
        assert_eq!(entries1[0].0, b"k_old".to_vec());
        assert_eq!(entries1[0].1, b"v_old".to_vec());

        let data2 = kvmap_from_ascii_pairs(&[("k_new", "v_new")]);
        let st2 = ctor.finish_impl(&options, &data2);
        assert!(st2.is_ok());

        let entries2 = unsafe { collect_entries_from_iterator(ctor.new_iterator()) };
        assert_eq!(entries2.len(), 1);
        assert_eq!(entries2[0].0, b"k_new".to_vec());
        assert_eq!(entries2[0].1, b"v_new".to_vec());

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_finish_impl_resets_db_between_calls(end)"
        );
    }

    #[traced_test]
    fn dbconstructor_new_iterator_returns_non_null_and_independent_instances() {
        let _guard = dbconstructor_global_serial_guard().lock().unwrap();

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_new_iterator_returns_non_null_and_independent_instances(begin)"
        );

        let mut ctor = make_dbconstructor_with_default_comparator();

        let options = Options::default();
        let data = kvmap_from_ascii_pairs(&[("a", "1"), ("b", "2"), ("c", "3")]);

        let st = ctor.finish_impl(&options, &data);
        assert!(st.is_ok());

        let it1 = ctor.new_iterator();
        let it2 = ctor.new_iterator();

        assert!(!it1.is_null());
        assert!(!it2.is_null());
        assert_ne!(it1, it2);

        unsafe {
            let i1 = &mut *it1;
            let i2 = &mut *it2;

            i1.seek_to_first();
            i2.seek_to_first();

            assert!(i1.valid());
            assert!(i2.valid());

            let k1_first = slice_to_vec(&i1.key());
            let k2_first = slice_to_vec(&i2.key());
            assert_eq!(k1_first, k2_first);

            i1.next();

            assert!(i2.valid());
            let k2_still_first = slice_to_vec(&i2.key());
            assert_eq!(k2_first, k2_still_first);

            let s1 = i1.status();
            let s2 = i2.status();
            assert!(s1.is_ok());
            assert!(s2.is_ok());

            drop(Box::from_raw(it1));
            drop(Box::from_raw(it2));
        }

        tracing::info!(
            target: "bitcoinleveldb_dbconstructor::db_constructor::tests",
            "dbconstructor_new_iterator_returns_non_null_and_independent_instances(end)"
        );
    }
}
