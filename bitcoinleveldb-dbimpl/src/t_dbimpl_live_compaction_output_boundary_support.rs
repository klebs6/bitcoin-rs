crate::ix!();

/// Opens one fully initialized live `DBImpl` through the real `DBOpen` path and
/// returns owned concrete storage for the harness.
///
/// Postcondition:
/// - the returned database has completed recover/open bootstrap
/// - `VersionSet` manifest state is initialized
/// - subsequent `VersionSet::log_and_apply(...)` calls append to a valid manifest
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_open_database_instance_via_dbopen(
    temporary_database_directory: &String,
) -> Box<DBImpl> {
    trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_open_database_instance_via_dbopen.entry",
        temporary_database_directory = %temporary_database_directory
    );

    let environment = PosixEnv::shared();

    let mut database_options = Options::with_env(environment.clone());
    database_options.set_create_if_missing(true);
    database_options.set_error_if_exists(false);

    let mut dispatcher = DBImpl::new(
        &database_options,
        temporary_database_directory,
    );

    let mut opened_database_ptr: *mut dyn DB =
        core::ptr::null_mut::<DBImpl>() as *mut dyn DB;

    let open_status: Status = <DBImpl as DBOpen>::open(
        &mut dispatcher,
        &database_options,
        temporary_database_directory,
        &mut opened_database_ptr as *mut *mut dyn DB,
    );

    assert!(open_status.is_ok());
    assert!(!opened_database_ptr.is_null());

    let opened_database_impl_ptr: *mut DBImpl = {
        let opened_database_data_ptr: *mut () = opened_database_ptr as *mut ();
        opened_database_data_ptr as *mut DBImpl
    };

    assert!(!opened_database_impl_ptr.is_null());

    let database_instance: Box<DBImpl> = unsafe {
        Box::from_raw(opened_database_impl_ptr)
    };

    debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_open_database_instance_via_dbopen.exit",
        temporary_database_directory = %temporary_database_directory,
        dbimpl_ptr = opened_database_impl_ptr as usize,
        status_ok = open_status.is_ok(),
        status = %open_status.to_string()
    );

    database_instance
}

/// Classifies the exact table entry semantics installed by the live compaction
/// boundary harness.
///
/// Invariant:
/// - `Value` contributes a visible point entry at its sequence number
/// - `Tombstone` contributes a deletion marker with an empty payload
#[derive(Clone,Debug)]
pub enum BitcoinLevelDbDbImplLiveCompactionEntryKind {
    /// Materialized point entry bytes.
    Value {
        /// Visible payload bytes stored in the generated SST entry.
        materialized_value: String,
    },
    /// Deletion marker with no payload bytes.
    Tombstone,
}

/// Records one observed output-file boundary after live compaction.
///
/// Invariant:
/// - `previous_largest_*` describes the terminal key of the left file
/// - `next_smallest_*` describes the initial key of the right file
/// - `same_user_key_boundary=true` is the forbidden split condition for H02
#[derive(Builder,Getters,Debug)]
#[getset(get="pub")]
#[builder(pattern="owned")]
pub struct BitcoinLevelDbDbImplLiveCompactionOutputBoundaryObservation {
    /// Stable identifier of the file on the left side of the boundary.
    previous_file_number: u64,
    /// Stable identifier of the file on the right side of the boundary.
    next_file_number: u64,
    /// User key at the terminal key of the left file.
    previous_largest_user_key: String,
    /// Sequence number at the terminal key of the left file.
    previous_largest_sequence_number: u64,
    /// User key at the initial key of the right file.
    next_smallest_user_key: String,
    /// Sequence number at the initial key of the right file.
    next_smallest_sequence_number: u64,
    /// True exactly when the output boundary bisects one user-key family.
    same_user_key_boundary: bool,
}

