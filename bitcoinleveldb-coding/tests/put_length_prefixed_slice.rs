// ---------------- [ File: bitcoinleveldb-coding/tests/put_length_prefixed_slice.rs ]
use bitcoinleveldb_coding::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

// Length-overflow boundaries are tested through the production conversion
// helper in src/put_length_prefixed_slice.rs. They require no fabricated Slice.

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
