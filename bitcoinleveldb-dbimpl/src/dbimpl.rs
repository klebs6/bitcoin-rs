// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_impl.cc]
pub struct DBImpl {

    /// Constant after construction
    /// 
    pub env:                    Box<dyn Env>,

    pub internal_comparator:    InternalKeyComparator,
    pub internal_filter_policy: InternalFilterPolicy,

    /// options.comparator == &internal_comparator
    /// 
    pub options:                Options,

    pub owns_info_log:          bool,
    pub owns_cache:             bool,
    pub dbname:                 String,

    /// table_cache provides its own synchronization
    /// 
    pub table_cache:            *const TableCache,

    /// Lock over the persistent DB state.
    /// 
    /// Non-null iff successfully acquired.
    /// 
    pub db_lock:                *mut Box<dyn FileLock>,

    /// State below is protected by mutex
    pub mutex:                  RawMutex,

    //--------------------------------------------[mutex-guarded-fields]

    /// Dedicated mutex used only to provide a `MutexGuard` for `Condvar::wait()`,
    /// since the DB state lock is a `RawMutex`.
    ///
    /// Lock-ordering rule:
    /// - always acquire `mutex` before acquiring this mutex
    /// - never acquire `mutex` while holding this mutex
    pub background_work_finished_mutex: Mutex<()>,

    pub background_work_finished_signal: Condvar,

    /// Memtable being compacted
    /// 
    pub imm: *mut MemTable,

    pub logfile_number: u64,

    /// For sampling.
    /// 
    pub seed: u32,

    /// Queue of writers.
    /// 
    pub writers: VecDeque<*mut DBImplWriter>,
    pub tmp_batch: *mut WriteBatch,
    pub snapshots: SnapshotList,

    /// Set of table files to protect from deletion
    /// because they are part of ongoing compactions.
    /// 
    pub pending_outputs: HashSet<u64>,

    /// Has a background compaction been scheduled
    /// or is running?
    /// 
    pub background_compaction_scheduled: bool,
    pub manual_compaction: *mut ManualCompaction,
    pub versions: *mut VersionSet,

    /// Have we encountered a background error
    /// in paranoid mode?
    /// 
    pub bg_error: Status,
    pub stats: [CompactionStats; NUM_LEVELS],

    //--------------------------------------------[marks-end-of-mutex-guarded-fields]

    pub shutting_down:          AtomicBool,

    pub mem:                    *mut MemTable,

    /// So bg thread can detect non-null imm
    /// 
    pub has_imm:                AtomicBool,

    pub logfile:                Rc<RefCell<dyn WritableFile>>,
    pub log:                    *mut LogWriter,

}

impl DB for DBImpl { }

impl DBImpl {
    pub fn clear_background_error_for_test(&mut self) {
        self.bg_error = Status::default();
    }
}

impl DBImpl {

    /// Guarantees the returned identifier is the exact next `VersionSet` file number
    /// for this database instance at the moment of observation.
    ///
    /// Postcondition:
    /// - the underlying `VersionSet` monotone allocation counter has advanced once
    /// - no manifest edit has been applied by this method
    #[cfg(test)]
    pub fn test_allocate_versionset_file_number_for_live_compaction_boundary_harness(&mut self) -> u64 {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_allocate_versionset_file_number_for_live_compaction_boundary_harness.entry",
            versions_ptr = self.versions as usize
        );

        assert!(!self.versions.is_null());

