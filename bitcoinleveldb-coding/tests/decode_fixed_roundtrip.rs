// ---------------- [ File: bitcoinleveldb-coding/tests/decode_fixed_roundtrip.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn decode_fixed32_known_pattern() {
    info!("decode_fixed32_known_pattern: start");
    let bytes = [0x01u8, 0x02, 0x03, 0x04];
    let value = unsafe { decode_fixed32(bytes.as_ptr()) };
    assert_eq!(
        0x0403_0201u32,
        value,
        "decode_fixed32_known_pattern: little-endian decode mismatch"
    );
    info!("decode_fixed32_known_pattern: success");
}

#[traced_test]
fn decode_fixed64_known_pattern() {
    info!("decode_fixed64_known_pattern: start");
    let bytes = [0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let value = unsafe { decode_fixed64(bytes.as_ptr()) };
    assert_eq!(
        0x0807_0605_0403_0201u64,
        value,
        "decode_fixed64_known_pattern: little-endian decode mismatch"
    );
    info!("decode_fixed64_known_pattern: success");
}
