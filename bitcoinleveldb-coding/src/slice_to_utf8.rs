// ---------------- [ File: bitcoinleveldb-coding/src/slice_to_utf8.rs ]
crate::ix!();

pub fn slice_to_utf8(slice: &Slice) -> String {
    unsafe {
        let data: *const u8 = *slice.data();
        let len: usize = *slice.size();
        if data.is_null() || len == 0 {
            trace!(
                len,
                "slice_to_utf8: empty or null-backed slice, returning empty string"
            );
            return String::new();
        }
        let bytes = core::slice::from_raw_parts(data, len);
        let s = String::from_utf8_lossy(bytes).into_owned();
        trace!(
            len,
            preview = %s,
            "slice_to_utf8: converted slice to UTF-8 string"
        );
        s
    }
}
