// ---------------- [ File: bitcoinleveldb-footer/src/footer_decode_from.rs ]
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
                let msg       = b"not an sstable (footer too short)";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "Footer::decode_from: footer too short (got {}, need {})",
                    *slice.size(),
                    FOOTER_ENCODED_LENGTH
                );
                return Status::corruption(&msg_slice, None);
            }

            let base_ptr   = *slice.data();
            let total_size = *slice.size();

            // Position of magic number within the footer: the last 8 bytes.
            let magic_offset = FOOTER_ENCODED_LENGTH - 8;
            let magic_ptr    = base_ptr.add(magic_offset);

            // Footer magic is stored as two little‑endian fixed32 values:
            // low 32 bits followed by high 32 bits.
            let magic_lo_bytes =
                core::slice::from_raw_parts(magic_ptr, 4);
            let magic_hi_bytes =
                core::slice::from_raw_parts(magic_ptr.add(4), 4);

            let magic_lo =
                decode_fixed32(magic_lo_bytes.as_ptr());
            let magic_hi =
                decode_fixed32(magic_hi_bytes.as_ptr());

            let magic: u64 =
                ((magic_hi as u64) << 32) | (magic_lo as u64);

            trace!(
                "Footer::decode_from: decoded magic={:#018x}",
                magic
            );

            if magic != TABLE_MAGIC_NUMBER {
                let msg       = b"not an sstable (bad magic number)";
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

            // Preserve existing accessor tracing while mutating the underlying fields.
            let _ = self.metaindex_handle();
            let mut status = self
                .metaindex_handle_mut()
                .decode_from(&mut handles_slice as *mut Slice);

            if status.is_ok() {
                let _ = self.index_handle();
                status = self
                    .index_handle_mut()
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

#[cfg(test)]
mod footer_decode_from_slice_behavior_tests {
    use super::*;

    fn make_block_handle_for_decode_tests(offset: u64, size: u64) -> BlockHandle {
        let mut h = BlockHandle::default();
        h.set_offset(offset);
        h.set_size(size);
        h
    }

    fn collect_slice_bytes(slice: &Slice) -> Vec<u8> {
        unsafe {
            let ptr = *slice.data();
            let len = *slice.size();
            if ptr.is_null() || len == 0 {
                return Vec::new();
            }
            let bytes = core::slice::from_raw_parts(ptr, len);
            bytes.to_vec()
        }
    }

    #[traced_test]
    fn footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice() {
        let metaindex = make_block_handle_for_decode_tests(10, 20);
        let index     = make_block_handle_for_decode_tests(30, 40);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        assert_eq!(
            encoded.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: base encoded footer length must equal FOOTER_ENCODED_LENGTH"
        );

        let trailing: &[u8] = b"TRAIL";
        let mut encoded_with_trailing = encoded.into_bytes();
        encoded_with_trailing.extend_from_slice(trailing);

        let mut slice = Slice::from(encoded_with_trailing.as_slice());
        let mut decoded = Footer::default();

        let status = decoded.decode_from(&mut slice as *mut Slice);

        assert!(
            status.is_ok(),
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: decode_from must succeed"
        );

        assert_eq!(
            decoded.metaindex_handle().offset(),
            metaindex.offset(),
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: metaindex offset must roundtrip"
        );
        assert_eq!(
            decoded.metaindex_handle().size(),
            metaindex.size(),
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: metaindex size must roundtrip"
        );
        assert_eq!(
            decoded.index_handle().offset(),
            index.offset(),
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: index offset must roundtrip"
        );
        assert_eq!(
            decoded.index_handle().size(),
            index.size(),
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: index size must roundtrip"
        );

        let remaining_bytes = collect_slice_bytes(&slice);

        assert_eq!(
            remaining_bytes.as_slice(),
            trailing,
            "footer_decode_from_with_trailing_bytes_leaves_trailing_bytes_in_input_slice: remaining slice must exactly equal trailing bytes"
        );
    }

    #[traced_test]
    fn footer_decode_from_too_short_input_does_not_modify_slice_contents() {
        let original_bytes = vec![0u8; FOOTER_ENCODED_LENGTH - 1];
        let mut slice      = Slice::from(original_bytes.as_slice());

        let before_bytes = collect_slice_bytes(&slice);

        let mut footer = Footer::default();
        let status     = footer.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_too_short_input_does_not_modify_slice_contents: expected non‑OK status for too short footer"
        );

        let after_bytes = collect_slice_bytes(&slice);

        assert_eq!(
            before_bytes, after_bytes,
            "footer_decode_from_too_short_input_does_not_modify_slice_contents: slice contents must be unchanged on failure"
        );
    }

    #[traced_test]
    fn footer_decode_from_bad_magic_does_not_modify_slice_pointer_or_length() {
        let metaindex = make_block_handle_for_decode_tests(10, 20);
        let index     = make_block_handle_for_decode_tests(30, 40);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        let mut bytes = encoded.into_bytes();
        let last      = bytes.len() - 1;
        bytes[last] ^= 0xFF;

        let mut slice = Slice::from(bytes.as_slice());

        let before_ptr  = *slice.data();
        let before_size = *slice.size();
        let before_bytes = collect_slice_bytes(&slice);

        let mut decoded = Footer::default();
        let status = decoded.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_bad_magic_does_not_modify_slice_pointer_or_length: expected non‑OK status for bad magic"
        );

        let after_ptr   = *slice.data();
        let after_size  = *slice.size();
        let after_bytes = collect_slice_bytes(&slice);

        assert_eq!(
            before_ptr, after_ptr,
            "footer_decode_from_bad_magic_does_not_modify_slice_pointer_or_length: data pointer must not change on failure"
        );
        assert_eq!(
            before_size, after_size,
            "footer_decode_from_bad_magic_does_not_modify_slice_pointer_or_length: size must not change on failure"
        );
        assert_eq!(
            before_bytes, after_bytes,
            "footer_decode_from_bad_magic_does_not_modify_slice_pointer_or_length: slice contents must not change on failure"
        );
    }

    #[traced_test]
    fn footer_decode_from_panics_on_null_input_pointer() {
        use std::panic;

        let mut footer = Footer::default();

        let result = panic::catch_unwind(move || {
            let null_ptr: *mut Slice = core::ptr::null_mut();
            let _ = footer.decode_from(null_ptr);
        });

        assert!(
            result.is_err(),
            "footer_decode_from_panics_on_null_input_pointer: expected panic when called with null pointer"
        );
    }
}
