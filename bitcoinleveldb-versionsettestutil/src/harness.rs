// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/harness.rs ]
crate::ix!();

/// Guarantees cleanup is best-effort only; callers must never rely on filesystem removal for
/// logical correctness of a test assertion.
pub fn remove_directory_tree_best_effort(directory_path: &Path) {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "remove_directory_tree_best_effort_enter",
        directory = %directory_path.display()
    );

    match StdFs::remove_dir_all(directory_path) {
        Ok(()) => {
            trace!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "remove_directory_tree_best_effort_exit",
                directory = %directory_path.display()
            );
        }
        Err(error) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "remove_directory_tree_best_effort_error",
                directory = %directory_path.display(),
                error = ?error
            );
        }
    }
}

/// Guarantees a non-OK status terminates the current test path immediately after structured
/// diagnostics are emitted.
pub fn assert_status_is_ok_or_panic(
    status: &Status,
    context_label: &'static str,
) {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "assert_status_is_ok_or_panic_enter",
        context_label = context_label
    );

    match status.is_ok() {
        true => {
            trace!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "assert_status_is_ok_or_panic_exit",
                context_label = context_label
            );
        }
        false => {
            error!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "assert_status_is_ok_or_panic_non_ok_status",
                context_label = context_label,
                status = ?status
            );
            panic!("assert_status_is_ok_or_panic_non_ok_status");
        }
    }
}

/// Preconditions: `raw_c_string_ptr` is non-null and points to a valid NUL-terminated byte
/// sequence. Postconditions: the returned string is the lossy UTF-8 view of that exact sequence.
pub fn read_utf8_lossy_c_string(raw_c_string_ptr: *const u8) -> String {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "read_utf8_lossy_c_string_enter",
        raw_c_string_ptr = ?raw_c_string_ptr
    );

    match raw_c_string_ptr.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "read_utf8_lossy_c_string_null_pointer"
            );
            panic!("read_utf8_lossy_c_string_null_pointer");
        }
        false => {
            let c_string = unsafe { CStr::from_ptr(raw_c_string_ptr as *const CoreCChar) };
            let owned_string = c_string.to_string_lossy().to_string();

            trace!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "read_utf8_lossy_c_string_exit",
                string_length = owned_string.len()
            );

            owned_string
        }
    }
}

/// Guarantees a directory required by the versionset-into-version scenario exists after return
/// or the current test aborts before any downstream filesystem mutation occurs.
pub fn create_versionset_into_version_directory_or_panic(
    directory_path: &Path,
) {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "create_versionset_into_version_directory_or_panic_enter",
        directory = %directory_path.display()
    );

    match StdFs::create_dir_all(directory_path) {
        Ok(()) => {
            trace!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "create_versionset_into_version_directory_or_panic_exit",
                directory = %directory_path.display()
            );
        }
        Err(error) => {
            error!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "create_versionset_into_version_directory_or_panic_error",
                directory = %directory_path.display(),
                error = ?error
            );
            panic!("create_versionset_into_version_directory_or_panic_error");
        }
    }
}

/// Guarantees cleanup remains non-fatal for versionset-into-version tests.
/// Best effort cleanup; tests should not fail solely due to filesystem cleanup issues.
pub fn remove_versionset_into_version_directory_best_effort(
    directory_path: &Path,
) {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "remove_versionset_into_version_directory_best_effort_enter",
        directory = %directory_path.display()
    );

    match StdFs::remove_dir_all(directory_path) {
        Ok(()) => {
            trace!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "remove_versionset_into_version_directory_best_effort_exit",
                directory = %directory_path.display()
            );
        }
        Err(error) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "remove_versionset_into_version_directory_best_effort_error",
                directory = %directory_path.display(),
                error = ?error
            );
        }
    }
}

