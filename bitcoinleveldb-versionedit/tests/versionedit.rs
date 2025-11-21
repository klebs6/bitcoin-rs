// ---------------- [ File: bitcoinleveldb-versionedit/tests/versionedit.rs ]
use bitcoinleveldb_versionedit::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_status::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit_test.cc]

fn encode_edit_to_bytes(edit: &VersionEdit) -> Vec<u8> {
    trace!(
        target: "versionedit",
        "encode_edit_to_bytes: encoding VersionEdit to manifest bytes"
    );

    let mut manifest_record = String::new();
    let dst_ptr: *mut String = &mut manifest_record;
    edit.encode_to(dst_ptr);

    let encoded_len = manifest_record.len();
    let encoded = manifest_record.into_bytes();

    debug!(
        target: "versionedit",
        "encode_edit_to_bytes: finished encoding, encoded_len = {}",
        encoded_len
    );

    encoded
}

fn decode_bytes_to_edit(bytes: &[u8]) -> (VersionEdit, Status) {
    trace!(
        target: "versionedit",
        "decode_bytes_to_edit: decoding manifest bytes into VersionEdit"
    );
    let slice = Slice::from(bytes);
    let mut edit = VersionEdit::default();
    let status = edit.decode_from(&slice);
    debug!(
        target: "versionedit",
        "decode_bytes_to_edit: decode_from status.is_ok()={}",
        status.is_ok()
    );
    (edit, status)
}

fn build_internal_key(user: &[u8], seq: u64, value_type: ValueType) -> InternalKey {
    trace!(
        target: "versionedit",
        "build_internal_key: user_key_len={}, seq={}, value_type={:?}",
        user.len(),
        seq,
        value_type
    );

    let user_slice = Slice::from(user);
    let sequence: SequenceNumber = seq as SequenceNumber;

    let internal_key = InternalKey::new(&user_slice, sequence, value_type);

    debug!(
        target: "versionedit",
        "build_internal_key: constructed InternalKey debug='{}'",
        internal_key.debug_string()
    );

    internal_key
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestLogicalView {
    compact_pointers: Vec<Vec<u8>>,
    deleted_files: Vec<Vec<u8>>,
    new_files: Vec<Vec<u8>>,
}

impl ManifestLogicalView {
    fn normalized(mut self) -> Self {
        trace!(
            target: "versionedit",
            "ManifestLogicalView::normalized: before sort, compact_pointers_len={}, deleted_files_len={}, new_files_len={}",
            self.compact_pointers.len(),
            self.deleted_files.len(),
            self.new_files.len()
        );
        self.deleted_files.sort();
        debug!(
            target: "versionedit",
            "ManifestLogicalView::normalized: after sort, deleted_files_len={}",
            self.deleted_files.len()
        );
        self
    }
}

fn decode_varint32_for_manifest(input: &[u8], offset: &mut usize) -> Option<u32> {
    trace!(
        target: "versionedit",
        "decode_varint32_for_manifest: start, offset={}, len={}",
        *offset,
        input.len()
    );

    let mut result: u32 = 0;
    let mut shift: u32 = 0;

    while shift <= 28 && *offset < input.len() {
        let byte = input[*offset];
        *offset += 1;

        let low_bits = (byte & 0x7F) as u32;
        result |= low_bits << shift;

        if byte < 0x80 {
            debug!(
                target: "versionedit",
                "decode_varint32_for_manifest: decoded value={}, new_offset={}",
                result,
                *offset
            );
            return Some(result);
        }

        shift += 7;
    }

    warn!(
        target: "versionedit",
        "decode_varint32_for_manifest: failed to decode varint32 (offset={}, len={})",
        *offset,
        input.len()
    );
    None
}

fn decode_varint64_for_manifest(input: &[u8], offset: &mut usize) -> Option<u64> {
    trace!(
        target: "versionedit",
        "decode_varint64_for_manifest: start, offset={}, len={}",
        *offset,
        input.len()
    );

    let mut result: u64 = 0;
    let mut shift: u32 = 0;

    while shift <= 63 && *offset < input.len() {
        let byte = input[*offset];
        *offset += 1;

        let low_bits = (byte & 0x7F) as u64;
        result |= low_bits << shift;

        if byte < 0x80 {
            debug!(
                target: "versionedit",
                "decode_varint64_for_manifest: decoded value={}, new_offset={}",
                result,
                *offset
            );
            return Some(result);
        }

        shift += 7;
    }

    warn!(
        target: "versionedit",
        "decode_varint64_for_manifest: failed to decode varint64 (offset={}, len={})",
        *offset,
        input.len()
    );
    None
}

fn decode_length_prefixed_slice_for_manifest<'a>(
    input: &'a [u8],
    offset: &mut usize,
) -> Option<&'a [u8]> {
    trace!(
        target: "versionedit",
        "decode_length_prefixed_slice_for_manifest: start, offset={}, len={}",
        *offset,
        input.len()
    );

    let length32 = decode_varint32_for_manifest(input, offset)?;
    let length = length32 as usize;

    if *offset + length > input.len() {
        warn!(
            target: "versionedit",
            "decode_length_prefixed_slice_for_manifest: slice extends beyond buffer (offset={}, slice_len={}, total_len={})",
            *offset,
            length,
            input.len()
        );
        return None;
    }

    let start = *offset;
    let end = start + length;
    *offset = end;

    debug!(
        target: "versionedit",
        "decode_length_prefixed_slice_for_manifest: decoded slice, start={}, end={}",
        start,
        end
    );

    Some(&input[start..end])
}

