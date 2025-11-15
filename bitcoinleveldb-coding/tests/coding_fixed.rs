// ---------------- [ File: bitcoinleveldb-coding/tests/coding_fixed.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_fixed32() {
    info!("coding_fixed32: start");
    let mut s = String::new();
    let dst = &mut s as *mut String;

    for v in 0u32..100_000 {
        put_fixed32(dst, v);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };
    let mut expected: u32 = 0;

    while expected < 100_000 {
        unsafe {
            assert!(
                (p as usize) + core::mem::size_of::<u32>() <= (limit as usize),
                "coding_fixed32: pointer advanced beyond limit"
            );
            let actual = decode_fixed32(p);
            assert_eq!(
                expected, actual,
                "coding_fixed32: mismatch at value {}",
                expected
            );
            p = p.add(core::mem::size_of::<u32>());
        }
        expected += 1;
    }

    assert_eq!(
        p, limit,
        "coding_fixed32: final pointer must equal buffer limit"
    );
    info!("coding_fixed32: success");
}

#[traced_test]
fn coding_fixed64() {
    info!("coding_fixed64: start");
    let mut s = String::new();
    let dst = &mut s as *mut String;

    for power in 0..=63 {
        let v: u64 = 1u64 << power;
        put_fixed64(dst, v - 1);
        put_fixed64(dst, v);
        put_fixed64(dst, v + 1);
    }

    let bytes = s.as_bytes();
    let mut p = bytes.as_ptr();
    let limit = unsafe { p.add(bytes.len()) };

    for power in 0..=63 {
        let v: u64 = 1u64 << power;
        unsafe {
            assert!(
                (p as usize) + 3 * core::mem::size_of::<u64>() <= (limit as usize),
                "coding_fixed64: buffer too short while decoding at power {}",
                power
            );
            let actual1 = decode_fixed64(p);
            assert_eq!(
                v - 1,
                actual1,
                "coding_fixed64: v-1 mismatch at power {}",
                power
            );
            p = p.add(core::mem::size_of::<u64>());

            let actual2 = decode_fixed64(p);
            assert_eq!(v, actual2, "coding_fixed64: v mismatch at power {}", power);
            p = p.add(core::mem::size_of::<u64>());

            let actual3 = decode_fixed64(p);
            assert_eq!(
                v + 1,
                actual3,
                "coding_fixed64: v+1 mismatch at power {}",
                power
            );
            p = p.add(core::mem::size_of::<u64>());
        }
    }

    assert_eq!(
        p, limit,
        "coding_fixed64: final pointer must equal buffer limit"
    );
    info!("coding_fixed64: success");
}
