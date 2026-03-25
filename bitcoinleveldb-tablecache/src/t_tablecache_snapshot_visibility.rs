// ---------------- [ File: bitcoinleveldb-tablecache/src/t_tablecache_snapshot_visibility.rs ]
crate::ix!();

const BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET: &str =
    "bitcoinleveldb_tablecache::t_tablecache_snapshot_visibility";

/**
  | Preserves the exact build-surface observation
  | for a support table constructed from encoded
  | internal-key entries.
  |
  | The observation must expose directory setup,
  | table build result, and produced file size
  | without reinterpreting filesystem status.
  */
pub enum BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation {
    /// The helper cannot proceed when the default
    /// `Options` surface does not expose an `Env`
    /// handle.
    MissingConfiguredEnv,

    /// The helper could not materialize the table
    /// directory required by `build_table`.
    DirectoryCreateFailed {
        /// The exact directory-creation status must
        /// remain available for later hardening.
        status: Status,
    },

    /// The table was built successfully.
    BuildOk {
        /// The produced file size must be preserved
        /// exactly so later reads target the same
        /// artifact boundary.
        file_size: u64,
    },

    /// `build_table` completed with a non-OK
    /// status.
    BuildNotOk {
        /// The exact build status must survive
        /// unchanged for later diagnosis.
        status: Status,
    },
}

/**
  | Preserves the exact read-surface observation
  | returned by `TableCache::get` for a support
  | lookup key.
  |
  | The observation must separate missing values
  | from non-OK status without weakening the
  | table-lookup boundary.
  */
pub enum BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation {
    /// The helper cannot proceed when the default
    /// `Options` surface does not expose an `Env`
    /// handle.
    MissingConfiguredEnv,

    /// `TableCache::get` returned `Status::ok()`
    /// and the callback was not invoked.
    StatusOkNoValue,

    /// `TableCache::get` returned `Status::ok()`
    /// and supplied exactly one value.
    StatusOkValue {
        /// The captured value bytes are preserved as
        /// a `String` because the support path is
        /// restricted to UTF-8 test payloads.
        value: String,
    },

    /// `TableCache::get` returned a non-OK status.
    StatusNotOk {
        /// The exact read status must survive
        /// unchanged for later diagnosis.
        status: Status,
    },
}

/**
  | Produces the canonical LevelDB internal-key
  | byte layout for a user key, sequence number,
  | and value-type tag.
  |
  | Preconditions:
  | - `value_type_tag` must fit in the low eight
  |   bits of the trailer.
  |
  | Postconditions:
  | - The returned bytes are exactly
  |   `user_key || fixed64((sequence_number << 8) |
  |   value_type_tag)` in little-endian order.
  | - No comparator-specific reinterpretation is
  |   performed here.
  */
pub fn bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
    user_key: &String,
    sequence_number: u64,
    value_type_tag: u8,
) -> Vec<u8> {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_encode_internal_key_bytes.entry",
        phase = "enter",
        user_key_len = user_key.len(),
        sequence_number,
        value_type_tag
    );

    let packed_sequence_and_type =
        (sequence_number << 8) | u64::from(value_type_tag);
    let trailer = packed_sequence_and_type.to_le_bytes();

    let mut encoded_internal_key = user_key.as_bytes().to_vec();
    encoded_internal_key.extend_from_slice(&trailer);

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_encode_internal_key_bytes.exit",
        phase = "return",
        encoded_len = encoded_internal_key.len()
    );

    encoded_internal_key
}

/**
  | Sorts encoded internal-key entries in the
  | canonical LevelDB visibility order: user key
  | ascending, packed sequence-and-type
  | descending.
  |
  | Preconditions:
  | - Every entry key must be encoded as an
  |   internal key with an eight-byte trailer.
  |
  | Postconditions:
  | - The input vector is reordered in place.
  | - Equal user keys retain descending sequence
  |   visibility order.
  |
  | Forbidden drift:
  | - This helper expresses logical internal-key
  |   visibility order only.
  | - It must not be used as the direct
  |   `build_table` admission order while
  |   `bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support`
  |   still uses the current `Options::default()`
  |   comparator surface.
  */
