// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_decode_from.rs ]
crate::ix!();

impl VersionEdit {

    pub fn decode_from(&mut self, src: &Slice) -> Status {
        trace!("VersionEdit::decode_from: decoding from manifest record");
        self.clear();

        // Working copy of the input Slice.
        let mut input = Slice::from_ptr_len(src.data(), src.size());

        let mut msg: Option<&'static str> = None;
        let mut tag: u32 = 0;

        // Temporary storage for parsing
        let mut level: i32 = 0;
        let mut number: u64 = 0;
        let mut f = FileMetaData::default();
        let mut str_slice = Slice::default();
        let mut key = InternalKey::default();

        while msg.is_none() && get_varint32(&mut input, &mut tag) {
            match tag {
                1 => {
                    // kComparator
                    if get_length_prefixed_slice(&mut input, &mut str_slice) {
                        self.comparator = str_slice.to_string();
                        self.has_comparator = true;
                        debug!(
                            "VersionEdit::decode_from: parsed comparator '{}'",
                            self.comparator
                        );
                    } else {
                        msg = Some("comparator name");
                    }
                }
                2 => {
                    // kLogNumber
                    if get_varint64(&mut input, &mut self.log_number) {
                        self.has_log_number = true;
                        debug!(
                            "VersionEdit::decode_from: parsed log_number={}",
                            self.log_number
                        );
                    } else {
                        msg = Some("log number");
                    }
                }
                9 => {
                    // kPrevLogNumber
                    if get_varint64(&mut input, &mut self.prev_log_number) {
                        self.has_prev_log_number = true;
                        debug!(
                            "VersionEdit::decode_from: parsed prev_log_number={}",
                            self.prev_log_number
                        );
                    } else {
                        msg = Some("previous log number");
                    }
                }
                3 => {
                    // kNextFileNumber
                    if get_varint64(&mut input, &mut self.next_file_number) {
                        self.has_next_file_number = true;
                        debug!(
                            "VersionEdit::decode_from: parsed next_file_number={}",
                            self.next_file_number
                        );
                    } else {
                        msg = Some("next file number");
                    }
                }
                4 => {
                    // kLastSequence
                    let mut seq: u64 = 0;
                    if get_varint64(&mut input, &mut seq) {
                        self.has_last_sequence = true;
                        self.last_sequence = seq as SequenceNumber;
                        debug!(
                            "VersionEdit::decode_from: parsed last_sequence={}",
                            self.last_sequence
                        );
                    } else {
                        msg = Some("last sequence number");
                    }
                }
                5 => {
                    // kCompactPointer
                    if get_level(&mut input, &mut level) && get_internal_key(&mut input, &mut key)
                    {
                        debug!(
                            "VersionEdit::decode_from: parsed compact pointer level={} key={:?}",
                            level, key
                        );
                        self.compact_pointers.push((level, key.clone()));
                    } else {
                        msg = Some("compaction pointer");
                    }
                }
                6 => {
                    // kDeletedFile
                    if get_level(&mut input, &mut level) && get_varint64(&mut input, &mut number) {
                        debug!(
                            "VersionEdit::decode_from: parsed deleted file level={} file={}",
                            level, number
                        );
                        self.deleted_files.insert((level, number));
                    } else {
                        msg = Some("deleted file");
                    }
                }
                7 => {
                    // kNewFile
                    f = FileMetaData::default();
                    if get_level(&mut input, &mut level)
                        && get_varint64(&mut input, &mut f.number)
                        && get_varint64(&mut input, &mut f.file_size)
                        && get_internal_key(&mut input, &mut f.smallest)
                        && get_internal_key(&mut input, &mut f.largest)
                    {
                        debug!(
                            "VersionEdit::decode_from: parsed new file level={} number={} size={}",
                            level, f.number, f.file_size
                        );
                        self.new_files.push((level, f));
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
            // TODO: When bitcoinleveldb-status exposes a specific constructor
            // for corruption statuses, return that here instead of default.
        } else {
            trace!("VersionEdit::decode_from: decode completed successfully");
        }

        Status::default()
    }
}
