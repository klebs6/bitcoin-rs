// ---------------- [ File: bitcoinleveldb-versionset/src/t_temporary_database_harness.rs ]
crate::ix!();

/// Holds a stable temporary database topology for VersionSet-facing tests.
///
/// The non-negotiable invariant is that every table writer and table reader
/// created through this harness must observe the same internal-key ordering
/// that `Version` and `VersionSet` use for metadata search. This item is
/// therefore responsible for keeping the internal comparator and the internal
/// filter-policy adapter alive for the full lifetime of `table_cache` and
/// `version_set`.
pub struct VersionSetTemporaryDatabaseHarness  {
    /// Temporary filesystem root for the database under test.
    ///
    /// This path must remain valid until teardown completes.
    temporary_database_directory: PathBuf,

    /// Canonical database identity used to derive CURRENT, MANIFEST, log, and
    /// table file names.
    ///
    /// This string must remain stable for the full harness lifetime.
    database_name: String,

    /// Sanitized `Options` surface presented to table builders and readers.
    ///
    /// Its comparator and filter-policy bindings must remain in the internal-key
    /// domain for the full lifetime of `table_cache` and `version_set`.
    database_options: Box<Options>,

    /// Comparator that defines the total order over encoded internal keys.
    ///
    /// This adapter must outlive every table read and write issued by the harness.
    internal_key_comparator: Box<InternalKeyComparator>,

    /// Filter-policy adapter aligned with the internal-key domain.
    ///
    /// This adapter must outlive every table opened through `table_cache`.
    internal_filter_policy: Box<InternalFilterPolicy>,

    /// Table cache consulted by `Version::get`.
    ///
    /// It must observe the same internal-key ordering used when probe SSTables
    /// are constructed.
    table_cache: Box<TableCache>,

    /// Exclusive manifest-edit mutex used by the harness.
    ///
    /// This mutex coordinates VersionSet mutations during tests.
    version_set_mutex: Box<RawMutex>,

    /// Installed `VersionSet` under test.
    ///
    /// When present, it must only observe `database_options`, `table_cache`,
    /// and `internal_key_comparator` that satisfy the internal-key ordering
    /// invariant above.
    version_set: Option<Box<VersionSet>>,
}

impl VersionSetTemporaryDatabaseHarness {

    /// Preconditions: callers hold the version-set mutex when the upstream implementation requires it.
    /// Postconditions: the supplied edit has been applied through the live `VersionSet` and the
    /// returned status is the exact upstream `log_and_apply` result.
    pub fn apply_version_edit(
        &mut self,
        version_edit: *mut VersionEdit,
    ) -> Status {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_apply_version_edit_enter",
            database_name = %self.database_name,
            version_edit_ptr = ?version_edit
        );

