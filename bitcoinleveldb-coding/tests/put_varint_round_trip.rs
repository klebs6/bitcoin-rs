// ---------------- [ File: bitcoinleveldb-coding/tests/put_varint_round_trip.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn put_varint32_roundtrip_small_set() {
    info!("put_varint32_roundtrip_small_set: start");
    let values: [u32; 5] = [0, 1, 127, 128, 300];

    let mut s = String::new();
    let dst = &mut s as *mut String;
    for &v in &values {
        put_varint32(dst, v);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };

    for &expected in &values {
        unsafe {
            let mut actual: u32 = 0;
            p = get_varint_32ptr(p, limit, &mut actual);
            assert!(
                !p.is_null(),
                "put_varint32_roundtrip_small_set: null decode pointer"
            );
            assert_eq!(
                expected, actual,
                "put_varint32_roundtrip_small_set: mismatch"
            );
        }
    }

    assert_eq!(
        p, limit,
        "put_varint32_roundtrip_small_set: final pointer must equal limit"
    );
    info!("put_varint32_roundtrip_small_set: success");
}

#[traced_test]
fn put_varint64_roundtrip_small_set() {
    info!("put_varint64_roundtrip_small_set: start");
    let values: [u64; 5] = [
        0,
        1,
        127,
        128,
        300,
    ];

    let mut s = String::new();
    let dst = &mut s as *mut String;
    for &v in &values {
        put_varint64(dst, v);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };

    for &expected in &values {
        unsafe {
            let mut actual: u64 = 0;
            p = get_varint_64ptr(p, limit, &mut actual);
            assert!(
                !p.is_null(),
                "put_varint64_roundtrip_small_set: null decode pointer"
            );
            assert_eq!(
                expected, actual,
                "put_varint64_roundtrip_small_set: mismatch"
            );
        }
    }

    assert_eq!(
        p, limit,
        "put_varint64_roundtrip_small_set: final pointer must equal limit"
    );
    info!("put_varint64_roundtrip_small_set: success");
}
