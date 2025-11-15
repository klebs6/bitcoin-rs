// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_test_key.rs ]
crate::ix!();

#[cfg(test)]
pub(crate) const VERBOSE: i32 = 1;

#[cfg(test)]
fn encode_fixed32_to_bytes(value: u32) -> [u8; 4] {
    trace!(
        value,
        "encode_fixed32_to_bytes: encoding u32 to little-endian bytes"
    );
    value.to_le_bytes()
}

#[cfg(test)]
fn encode_fixed32_into(value: u32, buffer: &mut [u8; 4]) {
    let bytes = encode_fixed32_to_bytes(value);
    buffer.copy_from_slice(&bytes);
}

#[cfg(test)]
pub(crate) fn key(i: i32, buffer: *mut u8) -> Slice {
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
