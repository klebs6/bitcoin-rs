// ---------------- [ File: bitcoinleveldb-coding/tests/get_varint_fast_path.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn get_varint32_single_byte_fast_path() {
    info!("get_varint32_single_byte_fast_path: start");
    let buf = [0x7fu8];
    let base = buf.as_ptr();
    let limit = unsafe { base.add(buf.len()) };
    let mut value: u32 = 0;
    let p = unsafe { get_varint_32ptr(base, limit, &mut value) };
    assert!(!p.is_null());
    assert_eq!(unsafe { base.add(1) }, p);
    assert_eq!(0x7fu32, value);
    info!("get_varint32_single_byte_fast_path: success");
}

#[traced_test]
fn get_varint64_single_byte_fast_path() {
    info!("get_varint64_single_byte_fast_path: start");
    let buf = [0x7fu8];
    let base = buf.as_ptr();
    let limit = unsafe { base.add(buf.len()) };
    let mut value: u64 = 0;
    let p = unsafe { get_varint_64ptr(base, limit, &mut value) };
    assert!(!p.is_null());
    assert_eq!(unsafe { base.add(1) }, p);
    assert_eq!(0x7fu64, value);
    info!("get_varint64_single_byte_fast_path: success");
}