fn parse_manifest_bytes_to_logical_view(manifest: &[u8]) -> ManifestLogicalView {
    trace!(
        target: "versionedit",
        "parse_manifest_bytes_to_logical_view: manifest_len={}",
        manifest.len()
    );

    let mut view = ManifestLogicalView {
        compact_pointers: Vec::new(),
        deleted_files: Vec::new(),
        new_files: Vec::new(),
    };

    let mut offset: usize = 0;

    while offset < manifest.len() {
        let entry_start = offset;

        let tag = decode_varint32_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
            error!(
                target: "versionedit",
                "parse_manifest_bytes_to_logical_view: failed to decode tag at offset={} (manifest_len={})",
                entry_start,
                manifest.len()
            );
            panic!("parse_manifest_bytes_to_logical_view: failed to decode tag");
        });

        match tag {
            // Tag::CompactPointer = 5
            5 => {
                trace!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: parsing CompactPointer at offset={}",
                    entry_start
                );

                let _level =
                    decode_varint32_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode CompactPointer level at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode CompactPointer level"
                        );
                    });

                let _key_slice =
                    decode_length_prefixed_slice_for_manifest(manifest, &mut offset)
                        .unwrap_or_else(|| {
                            error!(
                                target: "versionedit",
                                "parse_manifest_bytes_to_logical_view: failed to decode CompactPointer key at offset={}",
                                offset
                            );
                            panic!(
                                "parse_manifest_bytes_to_logical_view: failed to decode CompactPointer key"
                            );
                        });

                let entry_bytes = manifest[entry_start..offset].to_vec();
                debug!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: captured CompactPointer entry, len={}",
                    entry_bytes.len()
                );
                view.compact_pointers.push(entry_bytes);
            }

            // Tag::DeletedFile = 6
            6 => {
                trace!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: parsing DeletedFile at offset={}",
                    entry_start
                );

                let _level =
                    decode_varint32_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode DeletedFile level at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode DeletedFile level"
                        );
                    });

                let _file_number =
                    decode_varint64_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode DeletedFile file number at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode DeletedFile file number"
                        );
                    });

                let entry_bytes = manifest[entry_start..offset].to_vec();
                debug!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: captured DeletedFile entry, len={}",
                    entry_bytes.len()
                );
                view.deleted_files.push(entry_bytes);
            }

            // Tag::NewFile = 7
            7 => {
                trace!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: parsing NewFile at offset={}",
                    entry_start
                );

                let _level =
                    decode_varint32_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile level at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile level"
                        );
                    });

                let _file_number =
                    decode_varint64_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile file number at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile file number"
                        );
                    });

                let _file_size =
                    decode_varint64_for_manifest(manifest, &mut offset).unwrap_or_else(|| {
                        error!(
                            target: "versionedit",
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile file size at offset={}",
                            offset
                        );
                        panic!(
                            "parse_manifest_bytes_to_logical_view: failed to decode NewFile file size"
                        );
                    });

                let _smallest =
                    decode_length_prefixed_slice_for_manifest(manifest, &mut offset)
                        .unwrap_or_else(|| {
                            error!(
                                target: "versionedit",
                                "parse_manifest_bytes_to_logical_view: failed to decode NewFile smallest key at offset={}",
                                offset
                            );
                            panic!(
                                "parse_manifest_bytes_to_logical_view: failed to decode NewFile smallest key"
                            );
                        });

                let _largest =
                    decode_length_prefixed_slice_for_manifest(manifest, &mut offset)
                        .unwrap_or_else(|| {
                            error!(
                                target: "versionedit",
                                "parse_manifest_bytes_to_logical_view: failed to decode NewFile largest key at offset={}",
                                offset
                            );
                            panic!(
                                "parse_manifest_bytes_to_logical_view: failed to decode NewFile largest key"
                            );
                        });

                let entry_bytes = manifest[entry_start..offset].to_vec();
                debug!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: captured NewFile entry, len={}",
                    entry_bytes.len()
                );
                view.new_files.push(entry_bytes);
            }

            other => {
                error!(
                    target: "versionedit",
                    "parse_manifest_bytes_to_logical_view: unexpected tag {} at offset {}",
                    other,
                    entry_start
                );
                panic!("parse_manifest_bytes_to_logical_view: unexpected tag");
            }
        }
    }

    debug!(
        target: "versionedit",
        "parse_manifest_bytes_to_logical_view: done, compact_pointers_len={}, deleted_files_len={}, new_files_len={}",
        view.compact_pointers.len(),
        view.deleted_files.len(),
        view.new_files.len()
    );

    view
}