pub fn bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_test_support(
    entries: &mut Vec<(Vec<u8>, Vec<u8>)>,
) {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_sort_encoded_internal_entries.entry",
        phase = "enter",
        entry_count = entries.len()
    );

    entries.sort_by(|lhs_entry, rhs_entry| {
        let lhs_user_key =
            bitcoinleveldb_tablecache_snapshot_visibility_extract_user_key_prefix_for_sort(
                lhs_entry.0.as_slice(),
            );
        let rhs_user_key =
            bitcoinleveldb_tablecache_snapshot_visibility_extract_user_key_prefix_for_sort(
                rhs_entry.0.as_slice(),
            );

        let user_key_ordering = lhs_user_key.cmp(rhs_user_key);

        match user_key_ordering {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let lhs_trailer =
                    bitcoinleveldb_tablecache_snapshot_visibility_extract_trailer_for_sort(
                        lhs_entry.0.as_slice(),
                    );
                let rhs_trailer =
                    bitcoinleveldb_tablecache_snapshot_visibility_extract_trailer_for_sort(
                        rhs_entry.0.as_slice(),
                    );

                rhs_trailer.cmp(&lhs_trailer)
            }
        }
    });

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_sort_encoded_internal_entries.exit",
        phase = "return",
        entry_count = entries.len()
    );
}

/**
  | Reads a value from a support table using an
  | already-encoded internal lookup key.
  |
  | Preconditions:
  | - `dbname`, `file_number`, and `file_size`
  |   must identify a table previously built by
  |   the support path.
  |
  | Postconditions:
  | - The returned observation preserves exact
  |   read status.
  | - A callback hit and a callback miss remain
  |   distinguishable under `Status::ok()`.
  */
pub fn bitcoinleveldb_tablecache_snapshot_visibility_read_table_value_for_encoded_lookup_key_for_test_support(
    dbname: &String,
    file_number: u64,
    file_size: u64,
    lookup_internal_key: &[u8],
) -> BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_read_table_value.entry",
        phase = "enter",
        dbname_len = dbname.len(),
        file_number,
        file_size,
        lookup_internal_key_len = lookup_internal_key.len()
    );

    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => handle.clone(),
        None => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                label = "tablecache_snapshot_visibility_read_table_value.missing_env",
                phase = "decision"
            );
            return BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::MissingConfiguredEnv;
        }
    };

    drop(env_handle);

    let mut table_cache = TableCache::new(dbname, &options, 32);
    let read_options = ReadOptions::default();
    let lookup_slice = Slice::from(lookup_internal_key);
    let mut capture = BitcoinLeveldbTablecacheSnapshotVisibilityValueCaptureState::default();

    let status = table_cache.get(
        &read_options,
        file_number,
        file_size,
        &lookup_slice,
        &mut capture as *mut BitcoinLeveldbTablecacheSnapshotVisibilityValueCaptureState
            as *mut c_void,
        bitcoinleveldb_tablecache_snapshot_visibility_capture_value_from_table_cache_get_callback,
    );

    let observation = match status.is_ok() {
        true => match capture.seen_value {
            true => BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkValue {
                value: capture.value,
            },
            false => BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkNoValue,
        },
        false => BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusNotOk {
            status,
        },
    };

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_read_table_value.exit",
        phase = "return"
    );

    observation
}

fn bitcoinleveldb_tablecache_snapshot_visibility_extract_user_key_prefix_for_sort(
    encoded_internal_key: &[u8],
) -> &[u8] {
    match encoded_internal_key.len() >= 8 {
        true => &encoded_internal_key[..encoded_internal_key.len() - 8],
        false => &encoded_internal_key[..0],
    }
}

