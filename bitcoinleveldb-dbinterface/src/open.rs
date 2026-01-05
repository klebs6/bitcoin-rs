// ---------------- [ File: bitcoinleveldb-dbinterface/src/open.rs ]
crate::ix!();

pub trait DBOpen {

    /// Open the database with the specified "name".
    /// 
    /// Stores a pointer to a heap-allocated database in *dbptr and returns OK on success.
    /// 
    /// Stores nullptr in *dbptr and returns a non-OK status on error.
    /// 
    /// Caller should delete *dbptr when it is no longer needed.
    ///
    fn open(
        &mut self,
        options: &Options,
        dbname: &String,
        dbptr: *mut *mut dyn DB,
    ) -> Status;
}

#[cfg(test)]
mod open_pointer_contract_suite {
    use super::*;
    use core::ptr;
    use tracing::{info, trace};

    struct DummySnapshot;

    impl Snapshot for DummySnapshot {}

    struct DbStub;

    impl DBOpen for DbStub {
        fn open(
            &mut self,
            _options: &Options,
            _dbname: &String,
            dbptr: *mut *mut dyn DB,
        ) -> crate::Status {
            unsafe {
                core::ptr::write(dbptr, core::mem::zeroed());
            }
            crate::Status::not_supported(&Slice::from("not used"), None)
        }
    }

    impl DBWrite for DbStub {
        fn write(&mut self, _options: &WriteOptions, _updates: *mut WriteBatch) -> crate::Status {
            crate::Status::ok()
        }
    }

    impl DBPut for DbStub {}
    impl DBDelete for DbStub {}

    impl DBGet for DbStub {
        fn get(&mut self, _options: &ReadOptions, _key_: &Slice, _value: *mut String) -> crate::Status {
            crate::Status::not_found(&Slice::from("missing"), None)
        }
    }

    impl DBNewIterator for DbStub {
        fn new_iterator(&mut self, _options: &ReadOptions) -> *mut LevelDBIterator {
            Box::into_raw(Box::new(LevelDBIterator::default()))
        }
    }

    impl DBGetSnapshot for DbStub {
        fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
            Box::new(DummySnapshot)
        }
    }

    impl DBReleaseSnapshot for DbStub {
        fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
            drop(snapshot);
        }
    }

    impl DBGetProperty for DbStub {
        fn get_property(&mut self, _property: &str, _value: *mut String) -> bool {
            false
        }
    }

    impl DBGetApproximateSizes for DbStub {
        fn get_approximate_sizes(
            &mut self,
            _range: *const bitcoinleveldb_slice::Range,
            _n: i32,
            _sizes: *mut u64,
        ) {
        }
    }

    impl DBCompactRange for DbStub {
        fn compact_range(&mut self, _begin: *const Slice, _end: *const Slice) {}
    }

    impl DB for DbStub {}

    struct OpenProvider;

    impl DBOpen for OpenProvider {
        fn open(
            &mut self,
            _options: &Options,
            dbname: &String,
            dbptr: *mut *mut dyn DB,
        ) -> crate::Status {
            unsafe {
                core::ptr::write(dbptr, core::mem::zeroed());
            }

            if dbname.len() == 0 {
                return crate::Status::invalid_argument(&Slice::from("empty name"), None);
            }

            let db: Box<dyn DB> = Box::new(DbStub);
            let raw: *mut dyn DB = Box::into_raw(db);

            unsafe {
                *dbptr = raw;
            }

            return crate::Status::ok();
        }
    }

    #[traced_test]
    fn open_sets_dbptr_to_null_on_error() {
        let mut provider = OpenProvider;
        let options = Options::default();

        let mut out: *mut dyn DB = unsafe { core::mem::zeroed() };

        let out_ptr: *mut *mut dyn DB = &mut out as *mut *mut dyn DB;

        let empty = Slice::from("").to_string();

        trace!("calling open() with empty name");
        let s = provider.open(&options, &empty, out_ptr);

        assert!(s.is_invalid_argument());
        assert!(out.is_null());

        info!("verified open() leaves *dbptr as nullptr on error");
    }

    #[traced_test]
    fn open_sets_dbptr_to_heap_allocated_db_on_success_and_caller_can_drop_it() {
        let mut provider = OpenProvider;
        let options = Options::default();

        let mut out: *mut dyn DB = unsafe { core::mem::zeroed() };
        let out_ptr: *mut *mut dyn DB = &mut out as *mut *mut dyn DB;

        let name = Slice::from("testdb").to_string();

        trace!("calling open() expecting success");
        let s = provider.open(&options, &name, out_ptr);

        assert!(s.is_ok());
        assert!(!out.is_null());

        trace!("dropping returned db pointer");
        unsafe {
            drop(Box::from_raw(out));
        }

        info!("verified open() can hand ownership to caller via *mut dyn DB");
    }
}
