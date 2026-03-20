// ---------------- [ File: bitcoinleveldb-versionset/src/t_version_get_probe_support.rs ]
crate::ix!();

/// Distinguishes point-value entries from tombstones in synthetic SSTables used by read probes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VersionGetProbeEntryKind {
    /// The table entry encodes `ValueType::TypeValue` and carries a materialized payload.
    Value,
    /// The table entry encodes `ValueType::TypeDeletion` and therefore represents absence.
    Tombstone,
}

/// Records the externally visible outcome of probing `Version::Get` at a fixed snapshot.
pub struct VersionGetProbeExecutionResult {
    /// Exact status returned by `Version::Get`.
    status: Status,

    /// Materialized value buffer after the probe returns. This is meaningful only when `status`
    /// is OK and the lookup found a value.
    materialized_value: String,

    /// File number charged through `VersionGetStats`, if one was recorded.
    seek_file_number: Option<u64>,

    /// Level charged through `VersionGetStats`.
    seek_file_level: i32,
}

impl VersionGetProbeExecutionResult {
    /// Guarantees the returned status is the exact `Version::Get` result.
    pub fn status(&self) -> &Status {
        &self.status
    }

    /// Guarantees the returned string is the exact value buffer observed after the probe.
    pub fn materialized_value(&self) -> &str {
        self.materialized_value.as_str()
    }

    /// Guarantees the returned file number is either the charged file number or `None`.
    pub fn seek_file_number(&self) -> Option<u64> {
        self.seek_file_number
    }

    /// Guarantees the returned level is the exact level captured in `VersionGetStats`.
    pub fn seek_file_level(&self) -> i32 {
        self.seek_file_level
    }
}

/// Guarantees the returned internal key exactly matches the requested user key, sequence number,
/// and entry kind, so metadata and on-disk contents stay aligned in the probe.
pub fn build_internal_key_for_version_get_probe_entry(
    user_key: &str,
    sequence_number: SequenceNumber,
    entry_kind: VersionGetProbeEntryKind,
) -> InternalKey {
    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "build_internal_key_for_version_get_probe_entry_enter",
        user_key = user_key,
        sequence_number = sequence_number,
        entry_kind = ?entry_kind
    );

    let internal_key = match entry_kind {
        VersionGetProbeEntryKind::Value => {
            InternalKey::new(
                &Slice::from_str(user_key),
                sequence_number,
                ValueType::TypeValue,
            )
        }
        VersionGetProbeEntryKind::Tombstone => {
            InternalKey::new(
                &Slice::from_str(user_key),
                sequence_number,
                ValueType::TypeDeletion,
            )
        }
    };

    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "build_internal_key_for_version_get_probe_entry_exit",
        user_key = user_key,
        sequence_number = sequence_number,
        entry_kind = ?entry_kind
    );

    internal_key
}

