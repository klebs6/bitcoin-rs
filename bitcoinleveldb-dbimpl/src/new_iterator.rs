// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_iterator.rs ]
crate::ix!();

impl DBNewIterator for DBImpl {
    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator {
        tracing::trace!("DBImpl::new_iterator: begin");

        let mut latest_snapshot: SequenceNumber = 0;
        let mut seed: u32 = 0;

        let internal_iter_ptr: *mut LevelDBIterator =
            self.new_internal_iterator(options, &mut latest_snapshot, &mut seed);

        if internal_iter_ptr.is_null() {
            tracing::error!(
                latest_snapshot,
                seed,
                "DBImpl::new_iterator: new_internal_iterator returned null"
            );
            panic!();
        }

        tracing::debug!(
            internal_iter_ptr = internal_iter_ptr as usize,
            latest_snapshot,
            seed,
            "DBImpl::new_iterator: obtained internal iterator"
        );

        let snapshot: SequenceNumber = match options.snapshot().as_ref() {
            Some(snap) => {
                // SAFETY: DBImpl only hands out SnapshotImpl instances via the public Snapshot interface.
                let raw: *const dyn Snapshot = Arc::as_ptr(snap);
                let data: *const () = raw as *const ();
                let snap_impl: *const SnapshotImpl = data as *const SnapshotImpl;

                let seq = unsafe { *(*snap_impl).sequence_number() };
                tracing::trace!(
                    seq,
                    "DBImpl::new_iterator: using explicit ReadOptions snapshot sequence"
                );
                seq
            }
            None => {
                tracing::trace!(
                    latest_snapshot,
                    "DBImpl::new_iterator: using latest snapshot sequence"
                );
                latest_snapshot
            }
        };

        // Adapter bridging DBImpl (raw pointer) into Rc<RefCell<dyn DBIterReadSample>> expected by dbiter.
        struct DbImplReadSampleProxy {
            db: *mut DBImpl,
        }

        impl DbImplReadSampleProxy {
            #[inline]
            fn new(db: *mut DBImpl) -> Self {
                Self { db }
            }
        }

        impl DBIterReadSample for DbImplReadSampleProxy {
            fn record_read_sample(&mut self, key: Slice) {
                let db_ptr: *mut DBImpl = self.db;

                if db_ptr.is_null() {
                    tracing::error!(
                        key_len = *key.size() as u64,
                        "DbImplReadSampleProxy::record_read_sample: db pointer was null"
                    );
                    return;
                }

                let key_len: usize = *key.size();

                tracing::trace!(
                    db_ptr = db_ptr as usize,
                    key_len = key_len as u64,
                    "DbImplReadSampleProxy::record_read_sample: forwarding to DBImpl"
                );

                unsafe {
                    DBIterReadSample::record_read_sample(&mut *db_ptr, key);
                }
            }
        }

        let db_ptr: *mut DBImpl = self as *mut DBImpl;

        let read_sample_db: Rc<RefCell<dyn DBIterReadSample>> =
            Rc::new(RefCell::new(DbImplReadSampleProxy::new(db_ptr)));

        tracing::debug!(
            db_ptr = db_ptr as usize,
            "DBImpl::new_iterator: constructed read-sample adapter for DBIter"
        );

        // Convert internal iterator raw pointer into the Rc container required by DBIter.
        let internal_iter_box: Box<LevelDBIterator> = unsafe { Box::from_raw(internal_iter_ptr) };
        let internal_iter: Rc<RefCell<LevelDBIterator>> =
            Rc::new(RefCell::new(*internal_iter_box));

        tracing::debug!(
            internal_iter_strong = Rc::strong_count(&internal_iter) as u64,
            "DBImpl::new_iterator: wrapped internal iterator in Rc<RefCell<_>>"
        );

        let user_iter_rc: Rc<RefCell<LevelDBIterator>> =
            new_db_iterator(read_sample_db, self.user_comparator(), internal_iter, snapshot, seed);

        let strong: usize = Rc::strong_count(&user_iter_rc);
        if strong != 1 {
            tracing::error!(
                strong_count = strong as u64,
                "DBImpl::new_iterator: expected unique Rc from new_db_iterator"
            );
        } else {
            tracing::trace!(
                strong_count = strong as u64,
                "DBImpl::new_iterator: new_db_iterator returned unique Rc"
            );
        }

        let user_iter_cell: RefCell<LevelDBIterator> = match Rc::try_unwrap(user_iter_rc) {
            Ok(cell) => cell,
            Err(rc) => {
                tracing::error!(
                    strong_count = Rc::strong_count(&rc) as u64,
                    "DBImpl::new_iterator: unexpected Rc sharing; cannot convert iterator into raw pointer"
                );
                panic!();
            }
        };

        let user_iter: LevelDBIterator = user_iter_cell.into_inner();
        let user_iter_ptr: *mut LevelDBIterator = Box::into_raw(Box::new(user_iter));

        tracing::trace!(
            user_iter_ptr = user_iter_ptr as usize,
            snapshot,
            seed,
            "DBImpl::new_iterator: end"
        );

        user_iter_ptr
    }
}

