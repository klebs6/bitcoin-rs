// ---------------- [ File: bitcoinleveldb-versionedit/src/versionedit_encode_to.rs ]
crate::ix!();

impl VersionEdit {
    
    pub fn encode_to(&self, dst: *mut String) {
        trace!("VersionEdit::encode_to: encoding version edit");

        if dst.is_null() {
            error!("VersionEdit::encode_to: dst pointer is null");
            return;
        }

        unsafe {
            let dst_ref: &mut String = &mut *dst;

            if self.has_comparator {
                // kComparator = 1
                put_varint32(dst_ref, 1);
                let comparator_slice = Slice::from(&self.comparator);
                put_length_prefixed_slice(dst_ref, &comparator_slice);
            }
            if self.has_log_number {
                // kLogNumber = 2
                put_varint32(dst_ref, 2);
                put_varint64(dst_ref, self.log_number);
            }
            if self.has_prev_log_number {
                // kPrevLogNumber = 9
                put_varint32(dst_ref, 9);
                put_varint64(dst_ref, self.prev_log_number);
            }
            if self.has_next_file_number {
                // kNextFileNumber = 3
                put_varint32(dst_ref, 3);
                put_varint64(dst_ref, self.next_file_number);
            }
            if self.has_last_sequence {
                // kLastSequence = 4
                put_varint32(dst_ref, 4);
                put_varint64(dst_ref, self.last_sequence as u64);
            }

            for (level, key) in &self.compact_pointers {
                // kCompactPointer = 5
                put_varint32(dst_ref, 5);
                put_varint32(dst_ref, *level as u32);
                let encoded = key.encode();
                put_length_prefixed_slice(dst_ref, &encoded);
            }

            for (level, number) in &self.deleted_files {
                // kDeletedFile = 6
                put_varint32(dst_ref, 6);
                put_varint32(dst_ref, *level as u32);
                put_varint64(dst_ref, *number);
            }

            for (level, f) in &self.new_files {
                // kNewFile = 7
                put_varint32(dst_ref, 7);
                put_varint32(dst_ref, *level as u32);
                put_varint64(dst_ref, f.number);
                put_varint64(dst_ref, f.file_size);
                let smallest_encoded = f.smallest.encode();
                put_length_prefixed_slice(dst_ref, &smallest_encoded);
                let largest_encoded = f.largest.encode();
                put_length_prefixed_slice(dst_ref, &largest_encoded);
            }
        }

        trace!("VersionEdit::encode_to: encoding completed");
    }
}
