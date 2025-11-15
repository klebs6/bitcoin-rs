// ---------------- [ File: bitcoinleveldb-coding/tests/encode_varint_roundtrip.rs ]
use bitcoinleveldb_coding::*;
use bitcoin_imports::*;

#[traced_test]
fn encode_varint32_roundtrip_and_length() {
    info!("encode_varint32_roundtrip_and_length: start");
    let values: [u32; 7] = [
        0,
        1,
        127,
        128,
        255,
        300,
        u32::MAX,
    ];

    for &v in &values {
        let mut buf = [0u8; 5];
        let start = buf.as_mut_ptr();
        let end = unsafe { encode_varint32(start, v) };
        let encoded_len = unsafe { end.offset_from(start) as usize };
        assert!(
            (1..=5).contains(&encoded_len),
            "encode_varint32_roundtrip_and_length: invalid length {} for {}",
            encoded_len,
            v
        );
        assert_eq!(
            varint_length(v as u64) as usize,
            encoded_len,
            "encode_varint32_roundtrip_and_length: length mismatch for {}",
            v
        );

        let mut decoded: u32 = 0;
        let limit = unsafe { start.add(encoded_len) } as *const u8;
        let p = unsafe {
            get_varint_32ptr(start as *const u8, limit, &mut decoded)
        };
        assert!(
            !p.is_null(),
            "encode_varint32_roundtrip_and_length: null pointer for {}",
            v
        );
        assert_eq!(
            limit, p,
            "encode_varint32_roundtrip_and_length: pointer mismatch for {}",
            v
        );
        assert_eq!(
            v, decoded,
            "encode_varint32_roundtrip_and_length: decode mismatch for {}",
            v
        );
    }

    info!("encode_varint32_roundtrip_and_length: success");
}

#[traced_test]
fn encode_varint64_roundtrip_and_length() {
    info!("encode_varint64_roundtrip_and_length: start");
    let values: [u64; 8] = [
        0,
        1,
        127,
        128,
        255,
        300,
        (1u64 << 63) - 1,
        u64::MAX,
    ];

    for &v in &values {
        let mut buf = [0u8; 10];
        let start = buf.as_mut_ptr();
        let end = unsafe { encode_varint64(start, v) };
        let encoded_len = unsafe { end.offset_from(start) as usize };
        assert!(
            (1..=10).contains(&encoded_len),
            "encode_varint64_roundtrip_and_length: invalid length {} for {}",
            encoded_len,
            v
        );
        assert_eq!(
            varint_length(v) as usize,
            encoded_len,
            "encode_varint64_roundtrip_and_length: length mismatch for {}",
            v
        );

        let mut decoded: u64 = 0;
        let limit = unsafe { start.add(encoded_len) } as *const u8;
        let p = unsafe {
            get_varint_64ptr(start as *const u8, limit, &mut decoded)
        };
        assert!(
            !p.is_null(),
            "encode_varint64_roundtrip_and_length: null pointer for {}",
            v
        );
        assert_eq!(
            limit, p,
            "encode_varint64_roundtrip_and_length: pointer mismatch for {}",
            v
        );
        assert_eq!(
            v, decoded,
            "encode_varint64_roundtrip_and_length: decode mismatch for {}",
            v
        );
    }

    info!("encode_varint64_roundtrip_and_length: success");
}
