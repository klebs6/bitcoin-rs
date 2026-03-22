// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/constructors.rs ]
crate::ix!();

/// Guarantees the returned key always encodes `ValueType::TypeValue`; callers must not reuse
/// this helper for deletion markers or reinterpret its value type across refactors.
pub fn make_value_internal_key_for_user_key(
    user_key: &str,
    sequence_number: u64,
) -> InternalKey {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "make_value_internal_key_for_user_key_enter",
        user_key = user_key,
        sequence_number = sequence_number
    );

    let internal_key =
        InternalKey::new(&Slice::from(user_key), sequence_number, ValueType::TypeValue);

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "make_value_internal_key_for_user_key_exit",
        user_key = user_key,
        sequence_number = sequence_number
    );

    internal_key
}

/// Guarantees the comparator pointer captured by the returned wrapper stays tied to the exact
/// user comparator installed in `options` at construction time.
pub fn build_internal_key_comparator_from_database_options(
    options: &Options,
) -> InternalKeyComparator {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_internal_key_comparator_from_database_options_enter"
    );

    let user_comparator_ref: &dyn SliceComparator = options.comparator().as_ref();

    let comparator_name = match user_comparator_ref.name() {
        Cow::Borrowed(name) => name.to_owned(),
        Cow::Owned(name) => name,
    };

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_internal_key_comparator_from_database_options_decision",
        comparator_name = %comparator_name
    );

    /*
      The upstream options contract is responsible for installing a user comparator
      before any harness opens a version set. This helper preserves that invariant
      and intentionally traces the comparator identity before constructing the
      internal comparator so failures remain diagnosable without losing topology.
    */

    let internal_key_comparator =
        InternalKeyComparator::new(user_comparator_ref as *const dyn SliceComparator);

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_internal_key_comparator_from_database_options_exit",
        comparator_name = %comparator_name
    );

    internal_key_comparator
}

/// Guarantees `refs` starts at one and the smallest/largest key bounds are cloned exactly,
/// so the returned pointer is safe for tests that model a live file metadata entry.
pub fn allocate_test_file_metadata_for_key_range(
    file_number: u64,
    smallest_internal_key: &InternalKey,
    largest_internal_key: &InternalKey,
) -> *mut FileMetaData {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "allocate_test_file_metadata_for_key_range_enter",
        file_number = file_number
    );

    let mut file_metadata = Box::new(FileMetaData::default());
    *file_metadata.number_mut() = file_number;
    *file_metadata.file_size_mut() = 1;
    *file_metadata.smallest_mut() = smallest_internal_key.clone();
    *file_metadata.largest_mut() = largest_internal_key.clone();
    *file_metadata.refs_mut() = 1;

    let file_metadata_ptr = Box::into_raw(file_metadata);

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "allocate_test_file_metadata_for_key_range_exit",
        file_number = file_number,
        file_metadata_ptr = ?file_metadata_ptr
    );

    file_metadata_ptr
}

/// Guarantees the returned path is unique for the current process and current wall-clock instant;
/// callers must create the directory explicitly so filesystem side effects stay observable.
pub fn build_unique_temporary_database_directory_path(test_prefix: &str) -> PathBuf {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_unique_temporary_database_directory_path_enter",
        test_prefix = test_prefix
    );

    let process_identifier = StdProcess::id();
    let since_unix_epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(error) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::constructors",
                event = "build_unique_temporary_database_directory_path_system_time_before_epoch",
                error = ?error
            );
            StdDuration::from_nanos(0)
        }
    };

    let mut temporary_directory_path = StdEnv::temp_dir();
    temporary_directory_path.push(format!(
        "{}_{}_{}",
        test_prefix,
        process_identifier,
        since_unix_epoch.as_nanos()
    ));

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_unique_temporary_database_directory_path_exit",
        test_prefix = test_prefix,
        directory = %temporary_directory_path.display()
    );

    temporary_directory_path
}

/// Guarantees the returned path remains namespaced to the versionset-into-version scenario and
/// never aliases the generic temporary database helper for the same process and instant.
pub fn build_unique_versionset_into_version_directory_path(
    test_label: &str,
) -> PathBuf {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_unique_versionset_into_version_directory_path_enter",
        test_label = test_label
    );

    let process_identifier = StdProcess::id();

    let since_unix_epoch = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(error) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::constructors",
                event = "build_unique_versionset_into_version_directory_path_system_time_before_epoch",
                error = ?error
            );
            StdDuration::from_nanos(0)
        }
    };

    let mut temporary_directory_path = StdEnv::temp_dir();
    temporary_directory_path.push(format!(
        "bitcoinleveldb_versionset_into_version_{}_{}_{}",
        test_label,
        process_identifier,
        since_unix_epoch.as_nanos()
    ));

    trace!(
        target: "bitcoinleveldb_versionsettestutil::constructors",
        event = "build_unique_versionset_into_version_directory_path_exit",
        test_label = test_label,
        directory = %temporary_directory_path.display()
    );

    temporary_directory_path
}