fn bitcoinleveldb_tablecache_snapshot_visibility_extract_trailer_for_sort(
    encoded_internal_key: &[u8],
) -> u64 {
    match encoded_internal_key.len() >= 8 {
        true => {
            let trailer_start = encoded_internal_key.len() - 8;
            let trailer_bytes = &encoded_internal_key[trailer_start..];
            let trailer_array = [
                trailer_bytes[0],
                trailer_bytes[1],
                trailer_bytes[2],
                trailer_bytes[3],
                trailer_bytes[4],
                trailer_bytes[5],
                trailer_bytes[6],
                trailer_bytes[7],
            ];
            u64::from_le_bytes(trailer_array)
        }
        false => 0,
    }
}

#[derive(Default)]
struct BitcoinLeveldbTablecacheSnapshotVisibilityValueCaptureState {
    seen_value: bool,
    value:      String,
}

fn bitcoinleveldb_tablecache_snapshot_visibility_capture_value_from_table_cache_get_callback(
    arg: *mut c_void,
    _k:  &Slice,
    v:   &Slice,
) -> c_void {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_capture_value_callback.entry",
        phase = "enter",
        value_len = *v.size()
    );

    unsafe {
        let capture_state =
            &mut *(arg as *mut BitcoinLeveldbTablecacheSnapshotVisibilityValueCaptureState);
        capture_state.seen_value = true;
        capture_state.value = v.to_string();
    }

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_capture_value_callback.exit",
        phase = "return"
    );

    unsafe { zeroed() }
}

struct BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    entries: Vec<(Vec<u8>, Vec<u8>)>,
    index:   isize,
}

impl BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn new(entries: Vec<(Vec<u8>, Vec<u8>)>) -> Self {
        Self { entries, index: -1 }
    }

    fn bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index(
        &self,
    ) -> bool {
        self.index >= 0 && (self.index as usize) < self.entries.len()
    }
}

impl LevelDBIteratorValid for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn valid(&self) -> bool {
        self.bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index()
    }
}

impl LevelDBIteratorSeekToFirst for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn seek_to_first(&mut self) {
        match self.entries.is_empty() {
            true => {
                self.index = -1;
            }
            false => {
                self.index = 0;
            }
        }
    }
}

impl LevelDBIteratorSeekToLast for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn seek_to_last(&mut self) {
        match self.entries.is_empty() {
            true => {
                self.index = -1;
            }
            false => {
                self.index = (self.entries.len() - 1) as isize;
            }
        }
    }
}

impl LevelDBIteratorSeek for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn seek(&mut self, target: &Slice) {
        let target_bytes = unsafe {
            match ((*target.size()) == 0, (*target.data()).is_null()) {
                (true, _) => Vec::new(),
                (false, true) => Vec::new(),
                (false, false) => from_raw_parts(*target.data(), *target.size()).to_vec(),
            }
        };

        let mut found_index: isize = -1;
        let mut current_index: usize = 0;

        while current_index < self.entries.len() {
            if self.entries[current_index].0.as_slice() >= target_bytes.as_slice() {
                found_index = current_index as isize;
                break;
            }
            current_index += 1;
        }

        self.index = found_index;
    }
}

impl LevelDBIteratorNext for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn next(&mut self) {
        if self.bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index() {
            self.index += 1;
            if !self.bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index() {
                self.index = self.entries.len() as isize;
            }
        }
    }
}

impl LevelDBIteratorPrev for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn prev(&mut self) {
        match self.entries.is_empty() {
            true => {
                self.index = -1;
            }
            false => {
                if self.index <= 0 {
                    self.index = -1;
                } else {
                    self.index -= 1;
                }
            }
        }
    }
}

impl LevelDBIteratorKey for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn key(&self) -> Slice {
        match self.bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index() {
            true => {
                let valid_index = self.index as usize;
                Slice::from(self.entries[valid_index].0.as_slice())
            }
            false => Slice::default(),
        }
    }
}

impl LevelDBIteratorValue for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn value(&self) -> Slice {
        match self.bitcoinleveldb_tablecache_snapshot_visibility_iterator_has_valid_index() {
            true => {
                let valid_index = self.index as usize;
                Slice::from(self.entries[valid_index].1.as_slice())
            }
            false => Slice::default(),
        }
    }
}

impl LevelDBIteratorStatus for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {
    fn status(&self) -> Status {
        Status::ok()
    }
}

impl LevelDBIteratorInterface for BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator {}

