// ---------------- [ File: bitcoinleveldb-footer/src/footer_encode_to.rs ]
crate::ix!();

impl Footer {

    /// Append one fixed-size footer to a binary destination.
    /// `dst` must point to a live, exclusively accessible `Vec<u8>`.
    pub fn encode_to(&self, dst: *mut Vec<u8>) {
        assert!(
            !dst.is_null(),
            "Footer::encode_to: dst pointer is null"
        );

        // Construct the complete footer separately from the caller's prefix.
        // Block handles occupy at most forty bytes; padding follows both
        // handles, before the eight-byte little-endian table magic.
        let mut encoded = Vec::with_capacity(FOOTER_ENCODED_LENGTH);
        self.metaindex_handle().encode_to(&mut encoded as *mut Vec<u8>);
        self.index_handle().encode_to(&mut encoded as *mut Vec<u8>);
        let padding_target = 2 * BLOCK_HANDLE_MAX_ENCODED_LENGTH;
        assert!(
            encoded.len() <= padding_target,
            "Footer::encode_to: handle encoding exceeded maximum length"
        );
        encoded.resize(padding_target, 0);

        let magic_lo = (TABLE_MAGIC_NUMBER & 0xffff_ffffu64) as u32;
        let magic_hi = (TABLE_MAGIC_NUMBER >> 32) as u32;
        put_fixed32_bytes(&mut encoded, magic_lo);
        put_fixed32_bytes(&mut encoded, magic_hi);
        debug_assert_eq!(encoded.len(), FOOTER_ENCODED_LENGTH);

        // SAFETY: the public pointer contract requires a live, exclusive Vec.
        let output = unsafe { &mut *dst };
        trace!(
            target: "bitcoinleveldb_footer::footer_encode_to",
            original_size = output.len(),
            encoded_size = encoded.len(),
            "Footer::encode_to: appending binary footer"
        );
        output.extend_from_slice(&encoded);
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

        let mut encoded = Vec::<u8>::new();
        footer.encode_to(&mut encoded as *mut Vec<u8>);

        assert_eq!(
            encoded.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_encode_decode_roundtrip_preserves_handles_and_consumes_input: encoded length must equal FOOTER_ENCODED_LENGTH"
        );

        let mut slice   = Slice::from(encoded.as_slice());
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

        let mut encoded = Vec::<u8>::new();
        footer.encode_to(&mut encoded as *mut Vec<u8>);

        let mut bytes = encoded;
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
        let mut dst = prefix.as_bytes().to_vec();
        let prefix_bytes = dst.clone();
        let prefix_len   = prefix_bytes.len();

        footer.encode_to(&mut dst as *mut Vec<u8>);

        assert_eq!(
            dst.len(),
            prefix_len + FOOTER_ENCODED_LENGTH,
            "footer_encode_to_preserves_existing_destination_prefix_and_appends_fixed_size_footer: total length must grow by FOOTER_ENCODED_LENGTH"
        );

        let dst_bytes = dst.as_slice();

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

        let mut encoded = Vec::<u8>::new();
        footer.encode_to(&mut encoded as *mut Vec<u8>);

        let bytes = encoded.as_slice();
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
            let null_dst: *mut Vec<u8> = core::ptr::null_mut();
            footer.encode_to(null_dst);
        });

        assert!(
            result.is_err(),
            "footer_encode_to_panics_on_null_destination_pointer: expected panic when called with null destination pointer"
        );
    }
}
