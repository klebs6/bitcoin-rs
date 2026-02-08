// ---------------- [ File: bitcoinleveldb-dbimpl/src/cleanup_compaction.rs ]
crate::ix!();

impl DBImpl {
    #[EXCLUSIVE_LOCKS_REQUIRED(mutex)]
    pub fn cleanup_compaction(&mut self, compact: *mut CompactionState) {
        self.mutex.assert_held();

        unsafe {
            bitcoinleveldb_dbimplinner::cleanup_compaction_state_builder_and_pending_outputs_then_drop(
                compact,
                &mut self.pending_outputs,
            );
        }
    }
}

#[cfg(test)]
mod cleanup_compaction_contract_suite {
    use super::*;

    fn make_dbimpl_for_cleanup(
        pending_outputs: std::collections::HashSet<u64>,
    ) -> core::mem::ManuallyDrop<DBImpl> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|e| {
                tracing::error!(error = %format!("{:?}", e), "SystemTime before UNIX_EPOCH");
                panic!();
            })
            .as_nanos();

        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "bitcoinleveldb_dbimpl_cleanup_compaction_test_{}_{}",
            std::process::id(),
            nanos
        ));
        let dbname = path.to_string_lossy().to_string();

        tracing::info!(path = %dbname, "Allocated temp db path for cleanup_compaction tests");

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);

        let mut db: core::mem::ManuallyDrop<DBImpl> =
            core::mem::ManuallyDrop::new(DBImpl::new(&options, &dbname));

        unsafe {
            let db_mut: &mut DBImpl = &mut *(&mut db as *mut _ as *mut DBImpl);
            db_mut.pending_outputs = pending_outputs;

            tracing::debug!(
                pending_outputs_len = db_mut.pending_outputs.len() as u64,
                dbname = %db_mut.dbname,
                "Initialized DBImpl for cleanup_compaction tests"
            );
        }

        db
    }

    #[traced_test]
    fn cleanup_compaction_removes_pending_outputs_for_all_state_outputs() {
        tracing::info!("Testing cleanup_compaction removes file numbers from pending_outputs");

        let mut pending: std::collections::HashSet<u64> = Default::default();
        pending.insert(100);
        pending.insert(200);
        pending.insert(999); // should remain

        let mut db: core::mem::ManuallyDrop<DBImpl> = make_dbimpl_for_cleanup(pending);

        let env = PosixEnv::shared();
        let options: Options = Options::with_env(env);
        let compaction_ptr: *mut Compaction = Box::into_raw(Box::new(Compaction::new(
            &options as *const Options,
            1,
        )));

        let mut state: CompactionState = CompactionState::new(compaction_ptr);
        state.outputs_mut().push(CompactionStateOutputBuilder::default()
            .number(100)
            .file_size(0)
            .smallest(InternalKey::new_empty())
            .largest(InternalKey::new_empty())
            .build()
            .unwrap()
        );
        state.outputs_mut().push(CompactionStateOutputBuilder::default()
            .number(200)
            .file_size(0)
            .smallest(InternalKey::new_empty())
            .largest(InternalKey::new_empty())
            .build()
            .unwrap()
        );

        let compact_ptr: *mut CompactionState = Box::into_raw(Box::new(state));

        unsafe {
            (&mut *(&mut db as *mut _ as *mut DBImpl)).cleanup_compaction(compact_ptr);
        }

        let db_ref: &DBImpl = unsafe { &*(&db as *const _ as *const DBImpl) };

        tracing::debug!(
            pending_outputs_len = db_ref.pending_outputs.len() as u64,
            "pending_outputs after cleanup_compaction"
        );

        assert!(!db_ref.pending_outputs.contains(&100));
        assert!(!db_ref.pending_outputs.contains(&200));
        assert!(db_ref.pending_outputs.contains(&999));
    }

    #[traced_test]
    fn cleanup_compaction_exposes_expected_signature() {
        tracing::info!("Verifying DBImpl::cleanup_compaction signature is stable");
        let _f: fn(&mut DBImpl, *mut CompactionState) = DBImpl::cleanup_compaction;
        let _ = _f;
    }
}
