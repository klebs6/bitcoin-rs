// ---------------- [ File: bitcoinleveldb-dbimpl/src/write_level0_table.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex_)]
    pub fn write_level_0table(
        &mut self,
        mem: *mut MemTable,
        edit: *mut VersionEdit,
        base: *mut Version,
    ) -> crate::Status {
        self.mutex.assert_held();

        let start_micros: u64 = self.env_.borrow_mut().now_micros();

        let mut meta: FileMetaData = Default::default();
        meta.number = unsafe { (*self.versions_).new_file_number() };

        self.pending_outputs_.insert(meta.number);

        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };

        tracing::info!(file_number = meta.number, "Level-0 table started");

        let mut s: Status = Status::ok();

        self.mutex.unlock();
        s = build_table(
            &self.dbname_,
            &mut *self.env_.borrow_mut(),
            &self.options_,
            self.table_cache_,
            iter,
            &mut meta,
        );
        self.mutex.lock();

        tracing::info!(
            file_number = meta.number,
            bytes = meta.file_size,
            status = %s.to_string(),
            "Level-0 table finished"
        );

        unsafe {
            drop(Box::from_raw(iter));
        }

        self.pending_outputs_.remove(&meta.number);

        // Note that if file_size is zero, the file has been deleted and
        // should not be added to the manifest.
        let mut level: i32 = 0;

        if s.is_ok() && meta.file_size > 0 {
            let min_user_key: Slice = meta.smallest.user_key();
            let max_user_key: Slice = meta.largest.user_key();

            if !base.is_null() {
                level = unsafe { (*base).pick_level_for_mem_table_output(min_user_key, max_user_key) };
            }

            unsafe {
                (*edit).add_file(
                    level,
                    meta.number,
                    meta.file_size,
                    meta.smallest.clone(),
                    meta.largest.clone(),
                );
            }
        }

        let mut stats: CompactionStats = Default::default();
        stats.micros = (self.env_.borrow_mut().now_micros() - start_micros) as i64;
        stats.bytes_written = meta.file_size as i64;

        self.stats_[level as usize].add(stats);

        s
    }
}

#[cfg(test)]
#[disable]
mod write_level0_table_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn level0_table_creation_is_observable_via_num_files_at_level_property() {
        let (dbname, mut db) =
            open_dbimpl_for_test("level0_table_creation_is_observable_via_num_files_at_level_property");

        fill_sequential(&mut *db, "l0", 700, 256);
        force_manual_compaction_full_range(&mut *db);

        let mut out: String = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(
            &mut *db,
            "leveldb.num-files-at-level0",
            (&mut out) as *mut String,
        );

        tracing::info!(ok, out = %out, "num-files-at-level0");
        assert!(ok, "num-files-at-level0 should be available");
        assert!(!out.is_empty(), "property value should be non-empty");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