#[cfg(test)]
mod new_iterator_interface_and_smoke_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::{DB, DBNewIterator, DBOpen};

    fn build_temp_db_path_for_new_iterator_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!("bitcoinleveldb_dbimpl_new_iterator_suite_{}", nanos))
            .to_string_lossy()
            .to_string()
    }

    fn build_options_for_open_for_new_iterator_suite() -> Options {
        let env = PosixEnv::shared();
        let mut options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run new_iterator suite");
            panic!();
        }

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options
    }

    fn open_db_for_new_iterator_suite(options: &Options, dbname: &String) -> (*mut dyn DB, Status) {
        let mut dispatcher: DBImpl = DBImpl::new(options, dbname);
        let mut out_db: *mut dyn DB = core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let st: Status =
            <DBImpl as DBOpen>::open(&mut dispatcher, options, dbname, &mut out_db as *mut *mut dyn DB);

        (out_db, st)
    }

    #[inline]
    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    fn assert_dbimpl_implements_db_new_iterator() {
        fn _assert<T: DBNewIterator>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_new_iterator_trait_object(_db: &mut dyn DBNewIterator) {}

    fn compile_only_new_iterator_call_via_trait_object(
        db: &mut dyn DBNewIterator,
        options: &ReadOptions,
    ) -> *mut LevelDBIterator {
        db.new_iterator(options)
    }

    #[traced_test]
    fn new_iterator_trait_is_object_safe_and_dbimpl_implements_it() {
        tracing::info!("Asserting DBNewIterator is object-safe and DBImpl implements DBNewIterator");

        assert_dbimpl_implements_db_new_iterator();

        let _accept = compile_only_accepts_db_new_iterator_trait_object as fn(&mut dyn DBNewIterator);
        let _call = compile_only_new_iterator_call_via_trait_object
            as fn(&mut dyn DBNewIterator, &ReadOptions) -> *mut LevelDBIterator;

        tracing::debug!("Trait object acceptance + call wrapper compiled");
        let _ = (_accept, _call);
    }

    #[traced_test]
    fn new_iterator_method_item_is_addressable() {
        tracing::info!("Asserting <DBImpl as DBNewIterator>::new_iterator is addressable");
        let _m = <DBImpl as DBNewIterator>::new_iterator;
        let _ = _m;
    }

    #[traced_test]
    fn new_iterator_returns_non_null_iterator_on_open_database() {
        let dbname = build_temp_db_path_for_new_iterator_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_for_open_for_new_iterator_suite();

        tracing::info!(dbname = %dbname, "Opening database for new_iterator smoke test");
        let (db_ptr, open_status) = open_db_for_new_iterator_suite(&options, &dbname);

        tracing::debug!(
            status = %open_status.to_string(),
            db_ptr_is_null = db_ptr.is_null(),
            "DBOpen::open returned"
        );

        assert!(open_status.is_ok());
        assert!(!db_ptr.is_null());

        let dbimpl_ptr: *mut DBImpl = unsafe { db_ptr_to_dbimpl_mut(db_ptr) };
        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };
        dbimpl.clear_background_error_for_test();

        tracing::debug!(
            dbname = %dbimpl.dbname,
            mem_ptr = dbimpl.mem as usize,
            versions_ptr = dbimpl.versions as usize,
            "DBImpl state before calling new_iterator"
        );

        tracing::info!("Calling <DBImpl as DBNewIterator>::new_iterator");
        let it: *mut LevelDBIterator =
            <DBImpl as DBNewIterator>::new_iterator(dbimpl, &ReadOptions::default());

        tracing::debug!(it_is_null = it.is_null(), "new_iterator result");
        assert!(
            !it.is_null(),
            "new_iterator must return non-null iterator on open DB"
        );

        unsafe {
            drop(Box::from_raw(it));
            drop(Box::from_raw(db_ptr));
        }

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => tracing::debug!(path = %dbname, "Removed new_iterator test directory"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(path = %dbname, "No new_iterator test directory to remove");
            }
            Err(e) => tracing::warn!(
                path = %dbname,
                error = %format!("{:?}", e),
                "Failed to remove new_iterator test directory"
            ),
        }
    }
}
