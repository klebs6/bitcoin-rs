// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_decode_from.rs ]
crate::ix!();

impl VersionEdit {

    pub fn decode_from(&mut self, src: &Slice) -> Status {
        trace!("VersionEdit::decode_from: decoding from manifest record");

        self.reset_core_state();

        // Working copy of the input Slice.
        let mut input = unsafe {
            Slice::from_ptr_len(*src.data(), *src.size())
        };

        let mut msg: Option<&'static str> = None;
        let mut tag: u32 = 0;

        // Temporary storage for parsing
        let mut level: i32      = 0;
        let mut number: u64     = 0;
        let mut str_slice       = Slice::default();
        let mut key             = InternalKey::default();

        while msg.is_none() && get_varint32(&mut input, &mut tag) {
            match tag {
                1 => {
                    // kComparator
                    if get_length_prefixed_slice(&mut input, &mut str_slice) {
                        self.set_comparator_name(&str_slice);
                        debug!(
                            "VersionEdit::decode_from: parsed comparator '{}'",
                            str_slice.to_string()
                        );
                    } else {
                        msg = Some("comparator name");
                    }
                }
                2 => {
                    // kLogNumber
                    let mut log_num: u64 = 0;
                    if get_varint64(&mut input, &mut log_num) {
                        self.set_log_number(log_num);
                        debug!(
                            "VersionEdit::decode_from: parsed log_number={}",
                            self.log_number()
                        );
                    } else {
                        msg = Some("log number");
                    }
                }
                9 => {
                    // kPrevLogNumber
                    let mut prev_log: u64 = 0;
                    if get_varint64(&mut input, &mut prev_log) {
                        self.set_prev_log_number(prev_log);
                        debug!(
                            "VersionEdit::decode_from: parsed prev_log_number={}",
                            self.prev_log_number()
                        );
                    } else {
                        msg = Some("previous log number");
                    }
                }
                3 => {
                    // kNextFileNumber
                    let mut next_file: u64 = 0;
                    if get_varint64(&mut input, &mut next_file) {
                        self.set_next_file(next_file);
                        debug!(
                            "VersionEdit::decode_from: parsed next_file_number={}",
                            self.next_file_number()
                        );
                    } else {
                        msg = Some("next file number");
                    }
                }
                4 => {
                    // kLastSequence
                    let mut seq: u64 = 0;
                    if get_varint64(&mut input, &mut seq) {
                        self.set_last_sequence(seq as SequenceNumber);
                        debug!(
                            "VersionEdit::decode_from: parsed last_sequence={}",
                            self.last_sequence()
                        );
                    } else {
                        msg = Some("last sequence number");
                    }
                }
                5 => {
                    // kCompactPointer
                    if get_level(&mut input, &mut level)
                        && get_internal_key(&mut input, &mut key)
                    {
                        debug!(
                            "VersionEdit::decode_from: parsed compact pointer level={} key={:?}",
                            level,
                            key
                        );
                        self.set_compact_pointer(level, &key);
                    } else {
                        msg = Some("compaction pointer");
                    }
                }
                6 => {
                    // kDeletedFile
                    if get_level(&mut input, &mut level)
                        && get_varint64(&mut input, &mut number)
                    {
                        debug!(
                            "VersionEdit::decode_from: parsed deleted file level={} file={}",
                            level,
                            number
                        );
                        self.delete_file(level, number);
                    } else {
                        msg = Some("deleted file");
                    }
                }
                7 => {
                    // kNewFile
                    let mut file_level:  i32 = 0;
                    let mut file_number: u64 = 0;
                    let mut file_size:   u64 = 0;
                    let mut smallest_key      = InternalKey::default();
                    let mut largest_key       = InternalKey::default();

                    if get_level(&mut input, &mut file_level)
                        && get_varint64(&mut input, &mut file_number)
                        && get_varint64(&mut input, &mut file_size)
                        && get_internal_key(&mut input, &mut smallest_key)
                        && get_internal_key(&mut input, &mut largest_key)
                    {
                        let mut meta = FileMetaData::default();
                        meta.set_number(file_number);
                        meta.set_file_size(file_size);
                        meta.set_smallest(smallest_key.clone());
                        meta.set_largest(largest_key.clone());

                        debug!(
                            "VersionEdit::decode_from: parsed new file level={} number={} size={}",
                            file_level,
                            file_number,
                            file_size
                        );

                        self.new_files_mut().push((file_level, meta));
                    } else {
                        msg = Some("new-file entry");
                    }
                }
                _ => {
                    msg = Some("unknown tag");
                }
            }
        }

        if msg.is_none() && !input.empty() {
            msg = Some("invalid tag");
        }

        if let Some(reason) = msg {
            error!(
                "VersionEdit::decode_from: manifest record corruption: {}",
                reason
            );
            let context_slice = Slice::from("manifest parse error".as_bytes());
            let reason_slice  = Slice::from(reason.as_bytes());
            Status::corruption(&context_slice, Some(&reason_slice))
        } else {
            trace!("VersionEdit::decode_from: decode completed successfully");
            Status::ok()
        }
    }

}

#[cfg(test)]
mod version_edit_decode_from_tests {
    use super::*;

    #[traced_test]
    fn decode_from_reports_corruption_for_truncated_comparator_name() {
        trace!(
            "decode_from_reports_corruption_for_truncated_comparator_name: start"
        );

        // Tag 1 (kComparator) followed by length=3 but only 2 bytes payload.
        let mut bytes = Vec::new();
        bytes.push(1u8);  // tag kComparator
        bytes.push(3u8);  // length = 3
        bytes.push(b'a');
        bytes.push(b'b');

        let slice = unsafe { Slice::from_ptr_len(bytes.as_ptr(), bytes.len()) };
        let mut edit = VersionEdit::default();

        let status = edit.decode_from(&slice);

        assert!(
            status.is_corruption(),
            "DecodeFrom should return Corruption for truncated comparator payload, got: {}",
            status.to_string()
        );
    }

    #[traced_test]
    fn decode_from_reports_corruption_for_invalid_varint_tag_encoding() {
        trace!(
            "decode_from_reports_corruption_for_invalid_varint_tag_encoding: start"
        );

        // A single byte 0x80 encodes a varint with continuation bit set but no
        // following byte. get_varint32 will fail and we should surface Corruption.
        let bytes = vec![0x80u8];

        let slice = unsafe { Slice::from_ptr_len(bytes.as_ptr(), bytes.len()) };
        let mut edit = VersionEdit::default();

        let status = edit.decode_from(&slice);

        assert!(
            status.is_corruption(),
            "DecodeFrom should return Corruption for invalid varint tag encoding, got: {}",
            status.to_string()
        );
    }
}
