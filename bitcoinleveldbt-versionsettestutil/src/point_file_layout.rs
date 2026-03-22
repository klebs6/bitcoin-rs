// ---------------- [ File: bitcoinleveldb-versionsettestutil/src/point_file_layout.rs ]
crate::ix!();

/// Guarantees each placement describes one file whose smallest and largest internal-key bounds
/// belong to the same user key, with `max_sequence_number` equal to the maximum decoded sequence
/// observed on those two file boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointFileLevelSequencePlacement {
    user_key: String,
    level: usize,
    file_number: u64,
    max_sequence_number: u64,
}

impl PointFileLevelSequencePlacement {
    /// Guarantees the returned key string is the exact lossy UTF-8 form used during placement collection.
    pub fn user_key(&self) -> &str {
        self.user_key.as_str()
    }

    /// Guarantees the returned level is the exact level index from the source `Version`.
    pub fn level(&self) -> usize {
        self.level
    }

    /// Guarantees the returned number is the original file number from the source metadata.
    pub fn file_number(&self) -> u64 {
        self.file_number
    }

    /// Guarantees the returned sequence number is the maximum boundary sequence observed for the file.
    pub fn max_sequence_number(&self) -> u64 {
        self.max_sequence_number
    }
}

/// Guarantees each evidence item names a user key for which a shallower file has an older maximum
/// sequence number than a deeper file, which is the forbidden cross-level inversion targeted by the
/// point-file layout detector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointFileCrossLevelSequenceInversionEvidence {
    user_key: String,
    shallower_level: usize,
    shallower_file_number: u64,
    shallower_max_sequence_number: u64,
    deeper_level: usize,
    deeper_file_number: u64,
    deeper_max_sequence_number: u64,
}

impl PointFileCrossLevelSequenceInversionEvidence {
    /// Guarantees the returned key string is the exact lossy UTF-8 form used during evidence construction.
    pub fn user_key(&self) -> &str {
        self.user_key.as_str()
    }

    /// Guarantees the returned level is strictly less than `deeper_level()`.
    pub fn shallower_level(&self) -> usize {
        self.shallower_level
    }

    /// Guarantees the returned file number belongs to the shallower placement in this evidence item.
    pub fn shallower_file_number(&self) -> u64 {
        self.shallower_file_number
    }

    /// Guarantees the returned sequence number is strictly less than `deeper_max_sequence_number()`.
    pub fn shallower_max_sequence_number(&self) -> u64 {
        self.shallower_max_sequence_number
    }

    /// Guarantees the returned level is strictly greater than `shallower_level()`.
    pub fn deeper_level(&self) -> usize {
        self.deeper_level
    }

    /// Guarantees the returned file number belongs to the deeper placement in this evidence item.
    pub fn deeper_file_number(&self) -> u64 {
        self.deeper_file_number
    }

    /// Guarantees the returned sequence number is strictly greater than `shallower_max_sequence_number()`.
    pub fn deeper_max_sequence_number(&self) -> u64 {
        self.deeper_max_sequence_number
    }
}

fn collect_point_file_user_key_and_max_sequence_number(
    file_metadata: &FileMetaData,
) -> Option<(String, u64)> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "collect_point_file_user_key_and_max_sequence_number_enter",
        file_number = *file_metadata.number()
    );

    let mut smallest_parsed_internal_key = ParsedInternalKey::default();
    let mut largest_parsed_internal_key = ParsedInternalKey::default();

    let smallest_encoded_internal_key = file_metadata.smallest().encode();
    let largest_encoded_internal_key = file_metadata.largest().encode();

    let parsed_smallest =
        parse_internal_key(&smallest_encoded_internal_key, &mut smallest_parsed_internal_key);
    let parsed_largest =
        parse_internal_key(&largest_encoded_internal_key, &mut largest_parsed_internal_key);

    match (parsed_smallest, parsed_largest) {
        (true, true) => {
            let smallest_user_key = String::from_utf8_lossy(
                smallest_parsed_internal_key.user_key().as_bytes(),
            )
            .to_string();
            let largest_user_key = String::from_utf8_lossy(
                largest_parsed_internal_key.user_key().as_bytes(),
            )
            .to_string();

            match smallest_user_key == largest_user_key {
                true => {
                    let max_sequence_number = core::cmp::max(
                        *smallest_parsed_internal_key.sequence(),
                        *largest_parsed_internal_key.sequence(),
                    );

                    trace!(
                        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
                        event = "collect_point_file_user_key_and_max_sequence_number_exit_some",
                        file_number = *file_metadata.number(),
                        user_key = smallest_user_key.as_str(),
                        max_sequence_number = max_sequence_number
                    );

                    Some((smallest_user_key, max_sequence_number))
                }
                false => {
                    trace!(
                        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
                        event = "collect_point_file_user_key_and_max_sequence_number_exit_none_not_point_file",
                        file_number = *file_metadata.number(),
                        smallest_user_key = smallest_user_key.as_str(),
                        largest_user_key = largest_user_key.as_str()
                    );
                    None
                }
            }
        }
        (false, false) | (false, true) | (true, false) => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::point_file_layout",
                event = "collect_point_file_user_key_and_max_sequence_number_parse_failure",
                file_number = *file_metadata.number(),
                parsed_smallest = parsed_smallest,
                parsed_largest = parsed_largest
            );
            None
        }
    }
}