/// Owns one temporary database instance used to drive live-compaction boundary
/// specifications.
///
/// Invariant:
/// - `temporary_database_directory` is created before the database instance is built
/// - `database_instance` remains comparator-compatible with every SST constructed
///   through this harness
///
/// This harness intentionally does not implement `Drop`. The database instance
/// itself owns the runtime resources that must be released, and the temporary
/// directory is treated as a best-effort test artifact rather than a semantic
/// ownership obligation of the harness surface.
#[derive(Builder,Getters,MutGetters)]
#[getset(get="pub", get_mut="pub")]
#[builder(pattern="owned")]
pub struct BitcoinLevelDbDbImplLiveCompactionBoundaryHarness {
    /// Filesystem directory used as the DB path prefix for this harness run.
    temporary_database_directory: String,
    /// Live database instance under test.
    database_instance: Box<DBImpl>,
}

/// Builds one stable temporary database directory path for a live-compaction
/// boundary scenario.
///
/// Postcondition:
/// - the returned path is suitable for `Env::create_dir`
/// - the path is derived from the supplied prefix and one time-based uniqueness token
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_build_temporary_database_directory_path(
    test_prefix: &str
) -> String {
    trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_build_temporary_database_directory_path.entry",
        test_prefix = test_prefix
    );

    let environment = PosixEnv::shared();
    let uniqueness_token: u64 = environment.borrow_mut().now_micros();

    let directory_path = format!(
        "/tmp/{}_{}",
        test_prefix,
        uniqueness_token,
    );

    debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_build_temporary_database_directory_path.exit",
        test_prefix = test_prefix,
        directory_path = %directory_path
    );

    directory_path
}

/// Decodes the user key and sequence number from one internal key for boundary
/// observation.
///
/// Postcondition:
/// - the returned pair is the exact parsed view of the supplied key
/// - parse failure is treated as a violated test precondition
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_decode_user_key_and_sequence(
    internal_key: &InternalKey
) -> (String,u64) {
    trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_decode_user_key_and_sequence.entry",
        encoded_len = internal_key.encode().size()
    );

    let encoded_key: Slice = internal_key.encode();
    let mut parsed_internal_key = ParsedInternalKey::default();

    let parsed_ok: bool =
        parse_internal_key(&encoded_key, &mut parsed_internal_key as *mut ParsedInternalKey);

    assert!(parsed_ok);

    let user_key_string: String = parsed_internal_key.user_key().to_string();
    let sequence_number: u64 = *parsed_internal_key.sequence();

    debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_decode_user_key_and_sequence.exit",
        user_key = %user_key_string,
        sequence_number
    );

    (user_key_string, sequence_number)
}

/// Builds the exact internal key used for one single-entry SST installed by the
/// live-compaction boundary harness.
///
/// Postcondition:
/// - `Value` entries use `TypeValue`
/// - `Tombstone` entries use `TypeDeletion`
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_build_internal_key_for_entry(
    user_key: &str,
    sequence_number: u64,
    entry_kind: &BitcoinLevelDbDbImplLiveCompactionEntryKind
) -> InternalKey {
    trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_build_internal_key_for_entry.entry",
        user_key = user_key,
        sequence_number,
        entry_kind = ?entry_kind
    );

    let user_key_slice = Slice::from_str(user_key);

    let value_type: ValueType = match entry_kind {
        BitcoinLevelDbDbImplLiveCompactionEntryKind::Value { materialized_value: _ } => {
            ValueType::TypeValue
        }
        BitcoinLevelDbDbImplLiveCompactionEntryKind::Tombstone => {
            ValueType::TypeDeletion
        }
    };

    let internal_key = InternalKey::new(
        &user_key_slice,
        sequence_number,
        value_type,
    );

    debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_build_internal_key_for_entry.exit",
        user_key = user_key,
        sequence_number,
        value_type = ?value_type
    );

    internal_key
}

