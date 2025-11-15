// ---------------- [ File: bitcoinleveldb-coding/tests/length_prefixed_slice_limit.rs ]
use bitcoinleveldb_coding::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn get_length_prefixed_slice_with_limit_roundtrip() {
    info!("get_length_prefixed_slice_with_limit_roundtrip: start");

    let payload = b"hello world";
    let len_u32 = payload.len() as u32;

    let mut buf = [0u8; 32];
    let start = buf.as_mut_ptr();
    let end = unsafe { encode_varint32(start, len_u32) };
    let header_len = unsafe { end.offset_from(start) as usize };

    buf[header_len..header_len + payload.len()].copy_from_slice(payload);

    let base = buf.as_ptr();
    let limit = unsafe { base.add(header_len + payload.len()) };

    let mut out = Slice::from_ptr_len(core::ptr::null(), 0);
    let next =
        unsafe { get_length_prefixed_slice_with_limit(base, limit, &mut out) };

    assert!(
        !next.is_null(),
        "get_length_prefixed_slice_with_limit_roundtrip: null pointer"
    );
    assert_eq!(
        limit, next,
        "get_length_prefixed_slice_with_limit_roundtrip: pointer mismatch"
    );
    assert_eq!(
        "hello world".to_string(),
        slice_to_utf8(&out),
        "get_length_prefixed_slice_with_limit_roundtrip: payload mismatch"
    );

    info!("get_length_prefixed_slice_with_limit_roundtrip: success");
}

#[traced_test]
fn get_length_prefixed_slice_with_limit_truncation() {
    info!("get_length_prefixed_slice_with_limit_truncation: start");

    let payload = b"world";
    let len_u32 = payload.len() as u32;

    let mut buf = [0u8; 32];
    let start = buf.as_mut_ptr();
    let end = unsafe { encode_varint32(start, len_u32) };
    let header_len = unsafe { end.offset_from(start) as usize };

    buf[header_len..header_len + payload.len()].copy_from_slice(payload);

    let base = buf.as_ptr();
    let full_limit = unsafe { base.add(header_len + payload.len()) };
    let truncated_limit = unsafe { base.add(header_len + payload.len() - 1) };

    let mut out = Slice::from_ptr_len(core::ptr::null(), 0);
    let bad =
        unsafe { get_length_prefixed_slice_with_limit(base, truncated_limit, &mut out) };
    assert!(
        bad.is_null(),
        "get_length_prefixed_slice_with_limit_truncation: expected null for truncated input"
    );

    let good =
        unsafe { get_length_prefixed_slice_with_limit(base, full_limit, &mut out) };
    assert!(
        !good.is_null(),
        "get_length_prefixed_slice_with_limit_truncation: full buffer should succeed"
    );
    assert_eq!(
        full_limit, good,
        "get_length_prefixed_slice_with_limit_truncation: pointer mismatch on full decode"
    );
    assert_eq!(
        "world".to_string(),
        slice_to_utf8(&out),
        "get_length_prefixed_slice_with_limit_truncation: payload mismatch"
    );

    info!("get_length_prefixed_slice_with_limit_truncation: success");
}
