// ---------------- [ File: bitcoinleveldb-coding/src/put_length_prefixed_slice.rs ]
crate::ix!();

// Check the representable prefix before inspecting or copying payload bytes.
// Keeping this conversion separate lets its boundaries be tested with lengths
// alone, without constructing an invalid Slice or allocating a giant buffer.
fn checked_length_prefix(len: usize) -> Option<u32> {
    u32::try_from(len).ok()
}

pub fn put_length_prefixed_slice(dst: *mut String, value: &Slice) {
    unsafe {
        let len: usize = *value.size();
        trace!(
            len,
            ptr = ?dst,
            "put_length_prefixed_slice: appending length-prefixed slice"
        );
        let prefix = match checked_length_prefix(len) {
            Some(prefix) => prefix,
            None => {
                error!(
                    len,
                    "put_length_prefixed_slice: slice length exceeds u32::MAX"
                );
                return;
            }
        };

        put_varint32(dst, prefix);

        if len == 0 {
            debug!(
                "put_length_prefixed_slice: empty slice, nothing more to append"
            );
            return;
        }

        let data_ptr: *const u8 = *value.data();
        let s: &mut String = &mut *dst;
        let vec = s.as_mut_vec();
        let bytes = core::slice::from_raw_parts(data_ptr, len);
        vec.extend_from_slice(bytes);
        debug!(
            new_len = vec.len(),
            "put_length_prefixed_slice: buffer length after append"
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
