// ---------------- [ File: bitcoinleveldb-memtable/src/encode_key.rs ]
crate::ix!();

/// Encode a suitable internal key target for "target" and return it.
/// 
/// Uses *scratch as scratch space, and the returned pointer will point into
/// this scratch space.
///
pub fn encode_key(
    scratch: *mut String,
    target:  &Slice,
) -> *const u8 {
    trace!(
        "encode_key: scratch_ptr={:?}, target_len={}",
        scratch,
        *target.size()
    );

    assert!(
        !scratch.is_null(),
        "encode_key: scratch pointer must not be null"
    );

    unsafe {
        let s: &mut String = &mut *scratch;
        s.clear();

        let target_len_u32: u32 = (*target.size())
            .try_into()
            .expect("encode_key: target length does not fit into u32");

        let buf: &mut Vec<u8> = s.as_mut_vec();

        // Encode the length prefix as a varint32
        put_varint32_vec(buf, target_len_u32);

        // Append the raw key bytes
        let target_bytes = slice_as_bytes(target);
        buf.extend_from_slice(target_bytes);

        let ptr = buf.as_ptr();
        trace!(
            "encode_key: encoded varint_len={} total_len={}",
            varint_length(target_len_u32.into()),
            buf.len()
        );
        ptr
    }
}
