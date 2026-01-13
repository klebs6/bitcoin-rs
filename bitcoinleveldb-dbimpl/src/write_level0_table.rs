// ---------------- [ File: bitcoinleveldb-dbimpl/src/write_level0_table.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn write_level_0table(
        &mut self,
        mem: *mut MemTable,
        edit: *mut VersionEdit,
        base: *mut Version,
    ) -> crate::Status { 
        todo!(); 
        /*
        self.mutex.assert_held();

        let start_micros: u64 = self.env.borrow_mut().now_micros();

        let mut meta: FileMetaData = Default::default();
        meta.set_number(unsafe { (*self.versions).new_file_number() });

        self.pending_outputs.insert(meta.number());

        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };

        tracing::info!(file_number = meta.number(), "Level-0 table started");

        let mut s: Status = Status::ok();

        self.mutex.unlock();
        s = build_table(
            &self.dbname,
            &mut *self.env.borrow_mut(),
            &self.options,
            self.table_cache,
            iter,
            &mut meta,
        );
        self.mutex.lock();

        tracing::info!(
            file_number = meta.number(),
            bytes = meta.file_size(),
            status = %s.to_string(),
            "Level-0 table finished"
        );

        unsafe {
            drop(Box::from_raw(iter));
        }

        self.pending_outputs.remove(&meta.number());

        // Note that if file_size is zero, the file has been deleted and
        // should not be added to the manifest.
        let mut level: i32 = 0;

        if s.is_ok() && meta.file_size() > 0 {
            let min_user_key: Slice = meta.smallest().user_key();
            let max_user_key: Slice = meta.largest().user_key();

            if !base.is_null() {
                level = unsafe { (*base).pick_level_for_mem_table_output(min_user_key, max_user_key) };
            }

            unsafe {
                (*edit).add_file(
                    level,
                    *meta.number(),
                    *meta.file_size(),
                    meta.smallest(),
                    meta.largest(),
                );
            }
        }

        let mut stats: CompactionStats = Default::default();
        stats.set_micros((self.env.borrow_mut().now_micros() - start_micros) as i64);
        stats.set_bytes_written(meta.file_size() as i64);

        self.stats[level as usize].add(stats);

        s
                         */
    }
}
