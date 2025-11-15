// ---------------- [ File: bitcoinleveldb-coding/tests/put_length_prefixed_slice.rs ]
use bitcoinleveldb_coding::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn put_length_prefixed_slice_rejects_too_large_length() {
    info!("put_length_prefixed_slice_rejects_too_large_length: start");

    let mut s = String::new();
    let dst = &mut s as *mut String;

    let too_large: usize = (u32::MAX as usize).saturating_add(1);
    let huge_slice =
        Slice::from_ptr_len(core::ptr::null::<u8>(), too_large);

    put_length_prefixed_slice(dst, &huge_slice);

    assert_eq!(
        0,
        s.len(),
        "put_length_prefixed_slice_rejects_too_large_length: buffer should remain empty"
    );

    info!("put_length_prefixed_slice_rejects_too_large_length: success");
}

#[traced_test]
fn put_length_prefixed_slice_roundtrip_single_value() {
    info!("put_length_prefixed_slice_roundtrip_single_value: start");

    let payload = b"abc";

    let slice =
        Slice::from_ptr_len(payload.as_ptr(), payload.len());

    let mut s = String::new();
    let dst = &mut s as *mut String;
    put_length_prefixed_slice(dst, &slice);

    let bytes = s.as_bytes();
    let mut input_slice =
        Slice::from_ptr_len(bytes.as_ptr(), bytes.len());
    let mut out =
        Slice::from_ptr_len(core::ptr::null(), 0);

    assert!(
        get_length_prefixed_slice(&mut input_slice, &mut out),
        "put_length_prefixed_slice_roundtrip_single_value: decode failed"
    );
    assert_eq!(
        "abc".to_string(),
        slice_to_utf8(&out),
        "put_length_prefixed_slice_roundtrip_single_value: payload mismatch"
    );

    info!("put_length_prefixed_slice_roundtrip_single_value: success");
}
