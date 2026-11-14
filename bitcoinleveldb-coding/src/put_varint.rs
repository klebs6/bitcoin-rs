// ---------------- [ File: bitcoinleveldb-coding/src/put_varint.rs ]
crate::ix!();

/// Append the standard base-128 encoding to a byte owner.
pub fn put_varint32_bytes(dst: &mut Vec<u8>, mut value: u32) {
    while value >= 0x80 {
        dst.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    dst.push(value as u8);
}

/// Append the standard base-128 encoding to a byte owner.
pub fn put_varint64_bytes(dst: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        dst.push((value as u8 & 0x7f) | 0x80);
        value >>= 7;
    }
    dst.push(value as u8);
}

/// Append a base-128 value to a live byte-vector sink.
pub fn put_varint32(dst: *mut Vec<u8>, value: u32) {
    assert!(!dst.is_null(), "put_varint32: dst pointer is null");
    // SAFETY: the caller supplies an exclusively accessible live Vec<u8>.
    put_varint32_bytes(unsafe { &mut *dst }, value);
}

/// Append a base-128 value to a live byte-vector sink.
pub fn put_varint64(dst: *mut Vec<u8>, value: u64) {
    assert!(!dst.is_null(), "put_varint64: dst pointer is null");
    // SAFETY: the caller supplies an exclusively accessible live Vec<u8>.
    put_varint64_bytes(unsafe { &mut *dst }, value);
}
