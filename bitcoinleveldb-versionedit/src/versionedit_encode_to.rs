// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_encode_to.rs ]
crate::ix!();

impl VersionEdit {

    pub fn encode_to(&self, dst: *mut String) {
        trace!("VersionEdit::encode_to: encoding version edit");

        if dst.is_null() {
            error!("VersionEdit::encode_to: dst pointer is null");
            return;
        }

        // Ensure deterministic ordering for deleted_files so that
        // EncodeTo/DecodeFrom/EncodeTo produces identical manifest bytes.
        let mut deleted_files_sorted: Vec<(i32, u64)> =
            self.deleted_files().iter().copied().collect();
        deleted_files_sorted.sort_unstable();

        unsafe {
            if *self.has_comparator() {
                // kComparator = 1
                put_varint32(dst, 1);
                let comparator_slice = Slice::from(self.comparator());
                put_length_prefixed_slice(dst, &comparator_slice);
            }
            if *self.has_log_number() {
                // kLogNumber = 2
                put_varint32(dst, 2);
                put_varint64(dst, *self.log_number());
            }
            if *self.has_prev_log_number() {
                // kPrevLogNumber = 9
                put_varint32(dst, 9);
                put_varint64(dst, *self.prev_log_number());
            }
            if *self.has_next_file_number() {
                // kNextFileNumber = 3
                put_varint32(dst, 3);
                put_varint64(dst, *self.next_file_number());
            }
            if *self.has_last_sequence() {
                // kLastSequence = 4
                put_varint32(dst, 4);
                put_varint64(dst, *self.last_sequence() as u64);
            }

            for (level, key) in self.compact_pointers() {
                // kCompactPointer = 5
                put_varint32(dst, 5);
                put_varint32(dst, *level as u32);
                let encoded = key.encode();
                put_length_prefixed_slice(dst, &encoded);
            }

            for (level, number) in deleted_files_sorted {
                // kDeletedFile = 6
                put_varint32(dst, 6);
                put_varint32(dst, level as u32);
                put_varint64(dst, number);
            }

            for (level, f) in self.new_files() {
                // kNewFile = 7
                put_varint32(dst, 7);
                put_varint32(dst, *level as u32);
                put_varint64(dst, *f.number());
                put_varint64(dst, *f.file_size());

                let smallest_encoded = f.smallest().encode();
                put_length_prefixed_slice(dst, &smallest_encoded);

                let largest_encoded = f.largest().encode();
                put_length_prefixed_slice(dst, &largest_encoded);
            }
        }

        trace!("VersionEdit::encode_to: encoding completed");
    }
}

#[cfg(test)]
mod version_edit_encode_to_tests {
    use super::*;

    fn encode_edit_to_bytes(edit: &VersionEdit) -> Vec<u8> {
        trace!("encode_edit_to_bytes(version_edit_encode_to_tests): start");

        let mut buffer = String::new();
        edit.encode_to(&mut buffer as *mut String);

        let bytes = buffer.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        out.extend_from_slice(bytes);

        debug!(
            encoded_len = out.len(),
            "encode_edit_to_bytes(version_edit_encode_to_tests): done"
        );

        out
    }

    #[traced_test]
    fn encode_to_and_decode_from_roundtrip_for_empty_edit() {
        trace!(
            "encode_to_and_decode_from_roundtrip_for_empty_edit: start"
        );

        let edit = VersionEdit::default();

        let encoded = encode_edit_to_bytes(&edit);
        let mut parsed = VersionEdit::default();

        let slice = unsafe { Slice::from_ptr_len(encoded.as_ptr(), encoded.len()) };
        let status = parsed.decode_from(&slice);

        assert!(
            status.is_ok(),
            "DecodeFrom should return OK for encoding of default VersionEdit: {}",
            status.to_string()
        );

        assert_eq!(
            edit.debug_string(),
            parsed.debug_string(),
            "debug_string of parsed edit should match original for empty edit"
        );
    }

    #[traced_test]
    fn encode_to_and_decode_from_roundtrip_for_populated_edit() {
        trace!(
            "encode_to_and_decode_from_roundtrip_for_populated_edit: start"
        );

        const SEQ_BASE: SequenceNumber = 1000;

        let mut edit = VersionEdit::default();

        let cmp_name  = String::from("cmp-populated");
        let cmp_slice = Slice::from(&cmp_name);
        edit.set_comparator_name(&cmp_slice);
        edit.set_log_number(11);
        edit.set_prev_log_number(12);
        edit.set_next_file(13);
        edit.set_last_sequence(SEQ_BASE + 1);

        let smallest = InternalKey::new(
            &Slice::from("a".as_bytes()),
            SEQ_BASE + 2,
            ValueType::TypeValue,
        );
        let largest = InternalKey::new(
            &Slice::from("z".as_bytes()),
            SEQ_BASE + 3,
            ValueType::TypeValue,
        );
        edit.add_file(1, 100, 4096, &smallest, &largest);
        edit.delete_file(2, 200);

        let compact_key = InternalKey::new(
            &Slice::from("m".as_bytes()),
            SEQ_BASE + 4,
            ValueType::TypeValue,
        );
        edit.set_compact_pointer(0, &compact_key);

        let encoded = encode_edit_to_bytes(&edit);
        let mut parsed = VersionEdit::default();

        let slice = unsafe { Slice::from_ptr_len(encoded.as_ptr(), encoded.len()) };
        let status = parsed.decode_from(&slice);

        assert!(
            status.is_ok(),
            "DecodeFrom should return OK for encoding of populated VersionEdit: {}",
            status.to_string()
        );

        assert_eq!(
            edit.debug_string(),
            parsed.debug_string(),
            "debug_string of parsed edit should match original for populated edit"
        );
    }
}
