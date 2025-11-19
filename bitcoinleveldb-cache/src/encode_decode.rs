// ---------------- [ File: bitcoinleveldb-cache/src/encode_decode.rs ]
crate::ix!();

pub fn encode_key(k: i32) -> Vec<u8> {
    // Little-endian fixed32 encoding, matching PutFixed32
    let v = k as u32;
    let b0 = (v & 0xff) as u8;
    let b1 = ((v >> 8) & 0xff) as u8;
    let b2 = ((v >> 16) & 0xff) as u8;
    let b3 = ((v >> 24) & 0xff) as u8;
    vec![b0, b1, b2, b3]
}

pub fn decode_key(k: &Slice) -> i32 {
    unsafe {
        assert_eq!(*k.size(), 4, "decode_key expects 4-byte key");
        let data = k.data();
        assert!(!data.is_null(), "decode_key got null data pointer");
        let bytes = std::slice::from_raw_parts(*data as *const u8, 4);
        let v0 = bytes[0] as u32;
        let v1 = (bytes[1] as u32) << 8;
        let v2 = (bytes[2] as u32) << 16;
        let v3 = (bytes[3] as u32) << 24;
        let v = v0 | v1 | v2 | v3;
        v as i32
    }
}

/// Legacy helper used only by tests; production code should
/// treat cache values as opaque `*mut c_void`.
///
/// Here, `v` is expected to be a pointer to `CacheTestValue`
/// created by `CacheTest::insert*`.
pub fn decode_value(v: *mut c_void) -> i32 {
    unsafe {
        if v.is_null() {
            error!("decode_value: called with null value pointer");
            0
        } else {
            let payload = v as *mut crate::cache_test::CacheTestValue;
            let logical = *(*payload).value();
            trace!("decode_value: {:?} -> {}", v, logical);
            logical
        }
    }
}

/// Kept for completeness; not used by the current tests. It still
/// performs a simple integer->pointer reinterpretation, matching
/// the original minimal helper semantics.
pub fn encode_value(v: uintptr_t) -> *mut c_void {
    let ptr = v as usize as *mut c_void;
    trace!("encode_value: v={} -> {:?}", v, ptr);
    ptr
}
