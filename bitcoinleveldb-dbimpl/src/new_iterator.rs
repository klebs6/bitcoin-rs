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
