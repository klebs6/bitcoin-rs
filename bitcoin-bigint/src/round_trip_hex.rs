// ---------------- [ File: bitcoin-bigint/src/round_trip_hex.rs ]
crate::ix!();

pub fn round_trip_hex_32(val: &BaseUInt32) -> BaseUInt32 {
    let hex_string = val.get_hex();
    BaseUInt32::from(hex_string.as_str())
}

pub fn round_trip_hex_64(val: &BaseUInt64) -> BaseUInt64 {
    let hex_string = val.get_hex();
    BaseUInt64::from(hex_string.as_str())
}

pub fn round_trip_hex_256(val: &BaseUInt256) -> BaseUInt256 {
    let hex_string = val.get_hex();
    BaseUInt256::from(hex_string.as_str())
}