fn run_single_roundtrip_scenario<F>(scenario_name: &str, build_edit: F)
where
    F: FnOnce() -> VersionEdit,
{
    info!(
        target: "versionedit",
        "run_single_roundtrip_scenario: starting scenario '{}'",
        scenario_name
    );

    let original_edit = build_edit();
    let manifest_before = encode_edit_to_bytes(&original_edit);

    let (decoded_edit, status) = decode_bytes_to_edit(&manifest_before);
    assert!(
        status.is_ok(),
        "decode_from returned non-OK status for scenario '{}': {:?}",
        scenario_name,
        status
    );

    let manifest_after = encode_edit_to_bytes(&decoded_edit);

    debug!(
        target: "versionedit",
        "run_single_roundtrip_scenario: scenario '{}' encoded lengths -> before={}, after={}",
        scenario_name,
        manifest_before.len(),
        manifest_after.len()
    );

    assert_eq!(
        manifest_before.len(),
        manifest_after.len(),
        "encoded manifest lengths differ for scenario '{}'",
        scenario_name
    );

    let logical_before =
        parse_manifest_bytes_to_logical_view(&manifest_before).normalized();
    let logical_after =
        parse_manifest_bytes_to_logical_view(&manifest_after).normalized();

    if logical_before != logical_after {
        error!(
            target: "versionedit",
            "run_single_roundtrip_scenario: logical manifest mismatch for scenario '{}': cp_before={}, cp_after={}, deleted_before={}, deleted_after={}, new_before={}, new_after={}",
            scenario_name,
            logical_before.compact_pointers.len(),
            logical_after.compact_pointers.len(),
            logical_before.deleted_files.len(),
            logical_after.deleted_files.len(),
            logical_before.new_files.len(),
            logical_after.new_files.len()
        );
        panic!(
            "logical manifest record did not roundtrip through DecodeFrom/EncodeTo in scenario '{}'",
            scenario_name
        );
    }

    debug!(
        target: "versionedit",
        "run_single_roundtrip_scenario: scenario '{}' completed successfully (logical manifest views matched)",
        scenario_name
    );
}

