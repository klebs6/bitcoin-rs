// ---------------- [ File: bitcoinleveldb-version/src/t_version_read_order_specifications.rs ]
crate::ix!();

#[cfg(test)]
pub type VersionReadOrderVisitTraceEntry = (i32, u64, u64, ValueType);

#[cfg(test)]
pub fn build_empty_version_for_read_order_specifications() -> Version {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "build_empty_version_for_read_order_specifications_enter"
    );

    let files: [Vec<*mut FileMetaData>; NUM_LEVELS] =
        core::array::from_fn(|_| Vec::new());

    let mock_version_set_box = Box::new(MockVersionSet::new());
    let mock_version_set_trait_object: Box<dyn VersionSetInterface> =
        mock_version_set_box;
    let mock_version_set_ptr: *mut dyn VersionSetInterface =
        Box::into_raw(mock_version_set_trait_object);

    let built_version = VersionBuilder::default()
        .vset(mock_version_set_ptr)
        .next(core::ptr::null_mut())
        .prev(core::ptr::null_mut())
        .refs(0)
        .files(files)
        .file_to_compact(core::ptr::null_mut())
        .file_to_compact_level(0)
        .compaction_score(0.0)
        .compaction_level(0)
        .build();

    match built_version {
        Ok(version) => {
            trace!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "build_empty_version_for_read_order_specifications_exit"
            );
            version
        }
        Err(error) => {
            error!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "build_empty_version_for_read_order_specifications_build_error",
                error = ?error
            );
            panic!("build_empty_version_for_read_order_specifications_build_error");
        }
    }
}

#[cfg(test)]
pub fn allocate_exact_point_file_metadata_for_read_order_specifications(
    file_number: u64,
    user_key: &str,
    sequence_number: u64,
    value_type: ValueType,
) -> *mut FileMetaData {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "allocate_exact_point_file_metadata_for_read_order_specifications_enter",
        file_number = file_number,
        user_key = user_key,
        sequence_number = sequence_number,
        value_type = ?value_type
    );

    let point_internal_key =
        InternalKey::new(&Slice::from(user_key), sequence_number, value_type);

    let mut file_metadata = Box::new(FileMetaData::default());
    file_metadata.set_refs(1);
    file_metadata.set_allowed_seeks(1 << 30);
    file_metadata.set_number(file_number);
    file_metadata.set_file_size(1);
    file_metadata.set_smallest(point_internal_key.clone());
    file_metadata.set_largest(point_internal_key);

    let file_metadata_ptr = Box::into_raw(file_metadata);

    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "allocate_exact_point_file_metadata_for_read_order_specifications_exit",
        file_number = file_number,
        file_metadata_ptr = ?file_metadata_ptr
    );

    file_metadata_ptr
}

#[cfg(test)]
pub fn extract_exact_point_sequence_number_and_value_type_for_read_order_specifications(
    file_metadata_ptr: *mut FileMetaData,
) -> (u64, ValueType) {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_enter",
        file_metadata_ptr = ?file_metadata_ptr
    );

    match file_metadata_ptr.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_null_file_metadata"
            );
            panic!("extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_null_file_metadata");
        }
        false => {}
    }

    let smallest_encoded = unsafe { (*file_metadata_ptr).smallest().encode() };
    let largest_encoded = unsafe { (*file_metadata_ptr).largest().encode() };

    let mut smallest_parsed = ParsedInternalKey::default();
    let mut largest_parsed = ParsedInternalKey::default();

    let smallest_ok = parse_internal_key(
        &smallest_encoded,
        &mut smallest_parsed as *mut ParsedInternalKey,
    );
    let largest_ok = parse_internal_key(
        &largest_encoded,
        &mut largest_parsed as *mut ParsedInternalKey,
    );

    match (smallest_ok, largest_ok) {
        (true, true) => {}
        _ => {
            error!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_parse_error",
                smallest_ok = smallest_ok,
                largest_ok = largest_ok
            );
            panic!("extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_parse_error");
        }
    }

    match (
        smallest_parsed.user_key().as_bytes() == largest_parsed.user_key().as_bytes(),
        *smallest_parsed.sequence() == *largest_parsed.sequence(),
        *smallest_parsed.ty() == *largest_parsed.ty(),
    ) {
        (true, true, true) => {}
        _ => {
            error!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_not_exact_point_file"
            );
            panic!("extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_not_exact_point_file");
        }
    }

    let result = (*smallest_parsed.sequence(), *smallest_parsed.ty());

    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "extract_exact_point_sequence_number_and_value_type_for_read_order_specifications_exit",
        sequence_number = result.0,
        value_type = ?result.1
    );

    result
}

