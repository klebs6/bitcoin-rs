// ---------------- [ File: bitcoinleveldb-memtable/src/encode_key.rs ]
crate::ix!();

/// Encode a suitable internal key target for "target" and return it.
/// 
/// The byte-vector scratch owns the returned varint32-length-prefixed bytes.
/// The pointer remains valid until scratch is modified or dropped; target may
/// borrow its previous allocation because replacement is copied first.
///
pub fn encode_key(
    scratch: *mut Vec<u8>,
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

    let target_len_u32: u32 = (*target.size())
        .try_into()
        .expect("encode_key: target length does not fit into u32");

    // Build the complete replacement before mutating scratch: target may be a
    // borrowed view of its previous buffer. This encoding is binary, not text.
    let mut encoded = Vec::new();
    put_varint32_vec(&mut encoded, target_len_u32);
    encoded.extend_from_slice(slice_as_bytes(target));

    unsafe {
        *scratch = encoded;
        let ptr = (*scratch).as_ptr();
        trace!(
            "encode_key: encoded varint_len={} total_len={}",
            varint_length(target_len_u32.into()),
            (*scratch).len()
        );
        ptr
    }
}
