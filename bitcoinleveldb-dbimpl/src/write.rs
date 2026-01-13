// ---------------- [ File: bitcoinleveldb-dbimpl/src/write.rs ]
crate::ix!();

impl DBWrite for DBImpl {
    fn write(&mut self, options: &WriteOptions, updates: *mut WriteBatch) -> crate::Status { 
        todo!(); 
        /*
        let mut w: DBImplWriter = DBImplWriter::new(&mut self.mutex);
        w.set_batch(updates);
        w.set_sync(options.sync());
        w.set_done(false);

        self.mutex.lock();

        self.writers.push_back(&mut w as *mut DBImplWriter);

        while !w.done() && (self.writers.front().copied().unwrap() != (&mut w as *mut DBImplWriter)) {
            w.cv().wait();
        }

        if w.done() {
            let st = w.status().clone();
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
                write_batch_internal::set_sequence(write_batch, last_sequence + 1);
                last_sequence += write_batch_internal::count(write_batch);
            }

            // Add to log and apply to memtable.  We can release the lock
            // during this phase since &w is currently responsible for logging
            // and protects against concurrent loggers and concurrent writes
            // into mem.
            {
                self.mutex.unlock();

                status = unsafe {
                    (*self.log).add_record(write_batch_internal::contents(write_batch))
                };

                let mut sync_error: bool = false;

                if status.is_ok() && options.sync() {
                    status = unsafe { (*self.logfile).sync() };
                    if !status.is_ok() {
                        sync_error = true;
                    }
                }

                if status.is_ok() {
                    status = unsafe { write_batch_internal::insert_into(write_batch, self.mem) };
                }

                self.mutex.lock();

                if sync_error {
                    // The state of the log file is indeterminate: the log record we
                    // just added may or may not show up when the DB is re-opened.
                    // So we force the DB into a mode where all future writes fail.
                    self.record_background_error(&status);
                }
            }

            if write_batch == self.tmp_batch {
                unsafe { (*self.tmp_batch).clear(); }
            }

            unsafe {
                (*self.versions).set_last_sequence(last_sequence);
            }
        }

        loop {
            let ready: *mut DBImplWriter = self.writers.front().copied().unwrap();
            self.writers.pop_front();

            if ready != (&mut w as *mut DBImplWriter) {
                unsafe {
                    (*ready).set_status(status.clone());
                    (*ready).set_done(true);
                    (*ready).cv().signal();
                }
            }

            if ready == last_writer {
                break;
            }
        }

        // Notify new head of write queue
        if !self.writers.is_empty() {
            let head: *mut DBImplWriter = self.writers.front().copied().unwrap();
            unsafe {
                (*head).cv().signal();
            }
        }

        self.mutex.unlock();
        status
            */
    }
}
