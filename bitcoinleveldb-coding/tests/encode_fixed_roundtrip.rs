// ---------------- [ File: bitcoinleveldb-coding/tests/encode_fixed_roundtrip.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn encode_fixed32_little_endian_layout() {
    info!("encode_fixed32_little_endian_layout: start");
    let mut buf = [0u8; core::mem::size_of::<u32>()];
    let value: u32 = 0x0403_0201;
    unsafe { encode_fixed32(buf.as_mut_ptr(), value) };
    assert_eq!([0x01u8, 0x02, 0x03, 0x04], buf);
    info!("encode_fixed32_little_endian_layout: success");
}

#[traced_test]
fn encode_fixed64_little_endian_layout() {
    info!("encode_fixed64_little_endian_layout: start");
    let mut buf = [0u8; core::mem::size_of::<u64>()];
    let value: u64 = 0x0807_0605_0403_0201u64;
    unsafe { encode_fixed64(buf.as_mut_ptr(), value) };
    assert_eq!([0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08], buf);
    info!("encode_fixed64_little_endian_layout: success");
}