#[cfg(test)]
pub fn record_overlapping_visit_for_read_order_specifications(
    arg: *mut c_void,
    level: i32,
    file_metadata_ptr: *mut FileMetaData,
) -> bool {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "record_overlapping_visit_for_read_order_specifications_enter",
        arg = ?arg,
        level = level,
        file_metadata_ptr = ?file_metadata_ptr
    );

    match arg.is_null() {
        true => {
            error!(
                target: "bitcoinleveldb_version::t_version_read_order_specifications",
                event = "record_overlapping_visit_for_read_order_specifications_null_arg"
            );
            panic!("record_overlapping_visit_for_read_order_specifications_null_arg");
        }
        false => {}
    }

    let visit_trace: &mut Vec<VersionReadOrderVisitTraceEntry> =
        unsafe { &mut *(arg as *mut Vec<VersionReadOrderVisitTraceEntry>) };

    let file_number = unsafe { *(*file_metadata_ptr).number() };
    let (sequence_number, value_type) =
        extract_exact_point_sequence_number_and_value_type_for_read_order_specifications(
            file_metadata_ptr,
        );

    visit_trace.push((level, file_number, sequence_number, value_type));

    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "record_overlapping_visit_for_read_order_specifications_exit",
        level = level,
        file_number = file_number,
        sequence_number = sequence_number,
        value_type = ?value_type,
        visit_count = visit_trace.len()
    );

    true
}

#[cfg(test)]
pub fn record_first_overlapping_visit_and_stop_for_read_order_specifications(
    arg: *mut c_void,
    level: i32,
    file_metadata_ptr: *mut FileMetaData,
) -> bool {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "record_first_overlapping_visit_and_stop_for_read_order_specifications_enter",
        level = level,
        file_metadata_ptr = ?file_metadata_ptr
    );

    let _ = record_overlapping_visit_for_read_order_specifications(
        arg,
        level,
        file_metadata_ptr,
    );

    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "record_first_overlapping_visit_and_stop_for_read_order_specifications_exit",
        level = level
    );

    false
}

#[cfg(test)]
pub fn collect_overlapping_visit_trace_for_read_order_specifications(
    version: &mut Version,
    user_key: &str,
    sequence_number: u64,
    stop_after_first_match: bool,
) -> Vec<VersionReadOrderVisitTraceEntry> {
    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "collect_overlapping_visit_trace_for_read_order_specifications_enter",
        user_key = user_key,
        sequence_number = sequence_number,
        stop_after_first_match = stop_after_first_match
    );

    let user_key_slice = Slice::from(user_key);
    let lookup_key = LookupKey::new(&user_key_slice, sequence_number);

    let callback: fn(*mut c_void, i32, *mut FileMetaData) -> bool =
        match stop_after_first_match {
            true => record_first_overlapping_visit_and_stop_for_read_order_specifications,
            false => record_overlapping_visit_for_read_order_specifications,
        };

    let mut visit_trace: Vec<VersionReadOrderVisitTraceEntry> = Vec::new();

    version.for_each_overlapping(
        &lookup_key.user_key(),
        &lookup_key.internal_key(),
        &mut visit_trace as *mut Vec<VersionReadOrderVisitTraceEntry> as *mut c_void,
        callback,
    );

    trace!(
        target: "bitcoinleveldb_version::t_version_read_order_specifications",
        event = "collect_overlapping_visit_trace_for_read_order_specifications_exit",
        visit_count = visit_trace.len()
    );

    visit_trace
}

#[cfg(test)]
mod version_read_order_specifications {
    use super::*;

