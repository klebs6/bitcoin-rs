// ---------------- [ File: bitcoinleveldb-dbimpl/src/write.rs ]
crate::ix!();

impl DBWrite for DBImpl {
    fn write(&mut self, options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status {
        let mut w: DBImplWriter = DBImplWriter::new(&mut self.mutex);
        w.batch = updates;
        w.sync = options.sync;
        w.done = false;

        self.mutex.lock();

        self.writers_.push_back(&mut w as *mut DBImplWriter);

        while !w.done && (self.writers_.front().copied().unwrap() != (&mut w as *mut DBImplWriter)) {
            w.cv.wait();
        }

        if w.done {
            let st = w.status.clone();
            self.mutex.unlock();
            return st;
        }

        // May temporarily unlock and wait.
        let mut status: Status = self.make_room_for_write(updates.is_null());
        let mut last_sequence: u64 = unsafe { (*self.versions).last_sequence() };
        let mut last_writer: *mut DBImplWriter = &mut w as *mut DBImplWriter;

        // nullptr batch is for compactions
        if status.is_ok() && !updates.is_null() {

            let write_batch: *mut WriteBatch = self.build_batch_group(&mut last_writer);
            unsafe {
                WriteBatchInternal::set_sequence(write_batch, last_sequence + 1);
                last_sequence += WriteBatchInternal::count(write_batch);
            }

            // Add to log and apply to memtable.  We can release the lock
            // during this phase since &w is currently responsible for logging
            // and protects against concurrent loggers and concurrent writes
            // into mem_.
            {
                self.mutex.unlock();

                status = unsafe {
                    (*self.log_).add_record(WriteBatchInternal::contents(write_batch))
                };

                let mut sync_error: bool = false;

                if status.is_ok() && options.sync {
                    status = unsafe { (*self.logfile_).sync() };
                    if !status.is_ok() {
                        sync_error = true;
                    }
                }

                if status.is_ok() {
                    status = unsafe { WriteBatchInternal::insert_into(write_batch, self.mem_) };
                }

                self.mutex.lock();

                if sync_error {
                    // The state of the log file is indeterminate: the log record we
                    // just added may or may not show up when the DB is re-opened.
                    // So we force the DB into a mode where all future writes fail.
                    self.record_background_error(&status);
                }
            }

            if write_batch == self.tmp_batch_ {
                unsafe { (*self.tmp_batch_).clear(); }
            }

            unsafe {
                (*self.versions).set_last_sequence(last_sequence);
            }
        }

        loop {
            let ready: *mut DBImplWriter = self.writers_.front().copied().unwrap();
            self.writers_.pop_front();

            if ready != (&mut w as *mut DBImplWriter) {
                unsafe {
                    (*ready).status = status.clone();
                    (*ready).done = true;
                    (*ready).cv.signal();
                }
            }

            if ready == last_writer {
                break;
            }
        }

        // Notify new head of write queue
        if !self.writers_.is_empty() {
            let head: *mut DBImplWriter = self.writers_.front().copied().unwrap();
            unsafe {
                (*head).cv.signal();
            }
        }

        self.mutex.unlock();
        status
    }
}

#[cfg(test)]
#[disable]
mod write_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn write_applies_batch_atomically_and_persists_across_reopen() {
        let (dbname, mut db) = open_dbimpl_for_test("write_applies_batch_atomically_and_persists_across_reopen");

        // Write a batch with multiple operations.
        let mut batch: WriteBatch = Default::default();
        batch.put(&Slice::from_str("k1"), &Slice::from_str("v1"));
        batch.put(&Slice::from_str("k2"), &Slice::from_str("v2"));
        batch.delete(&Slice::from_str("k_missing"));

        let mut boxed: Box<WriteBatch> = Box::new(batch);
        let s: Status = <DBImpl as DBWrite>::write(
            &mut *db,
            &WriteOptions::default(),
            (&mut *boxed) as *mut WriteBatch,
        );

        tracing::info!(status = %s.to_string(), "batch write");
        assert!(s.is_ok(), "write should succeed: {}", s.to_string());

        assert_read_eq(&mut *db, "k1", "v1");
        assert_read_eq(&mut *db, "k2", "v2");

        drop(db);

        // Reopen and confirm persistence.
        let opts: Options = default_test_options();
        let mut db2: Box<DBImpl> = reopen_dbimpl_for_test(&dbname, opts);

        assert_read_eq(&mut *db2, "k1", "v1");
        assert_read_eq(&mut *db2, "k2", "v2");

        drop(db2);
        remove_db_dir_best_effort(&dbname);
    }
}
