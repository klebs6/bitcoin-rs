// ---------------- [ File: bitcoinleveldb-coding/tests/coding_encoding.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn coding_encoding_output() {
    info!("coding_encoding_output: start");

    let mut dst32 = String::new();
    let dst32_ptr = &mut dst32 as *mut String;
    put_fixed32(dst32_ptr, 0x0403_0201);
    let bytes32 = dst32.as_bytes();
    assert_eq!(4, bytes32.len(), "coding_encoding_output: fixed32 length");
    assert_eq!(0x01u8, bytes32[0]);
    assert_eq!(0x02u8, bytes32[1]);
    assert_eq!(0x03u8, bytes32[2]);
    assert_eq!(0x04u8, bytes32[3]);
    debug!("coding_encoding_output: fixed32 encoding validated");

    let mut dst64 = String::new();
    let dst64_ptr = &mut dst64 as *mut String;
    put_fixed64(dst64_ptr, 0x0807_0605_0403_0201u64);
    let bytes64 = dst64.as_bytes();
    assert_eq!(8, bytes64.len(), "coding_encoding_output: fixed64 length");
    assert_eq!(0x01u8, bytes64[0]);
    assert_eq!(0x02u8, bytes64[1]);
    assert_eq!(0x03u8, bytes64[2]);
    assert_eq!(0x04u8, bytes64[3]);
    assert_eq!(0x05u8, bytes64[4]);
    assert_eq!(0x06u8, bytes64[5]);
    assert_eq!(0x07u8, bytes64[6]);
    assert_eq!(0x08u8, bytes64[7]);
    debug!("coding_encoding_output: fixed64 encoding validated");

    info!("coding_encoding_output: success");
}
