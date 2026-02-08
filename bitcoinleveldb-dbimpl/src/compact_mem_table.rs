// ---------------- [ File: bitcoinleveldb-dbimpl/src/compact_mem_table.rs ]
crate::ix!();

impl DBImpl {
    /// Compact the in-memory write buffer to disk.
    ///
    /// Switches to a new log-file/memtable and writes a new descriptor iff successful.
    ///
    /// Errors are recorded in bg_error.
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn compact_mem_table(&mut self) {
        self.mutex.assert_held();
        assert!(!self.imm.is_null());

        let mut edit: VersionEdit = Default::default();

        let base: *mut Version =
            unsafe { bitcoinleveldb_dbimplinner::ref_current_version_from_versionset(self.versions) };

        let mut s: Status = self.write_level_0table(self.imm, &mut edit, base);

        unsafe {
            bitcoinleveldb_dbimplinner::unref_version(base);
        }

        bitcoinleveldb_dbimplinner::override_ok_status_with_memtable_compaction_shutdown_error(
            &mut s,
            self.shutting_down.load(core::sync::atomic::Ordering::Acquire),
        );

        if s.is_ok() {
            bitcoinleveldb_dbimplinner::prepare_version_edit_for_memtable_compaction_commit(
                &mut edit,
                self.logfile_number,
            );

            s = unsafe {
                bitcoinleveldb_dbimplinner::log_and_apply_version_edit_to_versionset(
                    self.versions,
                    &mut edit,
                    core::ptr::addr_of_mut!(self.mutex),
                )
            };
        }

        if s.is_ok() {
            unsafe {
                bitcoinleveldb_dbimplinner::unref_and_clear_immutable_memtable_and_flag(
                    &mut self.imm,
                    &self.has_imm,
                );
            }

            self.delete_obsolete_files();
        } else {
            self.record_background_error(&s);
        }
    }
}

#[cfg(test)]
mod compact_mem_table_interface_and_contract_suite {
    use super::*;

    #[traced_test]
    fn compact_mem_table_signature_is_stable() {
        tracing::info!("Asserting DBImpl::compact_mem_table signature is stable");
        type Sig = fn(&mut DBImpl);
        let _sig: Sig = DBImpl::compact_mem_table;
        tracing::debug!("Signature check compiled");
    }

    #[traced_test]
    fn compact_mem_table_panics_if_imm_is_null_by_contract() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dbname = std::env::temp_dir()
            .join(format!(
                "bitcoinleveldb_dbimpl_compact_mem_table_contract_{}_{}",
                std::process::id(),
                nanos
            ))
            .to_string_lossy()
            .to_string();

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        let mut db = std::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        db.mutex.lock();

        tracing::info!(
            dbname = %dbname,
            "Invoking compact_mem_table with imm=null; expecting panic"
        );

        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            db.compact_mem_table();
        }))
        .is_err();

        tracing::debug!(panicked, "Observed compact_mem_table contract panic behavior");

        unsafe { db.mutex.unlock() };

        match std::fs::remove_dir_all(&dbname) {
            Ok(()) => {
                tracing::debug!(path = %dbname, "Removed compact_mem_table contract test directory");
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                tracing::trace!(path = %dbname, "No compact_mem_table contract test directory to remove");
            }
            Err(e) => {
                tracing::warn!(
                    path = %dbname,
                    error = %format!("{:?}", e),
                    "Failed to remove compact_mem_table contract test directory"
                );
            }
        }

        assert!(panicked, "compact_mem_table must assert that imm is non-null");
    }
}