/// Writes one physical single-entry table file for the live-compaction boundary
/// harness and returns its actual file size.
///
/// Invariant:
/// - the written SST contains exactly one internal entry
/// - the physical table bytes are built with the same comparator and filter policy
///   surface as the live DB instance
pub fn bitcoinleveldb_dbimpl_live_compaction_boundary_write_single_entry_table_file(
    database_name: &String,
    options: &Options,
    file_number: u64,
    internal_key: &InternalKey,
    entry_kind: &BitcoinLevelDbDbImplLiveCompactionEntryKind
) -> u64 {
    trace!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_write_single_entry_table_file.entry",
        database_name = %database_name,
        file_number,
        entry_kind = ?entry_kind
    );

    let table_file_path = table_file_name(database_name, file_number);

    let environment = match options.env().as_ref() {
        Some(environment_handle) => {
            environment_handle.clone()
        }
        None => {
            error!(
                target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
                label = "dbimpl.live_compaction_boundary_write_single_entry_table_file.missing_env",
                database_name = %database_name,
                file_number
            );
            panic!();
        }
    };

    let mut writable_file_holder_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

    let open_status: Status = environment.borrow_mut().new_writable_file(
        &table_file_path,
        &mut writable_file_holder_ptr,
    );

    assert!(open_status.is_ok());
    assert!(!writable_file_holder_ptr.is_null());

    let mut writable_file_box: Box<dyn WritableFile> = unsafe {
        *Box::from_raw(writable_file_holder_ptr)
    };

    let writable_file_ptr: *mut dyn WritableFile = &mut *writable_file_box;

    let mut table_builder = TableBuilder::new(options, writable_file_ptr);

    let internal_key_slice: Slice = internal_key.encode();

    match entry_kind {
        BitcoinLevelDbDbImplLiveCompactionEntryKind::Value { materialized_value } => {
            let value_slice = Slice::from(materialized_value.as_bytes());
            table_builder.add(&internal_key_slice, &value_slice);
        }
        BitcoinLevelDbDbImplLiveCompactionEntryKind::Tombstone => {
            let tombstone_value = Slice::default();
            table_builder.add(&internal_key_slice, &tombstone_value);
        }
    }

    let finish_status: Status = table_builder.finish();
    let actual_file_size: u64 = table_builder.file_size();

    drop(table_builder);

    assert!(finish_status.is_ok());

    let sync_status: Status = writable_file_box.sync();
    assert!(sync_status.is_ok());

    let close_status: Status = writable_file_box.close();
    assert!(close_status.is_ok());

    debug!(
        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
        label = "dbimpl.live_compaction_boundary_write_single_entry_table_file.exit",
        database_name = %database_name,
        file_number,
        actual_file_size
    );

    actual_file_size
}

impl BitcoinLevelDbDbImplLiveCompactionBoundaryHarness {

    /// Opens one temporary live database harness whose `VersionSet` can be driven
    /// through real compaction code paths.
    ///
    /// Postcondition:
    /// - the backing directory exists
    /// - the returned harness owns a fully opened `DBImpl`
    pub fn open_for_test_prefix(test_prefix: &str) -> Self {
        trace!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_open_for_test_prefix.entry",
            test_prefix = test_prefix
        );

        let temporary_database_directory =
            bitcoinleveldb_dbimpl_live_compaction_boundary_build_temporary_database_directory_path(
                test_prefix,
            );

        let environment = PosixEnv::shared();
        let create_directory_status: Status =
            environment.borrow_mut().create_dir(&temporary_database_directory);

        assert!(create_directory_status.is_ok());

        let database_instance =
            bitcoinleveldb_dbimpl_live_compaction_boundary_open_database_instance_via_dbopen(
                &temporary_database_directory,
            );

        let harness_result = BitcoinLevelDbDbImplLiveCompactionBoundaryHarnessBuilder::default()
            .temporary_database_directory(temporary_database_directory.clone())
            .database_instance(database_instance)
            .build();