fn bitcoinleveldb_tablecache_snapshot_visibility_make_leveldb_iterator_from_entries_for_test_support(
    entries: &[(Vec<u8>, Vec<u8>)],
) -> *mut LevelDBIterator {
    let owned_entries = entries.to_vec();
    let iterator_interface: Box<dyn LevelDBIteratorInterface> = Box::new(
        BitcoinLeveldbTablecacheSnapshotVisibilityDeterministicLeveldbIterator::new(
            owned_entries,
        ),
    );
    let wrapper = LevelDBIterator::new(Some(iterator_interface));
    Box::into_raw(Box::new(wrapper))
}

fn bitcoinleveldb_tablecache_snapshot_visibility_unique_dbname_for_test_support(
    prefix: &str,
) -> Option<String> {
    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => handle.clone(),
        None => {
            return None;
        }
    };

    let mut base_dir = String::new();
    let directory_status = {
        let mut env_ref = env_handle.borrow_mut();
        env_ref.get_test_directory(&mut base_dir)
    };

    if !directory_status.is_ok() {
        return None;
    }

    let suffix = {
        let mut env_ref = env_handle.borrow_mut();
        env_ref.now_micros()
    };

    let mut dbname = base_dir;
    dbname.push('/');
    dbname.push_str(prefix);
    dbname.push('_');
    dbname.push_str(&suffix.to_string());

    Some(dbname)
}

fn bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
    dbname: &String,
    file_number: u64,
) {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_cleanup_table_artifacts.entry",
        phase = "enter",
        dbname_len = dbname.len(),
        file_number
    );

    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => handle.clone(),
        None => {
            return;
        }
    };

    let table_path = table_file_name(dbname, file_number);

    {
        let mut env_ref = env_handle.borrow_mut();
        let delete_file_status = env_ref.delete_file(&table_path);
        match delete_file_status.is_ok() {
            true => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                    label = "tablecache_snapshot_visibility_cleanup_table_artifacts.delete_file_ok",
                    phase = "state_transition"
                );
            }
            false => {
                warn!(
                    target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                    label = "tablecache_snapshot_visibility_cleanup_table_artifacts.delete_file_non_ok",
                    phase = "decision"
                );
            }
        }
    }

    {
        let mut env_ref = env_handle.borrow_mut();
        let delete_dir_status = env_ref.delete_dir(dbname);
        match delete_dir_status.is_ok() {
            true => {
                debug!(
                    target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                    label = "tablecache_snapshot_visibility_cleanup_table_artifacts.delete_dir_ok",
                    phase = "state_transition"
                );
            }
            false => {
                warn!(
                    target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                    label = "tablecache_snapshot_visibility_cleanup_table_artifacts.delete_dir_non_ok",
                    phase = "decision"
                );
            }
        }
    }

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_cleanup_table_artifacts.exit",
        phase = "return"
    );
}

/**
  | Sorts encoded internal-key entries in the
  | exact admission order required by the current
  | support build surface.
  |
  | Preconditions:
  | - Every entry key must already be encoded as
  |   the raw bytes that will be passed to
  |   `build_table` through this support path.
  | - This helper is coupled to the current
  |   `Options::default()` surface used by
  |   `bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support`.
  |
  | Postconditions:
  | - The input vector is reordered in ascending
  |   raw-byte order of encoded keys.
  | - No logical internal-key reinterpretation is
  |   applied at this stage.
  |
  | Forbidden drift:
  | - This helper must not silently switch to
  |   logical internal-key ordering unless the
  |   support build path also changes its
  |   comparator surface.
  */
pub fn bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_default_table_builder_surface_for_test_support(
    entries: &mut Vec<(Vec<u8>, Vec<u8>)>,
) {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_sort_encoded_internal_entries_for_default_table_builder_surface.entry",
        phase = "enter",
        entry_count = entries.len()
    );

    entries.sort_by(|lhs_entry, rhs_entry| lhs_entry.0.as_slice().cmp(rhs_entry.0.as_slice()));

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_sort_encoded_internal_entries_for_default_table_builder_surface.exit",
        phase = "return",
        entry_count = entries.len()
    );
}