        let file_number: u64 = unsafe { (*self.versions).new_file_number() };

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_allocate_versionset_file_number_for_live_compaction_boundary_harness.exit",
            versions_ptr = self.versions as usize,
            file_number
        );

        file_number
    }

    /// Guarantees the supplied edit is committed through the real
    /// `VersionSet::log_and_apply` manifest path while respecting the DB mutex
    /// discipline expected by the storage engine.
    ///
    /// Postcondition:
    /// - the mutex is released on return
    /// - the returned status is the exact manifest-commit outcome
    #[cfg(test)]
    pub fn test_apply_versionset_edit_for_live_compaction_boundary_harness(
        &mut self,
        edit: *mut VersionEdit
    ) -> Status {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_apply_versionset_edit_for_live_compaction_boundary_harness.entry",
            versions_ptr = self.versions as usize,
            edit_ptr = edit as usize
        );

        assert!(!self.versions.is_null());
        assert!(!edit.is_null());

        self.mutex.lock();

        let status: Status = unsafe {
            (*self.versions).log_and_apply(edit, core::ptr::addr_of_mut!(self.mutex))
        };

        unsafe {
            self.mutex.unlock();
        }

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_apply_versionset_edit_for_live_compaction_boundary_harness.exit",
            versions_ptr = self.versions as usize,
            edit_ptr = edit as usize,
            status_ok = status.is_ok(),
            status = %status.to_string()
        );

        status
    }

    /// Guarantees the returned vector is a snapshot clone of the current level file
    /// ordering at the moment of observation.
    ///
    /// Postcondition:
    /// - ownership of the pointed-to metadata remains with the installed versions
    /// - pointer order matches the current version’s level ordering exactly
    #[cfg(test)]
    pub fn test_collect_current_level_file_metadata_pointers_for_live_compaction_boundary_harness(
        &mut self,
        level: usize
    ) -> Vec<*mut FileMetaData> {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_collect_current_level_file_metadata_pointers_for_live_compaction_boundary_harness.entry",
            versions_ptr = self.versions as usize,
            level
        );

        assert!(level < NUM_LEVELS);
        assert!(!self.versions.is_null());

        self.mutex.lock();

        let current_ptr: *mut Version = unsafe { (*self.versions).current() };

        assert!(!current_ptr.is_null());

        let file_metadata_pointers: Vec<*mut FileMetaData> =
            unsafe { (*current_ptr).files()[level].clone() };

        unsafe {
            self.mutex.unlock();
        }

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_collect_current_level_file_metadata_pointers_for_live_compaction_boundary_harness.exit",
            versions_ptr = self.versions as usize,
            level,
            file_count = file_metadata_pointers.len()
        );

        file_metadata_pointers
    }

    /// Guarantees the returned status is a stable clone of the database background
    /// error state at the time of observation.
    ///
    /// Postcondition:
    /// - the mutex is released on return
    /// - no background-error state is mutated by this method
    #[cfg(test)]
    pub fn test_read_background_error_status_for_live_compaction_boundary_harness(
        &mut self
    ) -> Status {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_read_background_error_status_for_live_compaction_boundary_harness.entry",
            dbname = %self.dbname
        );

        self.mutex.lock();

        let status: Status = self.bg_error.clone();

        unsafe {
            self.mutex.unlock();
        }

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_read_background_error_status_for_live_compaction_boundary_harness.exit",
            dbname = %self.dbname,
            status_ok = status.is_ok(),
            status = %status.to_string()
        );

        status
    }

    /// Guarantees the returned database name is the stable path prefix used for
    /// SST, MANIFEST, and CURRENT file generation.
    #[cfg(test)]
    pub fn test_database_name_for_live_compaction_boundary_harness(&self) -> &String {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_database_name_for_live_compaction_boundary_harness.entry",
            dbname = %self.dbname
        );

        &self.dbname
    }

    /// Guarantees the returned options reference is comparator-compatible with the
    /// live database instance and may be reused for direct SST construction in tests.
    #[cfg(test)]
    pub fn test_options_for_live_compaction_boundary_harness(&self) -> &Options {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_options_for_live_compaction_boundary_harness.entry",
            dbname = %self.dbname
        );

        &self.options
    }

    #[cfg(test)]
    pub fn test_run_versionset_compaction_inline_for_live_compaction_boundary_harness(
        &mut self,
        level: i32,
        begin_user_key: &str,
        end_user_key: &str
    ) -> Status {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_run_versionset_compaction_inline_for_live_compaction_boundary_harness.entry",
            dbname = %self.dbname,
            level,
            begin_user_key = begin_user_key,
            end_user_key = end_user_key,
            versions_ptr = self.versions as usize
        );

        assert!(level >= 0);
        assert!(level + 1 < NUM_LEVELS as i32);
        assert!(!self.versions.is_null());

        let begin_user_key_slice = Slice::from_str(begin_user_key);
        let end_user_key_slice = Slice::from_str(end_user_key);

        let begin_internal_key = InternalKey::new(
            &begin_user_key_slice,
            MAX_SEQUENCE_NUMBER,
            VALUE_TYPE_FOR_SEEK,
        );

        let end_internal_key = InternalKey::new(
            &end_user_key_slice,
            0,
            ValueType::TypeDeletion,
        );

        self.mutex.lock();

        let compaction_ptr: *mut Compaction = unsafe {
            (*self.versions).compact_range(
                level,
                &begin_internal_key as *const InternalKey,
                &end_internal_key as *const InternalKey,
            )
        };

        if compaction_ptr.is_null() {
            unsafe {
                self.mutex.unlock();
            }

            debug!(
                target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
                label = "dbimpl.test_run_versionset_compaction_inline_for_live_compaction_boundary_harness.no_compaction_selected",
                dbname = %self.dbname,
                level,
                begin_user_key = begin_user_key,
                end_user_key = end_user_key,
                versions_ptr = self.versions as usize
            );

            return Status::ok();
        }

        let compaction_state_ptr: *mut CompactionState =
            bitcoinleveldb_dbimplinner::allocate_compaction_state_for_compaction(compaction_ptr);

        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_run_versionset_compaction_inline_for_live_compaction_boundary_harness.selected_compaction",
            dbname = %self.dbname,
            level,
            compaction_ptr = compaction_ptr as usize,
            compaction_state_ptr = compaction_state_ptr as usize
        );

        let compaction_status: Status = self.do_compaction_work(compaction_state_ptr);

        self.cleanup_compaction(compaction_state_ptr);

        unsafe {
            bitcoinleveldb_dbimplinner::release_compaction_inputs(compaction_ptr);
            bitcoinleveldb_dbimplinner::drop_boxed_compaction_if_non_null(compaction_ptr);
            self.mutex.unlock();
        }

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_run_versionset_compaction_inline_for_live_compaction_boundary_harness.exit",
            dbname = %self.dbname,
            level,
            begin_user_key = begin_user_key,
            end_user_key = end_user_key,
            status_ok = compaction_status.is_ok(),
            status = %compaction_status.to_string()
        );

        compaction_status
    }

    /// Preconditions: the caller has exclusive mutable access to this `DBImpl`
    /// and is not intentionally holding `mutex`.
    ///
    /// Postconditions: returns `true` only if this probe reacquired and then
    /// released `mutex` on the current thread.
    ///
    /// Invariant: this probe must not mutate durable database state. It may only
    /// observe lock availability and restore the lock state when reacquisition
    /// succeeds.
    pub fn test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness(
        &mut self,
        phase_label: &'static str,
    ) -> bool {
        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness.entry",
            phase = phase_label,
            dbname = %self.dbname,
            scheduled = self.background_compaction_scheduled,
            bg_error_ok = self.bg_error.is_ok(),
            shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
        );

        let reacquired: bool = self.mutex.try_lock();

        debug!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness.decision",
            phase = phase_label,
            dbname = %self.dbname,
            reacquired,
            scheduled = self.background_compaction_scheduled,
            bg_error_ok = self.bg_error.is_ok(),
            shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
        );

        match reacquired {
            true => {
                unsafe {
                    self.mutex.unlock();
                }

                trace!(
                    target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
                    label = "dbimpl.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness.state_transition",
                    phase = phase_label,
                    dbname = %self.dbname,
                    action = "unlock_after_probe_completed",
                );
            }
            false => {
                warn!(
                    target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
                    label = "dbimpl.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness.state_transition",
                    phase = phase_label,
                    dbname = %self.dbname,
                    reacquired,
                    scheduled = self.background_compaction_scheduled,
                    bg_error_ok = self.bg_error.is_ok(),
                    shutting_down = self.shutting_down.load(atomic::Ordering::Acquire),
                    action = "lock_not_reacquired",
                );
            }
        }

        trace!(
            target: "bitcoinleveldb_dbimpl::test_live_compaction_boundary_access",
            label = "dbimpl.test_probe_db_mutex_reacquire_for_live_compaction_boundary_harness.exit",
            phase = phase_label,
            dbname = %self.dbname,
            reacquired,
        );

        reacquired
    }
}