/// Guarantees the created SSTable contains exactly one internal entry whose key and payload match
/// the probe request, and returns the final on-disk file size recorded by the table builder.
pub fn write_single_entry_table_file_for_version_get_probe(
    database_name: &str,
    database_options: &Options,
    file_number: u64,
    internal_key: &InternalKey,
    entry_kind: VersionGetProbeEntryKind,
    stored_value: &str,
) -> u64 {
    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "write_single_entry_table_file_for_version_get_probe_enter",
        database_name = database_name,
        file_number = file_number,
        entry_kind = ?entry_kind
    );

    let env_rc = match database_options.env().as_ref() {
        Some(env) => env.clone(),
        None => {
            error!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "write_single_entry_table_file_for_version_get_probe_missing_env",
                database_name = database_name,
                file_number = file_number
            );
            panic!("write_single_entry_table_file_for_version_get_probe_missing_env");
        }
    };

    let database_name_owned = database_name.to_string();
    let table_path = table_file_name(&database_name_owned, file_number);

    let mut writable_file_holder_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let create_status = env_rc
        .borrow_mut()
        .new_writable_file(&table_path, &mut writable_file_holder_ptr);

    assert_status_is_ok_or_panic(
        &create_status,
        "write_single_entry_table_file_for_version_get_probe_new_writable_file",
    );

    match writable_file_holder_ptr.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "write_single_entry_table_file_for_version_get_probe_null_output_file",
                database_name = database_name,
                file_number = file_number,
                table_path = %table_path
            );
            panic!("write_single_entry_table_file_for_version_get_probe_null_output_file");
        }
        false => {}
    }

    let mut writable_file_box: Box<dyn WritableFile> =
        unsafe { *Box::from_raw(writable_file_holder_ptr) };

    let writable_file_ptr: *mut dyn WritableFile = &mut *writable_file_box;

    let mut table_builder = TableBuilder::new(database_options, writable_file_ptr);

    let encoded_internal_key = internal_key.encode();

    match entry_kind {
        VersionGetProbeEntryKind::Value => {
            let value_slice = Slice::from_str(stored_value);
            table_builder.add(&encoded_internal_key, &value_slice);
        }
        VersionGetProbeEntryKind::Tombstone => {
            let empty_value = Slice::from_str("");
            table_builder.add(&encoded_internal_key, &empty_value);
        }
    }

    let mut status = table_builder.finish();
    let file_size = table_builder.file_size();

    drop(table_builder);

    if status.is_ok() {
        status = writable_file_box.sync();
    }

    if status.is_ok() {
        status = writable_file_box.close();
    }

    drop(writable_file_box);

    if !status.is_ok() || file_size == 0 {
        let delete_status = env_rc.borrow_mut().delete_file(&table_path);

        if !delete_status.is_ok() {
            warn!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "write_single_entry_table_file_for_version_get_probe_delete_failed",
                database_name = database_name,
                file_number = file_number,
                table_path = %table_path,
                delete_status = ?delete_status
            );
        }
    }

    assert_status_is_ok_or_panic(
        &status,
        "write_single_entry_table_file_for_version_get_probe_finish_sync_close",
    );

    match file_size > 0 {
        true => {}
        false => {
            error!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "write_single_entry_table_file_for_version_get_probe_zero_file_size",
                database_name = database_name,
                file_number = file_number,
                table_path = %table_path
            );
            panic!("write_single_entry_table_file_for_version_get_probe_zero_file_size");
        }
    }

    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "write_single_entry_table_file_for_version_get_probe_exit",
        database_name = database_name,
        file_number = file_number,
        file_size = file_size
    );

    file_size
}

/// Guarantees one physical SSTable and one matching `VersionEdit::add_file` entry are installed
/// into the requested level, so `Version::Get` probes observe a real table-backed layout.
pub fn install_single_entry_table_file_into_level_for_version_get_probe(
    harness: &mut VersionSetAlgorithmScenarioHarness,
    level: i32,
    user_key: &str,
    sequence_number: SequenceNumber,
    entry_kind: VersionGetProbeEntryKind,
    stored_value: &str,
) -> u64 {
    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "install_single_entry_table_file_into_level_for_version_get_probe_enter",
        database_name = harness.database_name(),
        level = level,
        user_key = user_key,
        sequence_number = sequence_number,
        entry_kind = ?entry_kind
    );

    let internal_key = build_internal_key_for_version_get_probe_entry(
        user_key,
        sequence_number,
        entry_kind,
    );

    let _version_set_mutex_guard = harness.acquire_version_set_mutex();

    let file_number = {
        let version_set = harness.version_set_mut();
        version_set.new_file_number()
    };

    let file_size = write_single_entry_table_file_for_version_get_probe(
        harness.database_name(),
        harness.database_options(),
        file_number,
        &internal_key,
        entry_kind,
        stored_value,
    );

    let mut version_edit = VersionEdit::default();
    version_edit.add_file(
        level,
        file_number,
        file_size,
        &internal_key,
        &internal_key,
    );

    let status = harness.apply_version_edit(&mut version_edit as *mut VersionEdit);
    assert_status_is_ok_or_panic(
        &status,
        "install_single_entry_table_file_into_level_for_version_get_probe_log_and_apply",
    );

    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "install_single_entry_table_file_into_level_for_version_get_probe_exit",
        database_name = harness.database_name(),
        level = level,
        file_number = file_number,
        file_size = file_size
    );

    file_number
}

