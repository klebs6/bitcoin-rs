// ---------------- [ File: bitcoin-crc32c/src/read_le_unittest.rs ]
use bitcoin_crc32c::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_read_le_unittest.cc]

#[traced_test]
fn verifies_read_uint32le() {
    let bytes: [u8; 4] = [0x78, 0x56, 0x34, 0x12]; // LE 0x12345678
    let value = unsafe { read_uint32le(bytes.as_ptr()) };
    assert_eq!(0x1234_5678, value);
}

#[traced_test]
fn verifies_read_uint64le() {
    let bytes: [u8; 8] =
        [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]; // LE 0x123456789ABCDEF0
    let value = unsafe { read_uint64le(bytes.as_ptr()) };
    assert_eq!(0x1234_5678_9ABC_DEF0, value);
}


#[traced_test]
fn crc_32c_read_le_test_uint32le() {
    // little‑endian 0x12345678
    let bytes: [u8; 4] = [0x78, 0x56, 0x34, 0x12];

    // ASSERT_EQ(RoundUp<4>(bytes), bytes)  << "Stack array is not aligned";
    let ptr = bytes.as_ptr();
    assert_eq!(round_up::<4>(ptr), ptr, "Stack array is not aligned");

    let value = unsafe { read_uint32le(ptr) };
    assert_eq!(0x1234_5678, value);
}

#[traced_test]
fn crc_32c_read_le_test_uint64le() {
    // little‑endian 0x123456789ABCDEF0
    let bytes: [u8; 8] =
        [0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12];

    let ptr = bytes.as_ptr();
    assert_eq!(round_up::<8>(ptr), ptr, "Stack array is not aligned");

    let value = unsafe { read_uint64le(ptr) };
    assert_eq!(0x1234_5678_9ABC_DEF0, value);
}