/**
  | Builds a support table directly from encoded
  | internal-key entries.
  |
  | Preconditions:
  | - `dbname` must denote a directory path that
  |   the configured `Env` can create.
  | - `entries` must already represent valid table
  |   keys and values.
  |
  | Postconditions:
  | - The returned observation preserves exact
  |   build status.
  | - Successful build preserves the produced file
  |   size for subsequent table reads.
  | - The helper must sort entries in the exact
  |   order accepted by the current support build
  |   surface before invoking `build_table`.
  |
  | Forbidden drift:
  | - Logical internal-key visibility ordering
  |   must not be substituted here unless this
  |   helper also changes the comparator surface
  |   used during table construction.
  */
pub fn bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support(
    dbname: &String,
    file_number: u64,
    entries: &[(Vec<u8>, Vec<u8>)],
) -> BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation {
    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_build_table_from_encoded_entries.entry",
        phase = "enter",
        dbname_len = dbname.len(),
        file_number,
        entry_count = entries.len()
    );

    let options = Options::default();

    let env_handle = match options.env() {
        Some(handle) => handle.clone(),
        None => {
            warn!(
                target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
                label = "tablecache_snapshot_visibility_build_table_from_encoded_entries.missing_env",
                phase = "decision"
            );
            return BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::MissingConfiguredEnv;
        }
    };

    let create_dir_status = {
        let mut env_ref = env_handle.borrow_mut();
        env_ref.create_dir(dbname)
    };

    if !create_dir_status.is_ok() {
        warn!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_build_table_from_encoded_entries.create_dir_failed",
            phase = "decision"
        );
        return BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::DirectoryCreateFailed {
            status: create_dir_status,
        };
    }

    let mut sorted_entries = entries.to_vec();
    bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_default_table_builder_surface_for_test_support(
        &mut sorted_entries,
    );

    let iter_ptr =
        bitcoinleveldb_tablecache_snapshot_visibility_make_leveldb_iterator_from_entries_for_test_support(
            sorted_entries.as_slice(),
        );

    let mut meta = FileMetaData::default();
    meta.set_number(file_number);

    let mut table_cache = TableCache::new(dbname, &options, 32);

    let status = build_table(
        dbname,
        env_handle.clone(),
        &options,
        &mut table_cache,
        iter_ptr,
        &mut meta,
    );

    unsafe {
        let iter_box: Box<LevelDBIterator> = Box::from_raw(iter_ptr);
        drop(iter_box);
    }

    let observation = match status.is_ok() {
        true => BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildOk {
            file_size: *meta.file_size(),
        },
        false => BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildNotOk {
            status,
        },
    };

    trace!(
        target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
        label = "tablecache_snapshot_visibility_build_table_from_encoded_entries.exit",
        phase = "return"
    );

    observation
}