        match version_edit.is_null() {
            true => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_apply_version_edit_null_edit",
                    database_name = %self.database_name
                );
                panic!("version_set_temporary_database_harness_apply_version_edit_null_edit");
            }
            false => {}
        }

        let raw_mutex_ptr = self.version_set_mutex.as_mut() as *mut RawMutex;
        let version_set = self.version_set_mut_or_panic();
        let status = version_set.log_and_apply(version_edit, raw_mutex_ptr);

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_apply_version_edit_exit",
            database_name = %self.database_name,
            status = ?status
        );

        status
    }

    /// Opens a temporary database harness with a table-facing option topology
    /// aligned to internal-key ordering.
    ///
    /// Preconditions:
    /// - `table_cache_capacity` must be strictly positive.
    ///
    /// Postconditions:
    /// - probe-built SSTables and `Version::get` consult a comparator/filter
    ///   configuration in the same internal-key domain;
    /// - the comparator and filter-policy adapters outlive all VersionSet and
    ///   TableCache resources created by this harness.
    ///
    /// Forbidden drift:
    /// - constructing the `TableCache` from raw user-key `Options`;
    /// - allowing probe SSTables to be built with a comparator different from
    ///   the one used by `Version::get` through the table cache.
    pub fn open_temporary_database_with_flags(
        test_prefix:          &str,
        create_if_missing:    bool,
        error_if_exists:      bool,
        table_cache_capacity: i32
    ) -> Self
    {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_open_enter",
            test_prefix = test_prefix,
            create_if_missing = create_if_missing,
            error_if_exists = error_if_exists,
            table_cache_capacity = table_cache_capacity
        );

        match table_cache_capacity > 0 {
            true => {}
            false => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_open_invalid_table_cache_capacity",
                    table_cache_capacity = table_cache_capacity
                );
                panic!("version_set_temporary_database_harness_open_invalid_table_cache_capacity");
            }
        }

        let temporary_database_directory =
            build_unique_temporary_database_directory_path(test_prefix);

        match std::fs::create_dir_all(&temporary_database_directory) {
            Ok(()) => {
                trace!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_open_directory_created",
                    directory = %temporary_database_directory.display()
                );
            }
            Err(error) => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_open_directory_create_error",
                    directory = %temporary_database_directory.display(),
                    error = ?error
                );
                panic!("version_set_temporary_database_harness_open_directory_create_error");
            }
        }

        let database_name = temporary_database_directory.to_string_lossy().to_string();
        let environment = PosixEnv::shared();

        let mut raw_database_options = Box::new(Options::with_env(environment));
        raw_database_options.set_create_if_missing(create_if_missing);
        raw_database_options.set_error_if_exists(error_if_exists);

        let internal_key_comparator = Box::new(
            build_internal_key_comparator_from_database_options(raw_database_options.as_ref()),
        );

        let internal_filter_policy = Box::new(
            InternalFilterPolicy::new(raw_database_options.filter_policy().as_ref()),
        );

        let database_options = Box::new(
            sanitize_options(
                &database_name,
                internal_key_comparator.as_ref(),
                internal_filter_policy.as_ref(),
                raw_database_options.as_ref(),
            ),
        );

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_open_internalized_table_topology",
            database_name = %database_name,
            options_ptr = ?(database_options.as_ref() as *const Options),
            internal_key_comparator_ptr =
                ?(internal_key_comparator.as_ref() as *const InternalKeyComparator),
            internal_filter_policy_ptr =
                ?(internal_filter_policy.as_ref() as *const InternalFilterPolicy)
        );

        let mut table_cache = Box::new(TableCache::new(
            &database_name,
            database_options.as_ref(),
            table_cache_capacity,
        ));

        let version_set_mutex = Box::new(RawMutex::INIT);

        let version_set = Some(VersionSet::new(
            &database_name,
            database_options.as_ref(),
            table_cache.as_mut() as *mut TableCache,
            internal_key_comparator.as_ref() as *const InternalKeyComparator,
        ));

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_open_exit",
            database_name = %database_name,
            directory = %temporary_database_directory.display(),
            options_ptr = ?(database_options.as_ref() as *const Options),
            table_cache_ptr = ?(table_cache.as_ref() as *const TableCache)
        );

        Self {
            temporary_database_directory,
            database_name,
            database_options,
            internal_key_comparator,
            internal_filter_policy,
            table_cache,
            version_set_mutex,
            version_set,
        }
    }

    /// Postconditions: identical to `open_temporary_database_with_flags(test_prefix, true, false, 64)`.
    pub fn open_default_temporary_database(test_prefix: &str) -> Self {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_open_default_enter",
            test_prefix = test_prefix
        );

        let harness = Self::open_temporary_database_with_flags(
            test_prefix,
            true,
            false,
            64,
        );

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_open_default_exit",
            test_prefix = test_prefix
        );

        harness
    }

    fn version_set_mut_or_panic(&mut self) -> &mut VersionSet {
        match self.version_set.as_mut() {
            Some(version_set_box) => version_set_box.as_mut(),
            None => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_version_set_missing",
                    database_name = %self.database_name
                );
                panic!("version_set_temporary_database_harness_version_set_missing");
            }
        }
    }

    fn current_version_ptr_or_panic(&mut self) -> *mut Version {
        let current_version_ptr = self.version_set_mut_or_panic().current();

        match current_version_ptr.is_null() {
            true => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_current_version_null",
                    database_name = %self.database_name
                );
                panic!("version_set_temporary_database_harness_current_version_null");
            }
            false => current_version_ptr,
        }
    }

    /// Postconditions: the version-set performs its normal recover path and the returned tuple
    /// preserves both the upstream `Status` and `save_manifest` decision exactly.
    pub fn recover_into_current_version_set(&mut self) -> (Status, bool) {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_recover_enter",
            database_name = %self.database_name
        );

        let version_set = self.version_set_mut_or_panic();
        let mut save_manifest = false;
        let status = version_set.recover(&mut save_manifest as *mut bool);

        info!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_recover_exit",
            database_name = %self.database_name,
            save_manifest = save_manifest,
            status = ?status
        );

        (status, save_manifest)
    }

    /// Postconditions: the returned guard owns the mutex acquire_from_raw_mutex until drop; callers must not attempt
    /// re-entrant acquisition through the same raw mutex while the guard is live.
    pub fn acquire_version_set_mutex(&mut self) -> RawMutexExclusiveTestGuard {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_acquire_mutex_enter",
            database_name = %self.database_name
        );

        let guard = RawMutexExclusiveTestGuard::acquire_from_raw_mutex(
            self.version_set_mutex.as_mut() as *mut RawMutex,
        );

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_acquire_mutex_exit",
            database_name = %self.database_name
        );

        guard
    }

    /// Postconditions: the live `VersionSet` instance is dropped and subsequent operations that
    /// require it will terminate until a new instance is explicitly installed.
    pub fn drop_version_set_instance(&mut self) {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_drop_version_set_enter",
            database_name = %self.database_name,
            version_set_present = self.version_set.is_some()
        );

        let _ = self.version_set.take();

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_drop_version_set_exit",
            database_name = %self.database_name,
            version_set_present = self.version_set.is_some()
        );
    }

    /// Preconditions: callers hold the version-set mutex when the upstream implementation requires it.
    /// Postconditions: the new file is committed through `log_and_apply` and its file number is returned.
    pub fn add_level_file(
        &mut self,
        level: i32,
        file_size: u64,
        smallest_user_key: &str,
        smallest_sequence_number: u64,
        largest_user_key: &str,
        largest_sequence_number: u64,
    ) -> u64 {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_add_level_file_enter",
            database_name = %self.database_name,
            level = level,
            file_size = file_size,
            smallest_user_key = smallest_user_key,
            smallest_sequence_number = smallest_sequence_number,
            largest_user_key = largest_user_key,
            largest_sequence_number = largest_sequence_number
        );

        let raw_mutex_ptr = self.version_set_mutex.as_mut() as *mut RawMutex;
        let version_set = self.version_set_mut_or_panic();
        let mut version_edit = VersionEdit::default();
        let file_number = version_set.new_file_number();

        version_edit.add_file(
            level,
            file_number,
            file_size,
            &make_value_internal_key_for_user_key(
                smallest_user_key,
                smallest_sequence_number,
            ),
            &make_value_internal_key_for_user_key(
                largest_user_key,
                largest_sequence_number,
            ),
        );

        let status = version_set.log_and_apply(
            &mut version_edit as *mut VersionEdit,
            raw_mutex_ptr,
        );

        match status.is_ok() {
            true => {
                trace!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_add_level_file_exit",
                    database_name = %self.database_name,
                    level = level,
                    file_number = file_number
                );
                file_number
            }
            false => {
                error!(
                    target: "bitcoinleveldbt_versionsettestutil::harness",
                    event = "version_set_temporary_database_harness_add_level_file_non_ok_status",
                    database_name = %self.database_name,
                    level = level,
                    file_number = file_number,
                    status = ?status
                );
                panic!("version_set_temporary_database_harness_add_level_file_non_ok_status");
            }
        }
    }

    /// Postconditions: the returned vector preserves the exact current-level file order stored in
    /// the active version.
    pub fn current_level_file_metadata_pointers(
        &mut self,
        level: usize,
    ) -> Vec<*mut FileMetaData> {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_current_level_file_metadata_pointers_enter",
            database_name = %self.database_name,
            level = level
        );

        let current_version_ptr = self.current_version_ptr_or_panic();
        let file_metadata_pointers =
            unsafe { (*current_version_ptr).files()[level].clone() };

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_current_level_file_metadata_pointers_exit",
            database_name = %self.database_name,
            level = level,
            count = file_metadata_pointers.len()
        );

        file_metadata_pointers
    }

    /// Postconditions: the returned pointer is either a member of the current level vector or null.
    pub fn find_level_file_metadata_pointer_by_number(
        &mut self,
        level: usize,
        file_number: u64,
    ) -> *mut FileMetaData {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_find_level_file_metadata_pointer_by_number_enter",
            database_name = %self.database_name,
            level = level,
            file_number = file_number
        );

        let file_metadata_pointers =
            self.current_level_file_metadata_pointers(level);

        let file_metadata_ptr = find_file_metadata_pointer_by_number_in_vector(
            &file_metadata_pointers,
            file_number,
        );

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_find_level_file_metadata_pointer_by_number_exit",
            database_name = %self.database_name,
            level = level,
            file_number = file_number,
            found = !file_metadata_ptr.is_null()
        );

        file_metadata_ptr
    }

    /// Preconditions: every file number in `input_file_numbers_at_level` refers to the current
    /// version at `level`.
    /// Postconditions: the returned compaction owns a reference to the current version and its
    /// first input vector contains the requested files in the provided order.
    pub fn seed_compaction_from_current_version(
        &mut self,
        level: i32,
        input_file_numbers_at_level: &[u64],
    ) -> Box<Compaction> {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_seed_compaction_from_current_version_enter",
            database_name = %self.database_name,
            level = level,
            input_file_count = input_file_numbers_at_level.len()
        );

        let mut compaction = Box::new(Compaction::new(
            self.database_options.as_ref() as *const Options,
            level,
        ));

        let current_version_ptr = self.current_version_ptr_or_panic();
        unsafe { (*current_version_ptr).ref_() };
        compaction.set_input_version(current_version_ptr);

        for &file_number in input_file_numbers_at_level.iter() {
            let file_metadata_ptr = self.find_level_file_metadata_pointer_by_number(
                level as usize,
                file_number,
            );

            match file_metadata_ptr.is_null() {
                true => {
                    error!(
                        target: "bitcoinleveldbt_versionsettestutil::harness",
                        event = "version_set_temporary_database_harness_seed_compaction_from_current_version_missing_file",
                        database_name = %self.database_name,
                        level = level,
                        file_number = file_number
                    );
                    panic!("version_set_temporary_database_harness_seed_compaction_from_current_version_missing_file");
                }
                false => {
                    compaction.inputs_mut()[0].push(file_metadata_ptr);
                }
            }
        }

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_seed_compaction_from_current_version_exit",
            database_name = %self.database_name,
            level = level,
            input_file_count = compaction.inputs()[0].len()
        );

        compaction
    }

    /// Guarantees the returned path is the exact directory created for this harness instance.
    pub fn database_directory_path(&self) -> &Path {
        self.temporary_database_directory.as_path()
    }

    /// Guarantees the returned string remains stable for the lifetime of the harness.
    pub fn database_name(&self) -> &str {
        self.database_name.as_str()
    }

    /// Guarantees callers observe the same mutable options object used when the harness was opened.
    pub fn database_options_mut(&mut self) -> &mut Options {
        self.database_options.as_mut()
    }

    /// Guarantees callers observe the current immutable options object without copying it.
    pub fn database_options(&self) -> &Options {
        self.database_options.as_ref()
    }

    /// Guarantees the returned mutable reference is the exact live `VersionSet` instance owned by
    /// this harness and never an implicit replacement.
    pub fn version_set_mut(&mut self) -> &mut VersionSet {

        let db_name = self.database_name.clone();

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_version_set_mut_enter",
            database_name = %db_name
        );

        let version_set = self.version_set_mut_or_panic();

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_version_set_mut_exit",
            database_name = %db_name,
            version_set_ptr = ?(version_set as *mut VersionSet)
        );

        version_set
    }

    /// Guarantees the returned pointer equals `current()` for the live `VersionSet` instance and is
    /// non-null on the success path.
    pub fn current_version_ptr(&mut self) -> *mut Version {
        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_current_version_ptr_enter",
            database_name = %self.database_name
        );

        let current_version_ptr = self.current_version_ptr_or_panic();

        trace!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_current_version_ptr_exit",
            database_name = %self.database_name,
            current_version_ptr = ?current_version_ptr
        );

        current_version_ptr
    }
}

impl Drop for VersionSetTemporaryDatabaseHarness {

    fn drop(&mut self) {
        debug!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_drop_enter",
            database_name = %self.database_name,
            directory = %self.temporary_database_directory.display(),
            version_set_present = self.version_set.is_some(),
            options_ptr = ?(self.database_options.as_ref() as *const Options),
            table_cache_ptr = ?(self.table_cache.as_ref() as *const TableCache),
            internal_key_comparator_ptr =
                ?(self.internal_key_comparator.as_ref() as *const InternalKeyComparator),
            internal_filter_policy_ptr =
                ?(self.internal_filter_policy.as_ref() as *const InternalFilterPolicy)
        );

        let _ = self.version_set.take();
        remove_directory_tree_best_effort(self.temporary_database_directory.as_path());

        debug!(
            target: "bitcoinleveldbt_versionsettestutil::harness",
            event = "version_set_temporary_database_harness_drop_exit",
            database_name = %self.database_name,
            directory = %self.temporary_database_directory.display(),
            version_set_present = self.version_set.is_some()
        );
    }
}
