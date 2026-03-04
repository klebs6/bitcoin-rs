// ---------------- [ File: bitcoinleveldb-dbimpl/src/new_internal_iterator.rs ]
crate::ix!();

impl DBImpl {

    pub fn new_internal_iterator(
        &mut self,
        options: &ReadOptions,
        latest_snapshot: *mut SequenceNumber,
        seed: *mut u32,
    ) -> *mut LevelDBIterator {
        self.mutex.lock();

        unsafe {
            *latest_snapshot = (*self.versions).last_sequence();
        }

        // Collect together all needed child iterators.
        let mut list: Vec<*mut LevelDBIterator> = Vec::new();

        unsafe {
            list.push((*self.mem).new_iterator());
            (*self.mem).ref_();

            if !self.imm.is_null() {
                list.push((*self.imm).new_iterator());
                (*self.imm).ref_();
            }

            let current: *mut Version = (*self.versions).current();
            (*current).add_iterators(options, &mut list);

            let n: i32 = list
                .len()
                .try_into()
                .expect("DBImpl::new_internal_iterator: too many child iterators to merge");

            tracing::trace!(
                child_iters = n as u64,
                "DBImpl::new_internal_iterator: collected child iterators"
            );

            let internal_iter: *mut LevelDBIterator = if n == 1 {
                tracing::trace!(
                    "DBImpl::new_internal_iterator: single child iterator; returning directly without merger"
                );
                list[0]
            } else {
                let comparator: Box<dyn SliceComparator> = Box::new(InternalKeyComparator::new(
                    self.internal_comparator.user_comparator(),
                ));

                // IMPORTANT:
                // new_merging_iterator expects the child iterator pointer-array to be a heap allocation
                // with capacity == len. Convert to a boxed slice (len-sized allocation) and transfer
                // ownership of the backing buffer to the merger.
                let mut children_boxed: Box<[*mut LevelDBIterator]> = list.into_boxed_slice();
                let children_ptr: *mut *mut LevelDBIterator = children_boxed.as_mut_ptr();
                core::mem::forget(children_boxed);

                tracing::trace!(
                    child_iters = n as u64,
                    "DBImpl::new_internal_iterator: constructing merging iterator"
                );

                new_merging_iterator(comparator, children_ptr, n)
            };

            if internal_iter.is_null() {
                tracing::error!(
                    child_iters = n as u64,
                    "DBImpl::new_internal_iterator: internal iterator construction returned null"
                );
                panic!();
            }

            (*current).ref_();

            let cleanup: *mut IterState = Box::into_raw(Box::new(IterState::new(
                &mut self.mutex,
                self.mem,
                self.imm,
                current,
            )));

            (*internal_iter).register_cleanup(
                cleanup_iterator_state,
                cleanup as *mut core::ffi::c_void,
                core::ptr::null_mut(),
            );

            self.seed = self.seed.wrapping_add(1);
            *seed = self.seed;

            self.mutex.unlock();

            internal_iter
        }
    }

    /// Return an internal iterator over the current
    /// state of the database.
    /// 
    /// The keys of this iterator are internal keys
    /// (see format.h).
    /// 
    /// The returned iterator should be deleted when
    /// no longer needed.
    pub fn test_new_internal_iterator(&mut self) -> *mut LevelDBIterator {
        let mut ignored: SequenceNumber = 0;
        let mut ignored_seed: u32 = 0;
        self.new_internal_iterator(&ReadOptions::default(), &mut ignored, &mut ignored_seed)
    }
}