/// Preconditions: `current_version_ptr` is either null or points to a live `Version`.
/// Postconditions: the returned vector preserves level-major traversal order over all point files in the version.
pub fn collect_point_file_level_sequence_placements_from_version(
    current_version_ptr: *mut Version,
) -> Vec<PointFileLevelSequencePlacement> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "collect_point_file_level_sequence_placements_from_version_enter",
        current_version_ptr = ?current_version_ptr
    );

    let mut placements = Vec::<PointFileLevelSequencePlacement>::new();

    match current_version_ptr.is_null() {
        true => {
            warn!(
                target: "bitcoinleveldb_versionsettestutil::point_file_layout",
                event = "collect_point_file_level_sequence_placements_from_version_null_version"
            );
        }
        false => unsafe {
            for level in 0..NUM_LEVELS {
                for &file_metadata_ptr in (*current_version_ptr).files()[level].iter() {
                    match file_metadata_ptr.is_null() {
                        true => {
                            warn!(
                                target: "bitcoinleveldb_versionsettestutil::point_file_layout",
                                event = "collect_point_file_level_sequence_placements_from_version_null_file_metadata",
                                level = level
                            );
                        }
                        false => {
                            let file_metadata = &*file_metadata_ptr;
                            match collect_point_file_user_key_and_max_sequence_number(file_metadata) {
                                Some((user_key, max_sequence_number)) => {
                                    placements.push(PointFileLevelSequencePlacement {
                                        user_key,
                                        level,
                                        file_number: *file_metadata.number(),
                                        max_sequence_number,
                                    });
                                }
                                None => {}
                            }
                        }
                    }
                }
            }
        },
    }

    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "collect_point_file_level_sequence_placements_from_version_exit",
        placement_count = placements.len()
    );

    placements
}

/// Preconditions: `current_version_ptr` is either null or points to a live `Version`.
/// Postconditions: every returned evidence item witnesses a strict shallower/deeper inversion for one user key.
pub fn collect_point_file_cross_level_sequence_inversion_evidence_from_version(
    current_version_ptr: *mut Version,
) -> Vec<PointFileCrossLevelSequenceInversionEvidence> {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "collect_point_file_cross_level_sequence_inversion_evidence_from_version_enter",
        current_version_ptr = ?current_version_ptr
    );

    let placements =
        collect_point_file_level_sequence_placements_from_version(current_version_ptr);
    let mut evidence = Vec::<PointFileCrossLevelSequenceInversionEvidence>::new();

    for shallower_index in 0..placements.len() {
        for deeper_index in 0..placements.len() {
            let shallower_placement = &placements[shallower_index];
            let deeper_placement = &placements[deeper_index];

            let same_user_key =
                shallower_placement.user_key() == deeper_placement.user_key();
            let shallower_is_above =
                shallower_placement.level() < deeper_placement.level();
            let deeper_is_newer =
                deeper_placement.max_sequence_number()
                    > shallower_placement.max_sequence_number();

            match (same_user_key, shallower_is_above, deeper_is_newer) {
                (true, true, true) => {
                    evidence.push(PointFileCrossLevelSequenceInversionEvidence {
                        user_key: shallower_placement.user_key().to_string(),
                        shallower_level: shallower_placement.level(),
                        shallower_file_number: shallower_placement.file_number(),
                        shallower_max_sequence_number:
                            shallower_placement.max_sequence_number(),
                        deeper_level: deeper_placement.level(),
                        deeper_file_number: deeper_placement.file_number(),
                        deeper_max_sequence_number:
                            deeper_placement.max_sequence_number(),
                    });
                }
                (true, true, false)
                | (true, false, true)
                | (true, false, false)
                | (false, true, true)
                | (false, true, false)
                | (false, false, true)
                | (false, false, false) => {}
            }
        }
    }

    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "collect_point_file_cross_level_sequence_inversion_evidence_from_version_exit",
        evidence_count = evidence.len()
    );

    evidence
}

/// Preconditions: `current_version_ptr` is either null or points to a live `Version`.
/// Postconditions: returns `true` iff at least one point-file cross-level sequence inversion exists.
pub fn version_contains_point_file_cross_level_sequence_inversion(
    current_version_ptr: *mut Version,
) -> bool {
    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "version_contains_point_file_cross_level_sequence_inversion_enter",
        current_version_ptr = ?current_version_ptr
    );

    let evidence =
        collect_point_file_cross_level_sequence_inversion_evidence_from_version(
            current_version_ptr,
        );

    let contains_inversion = !evidence.is_empty();

    trace!(
        target: "bitcoinleveldb_versionsettestutil::point_file_layout",
        event = "version_contains_point_file_cross_level_sequence_inversion_exit",
        contains_inversion = contains_inversion,
        evidence_count = evidence.len()
    );

    contains_inversion
}
