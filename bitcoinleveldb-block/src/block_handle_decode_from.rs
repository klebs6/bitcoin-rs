// ---------------- [ File: bitcoinleveldb-block/src/block_handle_decode_from.rs ]
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
