// ---------------- [ File: bitcoinleveldb-coding/src/put_length_prefixed_slice.rs ]
crate::ix!();

// Check the representable prefix before inspecting or copying payload bytes.
// Keeping this conversion separate lets its boundaries be tested with lengths
// alone, without constructing an invalid Slice or allocating a giant buffer.
fn checked_length_prefix(len: usize) -> Option<u32> {
    u32::try_from(len).ok()
}

/// Append an exact byte payload with its varint32 length.
///
/// Returns false without changing `dst` when the length cannot be represented.
/// The Slice must remain readable for this call; it may refer into `dst`.
pub fn put_length_prefixed_slice_bytes(dst: &mut Vec<u8>, value: &Slice) -> bool {
    let prefix = match checked_length_prefix(*value.size()) {
        Some(prefix) => prefix,
        None => return false,
    };
    // Copy before appending the prefix: the Slice may borrow the old allocation,
    // and even the first prefix byte can cause the destination to reallocate.
    let payload = value.as_bytes().to_vec();
    put_varint32_bytes(dst, prefix);
    dst.extend_from_slice(&payload);
    true
}

/// Append a readable Slice to a live byte-vector sink, preserving all bytes.
/// A length above u32::MAX leaves the destination unchanged, as before.
/// The value may view the destination's existing allocation.
pub fn put_length_prefixed_slice(dst: *mut Vec<u8>, value: &Slice) {
    assert!(!dst.is_null(), "put_length_prefixed_slice: dst pointer is null");
    // The byte helper checks the length before any payload access and takes a
    // snapshot before appending the prefix, so a source view survives growth.
    let appended = put_length_prefixed_slice_bytes(unsafe { &mut *dst }, value);
    if !appended {
        error!(
            len = *value.size(),
            "put_length_prefixed_slice: slice length exceeds u32::MAX"
        );
    }
}

#[cfg(test)]
mod length_prefix_boundaries {
    use super::checked_length_prefix;

    #[test]
    fn zero_length_is_representable() {
        assert_eq!(checked_length_prefix(0), Some(0));
    }

    #[test]
    fn maximum_u32_length_is_representable() {
        assert_eq!(checked_length_prefix(u32::MAX as usize), Some(u32::MAX));
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    fn oversized_length_is_rejected_before_any_payload_access() {
        assert_eq!(checked_length_prefix(u32::MAX as usize + 1), None);
        assert_eq!(checked_length_prefix(usize::MAX), None);
    }
}