//-------------------------------------------------------
fn cleanup_dbimpl_internal_iterator_state(
    arg1: *mut core::ffi::c_void,
    arg2: *mut core::ffi::c_void,
) {
    let _ = arg2;

    type State = (*mut RawMutex, *mut MemTable, *mut MemTable, *mut Version);

    let state_ptr: *mut State = arg1 as *mut State;

    if state_ptr.is_null() {
        tracing::error!(
            "cleanup_dbimpl_internal_iterator_state: arg1/state pointer was null; skipping cleanup"
        );
        return;
    }

    // Take ownership so the allocation is released exactly once.
    let state_box: Box<State> = unsafe { Box::from_raw(state_ptr) };
    let (mu, mem, imm, version): State = *state_box;

    tracing::trace!(
        mu_ptr = mu as usize,
        mem_ptr = mem as usize,
        imm_ptr = imm as usize,
        version_ptr = version as usize,
        "cleanup_dbimpl_internal_iterator_state: begin"
    );

    if mu.is_null() {
        tracing::error!(
            mem_ptr = mem as usize,
            imm_ptr = imm as usize,
            version_ptr = version as usize,
            "cleanup_dbimpl_internal_iterator_state: mutex pointer was null; performing best-effort unrefs without locking"
        );

        unsafe {
            if !mem.is_null() {
                (*mem).unref();
            } else {
                tracing::warn!(
                    "cleanup_dbimpl_internal_iterator_state: mem pointer was null; skipping mem.unref()"
                );
            }

            if !imm.is_null() {
                (*imm).unref();
            }

            if !version.is_null() {
                (*version).unref();
            } else {
                tracing::warn!(
                    "cleanup_dbimpl_internal_iterator_state: version pointer was null; skipping version.unref()"
                );
            }
        }

        tracing::trace!("cleanup_dbimpl_internal_iterator_state: end (no mutex)");
        return;
    }

    unsafe {
        (*mu).lock();

        if !mem.is_null() {
            (*mem).unref();
        } else {
            tracing::warn!(
                "cleanup_dbimpl_internal_iterator_state: mem pointer was null; skipping mem.unref()"
            );
        }

        if !imm.is_null() {
            (*imm).unref();
        }

        if !version.is_null() {
            (*version).unref();
        } else {
            tracing::warn!(
                "cleanup_dbimpl_internal_iterator_state: version pointer was null; skipping version.unref()"
            );
        }

        (*mu).unlock();
    }

    tracing::trace!("cleanup_dbimpl_internal_iterator_state: end");
}

