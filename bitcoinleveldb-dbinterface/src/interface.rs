// ---------------- [ File: bitcoinleveldb-dbinterface/src/interface.rs ]
crate::ix!();

/// A DB is a persistent ordered map from keys to values.
/// 
/// A DB is safe for concurrent access from multiple threads without any external synchronization.
///
pub trait DB:
    DBPut
    + DBOpen
    + DBDelete
    + DBWrite
    + DBGet
    + DBNewIterator
    + DBGetSnapshot
    + DBReleaseSnapshot
    + DBGetProperty
    + DBGetApproximateSizes
    + DBCompactRange 
{ }

#[cfg(test)]
mod db_trait_composition_suite {
    use super::*;
    use core::ptr;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tracing::{info, trace};

    struct DummySnapshot;

    impl Snapshot for DummySnapshot {}

    struct MinimalDbForComposition {
        writes: Arc<AtomicUsize>,
    }

    impl MinimalDbForComposition {
        fn new(counter: Arc<AtomicUsize>) -> Self {
            Self { writes: counter }
        }
    }

    impl DBOpen for MinimalDbForComposition {
        fn open(
            &mut self,
            _options: &Options,
            _dbname: &String,
            dbptr: *mut *mut dyn DB,
        ) -> crate::Status {
            unsafe {
                // A well-formed "null" trait-object pointer: null data ptr + valid vtable.
                *dbptr = ptr::null_mut::<MinimalDbForComposition>() as *mut dyn DB;
            }
            crate::Status::not_supported(&Slice::from("open not implemented in test"), None)
        }
    }

    impl DBWrite for MinimalDbForComposition {
        fn write(&mut self, _options: &WriteOptions, _updates: *mut WriteBatch) -> crate::Status {
            let n = self.writes.fetch_add(1, Ordering::SeqCst) + 1;
            trace!(write_calls = n, "DBWrite::write called");
            crate::Status::ok()
        }
    }

    impl DBPut for MinimalDbForComposition {}
    impl DBDelete for MinimalDbForComposition {}

    impl DBGet for MinimalDbForComposition {
        fn get(&mut self, _options: &ReadOptions, _key_: &Slice, _value: *mut String) -> crate::Status {
            crate::Status::not_found(&Slice::from("missing"), None)
        }
    }

    impl DBNewIterator for MinimalDbForComposition {
        fn new_iterator(&mut self, _options: &ReadOptions) -> *mut LevelDBIterator {
            Box::into_raw(Box::new(LevelDBIterator::default()))
        }
    }

    impl DBGetSnapshot for MinimalDbForComposition {
        fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
            Box::new(DummySnapshot)
        }
    }

    impl DBReleaseSnapshot for MinimalDbForComposition {
        fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
            drop(snapshot);
        }
    }

    impl DBGetProperty for MinimalDbForComposition {
        fn get_property(&mut self, _property: &str, _value: *mut String) -> bool {
            false
        }
    }

    impl DBGetApproximateSizes for MinimalDbForComposition {
        fn get_approximate_sizes(
            &mut self,
            _range: *const bitcoinleveldb_slice::Range,
            _n: i32,
            _sizes: *mut u64,
        ) {
        }
    }

    impl DBCompactRange for MinimalDbForComposition {
        fn compact_range(&mut self, _begin: *const Slice, _end: *const Slice) {}
    }

    impl DB for MinimalDbForComposition {}

    fn exercise_db_object_surface(db: &mut dyn DB) {
        let opt = WriteOptions::default();
        let key = Slice::from("k");
        let value = Slice::from("v");

        trace!("exercising db.put()");
        let s1 = db.put(&opt, &key, &value);
        assert!(s1.is_ok());

        trace!("exercising db.delete()");
        let s2 = db.delete(&opt, &key);
        assert!(s2.is_ok());

        trace!("exercising db.get()");
        let ro = ReadOptions::default();
        let mut out = Slice::from("unchanged").to_string();
        let s3 = db.get(&ro, &key, &mut out as *mut String);
        assert!(s3.is_not_found());

        trace!("exercising db.new_iterator()");
        let it = db.new_iterator(&ro);
        assert!(!it.is_null());
        unsafe {
            drop(Box::from_raw(it));
        }

        trace!("exercising snapshot acquire/release");
        let snap = db.get_snapshot();
        db.release_snapshot(snap);

        trace!("exercising property lookup");
        let mut prop = Slice::from("p").to_string();
        let ok = db.get_property("leveldb.unknown", &mut prop as *mut String);
        assert!(!ok);
    }

    #[traced_test]
    fn db_trait_object_dispatches_across_all_composed_subtraits() {
        let writes = Arc::new(AtomicUsize::new(0));
        let mut concrete = MinimalDbForComposition::new(writes.clone());

        let db: &mut dyn DB = &mut concrete;

        trace!("exercising composed DB surface via &mut dyn DB");
        exercise_db_object_surface(db);

        assert_eq!(writes.load(Ordering::SeqCst), 2);

        info!("verified DB trait composition is usable as a trait object and routes calls correctly");
    }
}