#[traced_test]
pub fn version_edit_roundtrip_encoding_decoding() {
    info!(
        target: "versionedit",
        "===== BEGIN_TEST: version_edit_roundtrip_encoding_decoding ====="
    );

    // Scenario 1: single compact pointer, single deleted file, single new file
    run_single_roundtrip_scenario("basic_edit_with_single_entries", || {
        trace!(
            target: "versionedit",
            "Scenario basic_edit_with_single_entries: constructing VersionEdit"
        );
        let mut edit = VersionEdit::default();

        // Compact pointer at level 0
        let cp0 = build_internal_key(b"cp0", 100, ValueType::TypeValue);
        edit.set_compact_pointer(0, &cp0);

        // Deleted file at level 4
        edit.delete_file(4, 200);

        // New file with simple internal key range
        let smallest = build_internal_key(b"a", 300, ValueType::TypeValue);
        let largest = build_internal_key(b"z", 400, ValueType::TypeDeletion);
        edit.add_file(3, 500, 6_400, &smallest, &largest);

        edit
    });

    // Scenario 2: multiple compact pointers, multiple deleted files, single new file
    run_single_roundtrip_scenario("multiple_compact_pointers_and_deletes", || {
        trace!(
            target: "versionedit",
            "Scenario multiple_compact_pointers_and_deletes: constructing VersionEdit"
        );
        let mut edit = VersionEdit::default();

        // Compact pointers at multiple levels with deterministic seqs
        let cp0 = build_internal_key(b"cp0", 1_000, ValueType::TypeValue);
        edit.set_compact_pointer(0, &cp0);

        let cp1 = build_internal_key(b"cp1", 1_001, ValueType::TypeValue);
        edit.set_compact_pointer(1, &cp1);

        let cp2 = build_internal_key(b"cp2", 1_002, ValueType::TypeValue);
        edit.set_compact_pointer(2, &cp2);

        // Deleted files at level 4
        edit.delete_file(4, 2_000);
        edit.delete_file(4, 2_001);
        edit.delete_file(4, 2_002);

        // Single new file
        let smallest = build_internal_key(b"k0", 3_000, ValueType::TypeValue);
        let largest = build_internal_key(b"k9", 3_100, ValueType::TypeDeletion);
        edit.add_file(2, 2_500, 16_384, &smallest, &largest);

        edit
    });

    // Scenario 3: multiple new files with distinct key ranges
    run_single_roundtrip_scenario("multiple_new_files_with_distinct_ranges", || {
        trace!(
            target: "versionedit",
            "Scenario multiple_new_files_with_distinct_ranges: constructing VersionEdit"
        );
        let mut edit = VersionEdit::default();

        // Compact pointer at level 0 to keep parity with previous tests
        let cp0 = build_internal_key(b"cpX", 10_000, ValueType::TypeValue);
        edit.set_compact_pointer(0, &cp0);

        // First new file
        let smallest_1 = build_internal_key(b"f0", 20_000, ValueType::TypeValue);
        let largest_1 = build_internal_key(b"f9", 20_100, ValueType::TypeDeletion);
        edit.add_file(1, 30_000, 4_096, &smallest_1, &largest_1);

        // Second new file
        let smallest_2 = build_internal_key(b"g0", 21_000, ValueType::TypeValue);
        let largest_2 = build_internal_key(b"g9", 21_100, ValueType::TypeDeletion);
        edit.add_file(1, 30_001, 8_192, &smallest_2, &largest_2);

        // Third new file
        let smallest_3 = build_internal_key(b"h0", 22_000, ValueType::TypeValue);
        let largest_3 = build_internal_key(b"h9", 22_100, ValueType::TypeDeletion);
        edit.add_file(2, 30_002, 12_288, &smallest_3, &largest_3);

        edit
    });

    info!(
        target: "versionedit",
        "===== END_TEST: version_edit_roundtrip_encoding_decoding ====="
    );
}