    #[traced_test]
    fn for_each_overlapping_visits_level_zero_files_from_newest_to_oldest_for_same_user_key() {
        let mut version =
            build_empty_version_for_read_order_specifications();

        let older_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                100,
                "k",
                100,
                ValueType::TypeValue,
            );
        let newer_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                200,
                "k",
                200,
                ValueType::TypeValue,
            );

        version.files_mut()[0].push(older_file_metadata_ptr);
        version.files_mut()[0].push(newer_file_metadata_ptr);

        let visit_trace =
            collect_overlapping_visit_trace_for_read_order_specifications(
                &mut version,
                "k",
                300,
                false,
            );

        assert_eq!(
            visit_trace,
            vec![
                (0, 200, 200, ValueType::TypeValue),
                (0, 100, 100, ValueType::TypeValue),
            ],
            "level-0 overlap traversal must be newest-first by file number"
        );
    }

    #[traced_test]
    fn for_each_overlapping_visits_higher_levels_from_shallow_to_deep_for_same_user_key() {
        let mut version =
            build_empty_version_for_read_order_specifications();

        let level_one_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                101,
                "k",
                100,
                ValueType::TypeValue,
            );
        let level_two_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                202,
                "k",
                200,
                ValueType::TypeValue,
            );

        version.files_mut()[1].push(level_one_file_metadata_ptr);
        version.files_mut()[2].push(level_two_file_metadata_ptr);

        let visit_trace =
            collect_overlapping_visit_trace_for_read_order_specifications(
                &mut version,
                "k",
                300,
                false,
            );

        assert_eq!(
            visit_trace,
            vec![
                (1, 101, 100, ValueType::TypeValue),
                (2, 202, 200, ValueType::TypeValue),
            ],
            "higher-level overlap traversal must be shallow-to-deep"
        );
    }

    #[traced_test]
    fn first_overlapping_candidate_is_older_shallower_value_when_layout_is_cross_level_inverted() {
        let mut version =
            build_empty_version_for_read_order_specifications();

        let older_shallower_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                101,
                "k",
                100,
                ValueType::TypeValue,
            );
        let newer_deeper_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                202,
                "k",
                200,
                ValueType::TypeValue,
            );

        version.files_mut()[1].push(older_shallower_file_metadata_ptr);
        version.files_mut()[2].push(newer_deeper_file_metadata_ptr);

        let first_candidate_only =
            collect_overlapping_visit_trace_for_read_order_specifications(
                &mut version,
                "k",
                300,
                true,
            );

        assert_eq!(
            first_candidate_only,
            vec![(1, 101, 100, ValueType::TypeValue)],
            "an inverted cross-level point-file layout must present the older shallow file first"
        );
    }

    #[traced_test]
    fn first_overlapping_candidate_is_newer_shallower_value_when_layout_is_monotone() {
        let mut version =
            build_empty_version_for_read_order_specifications();

        let newer_shallower_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                101,
                "k",
                300,
                ValueType::TypeValue,
            );
        let older_deeper_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                202,
                "k",
                200,
                ValueType::TypeValue,
            );

        version.files_mut()[1].push(newer_shallower_file_metadata_ptr);
        version.files_mut()[2].push(older_deeper_file_metadata_ptr);

        let first_candidate_only =
            collect_overlapping_visit_trace_for_read_order_specifications(
                &mut version,
                "k",
                400,
                true,
            );

        assert_eq!(
            first_candidate_only,
            vec![(1, 101, 300, ValueType::TypeValue)],
            "a monotone cross-level point-file layout must still present the shallow file first, and it must be the newest one"
        );
    }

    #[traced_test]
    fn first_overlapping_candidate_can_be_older_shallower_tombstone_before_newer_deeper_value() {
        let mut version =
            build_empty_version_for_read_order_specifications();

        let older_shallower_tombstone_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                101,
                "k",
                100,
                ValueType::TypeDeletion,
            );
        let newer_deeper_value_file_metadata_ptr =
            allocate_exact_point_file_metadata_for_read_order_specifications(
                202,
                "k",
                200,
                ValueType::TypeValue,
            );

        version.files_mut()[1].push(older_shallower_tombstone_file_metadata_ptr);
        version.files_mut()[2].push(newer_deeper_value_file_metadata_ptr);

        let first_candidate_only =
            collect_overlapping_visit_trace_for_read_order_specifications(
                &mut version,
                "k",
                300,
                true,
            );

        assert_eq!(
            first_candidate_only,
            vec![(1, 101, 100, ValueType::TypeDeletion)],
            "an inverted cross-level layout can present an older shallow tombstone before a newer deep value"
        );
    }
}
