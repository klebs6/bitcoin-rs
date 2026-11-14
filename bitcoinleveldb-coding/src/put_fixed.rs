// ---------------- [ File: bitcoinleveldb-coding/src/put_fixed.rs ]
crate::ix!();

/// Append the four little-endian bytes without requiring UTF-8 storage.
pub fn put_fixed32_bytes(dst: &mut Vec<u8>, value: u32) {
    dst.extend_from_slice(&value.to_le_bytes());
}

/// Append the eight little-endian bytes without requiring UTF-8 storage.
pub fn put_fixed64_bytes(dst: &mut Vec<u8>, value: u64) {
    dst.extend_from_slice(&value.to_le_bytes());
}

/// Append a fixed-width little-endian value to a live byte-vector sink.
pub fn put_fixed32(dst: *mut Vec<u8>, value: u32) {
    assert!(!dst.is_null(), "put_fixed32: dst pointer is null");
    // SAFETY: the caller supplies an exclusively accessible live Vec<u8>.
    put_fixed32_bytes(unsafe { &mut *dst }, value);
}

/// Append a fixed-width little-endian value to a live byte-vector sink.
pub fn put_fixed64(dst: *mut Vec<u8>, value: u64) {
    assert!(!dst.is_null(), "put_fixed64: dst pointer is null");
    // SAFETY: the caller supplies an exclusively accessible live Vec<u8>.
    put_fixed64_bytes(unsafe { &mut *dst }, value);
}