#[cfg(test)]
mod new_internal_iterator_interface_and_smoke_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::{DB, DBOpen};

    fn build_temp_db_path_for_new_internal_iterator_suite() -> String {
        let tmp = TempDir::new().unwrap();
        let dbname = tmp.path().to_string_lossy().to_string();
        dbname
    }

    fn build_options_for_open_for_new_internal_iterator_suite() -> Options {
        let env = PosixEnv::shared();
        let mut options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run new_internal_iterator suite");
            panic!();
        }

        options.set_create_if_missing(true);
        options.set_error_if_exists(false);
        options
    }

    fn open_db_for_new_internal_iterator_suite(
        options: &Options,
        dbname: &String,
    ) -> (*mut dyn DB, Status) {

        let mut dispatcher: DBImpl = DBImpl::new(options, dbname);

        // IMPORTANT:
        // dyn DB is a fat pointer and cannot be constructed via null_mut::<dyn DB>().
        // Initialize via a null concrete pointer cast instead.
        let mut out_db: *mut dyn DB =
            core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

        let st: Status = <DBImpl as DBOpen>::open(
            &mut dispatcher,
            options,
            dbname,
            &mut out_db as *mut *mut dyn DB,
        );

        (out_db, st)
    }

    #[inline]
    unsafe fn db_ptr_to_dbimpl_mut(db_ptr: *mut dyn DB) -> *mut DBImpl {
        let data: *mut () = db_ptr as *mut ();
        data as *mut DBImpl
    }

    #[traced_test]
    fn new_internal_iterator_signature_is_stable() {
        tracing::info!("Asserting DBImpl::new_internal_iterator signature is stable");

        type Sig = fn(&mut DBImpl, &ReadOptions, *mut SequenceNumber, *mut u32) -> *mut LevelDBIterator;
        let _sig: Sig = DBImpl::new_internal_iterator;

        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn test_new_internal_iterator_signature_is_stable() {
        tracing::info!("Asserting DBImpl::test_new_internal_iterator signature is stable");

        type Sig = fn(&mut DBImpl) -> *mut LevelDBIterator;
        let _sig: Sig = DBImpl::test_new_internal_iterator;

        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn new_internal_iterator_increments_seed_and_returns_non_null_iterators() {
        //init_test_runtime();

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.entry",
            "phase=entry"
        );

        let dbname = build_temp_db_path_for_new_internal_iterator_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.dbname.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.create_dir_all.begin",
            "dbname={}",
            dbname
        );
        let __mkdir_res = std::fs::create_dir_all(&dbname);
        match __mkdir_res {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.increment_seed.create_dir_all.ok",
                    "dbname={}",
                    dbname
                );
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.increment_seed.create_dir_all.err",
                    "dbname={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
            }
        }

        let options = build_options_for_open_for_new_internal_iterator_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.options.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.open_db.trace_info.before",
            "dbname={}",
            dbname
        );
        tracing::info!(dbname = %dbname, "Opening database for new_internal_iterator smoke test");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.open_db.call.begin",
            "dbname={}",
            dbname
        );
        let (db_ptr, open_status) = open_db_for_new_internal_iterator_suite(&options, &dbname);
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.open_db.call.end",
            "dbname={} open_status={} open_ok={} db_ptr_is_null={} db_ptr_data=0x{:x}",
            dbname,
            open_status.to_string(),
            open_status.is_ok(),
            db_ptr.is_null(),
            (db_ptr as *mut ()) as usize
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.open_db.trace_debug.after",
            "open_status={} db_ptr_is_null={}",
            open_status.to_string(),
            db_ptr.is_null()
        );
        tracing::debug!(
            status = %open_status.to_string(),
            db_ptr_is_null = db_ptr.is_null(),
            "DBOpen::open returned"
        );

        assert!(open_status.is_ok(), "DBOpen::open must succeed");
        assert!(
            !db_ptr.is_null(),
            "DBOpen::open must return non-null DB pointer"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.downcast.begin",
            "db_ptr_is_null={} db_ptr_data=0x{:x}",
            db_ptr.is_null(),
            (db_ptr as *mut ()) as usize
        );
        let dbimpl_ptr: *mut DBImpl = unsafe { db_ptr_to_dbimpl_mut(db_ptr) };
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.downcast.end",
            "db_ptr_data=0x{:x} dbimpl_ptr=0x{:x} dbimpl_ptr_is_null={}",
            (db_ptr as *mut ()) as usize,
            dbimpl_ptr as usize,
            dbimpl_ptr.is_null()
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.downcast.trace_debug",
            "db_ptr_data=0x{:x} dbimpl_ptr=0x{:x}",
            (db_ptr as *mut ()) as usize,
            dbimpl_ptr as usize
        );
        tracing::debug!(
            db_ptr_data = (db_ptr as *mut ()) as usize,
            dbimpl_ptr = dbimpl_ptr as usize,
            "Downcast dyn DB to DBImpl data pointer"
        );

        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.clear_background_error.begin",
            "dbname={}",
            dbimpl.dbname
        );
        dbimpl.clear_background_error_for_test();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.clear_background_error.end",
            "bg_error={}",
            dbimpl.bg_error.to_string()
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.db_state.trace_debug",
            "dbname={} mem_ptr=0x{:x} imm_ptr=0x{:x} versions_ptr=0x{:x} table_cache_ptr=0x{:x} bg_error={}",
            dbimpl.dbname,
            dbimpl.mem as usize,
            dbimpl.imm as usize,
            dbimpl.versions as usize,
            dbimpl.table_cache as usize,
            dbimpl.bg_error.to_string()
        );
        tracing::debug!(
            dbname = %dbimpl.dbname,
            mem_ptr = dbimpl.mem as usize,
            imm_ptr = dbimpl.imm as usize,
            versions_ptr = dbimpl.versions as usize,
            table_cache_ptr = dbimpl.table_cache as usize,
            bg_error = %dbimpl.bg_error.to_string(),
            "DBImpl state after open (pre-iterator)"
        );

        let mut latest_snapshot_1: SequenceNumber = 999;
        let mut seed_1: u32 = 0;

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_1.trace_info.before",
            "latest_snapshot_1={} seed_1={}",
            latest_snapshot_1,
            seed_1
        );
        tracing::info!("Calling DBImpl::new_internal_iterator (1)");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_1.begin",
            "latest_snapshot_1={} seed_1={}",
            latest_snapshot_1,
            seed_1
        );
        let it1: *mut LevelDBIterator = dbimpl.new_internal_iterator(
            &ReadOptions::default(),
            &mut latest_snapshot_1,
            &mut seed_1,
        );
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_1.end",
            "it1_is_null={} it1_data=0x{:x} latest_snapshot_1={} seed_1={}",
            it1.is_null(),
            (it1 as *mut ()) as usize,
            latest_snapshot_1,
            seed_1
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_1.trace_debug.after",
            "it1_is_null={} latest_snapshot_1={} seed_1={}",
            it1.is_null(),
            latest_snapshot_1,
            seed_1
        );
        tracing::debug!(
            it1_is_null = it1.is_null(),
            latest_snapshot_1,
            seed_1,
            "First new_internal_iterator result"
        );

        assert!(!it1.is_null(), "First iterator must be non-null");
        assert_eq!(
            seed_1, 1,
            "First call must produce seed=1 for a fresh DB"
        );
        assert_eq!(
            latest_snapshot_1, 0,
            "Fresh DB should have latest_snapshot=0"
        );

        let mut latest_snapshot_2: SequenceNumber = 999;
        let mut seed_2: u32 = 0;

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_2.trace_info.before",
            "latest_snapshot_2={} seed_2={}",
            latest_snapshot_2,
            seed_2
        );
        tracing::info!("Calling DBImpl::new_internal_iterator (2)");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_2.begin",
            "latest_snapshot_2={} seed_2={}",
            latest_snapshot_2,
            seed_2
        );
        let it2: *mut LevelDBIterator = dbimpl.new_internal_iterator(
            &ReadOptions::default(),
            &mut latest_snapshot_2,
            &mut seed_2,
        );
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_2.end",
            "it2_is_null={} it2_data=0x{:x} latest_snapshot_2={} seed_2={}",
            it2.is_null(),
            (it2 as *mut ()) as usize,
            latest_snapshot_2,
            seed_2
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.call_new_internal_iterator_2.trace_debug.after",
            "it2_is_null={} latest_snapshot_2={} seed_2={}",
            it2.is_null(),
            latest_snapshot_2,
            seed_2
        );
        tracing::debug!(
            it2_is_null = it2.is_null(),
            latest_snapshot_2,
            seed_2,
            "Second new_internal_iterator result"
        );

        assert!(!it2.is_null(), "Second iterator must be non-null");
        assert_eq!(seed_2, 2, "Second call must produce seed=2");
        assert_eq!(
            latest_snapshot_2, 0,
            "Fresh DB should have latest_snapshot=0"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.mutex_try_lock.begin",
            "phase=try_lock"
        );
        let reacquired = dbimpl.mutex.try_lock();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.mutex_try_lock.trace_debug",
            "reacquired={}",
            reacquired
        );
        tracing::debug!(reacquired, "RawMutex try_lock after new_internal_iterator calls");
        assert!(
            reacquired,
            "new_internal_iterator must not leak the DB mutex lock"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.mutex_unlock.begin",
            "phase=unlock"
        );
        unsafe { dbimpl.mutex.unlock() };
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.mutex_unlock.end",
            "phase=unlock_done"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.drop.begin",
            "it2_data=0x{:x} it1_data=0x{:x} db_ptr_data=0x{:x}",
            (it2 as *mut ()) as usize,
            (it1 as *mut ()) as usize,
            (db_ptr as *mut ()) as usize
        );
        unsafe {
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.it2.begin",
                "it2_data=0x{:x}",
                (it2 as *mut ()) as usize
            );
            drop(Box::from_raw(it2));
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.it2.end",
                "it2_data=0x{:x}",
                (it2 as *mut ()) as usize
            );

            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.it1.begin",
                "it1_data=0x{:x}",
                (it1 as *mut ()) as usize
            );
            drop(Box::from_raw(it1));
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.it1.end",
                "it1_data=0x{:x}",
                (it1 as *mut ()) as usize
            );

            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.db_ptr.begin",
                "db_ptr_data=0x{:x}",
                (db_ptr as *mut ()) as usize
            );
            drop(Box::from_raw(db_ptr));
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.increment_seed.drop.db_ptr.end",
                "db_ptr_data=0x{:x}",
                (db_ptr as *mut ()) as usize
            );
        }

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.remove_dir_all.begin",
            "dbname={}",
            dbname
        );
        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.increment_seed.remove_dir_all.ok.trace_debug",
                    "path={}",
                    dbname
                );
                tracing::debug!(path = %dbname, "Removed new_internal_iterator test directory");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.increment_seed.remove_dir_all.not_found.trace_trace",
                    "path={}",
                    dbname
                );
                tracing::trace!(path = %dbname, "No new_internal_iterator test directory to remove");
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.increment_seed.remove_dir_all.err.trace_warn",
                    "path={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
                tracing::warn!(
                    path = %dbname,
                    error = %format!("{:?}", e),
                    "Failed to remove new_internal_iterator test directory"
                );
            }
        }

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.increment_seed.exit",
            "phase=exit"
        );
    }

    #[traced_test]
    fn test_new_internal_iterator_returns_non_null_iterator() {
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.entry",
            "phase=entry"
        );

        let dbname = build_temp_db_path_for_new_internal_iterator_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.dbname.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.create_dir_all.begin",
            "dbname={}",
            dbname
        );
        let __mkdir_res = std::fs::create_dir_all(&dbname);
        match __mkdir_res {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.smoke_non_null.create_dir_all.ok",
                    "dbname={}",
                    dbname
                );
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.smoke_non_null.create_dir_all.err",
                    "dbname={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
            }
        }

        let options = build_options_for_open_for_new_internal_iterator_suite();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.options.built",
            "dbname={}",
            dbname
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.open_db.trace_info.before",
            "dbname={}",
            dbname
        );
        tracing::info!(dbname = %dbname, "Opening database for test_new_internal_iterator smoke test");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.open_db.call.begin",
            "dbname={}",
            dbname
        );
        let (db_ptr, open_status) = open_db_for_new_internal_iterator_suite(&options, &dbname);
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.open_db.call.end",
            "dbname={} open_status={} open_ok={} db_ptr_is_null={} db_ptr_data=0x{:x}",
            dbname,
            open_status.to_string(),
            open_status.is_ok(),
            db_ptr.is_null(),
            (db_ptr as *mut ()) as usize
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.open_db.trace_debug.after",
            "open_status={} db_ptr_is_null={}",
            open_status.to_string(),
            db_ptr.is_null()
        );
        tracing::debug!(
            status = %open_status.to_string(),
            db_ptr_is_null = db_ptr.is_null(),
            "DBOpen::open returned"
        );

        assert!(open_status.is_ok());
        assert!(!db_ptr.is_null());

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.downcast.begin",
            "db_ptr_data=0x{:x}",
            (db_ptr as *mut ()) as usize
        );
        let dbimpl_ptr: *mut DBImpl = unsafe { db_ptr_to_dbimpl_mut(db_ptr) };
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.downcast.end",
            "dbimpl_ptr=0x{:x} dbimpl_ptr_is_null={}",
            dbimpl_ptr as usize,
            dbimpl_ptr.is_null()
        );

        let dbimpl: &mut DBImpl = unsafe { &mut *dbimpl_ptr };

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.clear_background_error.begin",
            "dbname={}",
            dbimpl.dbname
        );
        dbimpl.clear_background_error_for_test();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.clear_background_error.end",
            "bg_error={}",
            dbimpl.bg_error.to_string()
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.db_state.trace_debug",
            "dbname={} mem_ptr=0x{:x} versions_ptr=0x{:x}",
            dbimpl.dbname,
            dbimpl.mem as usize,
            dbimpl.versions as usize
        );
        tracing::debug!(
            dbname = %dbimpl.dbname,
            mem_ptr = dbimpl.mem as usize,
            versions_ptr = dbimpl.versions as usize,
            "DBImpl state before calling test_new_internal_iterator"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.call.trace_info.before",
            "phase=call"
        );
        tracing::info!("Calling DBImpl::test_new_internal_iterator()");

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.call.begin",
            "phase=call"
        );
        let it: *mut LevelDBIterator = dbimpl.test_new_internal_iterator();
        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.call.end",
            "it_is_null={} it_data=0x{:x}",
            it.is_null(),
            (it as *mut ()) as usize
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.call.trace_debug.after",
            "it_is_null={} it_data=0x{:x}",
            it.is_null(),
            (it as *mut ()) as usize
        );
        tracing::debug!(it_is_null = it.is_null(), "test_new_internal_iterator result");
        assert!(
            !it.is_null(),
            "test_new_internal_iterator must return a non-null iterator"
        );

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.drop.begin",
            "it_data=0x{:x} db_ptr_data=0x{:x}",
            (it as *mut ()) as usize,
            (db_ptr as *mut ()) as usize
        );
        unsafe {
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.smoke_non_null.drop.it.begin",
                "it_data=0x{:x}",
                (it as *mut ()) as usize
            );
            drop(Box::from_raw(it));
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.smoke_non_null.drop.it.end",
                "it_data=0x{:x}",
                (it as *mut ()) as usize
            );

            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.smoke_non_null.drop.db_ptr.begin",
                "db_ptr_data=0x{:x}",
                (db_ptr as *mut ()) as usize
            );
            drop(Box::from_raw(db_ptr));
            bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                "new_internal_iterator.smoke_non_null.drop.db_ptr.end",
                "db_ptr_data=0x{:x}",
                (db_ptr as *mut ()) as usize
            );
        }

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.remove_dir_all.begin",
            "dbname={}",
            dbname
        );
        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.smoke_non_null.remove_dir_all.ok.trace_debug",
                    "path={}",
                    dbname
                );
                tracing::debug!(path = %dbname, "Removed test_new_internal_iterator directory");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.smoke_non_null.remove_dir_all.not_found.trace_trace",
                    "path={}",
                    dbname
                );
                tracing::trace!(path = %dbname, "No test_new_internal_iterator directory to remove");
            }
            Err(e) => {
                bitcoinleveldb_dbimpl_realtime_probe_20260303!(
                    "new_internal_iterator.smoke_non_null.remove_dir_all.err.trace_warn",
                    "path={} error_kind={:?} error={:?}",
                    dbname,
                    e.kind(),
                    e
                );
                tracing::warn!(
                    path = %dbname,
                    error = %format!("{:?}", e),
                    "Failed to remove test_new_internal_iterator directory"
                );
            }
        }

        bitcoinleveldb_dbimpl_realtime_probe_20260303!(
            "new_internal_iterator.smoke_non_null.exit",
            "phase=exit"
        );
    }
}