/// Guarantees the returned vector preserves the current level ordering exactly, so the probe can
/// log or assert the concrete file topology seen by `Version::Get`.
pub fn collect_current_level_file_numbers_for_version_get_probe(
    harness: &mut VersionSetAlgorithmScenarioHarness,
    level: usize,
) -> Vec<u64> {
    harness
        .current_level_file_metadata_pointers(level)
        .iter()
        .copied()
        .filter(|file_metadata_ptr| !file_metadata_ptr.is_null())
        .map(|file_metadata_ptr| unsafe { *(*file_metadata_ptr).number() })
        .collect()
}

/// Guarantees the returned string is the exact `Version::debug_string()` of the current version.
pub fn current_version_debug_string_for_version_get_probe(
    harness: &mut VersionSetAlgorithmScenarioHarness,
) -> String {
    let current_version_ptr = harness.current_version_ptr();

    match current_version_ptr.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "current_version_debug_string_for_version_get_probe_null_current"
            );
            panic!("current_version_debug_string_for_version_get_probe_null_current");
        }
        false => unsafe { (*current_version_ptr).debug_string() },
    }
}

/// Guarantees the returned result is the exact externally visible outcome of `Version::Get` at
/// the requested snapshot sequence against the current version.
pub fn execute_version_get_against_current_version_for_snapshot_sequence(
    harness: &mut VersionSetAlgorithmScenarioHarness,
    user_key: &str,
    snapshot_sequence: SequenceNumber,
) -> VersionGetProbeExecutionResult {
    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "execute_version_get_against_current_version_for_snapshot_sequence_enter",
        database_name = harness.database_name(),
        user_key = user_key,
        snapshot_sequence = snapshot_sequence
    );

    let current_version_ptr = harness.current_version_ptr();

    match current_version_ptr.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_versionset::version_get_probe_support",
                event = "execute_version_get_against_current_version_for_snapshot_sequence_null_current",
                database_name = harness.database_name()
            );
            panic!("execute_version_get_against_current_version_for_snapshot_sequence_null_current");
        }
        false => {}
    }

    let lookup_key = LookupKey::new(&Slice::from_str(user_key), snapshot_sequence);
    let mut materialized_value = String::new();
    let mut stats = VersionGetStats::default();
    let read_options = ReadOptions::default();

    let status = unsafe {
        (*current_version_ptr).get(
            &read_options,
            &lookup_key,
            &mut materialized_value as *mut String,
            &mut stats as *mut VersionGetStats,
        )
    };

    let seek_file_ptr = *stats.seek_file();
    let seek_file_number = match seek_file_ptr.is_null() {
        true => None,
        false => Some(unsafe { *(*seek_file_ptr).number() }),
    };

    let result = VersionGetProbeExecutionResult {
        status,
        materialized_value,
        seek_file_number,
        seek_file_level: *stats.seek_file_level(),
    };

    trace!(
        target: "bitcoinleveldb_versionset::version_get_probe_support",
        event = "execute_version_get_against_current_version_for_snapshot_sequence_exit",
        database_name = harness.database_name(),
        user_key = user_key,
        snapshot_sequence = snapshot_sequence,
        status = ?result.status(),
        materialized_value = result.materialized_value(),
        seek_file_number = ?result.seek_file_number(),
        seek_file_level = result.seek_file_level()
    );

    result
}
