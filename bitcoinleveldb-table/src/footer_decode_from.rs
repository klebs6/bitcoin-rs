// ---------------- [ File: bitcoinleveldb-table/src/footer_decode_from.rs ]
crate::ix!();

impl Footer {

    pub fn decode_from(&mut self, input: *mut Slice) -> Status {
        unsafe {
            assert!(
                !input.is_null(),
                "Footer::decode_from: input pointer is null"
            );
            let slice = &mut *input;

            trace!(
                "Footer::decode_from: input_size={}",
                *slice.size()
            );

            if *slice.size() < FOOTER_ENCODED_LENGTH {
                let msg = b"not an sstable (footer too short)";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "Footer::decode_from: footer too short (got {}, need {})",
                    *slice.size(),
                    FOOTER_ENCODED_LENGTH
                );
                return Status::corruption(&msg_slice, None);
            }

            let base_ptr = *slice.data();
            let total_size = *slice.size();

            // Position of magic number within the footer.
            let magic_offset = FOOTER_ENCODED_LENGTH - 8;
            let magic_ptr = base_ptr.add(magic_offset);

            let magic_lo_bytes =
                core::slice::from_raw_parts(magic_ptr, 4);
            let magic_hi_bytes =
                core::slice::from_raw_parts(magic_ptr.add(4), 4);

            let magic_lo =
                bitcoinleveldb_coding::decode_fixed32(magic_lo_bytes);
            let magic_hi =
                bitcoinleveldb_coding::decode_fixed32(magic_hi_bytes);

            let magic: u64 =
                ((magic_hi as u64) << 32) | (magic_lo as u64);

            trace!(
                "Footer::decode_from: decoded magic={:#018x}",
                magic
            );

            if magic != TABLE_MAGIC_NUMBER {
                let msg = b"not an sstable (bad magic number)";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "Footer::decode_from: bad magic number (got={:#018x}, expected={:#018x})",
                    magic,
                    TABLE_MAGIC_NUMBER
                );
                return Status::corruption(&msg_slice, None);
            }

            // Decode the two BlockHandles from the prefix before magic.
            let mut handles_slice =
                Slice::from_ptr_len(base_ptr, magic_offset);

            let mut status = self
                .metaindex_handle
                .decode_from(&mut handles_slice as *mut Slice);

            if status.is_ok() {
                status = self
                    .index_handle
                    .decode_from(&mut handles_slice as *mut Slice);
            }

            if status.is_ok() {
                // Skip over any leftover data (padding) plus the magic.
                let end_ptr = magic_ptr.add(8);
                let new_size = (base_ptr as usize + total_size)
                    .saturating_sub(end_ptr as usize);

                *slice = Slice::from_ptr_len(end_ptr, new_size);

                trace!(
                    "Footer::decode_from: success; remaining_slice_size={}",
                    *slice.size()
                );
            } else {
                debug!(
                    "Footer::decode_from: failed to decode block handles"
                );
            }

            status
        }
    }
}
