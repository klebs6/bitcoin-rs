// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_test_key.rs ]
crate::ix!();

pub const VERBOSE: i32 = 1;

pub fn encode_fixed32_to_bytes(value: u32) -> [u8; 4] {
    trace!(
        value,
        "encode_fixed32_to_bytes: encoding u32 to little-endian bytes"
    );
    value.to_le_bytes()
}

pub fn encode_fixed32_into(value: u32, buffer: &mut [u8; 4]) {
    let bytes = encode_fixed32_to_bytes(value);
    buffer.copy_from_slice(&bytes);
}

pub fn key(i: i32, buffer: *mut u8) -> Slice {
    debug!(
        i,
        raw_buffer_ptr = ?buffer,
        "key(): constructing Slice key from encoded i32"
    );

    let encoded = encode_fixed32_to_bytes(i as u32);

    if buffer.is_null() {
        error!(
            "key(): buffer pointer is null; returning empty Slice via null pointer/zero length"
        );
        unsafe { Slice::from_ptr_len(std::ptr::null(), 0) }
    } else {
        unsafe {
            std::ptr::copy_nonoverlapping(encoded.as_ptr(), buffer, encoded.len());
            Slice::from_ptr_len(buffer as *const u8, encoded.len())
        }
    }
}

#[cfg(test)]
mod bloom_test_key_encoding_suite {
    use super::*;

    #[traced_test]
    fn encode_fixed32_to_bytes_produces_little_endian_encoding() {
        let value: u32 = 0x0102_0304;
        let bytes = encode_fixed32_to_bytes(value);

        info!(
            value = format!("0x{value:08x}"),
            encoded = ?bytes,
            "encode_fixed32_to_bytes_produces_little_endian_encoding"
        );

        assert_eq!(bytes, [0x04, 0x03, 0x02, 0x01]);
    }

    #[traced_test]
    fn encode_fixed32_into_writes_correct_bytes_into_buffer() {
        let value: u32 = 0x0a0b_0c0d;
        let mut buffer = [0u8; 4];

        encode_fixed32_into(value, &mut buffer);

        let expected = encode_fixed32_to_bytes(value);
        assert_eq!(buffer, expected);
    }

    #[traced_test]
    fn key_constructs_slice_pointing_to_encoded_value() {
        let value: i32 = 123456789;
        let mut buffer = [0u8; 4];

        let slice = key(value, buffer.as_mut_ptr());

        let data_ptr = *slice.data();
        let len = *slice.size();
        assert_eq!(len, 4);

        let slice_bytes =
            unsafe { std::slice::from_raw_parts(data_ptr, len) };

        let expected = encode_fixed32_to_bytes(value as u32);

        info!(
            value,
            slice_bytes = ?slice_bytes,
            expected = ?expected,
            "key_constructs_slice_pointing_to_encoded_value"
        );

        assert_eq!(slice_bytes, &buffer[..]);
        assert_eq!(slice_bytes, &expected[..]);
    }
}
