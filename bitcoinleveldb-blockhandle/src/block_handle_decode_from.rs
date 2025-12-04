// ---------------- [ File: bitcoinleveldb-blockhandle/src/block_handle_decode_from.rs ]
crate::ix!();

impl BlockHandle {

    pub fn decode_from(&mut self, input: *mut Slice) -> crate::Status {
        unsafe {
            assert!(
                !input.is_null(),
                "BlockHandle::decode_from: input pointer is null"
            );

            let slice = &mut *input;

            trace!(
                "BlockHandle::decode_from: input_size_before={}",
                *slice.size()
            );

            let mut offset: u64 = 0;
            let mut size:   u64 = 0;

            if get_varint64_from_slice(slice, &mut offset)
                && get_varint64_from_slice(slice, &mut size)
            {
                trace!(
                    "BlockHandle::decode_from: decoded offset={}, size={}, input_size_after={}",
                    offset,
                    size,
                    *slice.size()
                );

                self.set_offset(offset);
                self.set_size(size);
                Status::ok()
            } else {
                let msg       = b"bad block handle";
                let msg_slice = Slice::from(&msg[..]);

                error!(
                    "BlockHandle::decode_from: failed to decode varint64 pair, input_size_after={}",
                    *slice.size()
                );

                Status::corruption(&msg_slice, None)
            }
        }
    }
}

#[cfg(test)]
mod block_handle_decode_from_edge_case_tests {
    use super::*;

    #[traced_test]
    fn decode_from_does_not_modify_handle_on_truncated_input() {
        let sentinel = !0u64;
        let mut encoded = String::new();

        {
            let mut tmp = BlockHandle::new();
            tmp.set_offset(42);
            tmp.set_size(99);
            tmp.encode_to(&mut encoded as *mut String);
        }

        assert!(
            !encoded.is_empty(),
            "encoded representation should not be empty"
        );
        encoded.truncate(encoded.len() - 1);

        let mut slice = Slice::from(encoded.as_bytes());
        let mut handle = BlockHandle::new();
        let status = handle.decode_from(&mut slice as *mut Slice);

        trace!(
            "decode_from_does_not_modify_handle_on_truncated_input: status_ok={}, offset={}, size={}",
            status.is_ok(),
            handle.offset(),
            handle.size()
        );

        assert!(
            !status.is_ok(),
            "expected non-OK status when decoding from truncated input"
        );
        assert_eq!(
            handle.offset(),
            sentinel,
            "offset should remain sentinel on decode failure"
        );
        assert_eq!(
            handle.size(),
            sentinel,
            "size should remain sentinel on decode failure"
        );
    }

    #[traced_test]
    fn decode_from_leaves_unconsumed_suffix_in_slice() {
        let mut handle = BlockHandle::new();
        handle.set_offset(100);
        handle.set_size(200);

        let mut encoded = String::new();
        handle.encode_to(&mut encoded as *mut String);

        // Append a single extra byte that should remain after decoding.
        encoded.push('\x01');

        let mut slice = Slice::from(encoded.as_bytes());
        let mut decoded = BlockHandle::new();
        let status = decoded.decode_from(&mut slice as *mut Slice);

        trace!(
            "decode_from_leaves_unconsumed_suffix_in_slice: status_ok={}, remaining_size={}",
            status.is_ok(),
            *slice.size()
        );

        assert!(status.is_ok());
        assert_eq!(decoded.offset(), 100);
        assert_eq!(decoded.size(), 200);
        assert_eq!(
            *slice.size(),
            1,
            "expected exactly one byte of suffix to remain after decoding"
        );
    }
}
