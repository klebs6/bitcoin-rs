// ---------------- [ File: bitcoin-crc32c/tests/capi.rs ]
use bitcoin_crc32c::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_capi_unittest.c]

const SAMPLE48: [u8; 48] = [
    0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
    0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/* From rfc3720 section B.4. */
#[traced_test]
fn capi_vectors_and_extend() {
    unsafe {
        /* 32 × 0x00 */
        let zeros = [0u8; 32];
        assert_eq!(crc32c_value(zeros.as_ptr(), 32), 0x8a91_36aa);

        /* 32 × 0xFF */
        let ones  = [0xFFu8; 32];
        assert_eq!(crc32c_value(ones.as_ptr(), 32), 0x62a8_ab43);

        /* 0,1,…,31 */
        let mut asc = [0u8; 32];
        asc.iter_mut().enumerate().for_each(|(i, b)| *b = i as u8);
        assert_eq!(crc32c_value(asc.as_ptr(), 32), 0x46dd_794e);

        /* 31,30,…,0 */
        asc.iter_mut()
            .enumerate()
            .for_each(|(i, b)| *b = (31 - i) as u8);
        assert_eq!(crc32c_value(asc.as_ptr(), 32), 0x113f_db5c);

        /* 48‑byte sample */
        assert_eq!(crc32c_value(SAMPLE48.as_ptr(), 48), 0xd996_3a56);

        /* incremental “hello world” */
        let hello = b"hello ";
        let world = b"world";
        let hello_crc = crc32c_value(hello.as_ptr(), hello.len());
        let full_crc =
            crc32c_extend(hello_crc, world.as_ptr(), world.len());
        assert_eq!(
            crc32c_value(b"hello world".as_ptr(), 11),
            full_crc
        );
    }
}
