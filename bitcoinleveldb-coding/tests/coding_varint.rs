// ---------------- [ File: bitcoinleveldb-coding/tests/coding_varint.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_varint32() {
    info!("coding_varint32: start");
    let mut s = String::new();
    let dst = &mut s as *mut String;
    let limit_i = 32 * 32;

    for i in 0u32..limit_i {
        let v: u32 = (i / 32) << (i % 32);
        put_varint32(dst, v);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };

    for i in 0u32..limit_i {
        let expected: u32 = (i / 32) << (i % 32);
        unsafe {
            assert!(
                (p as usize) < (limit as usize),
                "coding_varint32: pointer beyond limit at iteration {}",
                i
            );
            let mut actual: u32 = 0;
            let start = p;
            p = get_varint_32ptr(p, limit, &mut actual as *mut u32);
            assert!(
                !p.is_null(),
                "coding_varint32: decoder returned null pointer at iteration {}",
                i
            );
            assert_eq!(
                expected, actual,
                "coding_varint32: mismatch at iteration {}",
                i
            );
            let consumed = p.offset_from(start) as usize;
            assert_eq!(
                varint_length(actual as u64) as usize,
                consumed,
                "coding_varint32: length mismatch at iteration {}",
                i
            );
        }
    }

    assert_eq!(
        p,
        unsafe { bytes.as_ptr().add(bytes.len()) },
        "coding_varint32: final pointer must equal end of buffer"
    );
    info!("coding_varint32: success");
}

#[traced_test]
fn coding_varint64() {
    info!("coding_varint64: start");
    let mut values: Vec<u64> = Vec::new();

    values.push(0);
    values.push(100);
    values.push(!0u64);
    values.push(!0u64 - 1);

    for k in 0u32..64 {
        let power: u64 = 1u64 << k;
        values.push(power);
        values.push(power - 1);
        values.push(power + 1);
    }

    debug!(
        count = values.len(),
        "coding_varint64: generated test values"
    );

    let mut s = String::new();
    let dst = &mut s as *mut String;
    for &v in &values {
        put_varint64(dst, v);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };

    for (i, &expected) in values.iter().enumerate() {
        unsafe {
            assert!(
                (p as usize) < (limit as usize),
                "coding_varint64: pointer beyond limit at index {}",
                i
            );
            let mut actual: u64 = 0;
            let start = p;
            p = get_varint_64ptr(p, limit, &mut actual as *mut u64);
            assert!(
                !p.is_null(),
                "coding_varint64: decoder returned null pointer at index {}",
                i
            );
            assert_eq!(
                expected, actual,
                "coding_varint64: mismatch at index {}",
                i
            );
            let consumed = p.offset_from(start) as usize;
            assert_eq!(
                varint_length(actual) as usize,
                consumed,
                "coding_varint64: length mismatch at index {}",
                i
            );
        }
    }

    assert_eq!(
        p, limit,
        "coding_varint64: final pointer must equal buffer limit"
    );
    info!("coding_varint64: success");
}