#[cfg(test)]
mod bitcoinleveldb_tablecache_snapshot_visibility_support_tests {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_tablecache_snapshot_visibility_support_orders_same_user_key_by_descending_sequence() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_orders_same_user_key_by_descending_sequence.entry",
            phase = "enter"
        );

        let user_key = String::from("alpha");
        let older_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );
        let newer_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                2,
                1,
            );

        let mut entries = vec![
            (older_key.clone(), b"v1".to_vec()),
            (newer_key.clone(), b"v2".to_vec()),
        ];

        bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_test_support(
            &mut entries,
        );

        assert_eq!(entries[0].0, newer_key);
        assert_eq!(entries[1].0, older_key);

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_orders_same_user_key_by_descending_sequence.exit",
            phase = "return"
        );
    }

    #[traced_test]
    fn bitcoinleveldb_tablecache_snapshot_visibility_support_roundtrips_single_entry_table_read() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_roundtrips_single_entry_table_read.entry",
            phase = "enter"
        );

        let dbname_option =
            bitcoinleveldb_tablecache_snapshot_visibility_unique_dbname_for_test_support(
                "bitcoinleveldb_tablecache_snapshot_visibility_single_entry",
            );

        let dbname = match dbname_option {
            Some(value) => value,
            None => {
                assert!(
                    false,
                    "snapshot-visibility support requires Options::default() to expose a usable Env and test directory"
                );
                String::new()
            }
        };

        let user_key = String::from("alpha");
        let file_number = 11;
        let encoded_entry_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );
        let entries = vec![(encoded_entry_key.clone(), b"v1".to_vec())];

        let build_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support(
                &dbname,
                file_number,
                entries.as_slice(),
            );

        let file_size = match build_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::MissingConfiguredEnv => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support requires a configured Env"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::DirectoryCreateFailed {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must create its table directory"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildOk { file_size } => {
                file_size
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildNotOk {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must build a single-entry table successfully"
                );
                0
            }
        };

        let lookup_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );

        let read_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_read_table_value_for_encoded_lookup_key_for_test_support(
                &dbname,
                file_number,
                file_size,
                lookup_key.as_slice(),
            );

        bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
            &dbname,
            file_number,
        );

        match read_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::MissingConfiguredEnv => {
                assert!(
                    false,
                    "snapshot-visibility read support requires a configured Env"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkNoValue => {
                assert!(
                    false,
                    "single-entry snapshot-visibility support table must produce one value"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkValue { value } => {
                assert_eq!(value, String::from("v1"));
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusNotOk {
                status: _status,
            } => {
                assert!(
                    false,
                    "single-entry snapshot-visibility support table read must not return a non-OK status"
                );
            }
        }

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_roundtrips_single_entry_table_read.exit",
            phase = "return"
        );
    }

    #[traced_test]
    fn bitcoinleveldb_tablecache_snapshot_visibility_support_reads_newest_visible_value_from_dual_version_table() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_reads_newest_visible_value_from_dual_version_table.entry",
            phase = "enter"
        );

        let dbname_option =
            bitcoinleveldb_tablecache_snapshot_visibility_unique_dbname_for_test_support(
                "bitcoinleveldb_tablecache_snapshot_visibility_dual_entry",
            );

        let dbname = match dbname_option {
            Some(value) => value,
            None => {
                assert!(
                    false,
                    "snapshot-visibility support requires Options::default() to expose a usable Env and test directory"
                );
                String::new()
            }
        };

        let user_key = String::from("alpha");
        let file_number = 12;
        let older_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );
        let newer_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                2,
                1,
            );
        let entries = vec![
            (older_key, b"v1".to_vec()),
            (newer_key, b"v2".to_vec()),
        ];

        let build_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support(
                &dbname,
                file_number,
                entries.as_slice(),
            );

        let file_size = match build_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::MissingConfiguredEnv => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support requires a configured Env"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::DirectoryCreateFailed {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must create its table directory"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildOk { file_size } => {
                file_size
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildNotOk {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must build a dual-version table successfully"
                );
                0
            }
        };

        let lookup_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                2,
                1,
            );

        let read_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_read_table_value_for_encoded_lookup_key_for_test_support(
                &dbname,
                file_number,
                file_size,
                lookup_key.as_slice(),
            );

        bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
            &dbname,
            file_number,
        );

        match read_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::MissingConfiguredEnv => {
                assert!(
                    false,
                    "snapshot-visibility read support requires a configured Env"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkNoValue => {
                assert!(
                    false,
                    "dual-version snapshot-visibility support table must return the newest value for the newest visible sequence"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkValue { value } => {
                assert_eq!(value, String::from("v2"));
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusNotOk {
                status: _status,
            } => {
                assert!(
                    false,
                    "dual-version snapshot-visibility support table read must not return a non-OK status for the newest visible sequence"
                );
            }
        }

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_reads_newest_visible_value_from_dual_version_table.exit",
            phase = "return"
        );
    }

    #[traced_test]
    fn bitcoinleveldb_tablecache_snapshot_visibility_support_distinguishes_visibility_order_from_default_table_builder_surface_order_for_same_user_key() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_distinguishes_visibility_order_from_default_table_builder_surface_order_for_same_user_key.entry",
            phase = "enter"
        );

        let user_key = String::from("alpha");
        let older_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );
        let newer_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                2,
                1,
            );

        let mut visibility_entries = vec![
            (older_key.clone(), b"v1".to_vec()),
            (newer_key.clone(), b"v2".to_vec()),
        ];
        let mut build_surface_entries = visibility_entries.clone();

        bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_test_support(
            &mut visibility_entries,
        );
        bitcoinleveldb_tablecache_snapshot_visibility_sort_encoded_internal_entries_for_default_table_builder_surface_for_test_support(
            &mut build_surface_entries,
        );

        assert_eq!(visibility_entries[0].0, newer_key);
        assert_eq!(visibility_entries[1].0, older_key);
        assert_eq!(build_surface_entries[0].0, older_key);
        assert_eq!(build_surface_entries[1].0, newer_key);

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_distinguishes_visibility_order_from_default_table_builder_surface_order_for_same_user_key.exit",
            phase = "return"
        );
    }

    #[traced_test]
    fn bitcoinleveldb_tablecache_snapshot_visibility_support_reads_older_visible_value_from_dual_version_table_for_older_lookup_sequence() {
        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_reads_older_visible_value_from_dual_version_table_for_older_lookup_sequence.entry",
            phase = "enter"
        );

        let dbname_option =
            bitcoinleveldb_tablecache_snapshot_visibility_unique_dbname_for_test_support(
                "bitcoinleveldb_tablecache_snapshot_visibility_dual_entry_older_lookup",
            );

        let dbname = match dbname_option {
            Some(value) => value,
            None => {
                assert!(
                    false,
                    "snapshot-visibility support requires Options::default() to expose a usable Env and test directory"
                );
                String::new()
            }
        };

        let user_key = String::from("alpha");
        let file_number = 13;
        let older_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );
        let newer_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                2,
                1,
            );
        let entries = vec![
            (older_key, b"v1".to_vec()),
            (newer_key, b"v2".to_vec()),
        ];

        let build_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_build_table_from_encoded_entries_for_test_support(
                &dbname,
                file_number,
                entries.as_slice(),
            );

        let file_size = match build_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::MissingConfiguredEnv => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support requires a configured Env"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::DirectoryCreateFailed {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must create its table directory"
                );
                0
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildOk { file_size } => {
                file_size
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityBuildObservation::BuildNotOk {
                status: _status,
            } => {
                bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
                    &dbname,
                    file_number,
                );
                assert!(
                    false,
                    "snapshot-visibility build support must build a dual-version table successfully for older lookup validation"
                );
                0
            }
        };

        let lookup_key =
            bitcoinleveldb_tablecache_snapshot_visibility_encode_internal_key_bytes_for_test_support(
                &user_key,
                1,
                1,
            );

        let read_observation =
            bitcoinleveldb_tablecache_snapshot_visibility_read_table_value_for_encoded_lookup_key_for_test_support(
                &dbname,
                file_number,
                file_size,
                lookup_key.as_slice(),
            );

        bitcoinleveldb_tablecache_snapshot_visibility_cleanup_table_artifacts_for_test_support(
            &dbname,
            file_number,
        );

        match read_observation {
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::MissingConfiguredEnv => {
                assert!(
                    false,
                    "snapshot-visibility read support requires a configured Env"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkNoValue => {
                assert!(
                    false,
                    "dual-version snapshot-visibility support table must return the older visible value for the older lookup sequence"
                );
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusOkValue { value } => {
                assert_eq!(value, String::from("v1"));
            }
            BitcoinLeveldbTablecacheSnapshotVisibilityTableReadObservation::StatusNotOk {
                status: _status,
            } => {
                assert!(
                    false,
                    "dual-version snapshot-visibility support table read must not return a non-OK status for the older visible sequence"
                );
            }
        }

        trace!(
            target: BITCOINLEVELDB_TABLECACHE_SNAPSHOT_VISIBILITY_TRACE_TARGET,
            label = "tablecache_snapshot_visibility_support_reads_older_visible_value_from_dual_version_table_for_older_lookup_sequence.exit",
            phase = "return"
        );
    }
}
