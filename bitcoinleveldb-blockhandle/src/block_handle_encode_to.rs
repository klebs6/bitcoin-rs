// ---------------- [ File: bitcoinleveldb-blockhandle/src/block_handle_encode_to.rs ]
crate::ix!();
    
impl BlockHandle {

    pub fn encode_to(&self, dst: *mut String) {
        unsafe {
            assert!(
                !dst.is_null(),
                "BlockHandle::encode_to: dst pointer is null"
            );

            let sentinel = !0u64;
            let offset   = self.offset();
            let size     = self.size();

            assert!(
                offset != sentinel && size != sentinel,
                "BlockHandle::encode_to: fields not fully initialized (offset={}, size={})",
                offset,
                size
            );

            let s = &mut *dst;
            trace!(
                "BlockHandle::encode_to: offset={}, size={}, dst_len_before={}",
                offset,
                size,
                s.len()
            );

            put_varint64_to_string(s, offset);
            put_varint64_to_string(s, size);

            trace!(
                "BlockHandle::encode_to: dst_len_after={}",
                s.len()
            );
        }
    }
}

#[cfg(test)]
mod block_handle_encode_decode_roundtrip_tests {
    use super::*;

    #[traced_test]
    fn encode_to_and_decode_from_round_trip_successfully() {
        let mut handle = BlockHandle::new();
        handle.set_offset(12345);
        handle.set_size(67890);

        let mut dst = String::new();
        handle.encode_to(&mut dst as *mut String);

        trace!("encoded BlockHandle into {} bytes", dst.len());

        let mut slice   = Slice::from(dst.as_bytes());
        let mut decoded = BlockHandle::new();
        let status      = decoded.decode_from(&mut slice as *mut Slice);

        debug!(
            "decoded BlockHandle: offset={}, size={}, remaining_input_size={}, status_ok={}",
            decoded.offset(),
            decoded.size(),
            *slice.size(),
            status.is_ok()
        );

        assert!(status.is_ok());
        assert_eq!(decoded.offset(), 12345);
        assert_eq!(decoded.size(), 67890);
        assert_eq!(*slice.size(), 0);
    }

    #[traced_test]
    fn decode_from_reports_corruption_on_invalid_varint() {
        let mut encoded = String::new();
        encoded.push('\u{0080}');

        let mut slice  = Slice::from(encoded.as_bytes());
        let mut handle = BlockHandle::new();

        trace!(
            "decoding BlockHandle from invalid varint, input_size={}",
            *slice.size()
        );

        let status = handle.decode_from(&mut slice as *mut Slice);
        assert!(
            !status.is_ok(),
            "expected non-OK status when decoding from invalid varint"
        );
    }

    #[test]
    #[should_panic(
        expected = "BlockHandle::encode_to: fields not fully initialized"
    )]
    fn encode_to_panics_when_fields_are_uninitialized() {
        let handle = BlockHandle::new();
        let mut dst = String::new();

        trace!(
            "encode_to_panics_when_fields_are_uninitialized: invoking encode_to with sentinel values"
        );

        // This should panic because the handle still contains sentinel values.
        handle.encode_to(&mut dst as *mut String);
    }

    #[traced_test]
    fn encode_to_and_decode_from_two_handles_back_to_back() {
        let mut handle1 = BlockHandle::new();
        handle1.set_offset(100);
        handle1.set_size(50);

        let mut handle2 = BlockHandle::new();
        handle2.set_offset(1_000);
        handle2.set_size(250);

        let mut dst = String::new();
        handle1.encode_to(&mut dst as *mut String);
        handle2.encode_to(&mut dst as *mut String);

        trace!(
            "encode_to_and_decode_from_two_handles_back_to_back: total_encoded_len={}",
            dst.len()
        );

        let mut slice = Slice::from(dst.as_bytes());

        let mut decoded1 = BlockHandle::new();
        let status1      = decoded1.decode_from(&mut slice as *mut Slice);
        debug!(
            "decoded first BlockHandle: offset={}, size={}, remaining_input_size={}, status_ok={}",
            decoded1.offset(),
            decoded1.size(),
            *slice.size(),
            status1.is_ok()
        );
        assert!(status1.is_ok());
        assert_eq!(decoded1.offset(), 100);
        assert_eq!(decoded1.size(), 50);

        let mut decoded2 = BlockHandle::new();
        let status2      = decoded2.decode_from(&mut slice as *mut Slice);
        debug!(
            "decoded second BlockHandle: offset={}, size={}, remaining_input_size={}, status_ok={}",
            decoded2.offset(),
            decoded2.size(),
            *slice.size(),
            status2.is_ok()
        );
        assert!(status2.is_ok());
        assert_eq!(decoded2.offset(), 1_000);
        assert_eq!(decoded2.size(), 250);

        assert_eq!(
            *slice.size(),
            0,
            "expected all bytes to be consumed after decoding two handles"
        );
    }
}
