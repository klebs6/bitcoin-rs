// ---------------- [ File: bitcoinleveldb-coding/tests/coding_varint_overflow.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_varint_32overflow() {
    info!("coding_varint_32overflow: start");
    let input: [u8; 6] = [0x81, 0x82, 0x83, 0x84, 0x85, 0x11];
    let p = input.as_ptr();
    let limit = unsafe { p.add(input.len()) };
    let mut result: u32 = 0;
    let decoded = unsafe { get_varint_32ptr(p, limit, &mut result as *mut u32) };
    assert!(
        decoded.is_null(),
        "coding_varint_32overflow: expected null pointer for overflow input"
    );
    info!("coding_varint_32overflow: success");
}

#[traced_test]
fn coding_varint_64overflow() {
    info!("coding_varint_64overflow: start");
    let input: [u8; 11] = [
        0x81, 0x82, 0x83, 0x84, 0x85, 0x81, 0x82, 0x83, 0x84, 0x85, 0x11,
    ];
    let p = input.as_ptr();
    let limit = unsafe { p.add(input.len()) };
    let mut result: u64 = 0;
    let decoded = unsafe { get_varint_64ptr(p, limit, &mut result as *mut u64) };
    assert!(
        decoded.is_null(),
        "coding_varint_64overflow: expected null pointer for overflow input"
    );
    info!("coding_varint_64overflow: success");
}
