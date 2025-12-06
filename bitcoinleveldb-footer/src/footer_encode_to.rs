// ---------------- [ File: bitcoinleveldb-footer/src/footer_encode_to.rs ]
crate::ix!();
    
impl Footer {

    pub fn encode_to(&self, dst: *mut String) {
        unsafe {
            assert!(
                !dst.is_null(),
                "Footer::encode_to: dst pointer is null"
            );
            let s: &mut String = &mut *dst;
            let original_size = s.len();

            trace!(
                "Footer::encode_to: original_size={}, metaindex(offset={}, size={}), index(offset={}, size={})",
                original_size,
                self.metaindex_handle().offset(),
                self.metaindex_handle().size(),
                self.index_handle().offset(),
                self.index_handle().size()
            );

            self.metaindex_handle().encode_to(s);
            self.index_handle().encode_to(s);

            // Pad to fixed length for the two handles
            let target_len =
                original_size + 2 * BLOCK_HANDLE_MAX_ENCODED_LENGTH;
            {
                let buf: &mut Vec<u8> = s.as_mut_vec();
                if buf.len() < target_len {
                    buf.resize(target_len, 0u8);
                }
            }

            // Append magic number (split into low/high 32 bits)
            let lower: u32 =
                (TABLE_MAGIC_NUMBER & 0xffffffffu64) as u32;
            let upper: u32 =
                (TABLE_MAGIC_NUMBER >> 32) as u32;

            bitcoinleveldb_coding::put_fixed32(
                s as *mut String,
                lower,
            );
            bitcoinleveldb_coding::put_fixed32(
                s as *mut String,
                upper,
            );

            debug_assert_eq!(
                s.len(),
                original_size + FOOTER_ENCODED_LENGTH
            );

            trace!(
                "Footer::encode_to: dst_len_after={}, expected_added={}",
                s.len(),
                FOOTER_ENCODED_LENGTH
            );
        }
    }
}

#[cfg(test)]
mod footer_encoding_roundtrip_and_layout_tests {
    use super::*;

    fn make_block_handle(offset: u64, size: u64) -> BlockHandle {
        let mut h = BlockHandle::default();
        h.set_offset(offset);
        h.set_size(size);
        h
    }

    #[traced_test]
    fn footer_encode_decode_roundtrip_preserves_handles_and_consumes_input() {
        let metaindex = make_block_handle(111, 222);
        let index     = make_block_handle(333, 444);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        assert_eq!(
            encoded.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_encode_decode_roundtrip_preserves_handles_and_consumes_input: encoded length must equal FOOTER_ENCODED_LENGTH"
        );

        let mut slice   = Slice::from(encoded.as_bytes());
        let mut decoded = Footer::default();

        let status = decoded.decode_from(&mut slice as *mut Slice);
        assert!(
            status.is_ok(),
            "footer_encode_decode_roundtrip_preserves_handles_and_consumes_input: decode_from returned non‑OK"
        );

        assert_eq!(
            decoded.metaindex_handle().offset(),
            metaindex.offset()
        );
        assert_eq!(
            decoded.metaindex_handle().size(),
            metaindex.size()
        );
        assert_eq!(
            decoded.index_handle().offset(),
            index.offset()
        );
        assert_eq!(
            decoded.index_handle().size(),
            index.size()
        );

        assert_eq!(
            *slice.size(),
            0,
            "footer_encode_decode_roundtrip_preserves_handles_and_consumes_input: remaining slice after decode must be empty"
        );
    }

    #[traced_test]
    fn footer_decode_from_too_short_input_returns_error() {
        let short_buf = vec![0u8; FOOTER_ENCODED_LENGTH - 1];
        let mut slice = Slice::from(short_buf.as_slice());
        let mut footer = Footer::default();

        let status = footer.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_too_short_input_returns_error: expected non‑OK status for short footer"
        );
    }

    #[traced_test]
    fn footer_decode_from_bad_magic_returns_error() {
        let metaindex = make_block_handle(10, 20);
        let index     = make_block_handle(30, 40);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        let mut bytes = encoded.into_bytes();
        // Flip a bit in the magic number region (last byte).
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;

        let mut slice   = Slice::from(bytes.as_slice());
        let mut decoded = Footer::default();

        let status = decoded.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_bad_magic_returns_error: expected non‑OK status for bad magic"
        );
    }

    #[traced_test]
    fn footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer() {
        let metaindex = make_block_handle(1, 2);
        let index     = make_block_handle(3, 4);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let prefix = "prefix‑data:";
        let mut dst = prefix.to_string();
        let prefix_bytes = dst.clone().into_bytes();
        let prefix_len   = prefix_bytes.len();

        footer.encode_to(&mut dst as *mut String);

        assert_eq!(
            dst.len(),
            prefix_len + FOOTER_ENCODED_LENGTH,
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: total length must grow by FOOTER_ENCODED_LENGTH"
        );

        let dst_bytes = dst.as_bytes();

        assert_eq!(
            &dst_bytes[..prefix_len],
            &prefix_bytes[..],
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: existing prefix bytes must be untouched"
        );

        let footer_region = &dst_bytes[prefix_len..];

        assert_eq!(
            footer_region.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: footer region length must equal FOOTER_ENCODED_LENGTH"
        );

        let mut slice   = Slice::from(footer_region);
        let mut decoded = Footer::default();

        let status = decoded.decode_from(&mut slice as *mut Slice);
        assert!(
            status.is_ok(),
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: decode_from on footer region must succeed"
        );

        assert_eq!(
            decoded.metaindex_handle().offset(),
            metaindex.offset()
        );
        assert_eq!(
            decoded.metaindex_handle().size(),
            metaindex.size()
        );
        assert_eq!(
            decoded.index_handle().offset(),
            index.offset()
        );
        assert_eq!(
            decoded.index_handle().size(),
            index.size()
        );

        assert_eq!(
            *slice.size(),
            0,
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: decoding footer region must consume it entirely"
        );
    }

    #[traced_test]
    fn footer_encode_to_places_magic_number_at_tail_of_footer() {
        let metaindex = make_block_handle(7, 8);
        let index     = make_block_handle(9, 10);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(metaindex);
        footer.set_index_handle(index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        let bytes = encoded.as_bytes();
        assert_eq!(
            bytes.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_encode_to_places_magic_number_at_tail_of_footer: encoded length must equal FOOTER_ENCODED_LENGTH"
        );

        let magic_offset = FOOTER_ENCODED_LENGTH - 8;

        unsafe {
            let magic_ptr = bytes.as_ptr().add(magic_offset);

            let magic_lo = decode_fixed32(magic_ptr);
            let magic_hi = decode_fixed32(magic_ptr.add(4));

            let magic: u64 =
                ((magic_hi as u64) << 32) | (magic_lo as u64);

            assert_eq!(
                magic,
                TABLE_MAGIC_NUMBER,
                "footer_encode_to_places_magic_number_at_tail_of_footer: magic value at tail must equal TABLE_MAGIC_NUMBER"
            );
        }
    }

    #[traced_test]
    fn footer_encode_to_panics_on_null_destination_pointer() {
        use std::panic;

        let footer = Footer::default();

        let result = panic::catch_unwind(|| {
            let null_dst: *mut String = core::ptr::null_mut();
            footer.encode_to(null_dst);
        });

        assert!(
            result.is_err(),
            "footer_encode_to_panics_on_null_destination_pointer: expected panic when called with null destination pointer"
        );
    }
}
