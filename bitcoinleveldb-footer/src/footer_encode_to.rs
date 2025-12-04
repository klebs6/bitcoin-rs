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
                self.metaindex_handle.offset(),
                self.metaindex_handle.size(),
                self.index_handle.offset(),
                self.index_handle.size()
            );

            self.metaindex_handle.encode_to(s);
            self.index_handle.encode_to(s);

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
mod footer_encoding_roundtrip_tests {
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
        let index = make_block_handle(333, 444);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(&metaindex);
        footer.set_index_handle(&index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        assert_eq!(
            encoded.len(),
            FOOTER_ENCODED_LENGTH,
            "footer_encode_decode_roundtrip_preserves_handles_and_consumes_input: encoded length must equal FOOTER_ENCODED_LENGTH"
        );

        let mut slice = Slice::from(encoded.as_bytes());
        let mut decoded = Footer::default();

        let status =
            decoded.decode_from(&mut slice as *mut Slice);
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
        let mut short_buf =
            vec![0u8; FOOTER_ENCODED_LENGTH - 1];
        let mut slice = Slice::from(short_buf.as_slice());
        let mut footer = Footer::default();

        let status =
            footer.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_too_short_input_returns_error: expected non‑OK status for short footer"
        );
    }

    #[traced_test]
    fn footer_decode_from_bad_magic_returns_error() {
        let metaindex = make_block_handle(10, 20);
        let index = make_block_handle(30, 40);

        let mut footer = Footer::default();
        footer.set_metaindex_handle(&metaindex);
        footer.set_index_handle(&index);

        let mut encoded = String::new();
        footer.encode_to(&mut encoded as *mut String);

        let mut bytes = encoded.into_bytes();
        // Flip a bit in the magic number region (last byte).
        let last = bytes.len() - 1;
        bytes[last] ^= 0xFF;

        let mut slice = Slice::from(bytes.as_slice());
        let mut decoded = Footer::default();

        let status =
            decoded.decode_from(&mut slice as *mut Slice);

        assert!(
            !status.is_ok(),
            "footer_decode_from_bad_magic_returns_error: expected non‑OK status for bad magic"
        );
    }
}