        let harness = match harness_result {
            Ok(harness_value) => {
                harness_value
            }
            Err(builder_error) => {
                error!(
                    target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
                    label = "dbimpl.live_compaction_boundary_harness_open_for_test_prefix.builder_error",
                    test_prefix = test_prefix,
                    temporary_database_directory = %temporary_database_directory,
                    builder_error = ?builder_error
                );
                panic!();
            }
        };

        debug!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_open_for_test_prefix.exit",
            test_prefix = test_prefix,
            temporary_database_directory = %temporary_database_directory
        );

        harness
    }


    /// Installs one physical single-entry table file into the requested level
    /// through the real manifest-application path.
    ///
    /// Postcondition:
    /// - the returned file number is present in the current version at `level`
    /// - `reported_file_size_override`, when supplied, is the exact metadata size
    ///   recorded in the manifest edit
    pub fn install_single_entry_table_file_at_level_for_live_compaction_boundary(
        &mut self,
        level: i32,
        reported_file_size_override: Option<u64>,
        user_key: &str,
        sequence_number: u64,
        entry_kind: BitcoinLevelDbDbImplLiveCompactionEntryKind
    ) -> u64 {
        trace!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_install_single_entry_table_file_at_level.entry",
            level,
            user_key = user_key,
            sequence_number,
            entry_kind = ?entry_kind,
            reported_file_size_override = ?reported_file_size_override
        );

        let database_instance: &mut DBImpl = self.database_instance_mut().as_mut();

        let file_number: u64 =
            database_instance.test_allocate_versionset_file_number_for_live_compaction_boundary_harness();

        let internal_key =
            bitcoinleveldb_dbimpl_live_compaction_boundary_build_internal_key_for_entry(
                user_key,
                sequence_number,
                &entry_kind,
            );

        let actual_file_size =
            bitcoinleveldb_dbimpl_live_compaction_boundary_write_single_entry_table_file(
                database_instance.test_database_name_for_live_compaction_boundary_harness(),
                database_instance.test_options_for_live_compaction_boundary_harness(),
                file_number,
                &internal_key,
                &entry_kind,
            );

        let reported_file_size: u64 = match reported_file_size_override {
            Some(overridden_file_size) => {
                overridden_file_size
            }
            None => {
                actual_file_size
            }
        };

        let mut version_edit = VersionEdit::default();
        version_edit.add_file(
            level,
            file_number,
            reported_file_size,
            &internal_key,
            &internal_key,
        );

        let apply_status =
            database_instance.test_apply_versionset_edit_for_live_compaction_boundary_harness(
                &mut version_edit as *mut VersionEdit
            );

        assert!(apply_status.is_ok());

        debug!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_install_single_entry_table_file_at_level.exit",
            level,
            user_key = user_key,
            sequence_number,
            file_number,
            actual_file_size,
            reported_file_size
        );

        file_number
    }

    /// Runs one real manual compaction over the supplied inclusive user-key range.
    ///
    /// Postcondition:
    /// - the database background-error status returned by this method is the
    ///   post-compaction status observed after the manual request has drained
    pub fn run_manual_compaction_over_user_key_range_for_live_compaction_boundary(
        &mut self,
        level: i32,
        begin_user_key: &str,
        end_user_key: &str
    ) -> Status {
        trace!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_run_manual_compaction_over_user_key_range.entry",
            level,
            begin_user_key = begin_user_key,
            end_user_key = end_user_key
        );

        let database_instance: &mut DBImpl = self.database_instance_mut().as_mut();

        database_instance.clear_background_error_for_test();

        let compaction_status: Status =
            database_instance
                .test_run_versionset_compaction_inline_for_live_compaction_boundary_harness(
                    level,
                    begin_user_key,
                    end_user_key,
                );

        let background_error_status: Status =
            database_instance.test_read_background_error_status_for_live_compaction_boundary_harness();

        let final_status: Status = if compaction_status.is_ok() {
            background_error_status
        } else {
            compaction_status
        };

        debug!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_run_manual_compaction_over_user_key_range.exit",
            level,
            begin_user_key = begin_user_key,
            end_user_key = end_user_key,
            status_ok = final_status.is_ok(),
            status = %final_status.to_string()
        );

        final_status
    }

    /// Collects every adjacent output-file boundary record for the requested level.
    ///
    /// Postcondition:
    /// - each returned record corresponds to one adjacent pair in current level order
    /// - an empty vector means the level currently has zero or one file
    pub fn collect_output_boundary_observations_at_level_for_live_compaction_boundary(
        &mut self,
        level: usize
    ) -> Vec<BitcoinLevelDbDbImplLiveCompactionOutputBoundaryObservation> {
        trace!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_collect_output_boundary_observations_at_level.entry",
            level
        );

        let database_instance: &mut DBImpl = self.database_instance_mut().as_mut();

        let file_metadata_pointers =
            database_instance
                .test_collect_current_level_file_metadata_pointers_for_live_compaction_boundary_harness(
                    level,
                );

        let mut boundary_observations:
            Vec<BitcoinLevelDbDbImplLiveCompactionOutputBoundaryObservation> = Vec::new();

        let mut index: usize = 1;
        while index < file_metadata_pointers.len() {
            let previous_file_ptr: *mut FileMetaData = file_metadata_pointers[index - 1];
            let next_file_ptr: *mut FileMetaData = file_metadata_pointers[index];

            assert!(!previous_file_ptr.is_null());
            assert!(!next_file_ptr.is_null());

            let previous_file_number: u64 = unsafe { *(*previous_file_ptr).number() };
            let next_file_number: u64 = unsafe { *(*next_file_ptr).number() };

            let previous_largest_internal_key: InternalKey =
                unsafe { (*previous_file_ptr).largest().clone() };
            let next_smallest_internal_key: InternalKey =
                unsafe { (*next_file_ptr).smallest().clone() };

            let (previous_largest_user_key, previous_largest_sequence_number) =
                bitcoinleveldb_dbimpl_live_compaction_boundary_decode_user_key_and_sequence(
                    &previous_largest_internal_key,
                );

            let (next_smallest_user_key, next_smallest_sequence_number) =
                bitcoinleveldb_dbimpl_live_compaction_boundary_decode_user_key_and_sequence(
                    &next_smallest_internal_key,
                );

            let same_user_key_boundary: bool =
                previous_largest_user_key == next_smallest_user_key;

            let boundary_observation_result =
                BitcoinLevelDbDbImplLiveCompactionOutputBoundaryObservationBuilder::default()
                    .previous_file_number(previous_file_number)
                    .next_file_number(next_file_number)
                    .previous_largest_user_key(previous_largest_user_key.clone())
                    .previous_largest_sequence_number(previous_largest_sequence_number)
                    .next_smallest_user_key(next_smallest_user_key.clone())
                    .next_smallest_sequence_number(next_smallest_sequence_number)
                    .same_user_key_boundary(same_user_key_boundary)
                    .build();

            let boundary_observation = match boundary_observation_result {
                Ok(observation_value) => {
                    observation_value
                }
                Err(builder_error) => {
                    error!(
                        target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
                        label = "dbimpl.live_compaction_boundary_harness_collect_output_boundary_observations_at_level.builder_error",
                        level,
                        previous_file_number,
                        next_file_number,
                        previous_largest_user_key = %previous_largest_user_key,
                        next_smallest_user_key = %next_smallest_user_key,
                        builder_error = ?builder_error
                    );
                    panic!();
                }
            };

            boundary_observations.push(boundary_observation);

            index = index.saturating_add(1);
        }

        debug!(
            target: "bitcoinleveldb_dbimpl::t_live_compaction_output_boundary_support",
            label = "dbimpl.live_compaction_boundary_harness_collect_output_boundary_observations_at_level.exit",
            level,
            boundary_count = boundary_observations.len()
        );

        boundary_observations
    }
}
