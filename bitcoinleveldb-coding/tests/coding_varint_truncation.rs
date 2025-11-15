// ---------------- [ File: bitcoinleveldb-coding/tests/coding_varint_truncation.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_varint_32truncation() {
    info!("coding_varint_32truncation: start");
    let large_value: u32 = (1u32 << 31) + 100;
    let mut s = String::new();
    let dst = &mut s as *mut String;
    put_varint32(dst, large_value);

    let bytes = s.as_bytes();
    let base = bytes.as_ptr();
    let size = bytes.len();

    for len in 0..(size - 1) {
        unsafe {
            let mut result: u32 = 0;
            let limit = base.add(len);
            let decoded = get_varint_32ptr(base, limit, &mut result as *mut u32);
            assert!(
                decoded.is_null(),
                "coding_varint_32truncation: expected null at truncated length {}",
                len
            );
        }
    }

    unsafe {
        let mut result: u32 = 0;
        let decoded =
            get_varint_32ptr(base, base.add(size), &mut result as *mut u32);
        assert!(
            !decoded.is_null(),
            "coding_varint_32truncation: expected success at full length"
        );
        assert_eq!(
            large_value, result,
            "coding_varint_32truncation: decoded value mismatch"
        );
    }

    info!("coding_varint_32truncation: success");
}

#[traced_test]
fn coding_varint_64truncation() {
    info!("coding_varint_64truncation: start");
    let large_value: u64 = (1u64 << 63) + 100u64;
    let mut s = String::new();
    let dst = &mut s as *mut String;
    put_varint64(dst, large_value);

    let bytes = s.as_bytes();
    let base = bytes.as_ptr();
    let size = bytes.len();

    for len in 0..(size - 1) {
        unsafe {
            let mut result: u64 = 0;
            let limit = base.add(len);
            let decoded = get_varint_64ptr(base, limit, &mut result as *mut u64);
            assert!(
                decoded.is_null(),
                "coding_varint_64truncation: expected null at truncated length {}",
                len
            );
        }
    }

    unsafe {
        let mut result: u64 = 0;
        let decoded =
            get_varint_64ptr(base, base.add(size), &mut result as *mut u64);
        assert!(
            !decoded.is_null(),
            "coding_varint_64truncation: expected success at full length"
        );
        assert_eq!(
            large_value, result,
            "coding_varint_64truncation: decoded value mismatch"
        );
    }

    info!("coding_varint_64truncation: success");
}
