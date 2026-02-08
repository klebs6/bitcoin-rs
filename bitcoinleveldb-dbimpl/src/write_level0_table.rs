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
        self.mutex.assert_held();

        let env_rc: Rc<RefCell<dyn Env>> = match self.options.env().as_ref() {
            Some(e) => e.clone(),
            None => {
                tracing::error!("write_level_0table: Options.env is None");
                return Status::invalid_argument(
                    &Slice::from_str("env"),
                    Some(&Slice::from_str("missing from Options")),
                );
            }
        };

        let start_micros: u64 = self.env.as_mut().now_micros();

        let mut meta: FileMetaData = Default::default();
        meta.set_number(unsafe { (*(self.versions as *mut VersionSet)).new_file_number() });

        self.pending_outputs.insert(*meta.number());

        let iter: *mut LevelDBIterator = unsafe { (*mem).new_iterator() };

        tracing::info!(file_number = meta.number(), "Level-0 table started");

        let mut s: Status = Status::ok();

        unsafe {
            self.mutex.unlock();
        }

        s = build_table(
            &self.dbname,
            env_rc.clone(),
            &self.options,
            self.table_cache as *mut TableCache,
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

        self.pending_outputs.remove(meta.number());

        // Note that if file_size is zero, the file has been deleted and
        // should not be added to the manifest.
        let mut level: i32 = 0;

        if s.is_ok() && *meta.file_size() > 0 {
            let min_user_key: Slice = meta.smallest().user_key();
            let max_user_key: Slice = meta.largest().user_key();

            if !base.is_null() {
                level =
                    unsafe { (*base).pick_level_for_mem_table_output(&min_user_key, &max_user_key) };
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
        stats.set_micros((self.env.as_mut().now_micros() - start_micros) as i64);
        stats.set_bytes_written(*meta.file_size() as i64);

        self.stats[level as usize].add(&stats);

        s
    }
}

#[cfg(test)]
mod write_level0_table_interface_contract_suite {
    use super::*;

    fn build_temp_db_path_for_write_level0_table_suite() -> String {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_write_level0_table_suite_{}",
                nanos
            ))
            .to_string_lossy()
            .to_string()
    }

    fn build_options_with_env_or_panic_for_write_level0_table_suite() -> Options {
        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        if options.env().is_none() {
            tracing::error!("Options::with_env(env) produced Options with env=None; cannot run write_level_0table suite");
            panic!();
        }

        options
    }

    #[traced_test]
    fn write_level_0table_signature_is_stable_for_dbimpl_public_method() {
        tracing::info!("Asserting DBImpl::write_level_0table signature is stable");

        type WriteL0Sig = fn(&mut DBImpl, *mut MemTable, *mut VersionEdit, *mut Version) -> Status;
        let _sig: WriteL0Sig = DBImpl::write_level_0table;

        tracing::debug!("DBImpl::write_level_0table signature check compiled");
    }

    #[traced_test]
    fn write_level_0table_method_item_is_addressable() {
        tracing::info!("Asserting DBImpl::write_level_0table method item is addressable");
        let _m = DBImpl::write_level_0table;
        let _ = _m;
    }

    #[traced_test]
    fn write_level_0table_returns_error_when_options_env_is_none() {
        let dbname = build_temp_db_path_for_write_level0_table_suite();
        let _ = std::fs::create_dir_all(&dbname);

        let options = build_options_with_env_or_panic_for_write_level0_table_suite();
        let mut db: DBImpl = DBImpl::new(&options, &dbname);

        db.options.set_env(None);

        let mut edit: VersionEdit = Default::default();

        db.mutex.lock();

        tracing::info!("Calling write_level_0table with Options.env=None; expecting non-OK Status without dereferencing inputs");

        let s: Status = db.write_level_0table(
            core::ptr::null_mut(),
            &mut edit as *mut VersionEdit,
            core::ptr::null_mut(),
        );

        unsafe { db.mutex.unlock() };

        tracing::debug!(status = %s.to_string(), "write_level_0table returned");
        assert!(!s.is_ok(), "write_level_0table must return non-OK when Options.env is None");

        drop(db);
        let _ = std::fs::remove_dir_all(&dbname);
    }
}