/// Guarantees the first manifest path matching the LevelDB naming prefix is returned unchanged,
/// and absence is reported as `None` rather than an error.
pub fn find_manifest_file_in_directory(
    directory_path: &Path,
) -> Option<PathBuf> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "find_manifest_file_in_directory_enter",
        directory = %directory_path.display()
    );

    let read_dir = match StdFs::read_dir(directory_path) {
        Ok(read_dir) => read_dir,
        Err(error) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::harness",
                event = "find_manifest_file_in_directory_read_dir_error",
                directory = %directory_path.display(),
                error = ?error
            );
            return None;
        }
    };

    for directory_entry_result in read_dir {
        match directory_entry_result {
            Ok(directory_entry) => {
                let entry_path = directory_entry.path();
                match entry_path.file_name().and_then(|file_name| file_name.to_str()) {
                    Some(file_name) => {
                        match file_name.starts_with("MANIFEST-") {
                            true => {
                                trace!(
                                    target: "bitcoinleveldb_versionsettestutil::harness",
                                    event = "find_manifest_file_in_directory_found",
                                    manifest_path = %entry_path.display()
                                );
                                return Some(entry_path);
                            }
                            false => {}
                        }
                    }
                    None => {}
                }
            }
            Err(error) => {
                warn!(
                    target: "bitcoinleveldb_versionsettestutil::harness",
                    event = "find_manifest_file_in_directory_entry_error",
                    directory = %directory_path.display(),
                    error = ?error
                );
            }
        }
    }

    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "find_manifest_file_in_directory_exit_none",
        directory = %directory_path.display()
    );

    None
}

/// Guarantees the returned pointer is either the exact pointer already present in `file_metadata_pointers`
/// or null; the function never allocates or reorders the vector.
pub fn find_file_metadata_pointer_by_number_in_vector(
    file_metadata_pointers: &Vec<*mut FileMetaData>,
    file_number: u64,
) -> *mut FileMetaData {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "find_file_metadata_pointer_by_number_in_vector_enter",
        file_number = file_number
    );

    for &file_metadata_ptr in file_metadata_pointers.iter() {
        match file_metadata_ptr.is_null() {
            true => {}
            false => unsafe {
                match *(*file_metadata_ptr).number() == file_number {
                    true => {
                        trace!(
                            target: "bitcoinleveldb_versionsettestutil::harness",
                            event = "find_file_metadata_pointer_by_number_in_vector_found",
                            file_number = file_number,
                            file_metadata_ptr = ?file_metadata_ptr
                        );
                        return file_metadata_ptr;
                    }
                    false => {}
                }
            },
        }
    }

    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "find_file_metadata_pointer_by_number_in_vector_not_found",
        file_number = file_number
    );

    CorePtr::null_mut()
}

/// Guarantees the returned numbers preserve the exact iteration order of the chosen compaction
/// input vector and therefore remain suitable for deterministic assertions.
pub fn collect_compaction_input_file_numbers(
    compaction: &Compaction,
    input_vector_index: usize,
) -> Vec<u64> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "collect_compaction_input_file_numbers_enter",
        input_vector_index = input_vector_index
    );

    let file_numbers = compaction.inputs()[input_vector_index]
        .iter()
        .map(|&file_metadata_ptr| unsafe { *(*file_metadata_ptr).number() })
        .collect::<Vec<u64>>();

    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "collect_compaction_input_file_numbers_exit",
        input_vector_index = input_vector_index,
        count = file_numbers.len()
    );

    file_numbers
}

/// Guarantees the returned numbers preserve the exact iteration order of `compaction.grandparents()`
/// and never dereference null pointers that are not already present in that vector.
pub fn collect_compaction_grandparent_file_numbers(
    compaction: &Compaction,
) -> Vec<u64> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "collect_compaction_grandparent_file_numbers_enter"
    );

    let file_numbers = compaction
        .grandparents()
        .iter()
        .map(|&file_metadata_ptr| unsafe { *(*file_metadata_ptr).number() })
        .collect::<Vec<u64>>();

    trace!(
        target: "bitcoinleveldb_versionsettestutil::harness",
        event = "collect_compaction_grandparent_file_numbers_exit",
        count = file_numbers.len()
    );

    file_numbers
}
